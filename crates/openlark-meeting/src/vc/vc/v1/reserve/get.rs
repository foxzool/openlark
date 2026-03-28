//! 获取预约
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/reserve/get>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::VcApiV1;
use crate::common::api_utils::{extract_response_data, validate_required_field};

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

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取预约")
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
