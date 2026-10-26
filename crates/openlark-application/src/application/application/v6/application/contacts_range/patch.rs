//! 更新应用通讯录权限范围配置
//! docPath: <https://open.feishu.cn/document/server-docs/application-v6/application/patch>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 通讯录权限范围成员/部门/用户组 id 列表（`app_contacts_range_id_list`）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AppContactsRangeIdList {
    /// 成员 id 列表（类型由 user_id_type 指定，最多 100 个）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    /// 部门 id 列表（类型由 department_id_type 指定，最多 100 个）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    /// 用户组列表（最多 100 个）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
}

/// 更新应用通讯录权限范围配置的请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PatchApplicationContactsRangeBody {
    /// 更新范围方式：equal_to_availability-与应用可用性一致 / some-部分成员 / all-全部成员。
    pub contacts_range_type: String,
    /// 通讯录权限范围新增列表（仅 contacts_range_type=some 时生效，增量更新）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_visible_list: Option<AppContactsRangeIdList>,
    /// 从通讯录权限范围删除的列表（仅 contacts_range_type=some 时生效，增量更新）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub del_visible_list: Option<AppContactsRangeIdList>,
}

impl PatchApplicationContactsRangeBody {
    fn validate(&self) -> SDKResult<()> {
        validate_required!(self.contacts_range_type, "contacts_range_type 不能为空");
        Ok(())
    }
}

/// 更新应用通讯录权限范围配置的请求。
#[derive(Debug, Clone)]
pub struct PatchApplicationContactsRangeRequest {
    config: Arc<Config>,
    app_id: String,
    /// 查询参数 `user_id_type`（可选，用户 ID 类型，默认 open_id）。
    user_id_type: Option<String>,
    /// 查询参数 `department_id_type`（可选，部门 ID 类型，默认 open_department_id）。
    department_id_type: Option<String>,
    body: PatchApplicationContactsRangeBody,
}

/// 更新应用通讯录权限范围配置的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchApplicationContactsRangeResponse {
    /// 响应数据。
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for PatchApplicationContactsRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl PatchApplicationContactsRangeRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>, app_id: impl Into<String>) -> Self {
        Self {
            config,
            app_id: app_id.into(),
            user_id_type: None,
            department_id_type: None,
            body: PatchApplicationContactsRangeBody::default(),
        }
    }

    /// 设置用户 ID 类型（可选，默认 open_id）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置部门 ID 类型（可选，默认 open_department_id）。
    pub fn department_id_type(mut self, department_id_type: impl Into<String>) -> Self {
        self.department_id_type = Some(department_id_type.into());
        self
    }

    /// 设置更新范围方式（必填：equal_to_availability / some / all）。
    pub fn contacts_range_type(mut self, contacts_range_type: impl Into<String>) -> Self {
        self.body.contacts_range_type = contacts_range_type.into();
        self
    }

    /// 设置通讯录权限范围新增列表（可选，仅 contacts_range_type=some 时生效）。
    pub fn add_visible_list(mut self, add_visible_list: AppContactsRangeIdList) -> Self {
        self.body.add_visible_list = Some(add_visible_list);
        self
    }

    /// 设置从通讯录权限范围删除的列表（可选，仅 contacts_range_type=some 时生效）。
    pub fn del_visible_list(mut self, del_visible_list: AppContactsRangeIdList) -> Self {
        self.body.del_visible_list = Some(del_visible_list);
        self
    }

    /// 执行更新应用通讯录权限范围配置请求。
    pub async fn execute(self) -> SDKResult<PatchApplicationContactsRangeResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<PatchApplicationContactsRangeResponse> {
        self.body.validate()?;
        let body = serde_json::to_value(&self.body)?;
        let path = format!(
            "/open-apis/application/v6/applications/{}/contacts_range",
            self.app_id
        );
        let req: ApiRequest<PatchApplicationContactsRangeResponse> = ApiRequest::patch(&path)
            .query_opt("user_id_type", self.user_id_type)
            .query_opt("department_id_type", self.department_id_type)
            .body(body);

        Transport::request_typed(
            req,
            &self.config,
            Some(option),
            "更新应用通讯录权限范围配置",
        )
        .await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：PATCH .../applications/{app_id}/contacts_range → 强类型
    /// PatchApplicationContactsRangeResponse 解析（双层 data 信封）。
    #[tokio::test]
    async fn test_patch_contacts_range_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path(
                "/open-apis/application/v6/applications/cli_test_app/contacts_range",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "data": { "updated": true } }
            })))
            .mount(&server)
            .await;

        let config = Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        let resp = PatchApplicationContactsRangeRequest::new(config, "cli_test_app")
            .contacts_range_type("some")
            .add_visible_list(AppContactsRangeIdList {
                user_ids: Some(vec!["ou_7dab8a3d3cdcc9da365777c7ad535d62".to_string()]),
                department_ids: None,
                group_ids: Some(vec!["g12334".to_string()]),
            })
            .execute()
            .await
            .expect("更新应用通讯录权限范围配置应成功");
        assert_eq!(resp.data.unwrap()["updated"], true);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/application/v6/applications/cli_test_app/contacts_range"
        );
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["contacts_range_type"], "some");
        assert_eq!(
            sent["add_visible_list"]["user_ids"][0],
            "ou_7dab8a3d3cdcc9da365777c7ad535d62"
        );
        assert_eq!(sent["add_visible_list"]["group_ids"][0], "g12334");
        assert!(sent.get("del_visible_list").is_none());
    }
}
