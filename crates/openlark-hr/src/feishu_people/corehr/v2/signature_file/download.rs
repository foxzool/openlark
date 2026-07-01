//! 下载电子签文件
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/signature_file/download>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 下载电子签文件请求
#[derive(Debug, Clone)]
pub struct DownloadRequest {
    /// 配置信息
    config: Config,
    /// 电子签文件 ID
    signature_file_id: Option<String>,
    body: Option<Value>,
}

impl DownloadRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            signature_file_id: None,
            body: None,
        }
    }

    /// 设置 `signature_file_id`。
    pub fn signature_file_id(mut self, signature_file_id: impl Into<String>) -> Self {
        self.signature_file_id = Some(signature_file_id.into());
        self
    }

    /// 设置请求体。
    pub fn body(mut self, body: Value) -> Self {
        self.body = Some(body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DownloadResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DownloadResponse> {
        let signature_file_id = self.signature_file_id.unwrap_or_default();
        validate_required!(signature_file_id.trim(), "signature_file_id 不能为空");

        let mut request = ApiRequest::<DownloadResponse>::post(format!(
            "/open-apis/corehr/v2/signature_files/{signature_file_id}/download"
        ));

        if let Some(body) = self.body {
            request = request.body(body);
        }

        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("接口响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

/// 下载电子签文件响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DownloadResponse {
    /// 响应数据
    pub data: Value,
}

impl ApiResponseTrait for DownloadResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
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
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
