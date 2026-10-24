//! 上传文件用于数据知识管理
//!
//! docPath: <https://open.feishu.cn/document/aily-v1/data-knowledge/data-knowledge-management/upload_file>

use crate::endpoints::AILY_V1_UPLOAD_FILE;
use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};

/// 上传文件用于数据知识管理请求
///
/// 用于上传文件到数据知识管理系统（multipart/form-data，仅支持 docx、txt、pdf、pptx 类型文件）。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `app_id`: 应用 ID，必填
/// - `tenant_type`: 应用环境（可选查询参数，`online` 线上环境默认值 / `dev` 开发环境）
///
/// # 请求体字段
///
/// - `file`: 需要上传的文件（multipart 文件部件，必填）
///
/// # 示例
///
/// ```rust,ignore
/// let file_bytes = std::fs::read("document.pdf")?;
/// let request = UploadFileRequest::new(config)
///     .app_id("app_xxx")
///     .tenant_type("dev")
///     .execute(file_bytes).await?;
/// ```
pub struct UploadFileRequest {
    config: Config,
    app_id: String,
    tenant_type: Option<String>,
    file_name: Option<String>,
}

impl UploadFileRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_id: String::new(),
            tenant_type: None,
            file_name: None,
        }
    }

    /// 应用 ID（路径参数）
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = app_id.into();
        self
    }

    /// 应用环境（可选查询参数）：`online` 线上环境（默认值）/ `dev` 开发环境。
    pub fn tenant_type(mut self, tenant_type: impl Into<String>) -> Self {
        self.tenant_type = Some(tenant_type.into());
        self
    }

    /// 可选：设置上传文件名（仅用于 multipart 的文件名，不作为字段发送）
    pub fn file_name(mut self, file_name: impl Into<String>) -> Self {
        self.file_name = Some(file_name.into());
        self
    }

    /// 执行请求
    ///
    /// 说明：该接口为 multipart 上传，请直接传入文件二进制内容。
    ///
    /// docPath: <https://open.feishu.cn/document/aily-v1/data-knowledge/data-knowledge-management/upload_file>
    pub async fn execute(self, file_bytes: Vec<u8>) -> SDKResult<serde_json::Value> {
        self.execute_with_options(
            file_bytes,
            openlark_core::req_option::RequestOption::default(),
        )
        .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        file_bytes: Vec<u8>,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // === 必填字段验证 ===
        validate_required!(self.app_id, "app_id 不能为空");
        if file_bytes.is_empty() {
            return Err(openlark_core::error::validation_error(
                "file",
                "上传文件内容不能为空",
            ));
        }

        // multipart 表单：__file_name 仅用于设置 multipart 文件名，不作为表单字段发送
        let mut body = serde_json::json!({});
        if let Some(file_name) = self.file_name
            && let Some(obj) = body.as_object_mut()
        {
            obj.insert(
                "__file_name".to_string(),
                serde_json::Value::String(file_name),
            );
        }

        let url = AILY_V1_UPLOAD_FILE.replace("{app_id}", &self.app_id);
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(&url)
            .query_opt("tenant_type", self.tenant_type)
            .body(body)
            .file_content(file_bytes);

        Transport::request_typed(req, &self.config, Some(option), "上传文件用于数据知识管理").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    #[test]
    fn test_upload_file_request_builder() {
        let config = Config::default();
        let request = UploadFileRequest::new(config)
            .app_id("app_xxx")
            .tenant_type("dev");
        assert_eq!(request.app_id, "app_xxx");
        assert_eq!(request.tenant_type, Some("dev".to_string()));
    }

    #[test]
    fn test_upload_file_request_default_values() {
        let config = Config::default();
        let request = UploadFileRequest::new(config);
        assert_eq!(request.app_id, "");
        assert!(request.tenant_type.is_none());
        assert!(request.file_name.is_none());
    }

    #[test]
    fn test_upload_file_request_with_multiple_ids() {
        let config = Config::default();
        let request1 = UploadFileRequest::new(config.clone()).app_id("app_111");
        let request2 = UploadFileRequest::new(config).app_id("app_222");
        assert_eq!(request1.app_id, "app_111");
        assert_eq!(request2.app_id, "app_222");
    }

    #[tokio::test]
    async fn test_upload_file_empty_bytes_rejected() {
        let config = Config::default();
        let request = UploadFileRequest::new(config).app_id("app_xxx");
        let result = request
            .execute_with_options(vec![], openlark_core::req_option::RequestOption::default())
            .await;
        assert!(result.is_err(), "空文件内容应被必填校验拒绝");
    }

    #[test]
    fn test_upload_file_request_url_construction() {
        use crate::endpoints::aily::AILY_V1_UPLOAD_FILE;
        let url = AILY_V1_UPLOAD_FILE.replace("{app_id}", "app_1");
        assert_eq!(url, "/open-apis/aily/v1/apps/app_1/data_assets/upload_file");
        assert!(
            !url.contains("{"),
            "URL should not contain unreplaced placeholders"
        );
    }

    /// 端到端：POST /open-apis/aily/v1/apps/app_1/data_assets/upload_file?tenant_type=dev（multipart）
    #[tokio::test]
    async fn test_upload_file_returns_file_info_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/aily/v1/apps/app_1/data_assets/upload_file",
            ))
            .and(query_param("tenant_type", "dev"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "file_info": {
                        "token": "file_token_abcd123",
                        "mime_type": "application/pdf"
                    }
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = UploadFileRequest::new(config)
            .app_id("app_1")
            .tenant_type("dev")
            .execute(b"%PDF-1.4 fake".to_vec())
            .await
            .expect("请求应成功");
        assert_eq!(resp["file_info"]["token"], "file_token_abcd123");
        assert_eq!(resp["file_info"]["mime_type"], "application/pdf");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
