/// 导出任务API数据模型
///
/// 定义文档导出任务API的数据结构。
use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 创建导出任务请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateExportTaskRequest {
    /// 文件token
    pub file_token: String,
    /// 导出文件类型
    pub file_type: String,
    /// 导出格式
    pub export_type: String,
    /// 导出名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_name: Option<String>,
    /// 导出语言
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    /// 是否包含评论
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_comments: Option<bool>,
    /// 是否启用下载密码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_enable: Option<bool>,
    /// 下载密码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// 是否分离导出附件
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_separate: Option<bool>,
}

impl CreateExportTaskRequest {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        use openlark_core::validate_required;
        validate_required!(self.file_token, "文件token不能为空");
        validate_required!(self.file_type, "导出文件类型不能为空");
        validate_required!(self.export_type, "导出格式不能为空");

        // 验证导出格式
        let valid_export_types = vec!["docx", "pdf", "html", "xlsx", "csv"];
        if !valid_export_types.contains(&self.export_type.as_str()) {
            return Err(openlark_core::CoreError::validation_msg(format!("不支持的导出格式: {}", self.export_type)));
        }

        // 验证文件类型
        let valid_file_types = vec!["doc", "docx", "sheet", "bitable"];
        if !valid_file_types.contains(&self.file_type.as_str()) {
            return Err(openlark_core::CoreError::validation_msg(format!("不支持的文件类型: {}", self.file_type)));
        }

        if let Some(ref export_name) = self.export_name {
            validate_required!(export_name, "导出名称不能为空");
            if export_name.len() > 200 {
                return Err(openlark_core::CoreError::validation_msg("导出名称长度不能超过200个字符"));
            }
        }

        if let Some(ref locale) = self.locale {
            validate_required!(locale, "导出语言不能为空");
        }

        if let Some(ref password) = self.password {
            if password.len() > 20 {
                return Err(openlark_core::CoreError::validation_msg("下载密码长度不能超过20个字符"));
            }
        }

        Ok(())
    }
    pub fn validate(&self) -> Result<(), String> {
        if self.file_token.trim().is_empty() {
            return Err("文件token不能为空".to_string());
        }

        if self.file_type.trim().is_empty() {
            return Err("导出文件类型不能为空".to_string());
        }

        if self.export_type.trim().is_empty() {
            return Err("导出格式不能为空".to_string());
        }

        // 验证导出格式
        let valid_export_types = vec!["docx", "pdf", "html", "xlsx", "csv"];
        if !valid_export_types.contains(&self.export_type.as_str()) {
            return Err(format!("不支持的导出格式: {}", self.export_type));
        }

        // 验证文件类型
        let valid_file_types = vec!["doc", "docx", "sheet", "bitable"];
        if !valid_file_types.contains(&self.file_type.as_str()) {
            return Err(format!("不支持的文件类型: {}", self.file_type));
        }

        if let Some(ref export_name) = self.export_name {
            if export_name.trim().is_empty() {
                return Err("导出名称不能为空".to_string());
            }
            if export_name.len() > 200 {
                return Err("导出名称长度不能超过200个字符".to_string());
            }
        }

        if let Some(ref locale) = self.locale {
            if locale.trim().is_empty() {
                return Err("导出语言不能为空".to_string());
            }
        }

        if let Some(ref password) = self.password {
            if password.len() > 20 {
                return Err("下载密码长度不能超过20个字符".to_string());
            }
        }

        Ok(())
    }
}

/// 创建导出任务响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CreateExportTaskResponse {
    /// 任务票据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket: Option<String>,
}

impl ApiResponseTrait for CreateExportTaskResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询导出任务结果请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetExportTaskRequest {
    /// 任务票据
    pub ticket: String,
}

impl GetExportTaskRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.ticket.trim().is_empty() {
            return Err("任务票据不能为空".to_string());
        }
        Ok(())
    }
}

