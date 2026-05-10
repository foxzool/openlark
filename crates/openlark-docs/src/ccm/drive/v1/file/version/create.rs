//! 创建文档版本
//!
//! 为源文档创建版本文档。
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file-version/create

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
    use openlark_core::testing::prelude::test_runtime;

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

    /// 测试 file_token 为空时的验证
    #[test]
    fn test_empty_file_token_validation() {
        let config = Config::default();
        let request = CreateFileVersionRequest::new(config, "", "name", "docx");

        let result = std::thread::spawn(move || {
            let rt = test_runtime();
            rt.block_on(async move {
                let _ = request.execute().await;
            })
        })
        .join();

        assert!(result.is_ok());
    }

    /// 测试 name 为空时的验证
    #[test]
    fn test_empty_name_validation() {
        let config = Config::default();
        let request = CreateFileVersionRequest::new(config, "token", "", "docx");

        let result = std::thread::spawn(move || {
            let rt = test_runtime();
            rt.block_on(async move {
                let _ = request.execute().await;
            })
        })
        .join();

        assert!(result.is_ok());
    }

    /// 测试 name 超长时的验证
    #[test]
    fn test_name_length_validation() {
        let config = Config::default();
        let long_name = "a".repeat(1025);
        let request = CreateFileVersionRequest::new(config, "token", long_name, "docx");

        let result = std::thread::spawn(move || {
            let rt = test_runtime();
            rt.block_on(async move {
                let _ = request.execute().await;
            })
        })
        .join();

        assert!(result.is_ok());
    }

    /// 测试 obj_type 枚举值验证
    #[test]
    fn test_obj_type_validation() {
        let config = Config::default();
        let request = CreateFileVersionRequest::new(config, "token", "name", "invalid");

        let result = std::thread::spawn(move || {
            let rt = test_runtime();
            rt.block_on(async move {
                let _ = request.execute().await;
            })
        })
        .join();

        assert!(result.is_ok());
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
}
