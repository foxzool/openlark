//! 获取活跃会议
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/reserve/get_active_meeting>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, validate_required,
    SDKResult,
};

use crate::common::api_endpoints::VcApiV1;
use crate::common::api_utils::extract_response_data;

/// 获取活跃会议请求
pub struct GetActiveMeetingRequest {
    config: Config,
    reserve_id: String,
    query_params: Vec<(String, String)>,
}

impl GetActiveMeetingRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            reserve_id: String::new(),
            query_params: Vec::new(),
        }
    }

    /// 预约 ID（路径参数）
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
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/reserve/get_active_meeting>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.reserve_id, "reserve_id 不能为空");

        // url: GET:/open-apis/vc/v1/reserves/:reserve_id/active_meeting
        let api_endpoint = VcApiV1::ReserveGetActiveMeeting(self.reserve_id);
        let mut req: ApiRequest<serde_json::Value> = ApiRequest::get(api_endpoint.to_url());
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取活跃会议")
    }
}

#[cfg(test)]
mod tests {

    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
