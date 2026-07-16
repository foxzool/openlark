//! 获取文档版本列表
//!
//! 获取指定源文档的版本文档列表。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/drive-v1/file-version/list>

use crate::common::{api_endpoints::DriveApi, api_utils::*};
use openlark_core::{SDKResult, config::Config, http::Transport, validate_required};

use super::models::ListFileVersionsData;

/// 获取文档版本列表请求
#[derive(Debug, Clone)]
pub struct ListFileVersionsRequest {
    config: Config,
    /// 源文档 token
    pub file_token: String,
    /// 分页大小（1~100）
    pub page_size: i32,
    /// 源文档类型（docx/sheet）
    pub obj_type: String,
    /// 分页标记
    pub page_token: Option<String>,
    /// 用户 ID 类型
    pub user_id_type: Option<String>,
}

impl ListFileVersionsRequest {
    /// 创建新的文档版本列表请求。
    pub fn new(
        config: Config,
        file_token: impl Into<String>,
        page_size: i32,
        obj_type: impl Into<String>,
    ) -> Self {
        Self {
            config,
            file_token: file_token.into(),
            page_size,
            obj_type: obj_type.into(),
            page_token: None,
            user_id_type: None,
        }
    }

    /// 设置分页标记。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<ListFileVersionsResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListFileVersionsResponse> {
        // ===== 验证必填字段 =====
        validate_required!(self.file_token, "file_token 不能为空");
        // ===== 验证数值范围 =====
        if !(1..=100).contains(&self.page_size) {
            return Err(openlark_core::error::validation_error(
                "page_size",
                "page_size 必须在 1~100 之间",
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

        let api_endpoint = DriveApi::ListFileVersions(self.file_token);
        let request = api_endpoint
            .to_request::<ListFileVersionsResponse>()
            .query("page_size", self.page_size.to_string())
            .query("obj_type", self.obj_type)
            .query_opt("page_token", self.page_token)
            .query_opt("user_id_type", self.user_id_type);

        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "列表")
    }
}

/// 获取文档版本列表响应（data）
pub type ListFileVersionsResponse = ListFileVersionsData;

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::api::ApiResponseTrait;

    /// 测试构建器模式
    #[test]
    fn test_list_file_versions_request_builder() {
        let config = Config::default();
        let request = ListFileVersionsRequest::new(config, "file_token", 20, "docx")
            .page_token("token123")
            .user_id_type("open_id");

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.page_size, 20);
        assert_eq!(request.obj_type, "docx");
        assert_eq!(request.page_token, Some("token123".to_string()));
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    /// 测试响应格式
    #[test]
    fn test_response_trait() {
        assert_eq!(
            <ListFileVersionsData as ApiResponseTrait>::data_format(),
            openlark_core::api::ResponseFormat::Data
        );
    }

    /// 测试支持的 obj_type 类型
    #[test]
    fn test_supported_obj_types() {
        let config = Config::default();

        for obj_type in ["docx", "sheet"] {
            let request =
                ListFileVersionsRequest::new(config.clone(), "token", 20, obj_type.to_string());
            assert_eq!(request.obj_type, obj_type);
        }
    }

    /// 测试分页参数
    #[test]
    fn test_pagination_parameters() {
        let config = Config::default();
        let request =
            ListFileVersionsRequest::new(config, "token", 20, "docx").page_token("next_page_token");

        assert_eq!(request.page_token, Some("next_page_token".to_string()));
    }

    /// 端到端：GET .../files/{file_token}/versions?page_size&obj_type → 强类型 ListFileVersionsData（单层 data 信封）。
    #[tokio::test]
    async fn test_list_file_versions_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/drive/v1/files/ftk001/versions"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        { "name": "项目文档 第 1 版", "version": "fnJfyX", "obj_type": "docx" },
                        { "name": "项目文档 第 2 版", "version": "fnJfyY", "obj_type": "docx" }
                    ],
                    "page_token": "next_page_token",
                    "has_more": true
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

        let resp = ListFileVersionsRequest::new(config, "ftk001", 20, "docx")
            .execute()
            .await
            .expect("获取文档版本列表应成功");
        assert_eq!(resp.items.len(), 2);
        assert_eq!(resp.items[0].version.as_deref(), Some("fnJfyX"));
        assert!(resp.has_more);
        assert_eq!(resp.page_token.as_deref(), Some("next_page_token"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/drive/v1/files/ftk001/versions"
        );
        assert!(
            received[0]
                .url
                .query()
                .unwrap_or("")
                .contains("page_size=20")
        );
        assert!(
            received[0]
                .url
                .query()
                .unwrap_or("")
                .contains("obj_type=docx")
        );
    }
}
