//! 复制文件
//!
//! 将文件复制到用户云空间的其他文件夹中。不支持复制文件夹。
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file/copy

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 复制文件请求
///
/// 将文件复制到用户云空间的其他文件夹中。不支持复制文件夹。
///
/// # 字段说明
///
/// - `file_token`: 被复制的源文件的 token，不能为空
/// - `name`: 复制的新文件名称，长度必须在 1~256 字节之间
/// - `r#type`: 被复制的源文件类型，必须与 file_token 对应的源文件实际类型一致
///   - 支持值：`file` | `doc` | `sheet` | `bitable` | `docx` | `mindnote` | `slides`
/// - `folder_token`: 目标文件夹的 token，不能为空
/// - `user_id_type`: 用户 ID 类型，支持 `open_id` | `union_id` | `user_id`
/// - `extra`: 自定义请求附加参数，用于实现特殊的复制语义
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_core::config::Config;
/// use openlark_docs::ccm::drive::v1::file::CopyFileRequest;
///
/// let config = Config::builder().app_id("app_id").app_secret("app_secret").build();
/// let request = CopyFileRequest::new(
///     config,
///     "source_file_token",
///     "复制的文件",
///     "docx",
///     "target_folder_token",
/// );
/// let response = request.execute().await?;
/// println!("新文件 token: {}", response.file.token);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct CopyFileRequest {
    #[serde(skip)]
    config: Config,
    /// 被复制的源文件的 token
    pub file_token: String,
    /// 用户 ID 类型（默认 open_id）
    #[serde(skip)]
    pub user_id_type: Option<String>,
    /// 复制的新文件的名称（最大 256 字节）
    pub name: String,
    /// 目标文件夹的 token
    pub folder_token: String,
    /// 被复制的源文件的类型。必须与 file_token 对应的源文件实际类型一致
    #[serde(rename = "type")]
    pub r#type: String,
    /// 自定义请求附加参数，用于实现特殊的复制语义
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<Vec<Property>>,
}

impl CopyFileRequest {
    /// 创建复制文件请求
    ///
    /// # 参数
    ///
    /// - `config`: SDK 配置实例
    /// - `file_token`: 被复制的源文件 token
    /// - `name`: 复制的新文件名称
    /// - `file_type`: 被复制的源文件类型
    /// - `folder_token`: 目标文件夹 token
    pub fn new(
        config: Config,
        file_token: impl Into<String>,
        name: impl Into<String>,
        file_type: impl Into<String>,
        folder_token: impl Into<String>,
    ) -> Self {
        Self {
            config,
            file_token: file_token.into(),
            user_id_type: None,
            name: name.into(),
            folder_token: folder_token.into(),
            r#type: file_type.into(),
            extra: None,
        }
    }

    /// 设置用户 ID 类型
    ///
    /// # 参数
    ///
    /// - `user_id_type`: 支持 `open_id` | `union_id` | `user_id`
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置自定义附加参数
    ///
    /// 用于实现特殊的复制语义。
    pub fn extra(mut self, extra: Vec<Property>) -> Self {
        self.extra = Some(extra);
        self
    }

    /// 执行请求（使用默认选项）
    ///
    /// 发送复制文件请求到飞书服务器。
    pub async fn execute(self) -> SDKResult<CopyFileResponse> {
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
    ) -> SDKResult<CopyFileResponse> {
        // === 必填字段验证 ===
        validate_required!(self.file_token, "file_token 不能为空");
        validate_required!(self.folder_token, "folder_token 不能为空");

        // === 枚举值验证 ===
        // 文档说明：type 为必填（请忽略必填列的"否"），为空会导致接口失败
        if self.r#type.is_empty() {
            return Err(openlark_core::error::validation_error(
                "type",
                "type 不能为空",
            ));
        }
        match self.r#type.as_str() {
            "file" | "doc" | "sheet" | "bitable" | "docx" | "mindnote" | "slides" => {}
            _ => {
                return Err(openlark_core::error::validation_error(
                    "type",
                    "type 仅支持 file/doc/sheet/bitable/docx/mindnote/slides",
                ));
            }
        }

        // user_id_type 枚举值验证
        if let Some(user_id_type) = &self.user_id_type {
            match user_id_type.as_str() {
                "open_id" | "union_id" | "user_id" => {}
                _ => {
                    return Err(openlark_core::error::validation_error(
                        "user_id_type",
                        "user_id_type 仅支持 open_id/union_id/user_id",
                    ));
                }
            }
        }

        // === 业务规则验证 ===
        // 文件名长度验证（1~256 字节）
        let name_len = self.name.len();
        if name_len == 0 || name_len > 256 {
            return Err(openlark_core::error::validation_error(
                "name",
                "name 长度必须在 1~256 字节之间",
            ));
        }

        // extra 参数验证：key 和 value 不能为空
        if let Some(extra) = &self.extra {
            for (idx, prop) in extra.iter().enumerate() {
                if prop.key.trim().is_empty() {
                    return Err(openlark_core::error::validation_error(
                        &format!("extra[{idx}].key"),
                        "key 不能为空",
                    ));
                }
                if prop.value.trim().is_empty() {
                    return Err(openlark_core::error::validation_error(
                        &format!("extra[{idx}].value"),
                        "value 不能为空",
                    ));
                }
            }
        }

        let api_endpoint = DriveApi::CopyFile(self.file_token.clone());
        let mut request = ApiRequest::<CopyFileResponse>::post(&api_endpoint.to_url());

        if let Some(user_id_type) = &self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        #[derive(Serialize)]
        struct CopyFileBody {
            name: String,
            #[serde(rename = "type")]
            r#type: String,
            folder_token: String,
            #[serde(skip_serializing_if = "Option::is_none")]
            extra: Option<Vec<Property>>,
        }

        request = request.body(serialize_params(
            &CopyFileBody {
                name: self.name,
                r#type: self.r#type,
                folder_token: self.folder_token,
                extra: self.extra,
            },
            "复制文件",
        )?);

        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "复制文件")
    }
}

