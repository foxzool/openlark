//! 创建导入任务
//!
//! 创建导入任务，支持导入为新版文档、电子表格、多维表格以及旧版文档（异步接口）。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/drive-v1/import_task/create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 创建导入任务请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateImportTaskRequest {
    #[serde(skip)]
    config: Config,
    /// 文件扩展名
    pub file_extension: String,
    /// 文件token
    pub file_token: String,
    /// 目标类型
    pub r#type: String,
    /// 文件名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_name: Option<String>,
    /// 目录token
    pub point: Point,
}

/// 目标目录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    /// 挂载类型
    pub mount_type: i32,
    /// 挂载key
    pub mount_key: String,
}

impl Point {
    /// 创建挂载点（挂载到云空间，mount_type 固定为 1）
    pub fn new(mount_key: impl Into<String>) -> Self {
        Self {
            mount_type: 1,
            mount_key: mount_key.into(),
        }
    }
}

impl CreateImportTaskRequest {
    pub fn new(
        config: Config,
        file_extension: impl Into<String>,
        file_token: impl Into<String>,
        r#type: impl Into<String>,
        point: Point,
    ) -> Self {
        Self {
            config,
            file_extension: file_extension.into(),
            file_token: file_token.into(),
            r#type: r#type.into(),
            file_name: None,
            point,
        }
    }

    pub fn file_name(mut self, file_name: impl Into<String>) -> Self {
        self.file_name = Some(file_name.into());
        self
    }

    pub async fn execute(self) -> SDKResult<CreateImportTaskResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateImportTaskResponse> {
        // ===== 验证必填字段 =====
        if self.file_extension.is_empty() {
            return Err(openlark_core::error::validation_error(
                "file_extension",
                "file_extension 不能为空",
            ));
        }
        // ===== 验证字段长度 =====
        let file_token_len = self.file_token.len();
        if file_token_len == 0 || file_token_len > 27 {
            return Err(openlark_core::error::validation_error(
                "file_token",
                "file_token 长度必须在 1~27 字节之间",
            ));
        }
        // ===== 验证字段枚举值 =====
        match self.r#type.as_str() {
            "docx" | "sheet" | "bitable" => {}
            _ => {
                return Err(openlark_core::error::validation_error(
                    "type",
                    "type 仅支持 docx/sheet/bitable",
                ))
            }
        }
        // ===== 验证固定值 =====
        if self.point.mount_type != 1 {
            return Err(openlark_core::error::validation_error(
                "point.mount_type",
                "point.mount_type 仅支持固定值 1（挂载到云空间）",
            ));
        }

        let api_endpoint = DriveApi::CreateImportTask;

        let api_request: ApiRequest<CreateImportTaskResponse> =
            ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&self, "创建导入任务")?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "创建")
    }
}

/// 创建导入任务响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateImportTaskResponse {
    /// 任务ticket
    pub ticket: String,
}

impl ApiResponseTrait for CreateImportTaskResponse {
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
    fn test_create_import_task_request_builder() {
        let config = Config::default();
        let request = CreateImportTaskRequest::new(
            config,
            "pdf",
            "file_token",
            "sheet",
            Point::new("AbqrfuRTjlJEIJduwDwcnIabcef"),
        )
        .file_name("test_file");

        assert_eq!(request.file_extension, "pdf");
        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.r#type, "sheet");
        assert_eq!(request.file_name, Some("test_file".to_string()));
    }

    /// 测试 Point 结构
    #[test]
    fn test_point_structure() {
        let point = Point::new("mount_key");

        assert_eq!(point.mount_type, 1);
        assert_eq!(point.mount_key, "mount_key");
    }

    /// 测试响应格式
    #[test]
    fn test_response_trait() {
        assert_eq!(
            CreateImportTaskResponse::data_format(),
            ResponseFormat::Data
        );
    }

    /// 测试 file_extension 为空时的验证
    #[test]
    fn test_empty_file_extension_validation() {
        let config = Config::default();
        let point = Point::new("mount_key");
        let request = CreateImportTaskRequest::new(config, "", "file_token", "sheet", point);

        let result = std::thread::spawn(move || {
            let rt = test_runtime();
            rt.block_on(async move {
                let _ = request.execute().await;
            })
        })
        .join();

        assert!(result.is_ok());
    }

    /// 测试 file_token 长度验证
    #[test]
    fn test_file_token_length_validation() {
        let config = Config::default();
        let point = Point::new("mount_key");

        // 空字符串
        let request1 =
            CreateImportTaskRequest::new(config.clone(), "pdf", "", "sheet", point.clone());

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
        let request2 = CreateImportTaskRequest::new(config, "pdf", long_token, "sheet", point);

        let result2 = std::thread::spawn(move || {
            let rt = test_runtime();
            rt.block_on(async move {
                let _ = request2.execute().await;
            })
        })
        .join();

        assert!(result2.is_ok());
    }

    /// 测试 type 枚举值验证
    #[test]
    fn test_type_validation() {
        let config = Config::default();
        let point = Point::new("mount_key");
        let request = CreateImportTaskRequest::new(config, "pdf", "file_token", "invalid", point);

        let result = std::thread::spawn(move || {
            let rt = test_runtime();
            rt.block_on(async move {
                let _ = request.execute().await;
            })
        })
        .join();

        assert!(result.is_ok());
    }

    /// 测试支持的 type 类型
    #[test]
    fn test_supported_types() {
        let config = Config::default();
        let point = Point::new("mount_key");

        for import_type in ["docx", "sheet", "bitable"] {
            let request = CreateImportTaskRequest::new(
                config.clone(),
                "pdf",
                "file_token",
                import_type.to_string(),
                point.clone(),
            );
            assert_eq!(request.r#type, import_type);
        }
    }

    /// 测试 file_name 可选参数
    #[test]
    fn test_file_name_optional() {
        let config = Config::default();
        let point = Point::new("mount_key");

        let request1 = CreateImportTaskRequest::new(
            config.clone(),
            "pdf",
            "file_token",
            "sheet",
            point.clone(),
        );
        assert!(request1.file_name.is_none());

        let request2 = CreateImportTaskRequest::new(config, "pdf", "file_token", "sheet", point)
            .file_name("custom_name");
        assert_eq!(request2.file_name, Some("custom_name".to_string()));
    }
}
