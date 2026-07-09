//! 上传进展记录图片
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v1/image/upload>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
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

        if !(1..=2).contains(&self.image_type) {
            return Err(openlark_core::error::validation_error(
                "image_type 无效",
                "image_type 必须为 1 或 2",
            ));
        }
        validate_required!(self.image_name.trim(), "image_name");
        validate_required!(self.image_content.trim(), "image_content");

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
                format!("无法序列化请求参数: {e}"),
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
mod tests {
    use super::*;
    use openlark_core::config::Config;

    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_okr_v1_image_upload_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value =
            serde_json::from_str(r#"{"image_id": "test", "image_url": "test"}"#).unwrap();
        Mock::given(method("POST"))
            .and(path("/open-apis/okr/v1/images/upload"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": data_body
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = UploadRequest::new(
            config,
            1,
            "sample_image_name".to_string(),
            "sample_image_content".to_string(),
        )
        .execute()
        .await
        .expect("okr_v1_image_upload 应成功");

        assert_eq!(data.image_id, "test");
        assert_eq!(data.image_url, "test");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/okr/v1/images/upload");
    }
}
