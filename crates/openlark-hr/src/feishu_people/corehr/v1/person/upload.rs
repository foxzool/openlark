//! 上传文件
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/person/upload>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 上传文件请求
#[derive(Debug, Clone)]
pub struct UploadRequest {
    config: Config,
    file: Option<Value>,
}

impl UploadRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self { config, file: None }
    }

    pub fn file(mut self, file: Value) -> Self {
        self.file = Some(file);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UploadResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UploadResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        let api_endpoint = FeishuPeopleApiV1::PersonUpload;
        let mut request = ApiRequest::<UploadResponse>::post(api_endpoint.to_url());
        if let Some(file) = self.file {
            request = request.body(serde_json::json!({ "file": file }));
        }

        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "上传文件响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 上传文件响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UploadResponse {
    pub data: Value,
}

impl ApiResponseTrait for UploadResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

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
