//! Bitable 列出记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::SDKResult,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use super::models::Record;

/// 列出记录请求
///
/// 用于获取多维表格数据表中的记录列表，支持分页、筛选、排序等功能。
///
/// # 字段说明
///
/// - `app_token`: 多维表格的 app_token，不能为空
/// - `table_id`: 数据表的 table_id，不能为空
/// - `page_token`: 分页标记，用于获取下一页数据
/// - `page_size`: 分页大小，最大 500
/// - `view_id`: 视图的唯一标识符
/// - `filter`: 筛选参数（公式字符串）
/// - `sort`: 排序参数（JSON 数组字符串）
/// - `field_names`: 字段名称（JSON 数组字符串）
/// - `text_field_as_array`: 多行文本字段数据是否以数组形式返回
/// - `user_id_type`: 用户 ID 类型
/// - `display_formula_ref`: 公式和查找引用字段是否以被引用字段格式返回
/// - `automatic_fields`: 是否返回自动计算字段
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_core::config::Config;
/// use openlark_docs::base::bitable::v1::app::table::record::ListRecordRequest;
///
/// let config = Config::builder().app_id("app_id").app_secret("app_secret").build();
/// let request = ListRecordRequest::new(config)
///     .app_token("app_token")
///     .table_id("table_id")
///     .page_size(100)
///     .user_id_type("open_id");
/// let response = request.execute().await?;
/// ```
#[derive(Debug, Clone)]
pub struct ListRecordRequest {
    /// 配置信息
    config: Config,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
    /// 分页标记
    page_token: Option<String>,
    /// 分页大小
    page_size: Option<i32>,
    /// 视图的唯一标识符
    view_id: Option<String>,
    /// 筛选参数（公式字符串）
    filter: Option<String>,
    /// 排序参数（JSON 数组字符串）
    sort: Option<Vec<String>>,
    /// 字段名称（JSON 数组字符串）
    field_names: Option<Vec<String>>,
    /// 控制多行文本字段数据的返回格式
    text_field_as_array: Option<bool>,
    /// 用户 ID 类型
    user_id_type: Option<String>,
    /// 公式和查找引用字段是否以被引用字段格式返回
    display_formula_ref: Option<bool>,
    /// 控制是否返回自动计算字段
    automatic_fields: Option<bool>,
}

impl ListRecordRequest {
    /// 创建列出记录请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            table_id: String::new(),
            page_token: None,
            page_size: None,
            view_id: None,
            filter: None,
            sort: None,
            field_names: None,
            text_field_as_array: None,
            user_id_type: None,
            display_formula_ref: None,
            automatic_fields: None,
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

