//! 获取父部门信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/department/parents>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};

use serde::{Deserialize, Serialize};

/// 获取父部门信息请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ParentsRequest {
    /// 部门 ID
    pub department_id: String,
}

impl ParentsRequest {
    /// 创建请求
    pub fn new(department_id: String) -> Self {
        Self { department_id }
    }
}

/// 获取父部门信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParentsResponse {
    /// 父部门列表
    pub data: Option<ParentsResponseData>,
}

/// `ParentsResponseData`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ParentsResponseData {
    /// 父部门列表（按层级从顶部门到直接父部门排序）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DepartmentItem>>,
}

/// `DepartmentItem`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DepartmentItem {
    /// 部门 ID
    pub id: String,
    /// 部门名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 父部门 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// 部门编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

impl ApiResponseTrait for ParentsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取父部门信息请求构建器
#[derive(Debug, Clone)]
pub struct ParentsRequestBuilder {
    config: Config,
    department_id: String,
}

impl ParentsRequestBuilder {
    /// 创建请求构建器
    pub fn new(config: Config, department_id: String) -> Self {
        Self {
            config,
            department_id,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ParentsResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ParentsResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        // 构建端点
        let api_endpoint = FeishuPeopleApiV2::DepartmentParents(self.department_id.clone());
        let request = ApiRequest::<ParentsResponse>::post(api_endpoint.to_url()).body(
            serde_json::to_value(ParentsRequest::new(self.department_id)).map_err(|e| {
                openlark_core::error::validation_error(
                    "获取父部门信息请求序列化失败",
                    e.to_string(),
                )
            })?,
        );

        // 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取父部门信息响应数据为空",
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v2/departments/parents
    #[tokio::test]
    async fn test_parents_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/departments/parents"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": {  }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        ParentsRequestBuilder::new(config, "test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
