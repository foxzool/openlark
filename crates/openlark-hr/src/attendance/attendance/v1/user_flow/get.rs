//! 获取打卡流水
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/user_flow/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};

use super::models::GetUserFlowResponse;

/// 获取打卡流水请求
#[derive(Debug, Clone)]
pub struct GetUserFlowRequest {
    /// 打卡流水 ID
    user_flow_id: String,
    /// 用户 ID 类型，可选值：open_id、union_id、user_id
    user_id_type: Option<String>,
    /// 配置信息
    config: Config,
}

impl GetUserFlowRequest {
    /// 创建获取打卡流水请求
    pub fn new(config: Config) -> Self {
        Self {
            user_flow_id: String::new(),
            user_id_type: None,
            config,
        }
    }

    /// 设置打卡流水 ID（必填）
    pub fn user_flow_id(mut self, user_flow_id: String) -> Self {
        self.user_flow_id = user_flow_id;
        self
    }

    /// 设置用户 ID 类型（可选）
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetUserFlowResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetUserFlowResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.user_flow_id.trim(), "打卡流水 ID 不能为空");

        // 2. 构建端点
        let api_endpoint = AttendanceApiV1::UserFlowGet
            .to_url()
            .replace("{}", &self.user_flow_id);
        let mut request = ApiRequest::<GetUserFlowResponse>::get(&api_endpoint);

        // 3. 添加查询参数（可选）
        if let Some(ref user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        // 4. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取打卡流水响应数据为空",
        )
        .await
    }
}

impl ApiResponseTrait for GetUserFlowResponse {
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
    fn test_get_user_flow_request_builder_new() {
        let request = GetUserFlowRequest::new(TestConfigBuilder::new().build())
            .user_flow_id("test".to_string());
        let _ = request;
    }

    #[test]
    fn test_get_user_flow_request_validation_fails_on_default_request() {
        let request = GetUserFlowRequest::new(TestConfigBuilder::new().build());
        let rt = tokio::runtime::Runtime::new().expect("创建 tokio runtime 失败");
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
    }
    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_attendance_v1_user_flow_get_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value =
            serde_json::from_str(r#"{"flow_info": {"user_flow_id": "test", "user_id": "test", "punch_date": "test", "punch_time": "test", "punch_type": 0, "punch_method": 0}}"#).unwrap();
        Mock::given(method("GET"))
            .and(path("/open-apis/attendance/v1/user_flows/user_flow_001"))
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

        let data = GetUserFlowRequest::new(config)
            .user_flow_id("user_flow_001".to_string())
            .execute()
            .await
            .expect("attendance_v1_user_flow_get 应成功");

        let _ = &data;

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/attendance/v1/user_flows/user_flow_001"
        );
    }
}
