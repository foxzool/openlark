//! 导入打卡流水
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/user_flow/batch_create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, validate_required_list,
};

use super::models::{BatchCreateUserFlowRequestBody, BatchCreateUserFlowResponse, UserFlowRecord};

/// 导入打卡流水请求
#[derive(Debug, Clone)]
pub struct BatchCreateUserFlowRequest {
    /// 打卡流水记录列表
    flow_records: Vec<UserFlowRecord>,
    /// 配置信息
    config: Config,
}

impl BatchCreateUserFlowRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            flow_records: Vec::new(),
            config,
        }
    }

    /// 添加打卡流水记录
    pub fn add_flow_record(mut self, record: UserFlowRecord) -> Self {
        self.flow_records.push(record);
        self
    }

    /// 设置打卡流水记录列表
    pub fn flow_records(mut self, records: Vec<UserFlowRecord>) -> Self {
        self.flow_records = records;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchCreateUserFlowResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchCreateUserFlowResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required_list!(
            self.flow_records,
            100,
            "打卡流水记录列表不能为空且不能超过 100 个"
        );
        for (idx, record) in self.flow_records.iter().enumerate() {
            validate_required!(
                record.user_id.trim(),
                &format!("第 {} 条记录的用户 ID 不能为空", idx + 1)
            );
            validate_required!(
                record.punch_time.trim(),
                &format!("第 {} 条记录的打卡时间不能为空", idx + 1)
            );
        }

        // 2. 构建端点
        let api_endpoint = AttendanceApiV1::UserFlowBatchCreate;
        let request = ApiRequest::<BatchCreateUserFlowResponse>::post(api_endpoint.to_url());

        // 3. 序列化请求体
        let request_body = BatchCreateUserFlowRequestBody {
            flow_records: self.flow_records,
        };
        let request = request.body(serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?);

        // 4. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 5. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "导入打卡流水响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

impl ApiResponseTrait for BatchCreateUserFlowResponse {
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
    fn test_batch_create_user_flow_request_builder_new() {
        let request = BatchCreateUserFlowRequest::new(TestConfigBuilder::new().build());
        let _ = request;
    }

    #[test]
    fn test_batch_create_user_flow_request_validation_fails_on_default_request() {
        let request = BatchCreateUserFlowRequest::new(TestConfigBuilder::new().build());
        let rt = tokio::runtime::Runtime::new().expect("创建 tokio runtime 失败");
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
    }
    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_attendance_v1_user_flow_batch_create_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value = serde_json::from_str(r#"{"results": []}"#).unwrap();
        Mock::given(method("POST"))
            .and(path("/open-apis/attendance/v1/user_flows/batch_create"))
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

        let data = BatchCreateUserFlowRequest::new(config)
            .add_flow_record(UserFlowRecord {
                user_id: "user_id_1".to_string(),
                punch_time: "2024-01-01 09:00:00".to_string(),
                punch_type: 1,
                punch_method: 1,
                punch_place_name: None,
                punch_place_id: None,
                longitude: None,
                latitude: None,
                wifi_name: None,
                wifi_mac: None,
                device_id: None,
                device_name: None,
                remark: None,
                photo_list: None,
                out_address: None,
                out_remark: None,
            })
            .execute()
            .await
            .expect("attendance_v1_user_flow_batch_create 应成功");

        let _ = &data;

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/attendance/v1/user_flows/batch_create"
        );
    }
}