    /// 设置分页标记
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size.min(500)); // 限制最大500
        self
    }

    /// 设置视图ID
    pub fn view_id(mut self, view_id: String) -> Self {
        self.view_id = Some(view_id);
        self
    }

    /// 设置筛选参数（公式字符串）
    pub fn filter(mut self, filter: impl Into<String>) -> Self {
        self.filter = Some(filter.into());
        self
    }

    /// 设置排序参数（数组会被序列化为 JSON 字符串）
    pub fn sort(mut self, sort: Vec<String>) -> Self {
        self.sort = Some(sort);
        self
    }

    /// 设置字段名称（数组会被序列化为 JSON 字符串）
    pub fn field_names(mut self, field_names: Vec<String>) -> Self {
        self.field_names = Some(field_names);
        self
    }

    /// 控制多行文本字段数据的返回格式，true 表示以数组形式返回
    pub fn text_field_as_array(mut self, text_field_as_array: bool) -> Self {
        self.text_field_as_array = Some(text_field_as_array);
        self
    }

    /// 设置用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 公式和查找引用字段是否以被引用字段格式返回
    pub fn display_formula_ref(mut self, display_formula_ref: bool) -> Self {
        self.display_formula_ref = Some(display_formula_ref);
        self
    }

    /// 控制是否返回自动计算字段
    pub fn automatic_fields(mut self, automatic_fields: bool) -> Self {
        self.automatic_fields = Some(automatic_fields);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListRecordResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListRecordResponse> {
        // === 必填字段验证 ===
        validate_required!(self.app_token.trim(), "应用token不能为空");
        validate_required!(self.table_id.trim(), "数据表ID不能为空");

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

        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::RecordList(self.app_token.clone(), self.table_id.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<ListRecordResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        // 构建查询参数
        if let Some(ref page_token) = self.page_token {
            api_request = api_request.query("page_token", page_token);
        }

        if let Some(page_size) = self.page_size {
            api_request = api_request.query("page_size", &page_size.to_string());
        }

        if let Some(ref view_id) = self.view_id {
            api_request = api_request.query("view_id", view_id);
        }

        api_request = api_request.query_opt("filter", self.filter);

        if let Some(sort) = self.sort {
            api_request = api_request.query("sort", serde_json::to_string(&sort)?);
        }

        if let Some(field_names) = self.field_names {
            api_request = api_request.query("field_names", serde_json::to_string(&field_names)?);
        }

        api_request = api_request.query_opt(
            "text_field_as_array",
            self.text_field_as_array.map(|v| v.to_string()),
        );
        api_request = api_request.query_opt("user_id_type", self.user_id_type);
        api_request = api_request.query_opt(
            "display_formula_ref",
            self.display_formula_ref.map(|v| v.to_string()),
        );
        api_request = api_request.query_opt(
            "automatic_fields",
            self.automatic_fields.map(|v| v.to_string()),
        );

        // 发送请求
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

/// 列出记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListRecordResponse {
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
    pub page_token: Option<String>,
    /// 总数
    pub total: i32,
    /// 本次请求返回的全部记录列表
    pub items: Vec<Record>,
}

impl ApiResponseTrait for ListRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_record_request_builder() {
        let config = Config::default();
        let request = ListRecordRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string())
            .page_size(100)
            .user_id_type("open_id".to_string());

        assert_eq!(request.app_token, "app_token");
        assert_eq!(request.table_id, "table_id");
        assert_eq!(request.page_size, Some(100));
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_page_size_auto_clamp() {
        let config = Config::default();
        let request = ListRecordRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string())
            .page_size(600); // 超过最大值500
        assert_eq!(request.page_size, Some(500)); // 自动限制为500
    }

    #[test]
    fn test_optional_fields() {
        let config = Config::default();
        let request = ListRecordRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string())
            .page_token("page_token".to_string())
            .view_id("view_id".to_string())
            .filter("status == 'active'")
            .sort(vec!["field1".to_string(), "field2".to_string()])
            .field_names(vec!["field1".to_string(), "field2".to_string()])
            .text_field_as_array(true)
            .display_formula_ref(true)
            .automatic_fields(true);

        assert_eq!(request.page_token, Some("page_token".to_string()));
        assert_eq!(request.view_id, Some("view_id".to_string()));
        assert_eq!(request.filter, Some("status == 'active'".to_string()));
        assert_eq!(
            request.sort,
            Some(vec!["field1".to_string(), "field2".to_string()])
        );
        assert_eq!(
            request.field_names,
            Some(vec!["field1".to_string(), "field2".to_string()])
        );
        assert_eq!(request.text_field_as_array, Some(true));
        assert_eq!(request.display_formula_ref, Some(true));
        assert_eq!(request.automatic_fields, Some(true));
    }

    #[test]
    fn test_empty_app_token_validation() {
        let config = Config::default();
        let request = ListRecordRequest::new(config).table_id("table_id".to_string());
        assert_eq!(request.app_token, "");
    }

    #[test]
    fn test_empty_table_id_validation() {
        let config = Config::default();
        let request = ListRecordRequest::new(config).app_token("app_token".to_string());
        assert_eq!(request.table_id, "");
    }

    #[test]
    fn test_invalid_page_size() {
        let config = Config::default();
        let request = ListRecordRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string())
            .page_size(0);
        assert_eq!(request.page_size, Some(0));
    }

    #[test]
    fn test_negative_page_size() {
        let config = Config::default();
        let request = ListRecordRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string())
            .page_size(-10);
        assert_eq!(request.page_size, Some(-10));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(ListRecordResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_response_fields() {
        let response = ListRecordResponse {
            has_more: true,
            page_token: Some("next_page".to_string()),
            total: 100,
            items: vec![],
        };
        assert!(response.has_more);
        assert_eq!(response.page_token, Some("next_page".to_string()));
        assert_eq!(response.total, 100);
        assert_eq!(response.items.len(), 0);
    }
}
