//! 创建导入任务
//!
//! 创建导入任务，支持导入为新版文档、电子表格、多维表格以及旧版文档（异步接口）。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/drive-v1/import_task/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 创建导入任务请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateImportTaskRequest {
    #[serde(skip)]
    /// SDK 配置。
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
    /// 创建新的导入任务请求。
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

    /// 设置导入后展示的文件名。
    pub fn file_name(mut self, file_name: impl Into<String>) -> Self {
        self.file_name = Some(file_name.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<CreateImportTaskResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateImportTaskResponse> {
        // ===== 验证必填字段 =====
        validate_required!(self.file_extension, "file_extension 不能为空");
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
                ));
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

    /// 端到端：POST /open-apis/drive/v1/import_tasks → CreateImportTaskResponse（ticket）。
    #[tokio::test]
    async fn test_create_import_task_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/drive/v1/import_tasks"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "ticket": "import_ticket_001"
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

        let resp = CreateImportTaskRequest::new(
            config,
            "pdf",
            "fileTokenAbc",
            "docx",
            Point::new("AbqrfuRTjlJEIJduwDwcnIabcef"),
        )
        .execute()
        .await
        .expect("创建导入任务应成功");
        assert_eq!(resp.ticket, "import_ticket_001");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/drive/v1/import_tasks");
    }
}
