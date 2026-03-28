//! Bitable 列出视图
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-view/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::SDKResult,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

// 从 patch 模块导入 View 类型
use super::patch::View;

/// 列出视图请求
///
/// 用于获取多维表格数据表中的所有视图列表。
///
/// # 字段说明
///
/// - `app_token`: 多维表格的 app_token
/// - `table_id`: 数据表的 table_id
/// - `user_id_type`: 用户 ID 类型（可选）
/// - `page_token`: 分页标记（可选）
/// - `page_size`: 分页大小，必须大于0，最大100（可选）
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_docs::base::bitable::v1::app::table::view::list::ListViewsRequest;
/// use openlark_core::Config;
///
/// let config = Config::default();
/// let request = ListViewsRequest::new(config)
///     .app_token("app_token_xyz".to_string())
///     .table_id("table_id_xyz".to_string())
///     .page_size(50);
/// ```
#[derive(Debug, Clone)]
pub struct ListViewsRequest {
    /// 配置信息
    config: Config,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
    /// 用户 ID 类型
    user_id_type: Option<String>,
    /// 分页标记
    page_token: Option<String>,
    /// 分页大小
    page_size: Option<i32>,
}

impl ListViewsRequest {
    /// 创建列出视图请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            table_id: String::new(),
            user_id_type: None,
            page_token: None,
            page_size: None,
        }
    }

    /// 设置应用token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    /// 设置数据表ID
    pub fn table_id(mut self, table_id: String) -> Self {
        self.table_id = table_id;
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size.min(100)); // 限制最大100
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListViewsResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<ListViewsResponse> {
        // === 必填字段验证 ===
        validate_required!(self.app_token.trim(), "app_token");

        validate_required!(self.table_id.trim(), "table_id");

        // === 业务规则验证 ===
        // 验证分页大小
        if let Some(page_size) = self.page_size {
            if page_size <= 0 {
                return Err(openlark_core::error::validation_error(
                    "page_size",
                    "分页大小必须大于0",
                ));
            }
        }

        // 🚀 使用新的enum+builder系统生成API端点
        // 替代传统的字符串拼接方式，提供类型安全和IDE自动补全
        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::ViewList(self.app_token.clone(), self.table_id.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<ListViewsResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        // 构建查询参数
        if let Some(ref user_id_type) = self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type);
        }

        if let Some(ref page_token) = self.page_token {
            api_request = api_request.query("page_token", page_token);
        }

        if let Some(page_size) = self.page_size {
            api_request = api_request.query("page_size", &page_size.to_string());
        }

        // 发送请求
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

/// 列出视图响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListViewsResponse {
    /// 视图信息
    pub items: Vec<View>,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
    pub page_token: Option<String>,
    /// 是否还有更多项
    pub has_more: bool,
    /// 总数
    pub total: i32,
}

impl ApiResponseTrait for ListViewsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::testing::prelude::test_runtime;

    #[test]
    fn test_empty_app_token() {
        let config = Config::default();
        let request = ListViewsRequest::new(config)
            .app_token("".to_string())
            .table_id("table_id".to_string());

        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("app_token"));
    }

    #[test]
    fn test_empty_table_id() {
        let config = Config::default();
        let request = ListViewsRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("".to_string());

        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("table_id"));
    }

    #[test]
    fn test_invalid_page_size_zero() {
        let config = Config::default();
        let request = ListViewsRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string())
            .page_size(0);

        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("page_size"));
    }

    #[test]
    fn test_invalid_page_size_negative() {
        let config = Config::default();
        let request = ListViewsRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string())
            .page_size(-1);

        let rt = test_runtime();
        let result = rt.block_on(request.execute());

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("page_size"));
    }

    #[test]
    fn test_list_views_request_builder() {
        let config = Config::default();
        let request = ListViewsRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string())
            .page_size(50);

        assert_eq!(request.app_token, "app_token");
        assert_eq!(request.table_id, "table_id");
        assert_eq!(request.page_size, Some(50));
    }

    #[test]
    fn test_page_size_limit() {
        let config = Config::default();
        let request = ListViewsRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string())
            .page_size(150); // 超过100，会被限制

        assert_eq!(request.page_size, Some(100)); // 应该被限制为100
    }
}
