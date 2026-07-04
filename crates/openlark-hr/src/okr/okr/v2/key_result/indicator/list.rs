//! 获取关键结果的量化指标
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v2/key_result.indicator/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::Deserialize;
use std::sync::Arc;

/// 获取关键结果的量化指标请求。
#[derive(Debug, Clone)]
pub struct Request {
    config: Arc<Config>,
    key_result_id: String,
}

impl Request {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            key_result_id: String::new(),
        }
    }

    /// 设置路径参数 `key_result_id`。
    pub fn key_result_id(mut self, val: impl Into<String>) -> Self {
        self.key_result_id = val.into();
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<ListKeyResultIndicatorResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListKeyResultIndicatorResponse> {
        validate_required!(self.key_result_id, "key_result_id 不能为空");
        let path = format!(
            "/open-apis/okr/v2/key_results/{}/indicators",
            self.key_result_id
        );
        let req: ApiRequest<ListKeyResultIndicatorResponse> = ApiRequest::get(path);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取关键结果的量化指标", "响应数据为空")
        })
    }
}

/// 获取关键结果的量化指标响应。
#[derive(Debug, Clone, Deserialize)]
pub struct ListKeyResultIndicatorResponse {
    /// 指标详情。
    #[serde(default)]
    pub indicator: Option<Indicator>,
}

impl ApiResponseTrait for ListKeyResultIndicatorResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 量化指标。
#[derive(Debug, Clone, Deserialize)]
pub struct Indicator {
    /// 指标的 ID。
    pub id: String,
    /// 指标的创建时间，毫秒级时间戳。
    pub create_time: String,
    /// 指标的更新时间，毫秒级时间戳。
    pub update_time: String,
    /// 所有者。
    pub owner: IndicatorOwner,
    /// 指标所属的实体类型。
    pub entity_type: i32,
    /// 指标所属的实体 ID。
    pub entity_id: String,
    /// 指标的状态。
    pub indicator_status: i32,
    /// 指标的状态的计算方式。
    pub status_calculate_type: i32,
    /// 指标的起始值。
    #[serde(default)]
    pub start_value: Option<f64>,
    /// 指标的目标值。
    #[serde(default)]
    pub target_value: Option<f64>,
    /// 指标的当前值。
    #[serde(default)]
    pub current_value: Option<f64>,
    /// 指标的当前值的计算方式。
    #[serde(default)]
    pub current_value_calculate_type: Option<i32>,
    /// 指标的单位。
    #[serde(default)]
    pub unit: Option<IndicatorUnit>,
}

/// 指标所有者。
#[derive(Debug, Clone, Deserialize)]
pub struct IndicatorOwner {
    /// 所有者类型（如 "user"）。
    pub owner_type: String,
    /// 员工 ID。
    #[serde(default)]
    pub user_id: Option<String>,
}

/// 指标单位。
#[derive(Debug, Clone, Deserialize)]
pub struct IndicatorUnit {
    /// 指标的单位类型。
    pub unit_type: i32,
    /// 指标单位的值。
    pub unit_value: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_initializes() {
        let config = Arc::new(Config::default());
        let _req = Request::new(config);
    }

    #[test]
    fn test_list_key_result_indicator_response_deserialize() {
        let json = serde_json::json!({
            "indicator": {
                "id": "I-123",
                "create_time": "1700000000000",
                "update_time": "1700000000000",
                "owner": {"owner_type": "user", "user_id": "ou_xxx"},
                "entity_type": 3,
                "entity_id": "KR-123",
                "indicator_status": 0,
                "status_calculate_type": 0,
                "start_value": 0.0,
                "target_value": 100.0,
                "current_value": 50.0,
                "current_value_calculate_type": 0,
                "unit": {"unit_type": 0, "unit_value": "PERCENT"}
            }
        });
        let resp: ListKeyResultIndicatorResponse =
            serde_json::from_value(json).expect("反序列化失败");
        let indicator = resp.indicator.expect("指标不应为空");
        assert_eq!(indicator.id, "I-123");
        assert_eq!(indicator.entity_type, 3);
        assert_eq!(indicator.entity_id, "KR-123");
        assert_eq!(indicator.current_value, Some(50.0));
        assert_eq!(indicator.unit.as_ref().unwrap().unit_value, "PERCENT");
        assert_eq!(indicator.owner.user_id, Some("ou_xxx".to_string()));
    }

    #[test]
    fn test_list_key_result_indicator_response_deserialize_empty() {
        let json = serde_json::json!({});
        let resp: ListKeyResultIndicatorResponse =
            serde_json::from_value(json).expect("反序列化失败");
        assert!(resp.indicator.is_none());
    }
}
