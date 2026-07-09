//! 按 ID 查询班次
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/shift/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};

use super::models::GetShiftResponse;

/// 按 ID 查询班次请求
#[derive(Debug, Clone)]
pub struct GetShiftRequest {
    /// 班次 ID（必填）
    shift_id: String,
    /// 配置信息
    config: Config,
}

impl GetShiftRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            shift_id: String::new(),
            config,
        }
    }

    /// 设置班次 ID（必填）
    pub fn shift_id(mut self, shift_id: String) -> Self {
        self.shift_id = shift_id;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetShiftResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetShiftResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.shift_id.trim(), "班次 ID 不能为空");

        // 2. 构建端点
        let api_endpoint = AttendanceApiV1::ShiftGet(self.shift_id.clone());
        let request = ApiRequest::<GetShiftResponse>::get(api_endpoint.to_url());

        // 3. 发送请求（GET 无请求体）
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 4. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "获取班次响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

impl ApiResponseTrait for GetShiftResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use openlark_core::config::Config;
    use openlark_core::testing::prelude::TestConfigBuilder;

    #[test]
    fn test_get_shift_request_builder_new() {
        let request =
            GetShiftRequest::new(TestConfigBuilder::new().build()).shift_id("test".to_string());
        let _ = request;
    }

    #[test]
    fn test_get_shift_request_validation_fails_on_default_request() {
        let request = GetShiftRequest::new(TestConfigBuilder::new().build());
        let rt = tokio::runtime::Runtime::new().expect("创建 tokio runtime 失败");
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
    }
    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_attendance_v1_shift_get_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value =
            serde_json::from_str(r#"{"shift": {"shift_id": "test", "shift_name": "test", "shift_type": 0, "punch_times": []}}"#).unwrap();
        Mock::given(method("GET"))
            .and(path("/open-apis/attendance/v1/shifts/shift_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": data_body
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = GetShiftRequest::new(config)
            .shift_id("shift_001".to_string())
            .execute()
            .await
            .expect("attendance_v1_shift_get 应成功");

        let _ = &data;

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/attendance/v1/shifts/shift_001"
        );
    }
}
