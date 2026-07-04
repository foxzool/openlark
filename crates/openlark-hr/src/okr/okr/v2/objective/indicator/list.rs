//! 获取目标的量化指标
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v2/objective.indicator/get>

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

/// 获取目标的量化指标请求。
#[derive(Debug, Clone)]
pub struct Request {
    config: Arc<Config>,
    objective_id: String,
}

impl Request {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            objective_id: String::new(),
        }
    }

    /// 设置路径参数 `objective_id`。
    pub fn objective_id(mut self, val: impl Into<String>) -> Self {
        self.objective_id = val.into();
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<ListObjectiveIndicatorResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListObjectiveIndicatorResponse> {
        validate_required!(self.objective_id, "objective_id 不能为空");
        let path = format!(
            "/open-apis/okr/v2/objectives/{}/indicators",
            self.objective_id
        );
        let req: ApiRequest<ListObjectiveIndicatorResponse> = ApiRequest::get(path);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取目标的量化指标", "响应数据为空")
        })
    }
}

/// 获取目标的量化指标响应。
#[derive(Debug, Clone, Deserialize)]
pub struct ListObjectiveIndicatorResponse {
    /// 指标详情。
    #[serde(default)]
    pub indicator: Option<Indicator>,
}

impl ApiResponseTrait for ListObjectiveIndicatorResponse {
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
    fn test_list_objective_indicator_response_deserialize() {
        let json = serde_json::json!({
            "indicator": {
                "id": "IND-1",
                "create_time": "1700000000000",
                "update_time": "1700000000000",
                "owner": {"owner_type": "user", "user_id": "ou_xxx"},
                "entity_type": 2,
                "entity_id": "O-123",
                "indicator_status": 1,
                "status_calculate_type": 1,
                "start_value": 0.0,
                "target_value": 100.0,
                "current_value": 50.0,
                "current_value_calculate_type": 1,
                "unit": {"unit_type": 1, "unit_value": "PERCENT"}
            }
        });
        let resp: ListObjectiveIndicatorResponse =
            serde_json::from_value(json).expect("反序列化失败");
        let indicator = resp.indicator.unwrap();
        assert_eq!(indicator.id, "IND-1");
        assert_eq!(indicator.entity_type, 2);
        assert_eq!(indicator.start_value, Some(0.0));
        assert_eq!(indicator.unit.as_ref().unwrap().unit_value, "PERCENT");
    }
}
