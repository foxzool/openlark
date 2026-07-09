//! 批量查询员工请假记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/leave/leave_request_history>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 批量查询员工请假记录请求
#[derive(Debug, Clone)]
pub struct LeaveRequestHistoryRequest {
    config: Config,
    body: Option<Value>,
    query_params: Vec<(String, String)>,
}

impl LeaveRequestHistoryRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: None,
            query_params: Vec::new(),
        }
    }

    /// 设置请求体。
    pub fn body(mut self, body: Value) -> Self {
        self.body = Some(body);
        self
    }

    /// 追加查询参数。
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<LeaveRequestHistoryResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<LeaveRequestHistoryResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        let api_endpoint = FeishuPeopleApiV1::LeaveLeaveRequestHistory;
        let mut request = ApiRequest::<LeaveRequestHistoryResponse>::get(api_endpoint.to_url());
        for (key, value) in self.query_params {
            request = request.query(&key, value);
        }
        if let Some(body) = self.body {
            request = request.body(body);
        }

        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "批量查询员工请假记录响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 批量查询员工请假记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LeaveRequestHistoryResponse {
    /// 响应数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<LeaveRequestHistoryInfo>,
}

/// 请假记录信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LeaveRequestHistoryInfo {
    /// 请假记录条目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<LeaveRequestHistoryItem>>,
    /// 分页令牌
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 透传的扩展字段
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

/// 请假记录条目
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LeaveRequestHistoryItem {
    /// 透传的扩展字段
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

impl ApiResponseTrait for LeaveRequestHistoryResponse {
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

    /// 端到端：GET /open-apis/corehr/v1/leaves/leave_request_history
    #[tokio::test]
    async fn test_leave_history_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v1/leaves/leave_request_history"))
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

        LeaveRequestHistoryRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
