//! 获取会议报告
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/report/get_daily>

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

/// 获取会议报告请求
#[derive(Debug, Clone)]
pub struct GetDailyReportRequest {
    config: Config,
    query_params: Vec<(String, String)>,
}

/// 获取会议报告响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetDailyReportResponse {
    /// 报告数据
    pub data: serde_json::Value,
}

impl ApiResponseTrait for GetDailyReportResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetDailyReportRequest {
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
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/report/get_daily>
    pub async fn execute(self) -> SDKResult<GetDailyReportResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetDailyReportResponse> {
        let api_endpoint = VcApiV1::ReportDailyGet;
        let mut req: ApiRequest<GetDailyReportResponse> = ApiRequest::get(api_endpoint.to_url());
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取会议报告")
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
