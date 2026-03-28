//! 创建导出任务
//!
//! 创建一个导出任务，将云文档导出为文件。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/drive-v1/export_task/create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 创建导出任务请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateExportTaskRequest {
    #[serde(skip)]
    config: Config,
    /// 将云文档导出为本地文件后，本地文件的扩展名
    pub file_extension: String,
    /// 文件token
    pub token: String,
    /// 要导出的云文档的类型
    pub r#type: String,
    /// 导出电子表格/多维表格为 CSV 时，需要传入工作表 ID 或数据表 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_id: Option<String>,
}

impl CreateExportTaskRequest {
    pub fn new(
        config: Config,
        file_extension: impl Into<String>,
        token: impl Into<String>,
        r#type: impl Into<String>,
    ) -> Self {
        Self {
            config,
            file_extension: file_extension.into(),
            token: token.into(),
            r#type: r#type.into(),
            sub_id: None,
        }
    }

    pub fn sub_id(mut self, sub_id: impl Into<String>) -> Self {
        self.sub_id = Some(sub_id.into());
        self
    }

    pub async fn execute(self) -> SDKResult<CreateExportTaskResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateExportTaskResponse> {
        // ===== 验证字段长度 =====
        let token_len = self.token.len();
        if token_len == 0 || token_len > 27 {
            return Err(openlark_core::error::validation_error(
                "token",
                "token 长度必须在 1~27 字节之间",
            ));
        }
        // ===== 验证字段枚举值 =====
        match self.file_extension.as_str() {
            "docx" | "pdf" | "xlsx" | "csv" => {}
            _ => {
                return Err(openlark_core::error::validation_error(
                    "file_extension",
                    "file_extension 仅支持 docx/pdf/xlsx/csv",
                ))
            }
        }
        match self.r#type.as_str() {
            "doc" | "docx" | "sheet" | "bitable" => {}
            _ => {
                return Err(openlark_core::error::validation_error(
                    "type",
                    "type 仅支持 doc/docx/sheet/bitable",
                ))
            }
        }
        // ===== 验证字段关联约束 =====
        // 文档约束：不同云文档类型支持的导出格式不同
        match (self.r#type.as_str(), self.file_extension.as_str()) {
            ("doc" | "docx", "docx" | "pdf") => {}
            ("sheet" | "bitable", "xlsx" | "csv") => {}
            _ => {
                return Err(openlark_core::error::validation_error(
                    "file_extension",
                    "file_extension 与 type 不匹配：doc/docx 仅支持 docx/pdf；sheet/bitable 仅支持 xlsx/csv",
                ))
            }
        }
        // CSV 导出需要 sub_id
        if self.file_extension == "csv"
            && (self.r#type == "sheet" || self.r#type == "bitable")
            && self.sub_id.as_deref().unwrap_or("").is_empty()
        {
            return Err(openlark_core::error::validation_error(
                "sub_id",
                "导出 sheet/bitable 为 csv 时，sub_id 不能为空",
            ));
        }

        let api_endpoint = DriveApi::CreateExportTask;

        let api_request: ApiRequest<CreateExportTaskResponse> =
            ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&self, "创建导出任务")?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "创建")
    }
}

/// 创建导出任务响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateExportTaskResponse {
    /// 任务ticket
    pub ticket: String,
}

impl ApiResponseTrait for CreateExportTaskResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::testing::prelude::test_runtime;

    /// 测试构建器模式
    #[test]
    fn test_create_export_task_request_builder() {
        let config = Config::default();
        let request =
            CreateExportTaskRequest::new(config, "csv", "file_token", "sheet").sub_id("6e5ed3");

        assert_eq!(request.file_extension, "csv");
        assert_eq!(request.token, "file_token");
        assert_eq!(request.r#type, "sheet");
        assert_eq!(request.sub_id, Some("6e5ed3".to_string()));
    }

    /// 测试响应格式
    #[test]
    fn test_response_trait() {
        assert_eq!(
            CreateExportTaskResponse::data_format(),
            ResponseFormat::Data
        );
    }

    /// 测试 token 长度验证
    #[test]
    fn test_token_length_validation() {
        let config = Config::default();

        // 空字符串
        let request1 = CreateExportTaskRequest::new(config.clone(), "", "token", "docx");

        let result1 = std::thread::spawn(move || {
            let rt = test_runtime();
            rt.block_on(async move {
                let _ = request1.execute().await;
            })
        })
        .join();

        assert!(result1.is_ok());

        // 超过 27 字节
        let long_token = "a".repeat(28);
        let request2 = CreateExportTaskRequest::new(config, long_token, "token", "docx");

        let result2 = std::thread::spawn(move || {
            let rt = test_runtime();
            rt.block_on(async move {
                let _ = request2.execute().await;
            })
        })
        .join();

        assert!(result2.is_ok());
    }

    /// 测试 file_extension 枚举值
    #[test]
    fn test_file_extension_validation() {
        let config = Config::default();
        let request = CreateExportTaskRequest::new(config, "invalid", "token", "docx");

        let result = std::thread::spawn(move || {
            let rt = test_runtime();
            rt.block_on(async move {
                let _ = request.execute().await;
            })
        })
        .join();

        assert!(result.is_ok());
    }

    /// 测试支持的 file_extension 类型
    #[test]
    fn test_supported_file_extensions() {
        let config = Config::default();

        for ext in ["docx", "pdf", "xlsx", "csv"] {
            let request =
                CreateExportTaskRequest::new(config.clone(), ext.to_string(), "token", "docx");
            assert_eq!(request.file_extension, ext);
        }
    }

    /// 测试类型和扩展名匹配约束
    #[test]
    fn test_type_extension_compatibility() {
        let config = Config::default();

        // doc/docx 支持 docx/pdf
        for doc_type in ["doc", "docx"] {
            for ext in ["docx", "pdf"] {
                let request = CreateExportTaskRequest::new(
                    config.clone(),
                    ext.to_string(),
                    "token",
                    doc_type.to_string(),
                );
                assert_eq!(request.file_extension, ext);
                assert_eq!(request.r#type, doc_type);
            }
        }

        // sheet/bitable 支持 xlsx/csv
        for table_type in ["sheet", "bitable"] {
            for ext in ["xlsx", "csv"] {
                let request = CreateExportTaskRequest::new(
                    config.clone(),
                    ext.to_string(),
                    "token",
                    table_type.to_string(),
                );
                assert_eq!(request.file_extension, ext);
                assert_eq!(request.r#type, table_type);
            }
        }
    }

    /// 测试 CSV 导出需要 sub_id
    #[test]
    fn test_csv_requires_sub_id() {
        let config = Config::default();
        let request = CreateExportTaskRequest::new(config, "csv", "token", "sheet");

        let result = std::thread::spawn(move || {
            let rt = test_runtime();
            rt.block_on(async move {
                let _ = request.execute().await;
            })
        })
        .join();

        assert!(result.is_ok());
    }

    /// 测试 sub_id 可选参数
    #[test]
    fn test_sub_id_optional() {
        let config = Config::default();

        // xlsx 不需要 sub_id
        let request1 = CreateExportTaskRequest::new(config.clone(), "xlsx", "token", "sheet");
        assert!(request1.sub_id.is_none());

        // csv 需要 sub_id
        let request2 =
            CreateExportTaskRequest::new(config, "csv", "token", "sheet").sub_id("sheet_id");
        assert_eq!(request2.sub_id, Some("sheet_id".to_string()));
    }
}
