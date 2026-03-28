//! 创建 OKR 周期
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v1/period/create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 创建 OKR 周期请求
#[derive(Debug, Clone)]
pub struct CreateRequest {
    /// 周期名称（必填）
    name: String,
    /// 周期开始时间（Unix 时间戳）（必填）
    start_time: i64,
    /// 周期结束时间（Unix 时间戳）（必填）
    end_time: i64,
    /// 周期描述（可选）
    description: Option<String>,
    /// 目标制定截止时间（Unix 时间戳）（可选）
    target_setting_deadline: Option<i64>,
    /// 复盘时间（Unix 时间戳）（可选）
    review_time: Option<i64>,
    /// 配置信息
    config: Config,
}

impl CreateRequest {
    /// 创建请求
    pub fn new(config: Config, name: String, start_time: i64, end_time: i64) -> Self {
        Self {
            name,
            start_time,
            end_time,
            description: None,
            target_setting_deadline: None,
            review_time: None,
            config,
        }
    }

    /// 设置周期描述（可选）
    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// 设置目标制定截止时间（可选）
    pub fn target_setting_deadline(mut self, deadline: i64) -> Self {
        self.target_setting_deadline = Some(deadline);
        self
    }

    /// 设置复盘时间（可选）
    pub fn review_time(mut self, review_time: i64) -> Self {
        self.review_time = Some(review_time);
        self
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
        validate_required!(self.name.trim(), "周期名称不能为空");

        // 2. 构建端点
        let api_endpoint = OkrApiV1::PeriodCreate;
        let request = ApiRequest::<CreateResponse>::post(api_endpoint.to_url());

        // 3. 序列化请求体
        let request_body = CreateRequestBody {
            name: self.name,
            start_time: self.start_time,
            end_time: self.end_time,
            description: self.description,
            target_setting_deadline: self.target_setting_deadline,
            review_time: self.review_time,
        };
        let request = request.body(serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {}", e),
            )
        })?);

        // 4. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 5. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "创建 OKR 周期响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 创建 OKR 周期请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequestBody {
    /// 周期名称（必填）
    pub name: String,
    /// 周期开始时间（Unix 时间戳）（必填）
    pub start_time: i64,
    /// 周期结束时间（Unix 时间戳）（必填）
    pub end_time: i64,
    /// 周期描述（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 目标制定截止时间（Unix 时间戳）（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_setting_deadline: Option<i64>,
    /// 复盘时间（Unix 时间戳）（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_time: Option<i64>,
}

/// 创建 OKR 周期响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponse {
    /// 周期 ID
    pub period_id: String,
    /// 周期名称
    pub name: String,
    /// 周期开始时间
    pub start_time: i64,
    /// 周期结束时间
    pub end_time: i64,
    /// 周期状态
    pub status: i32,
    /// 创建时间
    pub created_at: i64,
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
            "2026Q1".to_string(),
            1_706_000_000,
            1_708_000_000,
        )
        .description("第一季度 OKR".to_string())
        .review_time(1_708_100_000);

        assert_eq!(request.name, "2026Q1");
        assert_eq!(request.start_time, 1_706_000_000);
        assert_eq!(request.review_time, Some(1_708_100_000));
    }

    #[test]
    fn test_okr_period_create_request_body_serialize() {
        let body = CreateRequestBody {
            name: "2026Q2".to_string(),
            start_time: 1_709_000_000,
            end_time: 1_711_000_000,
            description: Some("第二季度 OKR".to_string()),
            target_setting_deadline: Some(1_709_100_000),
            review_time: None,
        };

        let value = serde_json::to_value(body).expect("序列化请求体失败");
        assert_eq!(value["name"], json!("2026Q2"));
        assert_eq!(value["target_setting_deadline"], json!(1_709_100_000));
        assert!(value.get("review_time").is_none());
    }

    #[test]
    fn test_okr_period_create_response_deserialize() {
        let value = json!({
            "period_id": "period_1",
            "name": "2026Q1",
            "start_time": 1706000000,
            "end_time": 1708000000,
            "status": 1,
            "created_at": 1705000000
        });
        let response: CreateResponse = serde_json::from_value(value).expect("反序列化响应失败");
        assert_eq!(response.period_id, "period_1");
        assert_eq!(response.status, 1);
    }

    #[test]
    fn test_okr_period_create_validation() {
        let request = CreateRequest::new(create_test_config(), " ".to_string(), 1, 2);
        let result: SDKResult<()> = (|| {
            validate_required!(request.name.trim(), "周期名称不能为空");
            Ok(())
        })();
        assert!(result.is_err());
    }
}