/// 自定义属性键值对
///
/// 用于实现特殊的复制语义。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    /// 自定义属性键对象
    pub key: String,
    /// 自定义属性值对象
    pub value: String,
}

/// 复制文件响应
///
/// 包含复制成功后的新文件信息。
///
/// # 字段说明
///
/// - `file`: 复制的新文件的详细信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyFileResponse {
    /// 复制的新文件信息
    pub file: CopiedFile,
}

/// 快捷方式信息（该参数不会返回）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyShortcutInfo {
    /// 快捷方式指向的源文件类型
    pub target_type: String,
    /// 快捷方式指向的源文件 token
    pub target_token: String,
}

/// 复制的新文件信息
///
/// 包含复制后新文件的完整信息。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopiedFile {
    /// 复制的新文件 token
    pub token: String,
    /// 新文件的名称
    pub name: String,
    /// 新文件的类型
    #[serde(rename = "type")]
    pub r#type: String,
    /// 新文件的父文件夹 token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_token: Option<String>,
    /// 文件在浏览器中的 URL 链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 快捷方式文件信息（该参数不会返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shortcut_info: Option<CopyShortcutInfo>,
    /// 文件创建时间（该参数不会返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// 文件最近修改时间（该参数不会返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_time: Option<String>,
    /// 文件所有者（该参数不会返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
}

impl ApiResponseTrait for CopyFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_file_request_builder() {
        let config = Config::default();
        let request =
            CopyFileRequest::new(config, "file_token", "Demo copy", "docx", "folder_token");

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.name, "Demo copy");
        assert_eq!(request.r#type, "docx");
        assert_eq!(request.folder_token, "folder_token");
        assert!(request.extra.is_none());
    }

    #[test]
    fn test_copy_file_request_with_extra() {
        let config = Config::default();
        let request =
            CopyFileRequest::new(config, "file_token", "Demo copy", "docx", "folder_token").extra(
                vec![Property {
                    key: "target_type".to_string(),
                    value: "docx".to_string(),
                }],
            );

        assert_eq!(
            request
                .extra
                .expect("extra should be set when .extra() is called")
                .len(),
            1
        );
    }

    #[test]
    fn test_copy_file_with_empty_file_token() {
        let config = Config::default();
        let request = CopyFileRequest::new(config, "", "name", "docx", "folder_token");
        assert_eq!(request.file_token, "");
        // 空 file_token 应在 execute_with_options 时被校验拦截
    }

    #[test]
    fn test_copy_file_with_invalid_type() {
        let config = Config::default();
        let request = CopyFileRequest::new(config, "file_token", "name", "invalid", "folder_token");
        assert_eq!(request.r#type, "invalid");
        // 无效 type 应在 execute_with_options 时被校验拦截
    }

    #[test]
    fn test_copy_file_with_empty_name() {
        let config = Config::default();
        let request = CopyFileRequest::new(config, "file_token", "", "docx", "folder_token");
        assert_eq!(request.name, "");
        // 空名称应在 execute_with_options 时被校验拦截
    }

    #[test]
    fn test_copy_file_with_too_long_name() {
        let config = Config::default();
        let long_name = "a".repeat(257);
        let request = CopyFileRequest::new(config, "file_token", long_name, "docx", "folder_token");
        assert!(request.name.len() > 256);
        // 过长名称应在 execute_with_options 时被校验拦截
    }

    #[test]
    fn test_copy_file_with_max_length_name() {
        let config = Config::default();
        let max_name = "a".repeat(256);
        let request = CopyFileRequest::new(
            config,
            "file_token",
            max_name.clone(),
            "docx",
            "folder_token",
        );
        assert_eq!(request.name, max_name);
        assert_eq!(request.name.len(), 256);
    }

    #[test]
    fn test_copy_file_with_invalid_user_id_type() {
        let config = Config::default();
        let request = CopyFileRequest::new(config, "file_token", "name", "docx", "folder_token")
            .user_id_type("invalid_type");
        assert_eq!(request.user_id_type, Some("invalid_type".to_string()));
        // 无效 user_id_type 应在 execute_with_options 时被校验拦截
    }

    #[test]
    fn test_copy_file_with_extra_empty_key() {
        let config = Config::default();
        let _request = CopyFileRequest::new(config, "file_token", "name", "docx", "folder_token")
            .extra(vec![Property {
                key: "".to_string(),
                value: "value".to_string(),
            }]);
        // extra 中的空 key 应在 execute_with_options 时被校验拦截
    }

    #[test]
    fn test_copied_file_structure() {
        let file_data = CopiedFile {
            token: "doxcnUkUOWtOelpFcha2Zabcef".to_string(),
            name: "Demo copy".to_string(),
            r#type: "docx".to_string(),
            parent_token: Some("fldbcO1UuPz8VwnpPx5a92abcef".to_string()),
            url: Some("https://feishu.cn/docx/doxcnUkUOWtOelpFcha2Zabcef".to_string()),
            shortcut_info: None,
            created_time: None,
            modified_time: None,
            owner_id: None,
        };

        assert_eq!(file_data.token, "doxcnUkUOWtOelpFcha2Zabcef");
        assert_eq!(file_data.name, "Demo copy");
        assert_eq!(file_data.r#type, "docx");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(CopyFileResponse::data_format(), ResponseFormat::Data);
    }
}
