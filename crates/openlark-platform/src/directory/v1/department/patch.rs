//! 更新部门
//!
//! 文档: <https://open.feishu.cn/document/directory-v1/department/patch>
//! docPath: <https://open.feishu.cn/document/directory-v1/department/patch>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 更新部门 Builder
#[derive(Debug, Clone)]
pub struct DepartmentPatchRequestBuilder {
    config: Config,
    /// 部门 ID
    department_id: String,
    /// 部门名称
    name: Option<String>,
    /// 父部门 ID
    parent_id: Option<String>,
    /// 部门负责人 ID
    leader_user_id: Option<String>,
}

impl DepartmentPatchRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, department_id: impl Into<String>) -> Self {
        Self {
            config,
            department_id: department_id.into(),
            name: None,
            parent_id: None,
            leader_user_id: None,
        }
    }

    /// 设置部门名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置父部门 ID
    pub fn parent_id(mut self, parent_id: impl Into<String>) -> Self {
        self.parent_id = Some(parent_id.into());
        self
    }

    /// 设置部门负责人 ID
    pub fn leader_user_id(mut self, leader_user_id: impl Into<String>) -> Self {
        self.leader_user_id = Some(leader_user_id.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DepartmentPatchResponse> {
        let url = format!("/open-apis/directory/v1/departments/{}", self.department_id);

        let request = DepartmentPatchRequest {
            name: self.name,
            parent_id: self.parent_id,
            leader_user_id: self.leader_user_id,
        };

        let req: ApiRequest<DepartmentPatchResponse> =
            ApiRequest::patch(&url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(RequestOption::default())).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DepartmentPatchResponse> {
        let url = format!("/open-apis/directory/v1/departments/{}", self.department_id);

        let request = DepartmentPatchRequest {
            name: self.name,
            parent_id: self.parent_id,
            leader_user_id: self.leader_user_id,
        };

        let req: ApiRequest<DepartmentPatchResponse> =
            ApiRequest::patch(&url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 更新部门请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct DepartmentPatchRequest {
    /// 部门名称
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    /// 父部门 ID
    #[serde(rename = "parent_id", skip_serializing_if = "Option::is_none")]
    parent_id: Option<String>,
    /// 部门负责人 ID
    #[serde(rename = "leader_user_id", skip_serializing_if = "Option::is_none")]
    leader_user_id: Option<String>,
}

/// 更新部门响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DepartmentPatchResponse {
    /// 部门 ID
    #[serde(rename = "department_id")]
    pub department_id: String,
    /// 更新后的名称
    #[serde(rename = "name")]
    pub name: String,
    /// 更新时间
    #[serde(rename = "updated_at")]
    pub updated_at: i64,
}

impl ApiResponseTrait for DepartmentPatchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to DepartmentPatchRequestBuilder, will be removed in v1.0 (#271)")]
pub type DepartmentPatchBuilder = DepartmentPatchRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：PATCH .../directory/v1/departments/{id} → 强类型 DepartmentPatchResponse。
    #[tokio::test]
    async fn test_patch_department_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/directory/v1/departments/dept_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "department_id": "dept_001",
                    "name": "Engineering",
                    "updated_at": 1700000000
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = DepartmentPatchRequestBuilder::new(config, "dept_001")
            .name("Engineering")
            .parent_id("dept_000")
            .execute()
            .await
            .expect("更新部门应成功");
        assert_eq!(resp.department_id, "dept_001");
        assert_eq!(resp.name, "Engineering");
        assert_eq!(resp.updated_at, 1700000000);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/directory/v1/departments/dept_001"
        );
        assert_eq!(received[0].method, "PATCH");
    }
}
