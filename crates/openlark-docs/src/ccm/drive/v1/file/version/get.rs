//! 获取文档版本信息
//!
//! 获取指定源文档的指定版本信息。
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file-version/get

use crate::common::{api_endpoints::DriveApi, api_utils::*};
use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};

use super::models::FileVersionInfo;

/// 获取文档版本信息请求
#[derive(Debug, Clone)]
pub struct GetFileVersionRequest {
    config: Config,
    /// 源文档 token
    pub file_token: String,
    /// 版本文档的版本标识
    pub version_id: String,
    /// 源文档类型（docx/sheet）
    pub obj_type: String,
    /// 用户 ID 类型
    pub user_id_type: Option<String>,
}

impl GetFileVersionRequest {
    /// 创建新的文档版本查询请求。
    pub fn new(
        config: Config,
        file_token: impl Into<String>,
        version_id: impl Into<String>,
        obj_type: impl Into<String>,
    ) -> Self {
        Self {
            config,
            file_token: file_token.into(),
            version_id: version_id.into(),
            obj_type: obj_type.into(),
            user_id_type: None,
        }
    }

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<GetFileVersionResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetFileVersionResponse> {
        // ===== 验证必填字段 =====
        validate_required!(self.file_token, "file_token 不能为空");
        validate_required!(self.version_id, "version_id 不能为空");
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

        let api_endpoint = DriveApi::GetFileVersion(self.file_token, self.version_id);
        let request = ApiRequest::<GetFileVersionResponse>::get(&api_endpoint.to_url())
            .query("obj_type", self.obj_type)
            .query_opt("user_id_type", self.user_id_type);

        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取")
    }
}

/// 获取文档版本信息响应（data）
pub type GetFileVersionResponse = FileVersionInfo;

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::api::ApiResponseTrait;
    use openlark_core::testing::prelude::test_runtime;

    /// 测试构建器模式
    #[test]
    fn test_get_file_version_request_builder() {
        let config = Config::default();
        let request = GetFileVersionRequest::new(config, "file_token", "fnJfyX", "docx")
            .user_id_type("open_id");

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.version_id, "fnJfyX");
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
        let request = GetFileVersionRequest::new(config, "", "version_id", "docx");

        let result = std::thread::spawn(move || {
            let rt = test_runtime();
            rt.block_on(async move {
                let _ = request.execute().await;
            })
        })
        .join();

        assert!(result.is_ok());
    }

    /// 测试 version_id 为空时的验证
    #[test]
    fn test_empty_version_id_validation() {
        let config = Config::default();
        let request = GetFileVersionRequest::new(config, "token", "", "docx");

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
        let request = GetFileVersionRequest::new(config, "token", "version", "invalid");

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
            let request = GetFileVersionRequest::new(
                config.clone(),
                "token",
                "version",
                obj_type.to_string(),
            );
            assert_eq!(request.obj_type, obj_type);
        }
    }

    /// 测试 user_id_type 可选参数
    #[test]
    fn test_user_id_type_optional() {
        let config = Config::default();
        let request1 = GetFileVersionRequest::new(config.clone(), "token", "version", "docx");
        assert!(request1.user_id_type.is_none());

        let request2 =
            GetFileVersionRequest::new(config, "token", "version", "docx").user_id_type("user_id");
        assert_eq!(request2.user_id_type, Some("user_id".to_string()));
    }
}
