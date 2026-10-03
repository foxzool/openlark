//! 获取预约
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/reserve/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::VcApiV1;
use crate::common::api_utils::validate_required_field;

/// 获取预约请求

#[derive(Debug, Clone)]
pub struct GetReserveRequest {
    /// 配置信息
    config: Config,
    /// 预约 ID（路径参数）
    reserve_id: String,
    /// 查询参数
    query_params: Vec<(String, String)>,
}

/// 获取预约响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetReserveResponse {
    /// 预约 ID
    pub reserve_id: String,
    /// 会议 ID
    pub meeting_id: String,
    /// 预约主题
    pub topic: String,
    /// 开始时间
    pub start_time: String,
    /// 结束时间
    pub end_time: String,
}

impl ApiResponseTrait for GetReserveResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetReserveRequest {
    /// 创建新的请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            reserve_id: String::new(),
            query_params: Vec::new(),
        }
    }

    /// 设置预约 ID（路径参数）
    pub fn reserve_id(mut self, reserve_id: impl Into<String>) -> Self {
        self.reserve_id = reserve_id.into();
        self
    }

    /// 追加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/reserve/get>
    pub async fn execute(self) -> SDKResult<GetReserveResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetReserveResponse> {
        validate_required_field("reserve_id", Some(&self.reserve_id), "预约 ID 不能为空")?;

        let api_endpoint = VcApiV1::ReserveGet(self.reserve_id.clone());
        let mut api_request: ApiRequest<GetReserveResponse> =
            ApiRequest::get(api_endpoint.to_url());

        for (key, value) in self.query_params {
            api_request = api_request.query(key, value);
        }

        Transport::request_typed(api_request, &self.config, Some(option), "获取预约").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    /// 端到端：GET .../vc/v1/reserves/{reserve_id} → 强类型 GetReserveResponse 解析（单层 data 信封）。
    #[tokio::test]
    async fn test_get_reserve_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/vc/v1/reserves/r_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "reserve_id": "r_001",
                    "meeting_id": "m_001",
                    "topic": "周会",
                    "start_time": "2026-07-08 10:00",
                    "end_time": "2026-07-08 11:00"
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

        let resp = GetReserveRequest::new(config)
            .reserve_id("r_001")
            .execute()
            .await
            .expect("获取预约应成功");
        assert_eq!(resp.reserve_id, "r_001");
        assert_eq!(resp.meeting_id, "m_001");
        assert_eq!(resp.topic, "周会");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/vc/v1/reserves/r_001");
    }
}
