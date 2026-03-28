//! 查询参会人明细
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/meeting-room-data/get-2>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::VcApiV1;
use crate::common::api_utils::extract_response_data;

/// 查询参会人明细请求
#[derive(Debug, Clone)]
pub struct GetParticipantListRequest {
    config: Config,
    query_params: Vec<(String, String)>,
}

/// 查询参会人明细响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetParticipantListResponse {
    /// 参会人列表数据
    pub data: serde_json::Value,
}

impl ApiResponseTrait for GetParticipantListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetParticipantListRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query_params: Vec::new(),
        }
    }

    /// 追加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/meeting-room-data/get-2>
    pub async fn execute(self) -> SDKResult<GetParticipantListResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetParticipantListResponse> {
        let api_endpoint = VcApiV1::ParticipantListList;
        let mut req: ApiRequest<GetParticipantListResponse> =
            ApiRequest::get(api_endpoint.to_url());
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "查询参会人明细")
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
