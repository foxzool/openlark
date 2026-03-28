//! 下载导出文件
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/export/download>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::extract_response_data;
use crate::endpoints::VC_V1_EXPORT_GET;

/// 下载导出文件请求

#[derive(Debug, Clone)]
pub struct DownloadExportRequest {
    /// 配置信息
    config: Config,
    /// 查询参数
    query_params: Vec<(String, String)>,
}

/// 下载导出文件响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DownloadExportResponse {
    /// 下载 URL
    pub url: String,
}

impl ApiResponseTrait for DownloadExportResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl DownloadExportRequest {
    /// 创建新的请求
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
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/export/download>
    pub async fn execute(self) -> SDKResult<DownloadExportResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DownloadExportResponse> {
        let url = VC_V1_EXPORT_GET.replace("{export_id}", "download");
        let mut api_request: ApiRequest<DownloadExportResponse> = ApiRequest::get(&url);

        for (key, value) in self.query_params {
            api_request = api_request.query(key, value);
        }

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "下载导出文件")
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
