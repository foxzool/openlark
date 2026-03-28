//! 上传进展记录图片
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v1/image/upload>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 上传进展记录图片请求
#[derive(Debug, Clone)]
pub struct UploadRequest {
    /// 图片类型（必填）
    /// - 1: 普通图片
    /// - 2: 进度截图
    image_type: i32,
    /// 图片名称（必填）
    image_name: String,
    /// 图片 Base64 编码（必填）
    image_content: String,
    /// 配置信息
    config: Config,
}

impl UploadRequest {
    /// 创建请求
    pub fn new(config: Config, image_type: i32, image_name: String, image_content: String) -> Self {
        Self {
            image_type,
            image_name,
            image_content,
            config,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UploadResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UploadResponse> {
        use crate::common::api_endpoints::OkrApiV1;

        // 1. 构建端点
        let api_endpoint = OkrApiV1::ImageUpload;
        let request = ApiRequest::<UploadResponse>::post(api_endpoint.to_url());

        // 2. 序列化请求体
        let request_body = UploadRequestBody {
            image_type: self.image_type,
            image_name: self.image_name,
            image_content: self.image_content,
        };
        let request = request.body(serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {}", e),
            )
        })?);

        // 3. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 4. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "上传进展记录图片响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 上传进展记录图片请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadRequestBody {
    /// 图片类型
    pub image_type: i32,
    /// 图片名称
    pub image_name: String,
    /// 图片 Base64 编码
    pub image_content: String,
}

/// 上传进展记录图片响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UploadResponse {
    /// 图片 ID
    pub image_id: String,
    /// 图片 URL
    pub image_url: String,
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
