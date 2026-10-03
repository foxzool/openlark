//! 删除工时制度
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/working_hours_type/delete>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 删除工时制度请求
#[derive(Debug, Clone)]
pub struct DeleteRequest {
    /// 配置信息
    config: Config,
    /// 工时制度 ID（必填）
    working_hours_type_id: String,
}

impl DeleteRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            working_hours_type_id: String::new(),
        }
    }

    /// 设置工时制度 ID（必填）
    pub fn working_hours_type_id(mut self, working_hours_type_id: String) -> Self {
        self.working_hours_type_id = working_hours_type_id;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DeleteResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteResponse> {
        use crate::common::api_endpoints::CorehrApiV1;

        validate_required!(self.working_hours_type_id.trim(), "工时制度 ID 不能为空");

        let api_endpoint = CorehrApiV1::WorkingHoursTypeDelete(self.working_hours_type_id);
        let endpoint_url = api_endpoint.to_url();
        validate_required!(endpoint_url.as_str(), "API 端点不能为空");

        let request = ApiRequest::<DeleteResponse>::delete(endpoint_url);
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "删除工时制度响应数据为空",
        )
        .await
    }
}

/// 删除工时制度响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteResponse {
    /// 响应数据
    /// 操作结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<WorkingHoursTypeItem>,
}

/// 工时制度条目
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WorkingHoursTypeItem {
    /// 透传的扩展字段
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

impl ApiResponseTrait for DeleteResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：DELETE /open-apis/corehr/v1/working_hours_types/test001
    #[tokio::test]
    async fn test_delete_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path("/open-apis/corehr/v1/working_hours_types/test001"))
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

        DeleteRequest::new(config)
            .working_hours_type_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
