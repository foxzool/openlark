//! 删除部门
//!
//! 文档: <https://open.feishu.cn/document/directory-v1/department/delete>
//! docPath: <https://open.feishu.cn/document/directory-v1/department/delete>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 删除部门 Builder
#[derive(Debug, Clone)]
pub struct DepartmentDeleteRequestBuilder {
    config: Config,
    /// 部门 ID
    department_id: String,
}

impl DepartmentDeleteRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, department_id: impl Into<String>) -> Self {
        Self {
            config,
            department_id: department_id.into(),
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DepartmentDeleteResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DepartmentDeleteResponse> {
        let url = format!("/open-apis/directory/v1/departments/{}", self.department_id);

        let req: ApiRequest<DepartmentDeleteResponse> = ApiRequest::delete(&url);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("删除部门", "响应数据为空"))
    }
}

/// 删除部门响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DepartmentDeleteResponse {
    /// 部门 ID
    #[serde(rename = "department_id")]
    pub department_id: String,
    /// 结果消息
    #[serde(rename = "message")]
    pub message: String,
}

impl ApiResponseTrait for DepartmentDeleteResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to DepartmentDeleteRequestBuilder, will be removed in v1.0 (#271)")]
pub type DepartmentDeleteBuilder = DepartmentDeleteRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：DELETE .../directory/v1/departments/{id} → 强类型 DepartmentDeleteResponse。
    #[tokio::test]
    async fn test_delete_department_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/open-apis/directory/v1/departments/dept_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "department_id": "dept_001",
                    "message": "deleted"
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

        let resp = DepartmentDeleteRequestBuilder::new(config, "dept_001")
            .execute()
            .await
            .expect("删除部门应成功");
        assert_eq!(resp.department_id, "dept_001");
        assert_eq!(resp.message, "deleted");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/directory/v1/departments/dept_001"
        );
        assert_eq!(received[0].method, "DELETE");
    }
}
