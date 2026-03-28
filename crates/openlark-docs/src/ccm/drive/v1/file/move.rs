//! 移动文件或文件夹
//!
//! 将文件或者文件夹移动到用户云空间的其他位置。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/drive-v1/file/move>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 移动文件请求
///
/// 将文件或文件夹移动到用户云空间的其他位置。
///
/// # 字段说明
///
/// - `file_token`: 需要移动的文件或文件夹的 token，不能为空
/// - `r#type`: 文件类型，必须与 file_token 对应的实际类型一致
///   - 支持值：`file` | `docx` | `bitable` | `doc` | `sheet` | `mindnote` | `folder` | `slides`
/// - `folder_token`: 目标文件夹的 token，不能为空
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_core::config::Config;
/// use openlark_docs::ccm::drive::v1::file::MoveFileRequest;
///
/// let config = Config::builder().app_id("app_id").app_secret("app_secret").build();
/// let request = MoveFileRequest::new(config, "file_token", "docx", "target_folder_token");
/// let response = request.execute().await?;
/// println!("移动完成，task_id: {:?}", response.task_id);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct MoveFileRequest {
    #[serde(skip)]
    config: Config,
    /// 需要移动的文件或文件夹的 token
    pub file_token: String,
    /// 文件类型（必填）
    #[serde(rename = "type")]
    pub r#type: String,
    /// 目标文件夹的 token
    pub folder_token: String,
}

impl MoveFileRequest {
    /// 创建移动文件请求
    ///
    /// # 参数
    ///
    /// - `config`: SDK 配置实例
    /// - `file_token`: 需要移动的文件或文件夹 token
    /// - `file_type`: 文件类型，支持 `file/docx/bitable/doc/sheet/mindnote/folder/slides`
    /// - `folder_token`: 目标文件夹 token
    ///
    /// # 示例
    ///
    /// ```rust,ignore
    /// let request = MoveFileRequest::new(config, "file_token", "docx", "folder_token");
    /// ```
    pub fn new(
        config: Config,
        file_token: impl Into<String>,
        file_type: impl Into<String>,
        folder_token: impl Into<String>,
    ) -> Self {
        Self {
            config,
            file_token: file_token.into(),
            r#type: file_type.into(),
            folder_token: folder_token.into(),
        }
    }

    /// 执行请求（使用默认选项）
    ///
    /// 发送移动文件请求到飞书服务器。
    pub async fn execute(self) -> SDKResult<MoveFileResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用自定义选项执行请求
    ///
    /// # 参数
    ///
    /// - `option`: 请求选项，可用于设置超时、重试策略等
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<MoveFileResponse> {
        // === 必填字段验证 ===
        if self.file_token.is_empty() {
            return Err(openlark_core::error::validation_error(
                "file_token",
                "file_token 不能为空",
            ));
        }
        if self.folder_token.is_empty() {
            return Err(openlark_core::error::validation_error(
                "folder_token",
                "folder_token 不能为空",
            ));
        }

        // === 枚举值验证 ===
        if self.r#type.is_empty() {
            return Err(openlark_core::error::validation_error(
                "type",
                "type 不能为空",
            ));
        }
        match self.r#type.as_str() {
            "file" | "docx" | "bitable" | "doc" | "sheet" | "mindnote" | "folder" | "slides" => {}
            _ => {
                return Err(openlark_core::error::validation_error(
                    "type",
                    "type 仅支持 file/docx/bitable/doc/sheet/mindnote/folder/slides",
                ));
            }
        }

        let api_endpoint = DriveApi::MoveFile(self.file_token.clone());
        #[derive(Serialize)]
        struct MoveFileBody {
            #[serde(rename = "type")]
            r#type: String,
            folder_token: String,
        }
        let request =
            ApiRequest::<MoveFileResponse>::post(&api_endpoint.to_url()).body(serialize_params(
                &MoveFileBody {
                    r#type: self.r#type,
                    folder_token: self.folder_token,
                },
                "移动文件或文件夹",
            )?);

        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "移动文件或文件夹")
    }
}

/// 移动文件响应
///
/// 包含移动操作的结果信息。
///
/// # 字段说明
///
/// - `task_id`: 异步任务 ID，移动文件夹时返回此字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveFileResponse {
    /// 异步任务 ID，移动文件夹时返回
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

impl ApiResponseTrait for MoveFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_file_request_builder() {
        let config = Config::default();
        let request = MoveFileRequest::new(config, "file_token", "file", "folder_token");

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.r#type, "file");
        assert_eq!(request.folder_token, "folder_token");
    }

    #[test]
    fn test_move_file_with_empty_file_token() {
        let config = Config::default();
        let request = MoveFileRequest::new(config, "", "file", "folder_token");
        assert_eq!(request.file_token, "");
        // 空 file_token 应在 execute_with_options 时被校验拦截
    }

    #[test]
    fn test_move_file_with_invalid_type() {
        let config = Config::default();
        let request = MoveFileRequest::new(config, "file_token", "invalid", "folder_token");
        assert_eq!(request.r#type, "invalid");
        // 无效 type 应在 execute_with_options 时被校验拦截
    }

    #[test]
    fn test_move_file_with_empty_folder_token() {
        let config = Config::default();
        let request = MoveFileRequest::new(config, "file_token", "file", "");
        assert_eq!(request.folder_token, "");
        // 空 folder_token 应在 execute_with_options 时被校验拦截
    }

    #[test]
    fn test_move_file_with_all_valid_types() {
        let config = Config::default();
        let valid_types = [
            "file", "docx", "bitable", "doc", "sheet", "mindnote", "folder", "slides",
        ];
        for file_type in valid_types {
            let request =
                MoveFileRequest::new(config.clone(), "file_token", file_type, "folder_token");
            assert_eq!(request.r#type, file_type);
        }
    }

    #[test]
    fn test_move_file_response() {
        let response = MoveFileResponse {
            task_id: Some("task_123".to_string()),
        };
        assert_eq!(response.task_id, Some("task_123".to_string()));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(MoveFileResponse::data_format(), ResponseFormat::Data);
    }
}
