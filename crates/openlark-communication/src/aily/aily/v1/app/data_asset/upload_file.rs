//! 上传文件用于数据知识管理
//!
//! docPath: <https://open.feishu.cn/document/aily-v1/data-knowledge/data-knowledge-management/upload_file>

use crate::{common::api_utils::extract_response_data, endpoints::AILY_V1_UPLOAD_FILE};
use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

/// 上传文件用于数据知识管理请求
///
/// 用于上传文件到数据知识管理系统。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `app_id`: 应用 ID，必填
///
/// # 请求体字段
///
/// 请求体为动态 JSON，根据具体需求传递
///
/// # 示例
///
/// ```rust,ignore
/// let body = serde_json::json!({
///     "file_name": "document.pdf",
///     "file_type": "pdf"
/// });
/// let request = UploadFileRequest::new(config)
///     .app_id("app_xxx")
///     .execute(body).await?;
/// ```
pub struct UploadFileRequest {
    config: Config,
    app_id: String,
}

impl UploadFileRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_id: String::new(),
        }
    }

    /// 应用 ID（路径参数）
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = app_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/aily-v1/data-knowledge/data-knowledge-management/upload_file>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // === 必填字段验证 ===
        validate_required!(self.app_id, "app_id 不能为空");

        let url = AILY_V1_UPLOAD_FILE.replace("{app_id}", &self.app_id);
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(&url).json_body(&body);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "上传文件用于数据知识管理")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_upload_file_request_builder() {
        let config = Config::default();
        let request = UploadFileRequest::new(config).app_id("app_xxx");
        assert_eq!(request.app_id, "app_xxx");
    }

    #[test]
    fn test_upload_file_request_default_values() {
        let config = Config::default();
        let request = UploadFileRequest::new(config);
        assert_eq!(request.app_id, "");
    }

    #[test]
    fn test_upload_file_request_with_multiple_ids() {
        let config = Config::default();
        let request1 = UploadFileRequest::new(config.clone()).app_id("app_111");
        let request2 = UploadFileRequest::new(config).app_id("app_222");
        assert_eq!(request1.app_id, "app_111");
        assert_eq!(request2.app_id, "app_222");
    }

    #[test]
    fn test_upload_file_json_body() {
        let body = serde_json::json!({
            "file_name": "document.pdf",
            "file_type": "pdf"
        });
        assert_eq!(body["file_name"], "document.pdf");
    }
}