/// 导出任务状态
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExportTaskStatus {
    /// 处理中
    Processing,
    /// 成功
    Success,
    /// 失败
    Failed,
    /// 取消
    Cancelled,
}

/// 导出任务结果
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ExportTaskResult {
    /// 导出状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ExportTaskStatus>,
    /// 错误信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// 导出文件token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_token: Option<String>,
    /// 导出文件URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_url: Option<String>,
    /// 导出文件大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
}

/// 查询导出任务结果响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetExportTaskResponse {
    /// 任务结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<ExportTaskResult>,
}

impl ApiResponseTrait for GetExportTaskResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 下载导出文件请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DownloadExportFileRequest {
    /// 导出文件token
    pub file_token: String,
}

impl DownloadExportFileRequest {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.file_token.trim().is_empty() {
            return Err("导出文件token不能为空".to_string());
        }
        Ok(())
    }
}

/// 下载导出文件响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DownloadExportFileResponse {
    /// 文件内容（Base64编码）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_content: Option<String>,
    /// 文件名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// 文件大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_size: Option<i64>,
    /// 文件类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
}

impl ApiResponseTrait for DownloadExportFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_roundtrip<T: Serialize + for<'de> Deserialize<'de> + PartialEq + std::fmt::Debug>(
        original: &T,
    ) {
        let json = serde_json::to_string(original).expect("序列化失败");
        let deserialized: T = serde_json::from_str(&json).expect("反序列化失败");
        assert_eq!(original, &deserialized, "roundtrip 后数据不一致");
    }

    #[test]
    fn test_create_export_task_request_serialization() {
        let req = CreateExportTaskRequest {
            file_token: "file123".to_string(),
            file_type: "docx".to_string(),
            export_type: "pdf".to_string(),
            export_name: Some("导出文件".to_string()),
            locale: Some("zh-CN".to_string()),
            include_comments: Some(true),
            password_enable: Some(false),
            password: None,
            attachment_separate: None,
        };
        test_roundtrip(&req);
    }

    #[test]
    fn test_create_export_task_response_serialization() {
        let resp = CreateExportTaskResponse {
            ticket: Some("ticket123".to_string()),
        };
        test_roundtrip(&resp);
    }

    #[test]
    fn test_get_export_task_request_serialization() {
        let req = GetExportTaskRequest {
            ticket: "ticket123".to_string(),
        };
        test_roundtrip(&req);
    }

    #[test]
    fn test_export_task_status_serialization() {
        test_roundtrip(&ExportTaskStatus::Processing);
        test_roundtrip(&ExportTaskStatus::Success);
        test_roundtrip(&ExportTaskStatus::Failed);
        test_roundtrip(&ExportTaskStatus::Cancelled);
    }

    #[test]
    fn test_export_task_result_serialization() {
        let result = ExportTaskResult {
            status: Some(ExportTaskStatus::Success),
            error_message: None,
            file_token: Some("file456".to_string()),
            file_url: Some("https://example.com/file".to_string()),
            file_size: Some(1024000),
        };
        test_roundtrip(&result);
    }

    #[test]
    fn test_get_export_task_response_serialization() {
        let resp = GetExportTaskResponse {
            result: Some(ExportTaskResult {
                status: Some(ExportTaskStatus::Processing),
                error_message: None,
                file_token: None,
                file_url: None,
                file_size: None,
            }),
        };
        test_roundtrip(&resp);
    }

    #[test]
    fn test_download_export_file_request_serialization() {
        let req = DownloadExportFileRequest {
            file_token: "file789".to_string(),
        };
        test_roundtrip(&req);
    }

    #[test]
    fn test_download_export_file_response_serialization() {
        let resp = DownloadExportFileResponse {
            file_content: Some("base64content".to_string()),
            file_name: Some("exported.pdf".to_string()),
            file_size: Some(1024000),
            content_type: Some("application/pdf".to_string()),
        };
        test_roundtrip(&resp);
    }
}
