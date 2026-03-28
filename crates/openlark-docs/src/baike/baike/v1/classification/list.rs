//! 获取词典分类
//!
//! 获取百科知识库的分类列表。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/baike-v1/classification/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::super::models::ClassificationItem;
use crate::common::api_endpoints::BaikeApiV1;

/// 获取词典分类请求
///
/// 用于获取百科知识库的分类列表，支持分页。
///
/// # 字段说明
///
/// - `page_size`: 分页大小，范围 1~500
/// - `page_token`: 分页标记
///
/// # 示例
///
/// ```rust,no_run
/// use openlark_docs::baike::baike::v1::classification::ListClassificationRequest;
/// use openlark_core::prelude::Config;
///
/// # fn example() {
/// let config = Config::builder()
///     .app_id("app_id")
///     .app_secret("app_secret")
///     .build();
/// let request = ListClassificationRequest::new(config).page_size(50);
/// # }
/// ```
pub struct ListClassificationRequest {
    config: Config,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl ListClassificationRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
        }
    }

    /// 分页大小（默认 20）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 分页 token
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    pub async fn execute(self) -> SDKResult<ListClassificationResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListClassificationResponse> {
        // ===== 参数校验 =====
        if let Some(page_size) = self.page_size {
            if !(1..=500).contains(&page_size) {
                return Err(openlark_core::error::validation_error(
                    "page_size",
                    "page_size 取值范围必须为 1~500",
                ));
            }
        }

        // ===== 构建请求 =====
        let mut api_request: ApiRequest<ListClassificationResponse> =
            ApiRequest::get(&BaikeApiV1::ClassificationList.to_url());
        if let Some(page_size) = self.page_size {
            api_request = api_request.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = &self.page_token {
            api_request = api_request.query("page_token", page_token);
        }

        // ===== 发送请求 =====
        let response: Response<ListClassificationResponse> =
            Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

/// 获取词典分类响应（data）
///
/// 包含分类列表和分页信息。
///
/// # 字段说明
///
/// - `items`: 分类列表
/// - `page_token`: 下一页的分页标记
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ListClassificationResponse {
    #[serde(default)]
    pub items: Vec<ClassificationItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListClassificationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_classification_request_builder() {
        let config = Config::default();
        let request = ListClassificationRequest::new(config);

        assert!(request.page_size.is_none());
        assert!(request.page_token.is_none());
    }

    #[test]
    fn test_list_classification_request_with_params() {
        let config = Config::default();
        let request = ListClassificationRequest::new(config)
            .page_size(100)
            .page_token("token123");

        assert_eq!(request.page_size, Some(100));
        assert_eq!(request.page_token, Some("token123".to_string()));
    }

    #[test]
    fn test_list_classification_response_structure() {
        let response = ListClassificationResponse {
            items: vec![],
            page_token: Some("next_token".to_string()),
        };

        assert!(response.items.is_empty());
        assert_eq!(response.page_token, Some("next_token".to_string()));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            ListClassificationResponse::data_format(),
            ResponseFormat::Data
        );
    }

    #[test]
    fn test_page_size_validation() {
        let sizes = vec![1, 50, 100, 500];
        for size in sizes {
            let config = Config::default();
            let request = ListClassificationRequest::new(config).page_size(size);
            assert_eq!(request.page_size, Some(size));
        }
    }
}
