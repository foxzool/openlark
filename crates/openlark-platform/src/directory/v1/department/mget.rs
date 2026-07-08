//! 批量获取部门信息
//!
//! 文档: <https://open.feishu.cn/document/directory-v1/department/mget>
//! docPath: <https://open.feishu.cn/document/directory-v1/department/mget>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 批量获取部门信息 Builder
#[derive(Debug, Clone)]
pub struct DepartmentMgetRequestBuilder {
    config: Config,
    /// 部门 ID 列表
    department_ids: Vec<String>,
}

impl DepartmentMgetRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config) -> Self {
        Self {
            config,
            department_ids: Vec::new(),
        }
    }

    /// 添加部门 ID
    pub fn department_id(mut self, department_id: impl Into<String>) -> Self {
        self.department_ids.push(department_id.into());
        self
    }

    /// 添加多个部门 ID
    pub fn department_ids(
        mut self,
        department_ids: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.department_ids
            .extend(department_ids.into_iter().map(Into::into));
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DepartmentMgetResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DepartmentMgetResponse> {
        let url = "/open-apis/directory/v1/departments/mget".to_string();

        let request = DepartmentMgetRequest {
            department_ids: self.department_ids,
        };

        let req: ApiRequest<DepartmentMgetResponse> =
            ApiRequest::post(&url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 批量获取部门信息请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct DepartmentMgetRequest {
    /// 部门 ID 列表
    #[serde(rename = "department_ids")]
    department_ids: Vec<String>,
}

/// 部门信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DepartmentInfo {
    /// 部门 ID
    #[serde(rename = "department_id")]
    department_id: String,
    /// 部门名称
    #[serde(rename = "name")]
    name: String,
    /// 父部门 ID
    #[serde(rename = "parent_id", skip_serializing_if = "Option::is_none")]
    parent_id: Option<String>,
    /// 部门负责人 ID
    #[serde(rename = "leader_user_id", skip_serializing_if = "Option::is_none")]
    leader_user_id: Option<String>,
    /// 排序序号
    #[serde(rename = "order", skip_serializing_if = "Option::is_none")]
    order: Option<u32>,
}

/// 批量获取部门信息响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DepartmentMgetResponse {
    /// 部门信息列表
    #[serde(rename = "items")]
    pub items: Vec<DepartmentInfo>,
}

impl ApiResponseTrait for DepartmentMgetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to DepartmentMgetRequestBuilder, will be removed in v1.0 (#271)")]
pub type DepartmentMgetBuilder = DepartmentMgetRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../directory/v1/departments/mget → 强类型 DepartmentMgetResponse。
    #[tokio::test]
    async fn test_mget_department_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/directory/v1/departments/mget"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "department_id": "dept_001",
                            "name": "Engineering",
                            "parent_id": "dept_000",
                            "leader_user_id": "u_001",
                            "order": 1
                        }
                    ]
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

        let resp = DepartmentMgetRequestBuilder::new(config)
            .department_id("dept_001")
            .execute()
            .await
            .expect("批量获取部门信息应成功");
        assert_eq!(resp.items.len(), 1);
        assert_eq!(resp.items[0].department_id, "dept_001");
        assert_eq!(resp.items[0].name, "Engineering");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/directory/v1/departments/mget"
        );
        assert_eq!(received[0].method, "POST");
    }
}
