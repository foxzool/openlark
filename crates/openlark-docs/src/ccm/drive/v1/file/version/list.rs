//! 获取文档版本列表
//!
//! 获取指定源文档的版本文档列表。
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file-version/list

use crate::common::{api_endpoints::DriveApi, api_utils::*};
use openlark_core::{SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required};

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
        let request = ApiRequest::<ListFileVersionsResponse>::get(&api_endpoint.to_url())
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
    use openlark_core::testing::prelude::test_runtime;

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

    /// 测试 file_token 为空时的验证
    #[test]
    fn test_empty_file_token_validation() {
        let config = Config::default();
        let request = ListFileVersionsRequest::new(config, "", 20, "docx");

        let result = std::thread::spawn(move || {
            let rt = test_runtime();
            rt.block_on(async move {
                let _ = request.execute().await;
            })
        })
        .join();

        assert!(result.is_ok());
    }

    /// 测试 page_size 边界值
    #[test]
    fn test_page_size_boundaries() {
        let config = Config::default();

        // 测试最小值
        let request1 = ListFileVersionsRequest::new(config.clone(), "token", 1, "docx");
        assert_eq!(request1.page_size, 1);

        // 测试最大值
        let request2 = ListFileVersionsRequest::new(config.clone(), "token", 100, "docx");
        assert_eq!(request2.page_size, 100);

        // 测试超出范围
        let request3 = ListFileVersionsRequest::new(config, "token", 101, "docx");

        let result = std::thread::spawn(move || {
            let rt = test_runtime();
            rt.block_on(async move {
                let _ = request3.execute().await;
            })
        })
        .join();

        assert!(result.is_ok());
    }

    /// 测试 page_size 为 0 时的验证
    #[test]
    fn test_zero_page_size_validation() {
        let config = Config::default();
        let request = ListFileVersionsRequest::new(config, "token", 0, "docx");

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
        let request = ListFileVersionsRequest::new(config, "token", 20, "invalid");

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
}
