//! 创建 OKR 周期
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v1/period/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 创建 OKR 周期请求
#[derive(Debug, Clone)]
pub struct CreateRequest {
    /// 周期规则 ID（必填，可通过 period_rule/list 接口获取）
    period_rule_id: String,
    /// 周期起始年月（必填，格式 YYYY-MM）
    start_month: String,
    /// 配置信息
    config: Config,
}

impl CreateRequest {
    /// 创建请求
    pub fn new(config: Config, period_rule_id: String, start_month: String) -> Self {
        Self {
            period_rule_id,
            start_month,
            config,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateResponse> {
        use crate::common::api_endpoints::OkrApiV1;

        // 1. 验证必填字段
        validate_required!(self.period_rule_id.trim(), "周期规则 ID 不能为空");
        validate_required!(self.start_month.trim(), "周期起始年月不能为空");

        // 2. 构建端点
        let api_endpoint = OkrApiV1::PeriodCreate;
        let request = ApiRequest::<CreateResponse>::post(api_endpoint.to_url());

        // 3. 序列化请求体
        let request_body = CreateRequestBody {
            period_rule_id: self.period_rule_id,
            start_month: self.start_month,
        };
        let request = request.body(serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?);

        // 4. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "创建 OKR 周期响应数据为空",
        )
        .await
    }
}

/// 创建 OKR 周期请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequestBody {
    /// 周期规则 ID（必填）
    pub period_rule_id: String,
    /// 周期起始年月（必填，格式 YYYY-MM）
    pub start_month: String,
}

/// 创建 OKR 周期响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponse {
    /// 周期 ID
    pub period_id: String,
    /// 周期起始年月
    pub start_month: String,
    /// 周期结束年月
    pub end_month: String,
}

impl ApiResponseTrait for CreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use openlark_core::config::Config;
    use serde_json::json;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build()
    }

    #[test]
    fn test_okr_period_create_request_builder() {
        let request = CreateRequest::new(
            create_test_config(),
            "6969864184272078374".to_string(),
            "2022-01".to_string(),
        );

        assert_eq!(request.period_rule_id, "6969864184272078374");
        assert_eq!(request.start_month, "2022-01");
    }

    #[test]
    fn test_okr_period_create_request_body_serialize() {
        let body = CreateRequestBody {
            period_rule_id: "6969864184272078374".to_string(),
            start_month: "2022-01".to_string(),
        };

        let value = serde_json::to_value(body).expect("序列化请求体失败");
        assert_eq!(value["period_rule_id"], json!("6969864184272078374"));
        assert_eq!(value["start_month"], json!("2022-01"));
        // 官方请求体仅此两个字段
        assert_eq!(value.as_object().expect("请求体应为 object").len(), 2);
    }

    #[test]
    fn test_okr_period_create_response_deserialize() {
        let value = json!({
            "period_id": "6969864184272078374",
            "start_month": "2022-01",
            "end_month": "2022-01"
        });
        let response: CreateResponse = serde_json::from_value(value).expect("反序列化响应失败");
        assert_eq!(response.period_id, "6969864184272078374");
        assert_eq!(response.end_month, "2022-01");
    }

    #[test]
    fn test_okr_period_create_validation() {
        let request =
            CreateRequest::new(create_test_config(), " ".to_string(), "2022-01".to_string());
        let result: SDKResult<()> = (|| {
            validate_required!(request.period_rule_id.trim(), "周期规则 ID 不能为空");
            Ok(())
        })();
        assert!(result.is_err());
    }
    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_okr_v1_period_create_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{body_json, method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{"period_id": "test", "start_month": "2022-01", "end_month": "2022-01"}"#,
        )
        .unwrap();
        Mock::given(method("POST"))
            .and(path("/open-apis/okr/v1/periods"))
            .and(body_json(json!({
                "period_rule_id": "6969864184272078374",
                "start_month": "2022-01"
            })))
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

        let data = CreateRequest::new(
            config,
            "6969864184272078374".to_string(),
            "2022-01".to_string(),
        )
        .execute()
        .await
        .expect("okr_v1_period_create 应成功");

        assert_eq!(data.period_id, "test");
        assert_eq!(data.start_month, "2022-01");
        assert_eq!(data.end_month, "2022-01");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/okr/v1/periods");
    }
}
