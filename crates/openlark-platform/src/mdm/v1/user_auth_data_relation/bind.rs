//! 用户数据维度绑定
//!
//! 文档: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/mdm-v1/user_auth_data_relation/bind>
//! docPath: <https://open.feishu.cn/document/server-docs/mdm-v1/user_auth_data_relation/bind>
//!
//! 请求体四个字段全部必填；成功响应 `data` 为空对象 `{}`。

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 用户数据维度绑定 Builder
#[derive(Debug, Clone)]
pub struct UserAuthDataRelationBindRequestBuilder {
    config: Config,
    root_dimension_type: String,
    sub_dimension_types: Vec<String>,
    authorized_user_ids: Vec<String>,
    uams_app_id: String,
    /// 用户 ID 类型（查询参数，可选：open_id/union_id）
    user_id_type: Option<String>,
}

impl UserAuthDataRelationBindRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config) -> Self {
        Self {
            config,
            root_dimension_type: String::new(),
            sub_dimension_types: Vec::new(),
            authorized_user_ids: Vec::new(),
            uams_app_id: String::new(),
            user_id_type: None,
        }
    }

    /// 设置数据类型编码（必填，长度 1~200，示例 "gongsi"）
    pub fn root_dimension_type(mut self, root_dimension_type: impl Into<String>) -> Self {
        self.root_dimension_type = root_dimension_type.into();
        self
    }

    /// 设置数据编码列表（必填，示例 ["code_dimension"]）
    pub fn sub_dimension_types(
        mut self,
        sub_dimension_types: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.sub_dimension_types = sub_dimension_types.into_iter().map(Into::into).collect();
        self
    }

    /// 设置授权人的 lark ID 列表（必填）
    pub fn authorized_user_ids(
        mut self,
        authorized_user_ids: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.authorized_user_ids = authorized_user_ids.into_iter().map(Into::into).collect();
        self
    }

    /// 设置 uams 系统中应用 ID（必填，示例 "uams-tenant-test"）
    pub fn uams_app_id(mut self, uams_app_id: impl Into<String>) -> Self {
        self.uams_app_id = uams_app_id.into();
        self
    }

    /// 设置用户 ID 类型（查询参数，可选：open_id/union_id，示例 "open_id"）
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UserAuthDataRelationBindResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<UserAuthDataRelationBindResponse> {
        validate_required!(self.root_dimension_type, "root_dimension_type 不能为空");
        validate_required!(self.sub_dimension_types, "sub_dimension_types 不能为空");
        validate_required!(self.authorized_user_ids, "authorized_user_ids 不能为空");
        validate_required!(self.uams_app_id, "uams_app_id 不能为空");

        let url = "/open-apis/mdm/v1/user_auth_data_relations/bind";

        let request = UserAuthDataRelationBindRequest {
            root_dimension_type: self.root_dimension_type,
            sub_dimension_types: self.sub_dimension_types,
            authorized_user_ids: self.authorized_user_ids,
            uams_app_id: self.uams_app_id,
        };

        let req: ApiRequest<UserAuthDataRelationBindResponse> = ApiRequest::post(url)
            .body(serde_json::to_value(&request)?)
            .query_opt("user_id_type", self.user_id_type);
        Transport::request_typed(req, &self.config, Some(option), "用户数据维度绑定").await
    }
}

/// 用户数据维度绑定请求体（四个字段全部必填）
#[derive(Debug, Clone, Deserialize, Serialize)]
struct UserAuthDataRelationBindRequest {
    /// 数据类型编码（长度 1~200）
    root_dimension_type: String,
    /// 数据编码列表
    sub_dimension_types: Vec<String>,
    /// 授权人的 lark ID 列表
    authorized_user_ids: Vec<String>,
    /// uams 系统中应用 ID
    uams_app_id: String,
}

/// 用户数据维度绑定响应 `data`（空对象）。
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct UserAuthDataRelationBindResponse {}

impl ApiResponseTrait for UserAuthDataRelationBindResponse {}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(
    note = "renamed to UserAuthDataRelationBindRequestBuilder, will be removed in v1.0 (#271)"
)]
pub type UserAuthDataRelationBindBuilder = UserAuthDataRelationBindRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{body_json, method, path, query_param},
    };

    /// 端到端：POST 官方四必填字段请求体 → 成功响应 data 为空对象。
    #[tokio::test]
    async fn test_bind_sends_official_body_and_returns_empty_data() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/mdm/v1/user_auth_data_relations/bind"))
            .and(query_param("user_id_type", "open_id"))
            .and(body_json(json!({
                "root_dimension_type": "gongsi",
                "sub_dimension_types": ["code_dimension"],
                "authorized_user_ids": ["ou_7dab8a3d3cdcc9da365777c7ad535d62"],
                "uams_app_id": "uams-tenant-test"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {}
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = UserAuthDataRelationBindRequestBuilder::new(config)
            .root_dimension_type("gongsi")
            .sub_dimension_types(vec!["code_dimension".to_string()])
            .authorized_user_ids(vec!["ou_7dab8a3d3cdcc9da365777c7ad535d62".to_string()])
            .uams_app_id("uams-tenant-test")
            .user_id_type("open_id")
            .execute()
            .await
            .expect("用户数据维度绑定应成功");
        let _ = resp;

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/mdm/v1/user_auth_data_relations/bind"
        );
    }

    /// 缺少必填字段时应在发请求前快速失败。
    #[tokio::test]
    async fn test_bind_rejects_missing_required_fields() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build();

        let err = UserAuthDataRelationBindRequestBuilder::new(config)
            .execute()
            .await
            .expect_err("缺少必填字段应返回错误");
        assert!(err.to_string().contains("root_dimension_type"));
    }
}
