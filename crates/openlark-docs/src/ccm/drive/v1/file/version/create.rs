//! 创建文档版本
//!
//! 为源文档创建版本文档。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/drive-v1/file-version/create>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};
use serde::Serialize;

use crate::common::{api_endpoints::DriveApi, api_utils::*};

use super::models::FileVersionInfo;

/// 创建文档版本请求
#[derive(Debug, Clone)]
pub struct CreateFileVersionRequest {
    config: Config,
    /// 源文档 token
    pub file_token: String,
    /// 用户 ID 类型
    pub user_id_type: Option<String>,
    /// 版本文档标题
    pub name: String,
    /// 源文档类型（docx/sheet）
    pub obj_type: String,
}

impl CreateFileVersionRequest {
    /// 创建新的文档版本请求。
    pub fn new(
        config: Config,
        file_token: impl Into<String>,
        name: impl Into<String>,
        obj_type: impl Into<String>,
    ) -> Self {
        Self {
            config,
            file_token: file_token.into(),
            user_id_type: None,
            name: name.into(),
            obj_type: obj_type.into(),
        }
    }

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<CreateFileVersionResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateFileVersionResponse> {
        // ===== 验证必填字段 =====
        validate_required!(self.file_token, "file_token 不能为空");
        validate_required!(self.name, "name 不能为空");
        // ===== 验证字段长度 =====
        if self.name.chars().count() > 1024 {
            return Err(openlark_core::error::validation_error(
                "name",
                "name 最大长度为 1024 个 Unicode 码点",
            ));
        }
        // ===== 验证字段枚举值 =====
        match self.obj_type.as_str() {
            "docx" | "sheet" => {}
            _ => {
                return Err(openlark_core::error::validation_error(
                    "obj_type",
                    "obj_type 仅支持 docx/sheet",
                ));
            }
        }

        #[derive(Serialize)]
        struct VersionPayload {
            name: String,
            obj_type: String,
        }

        let api_endpoint = DriveApi::CreateFileVersion(self.file_token);
        let payload = VersionPayload {
            name: self.name,
            obj_type: self.obj_type,
        };

        let request = ApiRequest::<CreateFileVersionResponse>::post(&api_endpoint.to_url())
            .query_opt("user_id_type", self.user_id_type)
            .body(serialize_params(&payload, "创建文档版本")?);

        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "创建版本")
    }
}

/// 创建文档版本响应（data）
pub type CreateFileVersionResponse = FileVersionInfo;

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::api::ApiResponseTrait;

    /// 测试构建器模式
    #[test]
    fn test_create_file_version_request_builder() {
        let config = Config::default();
        let request =
            CreateFileVersionRequest::new(config, "file_token", "项目文档 第 1 版", "docx")
                .user_id_type("open_id");

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.name, "项目文档 第 1 版");
        assert_eq!(request.obj_type, "docx");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    /// 测试响应格式
    #[test]
    fn test_response_trait() {
        assert_eq!(
            <FileVersionInfo as ApiResponseTrait>::data_format(),
            openlark_core::api::ResponseFormat::Data
        );
    }

    /// 测试支持的 obj_type 类型
    #[test]
    fn test_supported_obj_types() {
        let config = Config::default();

        for obj_type in ["docx", "sheet"] {
            let request = CreateFileVersionRequest::new(
                config.clone(),
                "token",
                "name",
                obj_type.to_string(),
            );
            assert_eq!(request.obj_type, obj_type);
        }
    }

    /// 测试 Unicode 字符计数
    #[test]
    fn test_unicode_name_length() {
        let config = Config::default();
        // 中文 emoji 组合，测试 Unicode 码点计数
        let name = "🎉🎊🎈"; // 3 个码点
        let request = CreateFileVersionRequest::new(config, "token", name, "docx");
        assert_eq!(request.name.chars().count(), 3);
    }

    /// 端到端：POST .../files/{file_token}/versions → 强类型 FileVersionInfo（单层 data 信封）。
    #[tokio::test]
    async fn test_create_file_version_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/drive/v1/files/ftk001/versions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "name": "项目文档 第 1 版",
                    "version": "fnJfyX",
                    "parent_token": "ftk001",
                    "status": "active",
                    "obj_type": "docx"
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

        let resp = CreateFileVersionRequest::new(config, "ftk001", "项目文档 第 1 版", "docx")
            .execute()
            .await
            .expect("创建文档版本应成功");
        assert_eq!(resp.name.as_deref(), Some("项目文档 第 1 版"));
        assert_eq!(resp.version.as_deref(), Some("fnJfyX"));
        assert_eq!(resp.parent_token.as_deref(), Some("ftk001"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/drive/v1/files/ftk001/versions"
        );
    }
}
