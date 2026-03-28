//! Bitable 检索记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/get>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::SDKResult,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use super::models::Record;

/// 获取记录请求
///
/// 用于获取多维表格数据表中的单条记录详情。
///
/// # 字段说明
///
/// - `app_token`: 多维表格的 app_token，不能为空
/// - `table_id`: 数据表的 table_id，不能为空
/// - `record_id`: 记录 ID，不能为空
/// - `text_field_as_array`: 多行文本字段数据是否以数组形式返回
/// - `user_id_type`: 用户 ID 类型
/// - `display_formula_ref`: 公式、查找引用是否显示完整原样的返回结果
/// - `with_shared_url`: 是否返回该记录的链接（record_url）
/// - `automatic_fields`: 是否返回自动计算字段
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_core::config::Config;
/// use openlark_docs::base::bitable::v1::app::table::record::GetRecordRequest;
///
/// let config = Config::builder().app_id("app_id").app_secret("app_secret").build();
/// let request = GetRecordRequest::new(config)
///     .app_token("app_token")
///     .table_id("table_id")
///     .record_id("record_id")
///     .user_id_type("open_id");
/// let response = request.execute().await?;
/// ```
#[derive(Debug, Clone)]
pub struct GetRecordRequest {
    config: Config,
    app_token: String,
    table_id: String,
    record_id: String,
    text_field_as_array: Option<bool>,
    user_id_type: Option<String>,
    display_formula_ref: Option<bool>,
    with_shared_url: Option<bool>,
    automatic_fields: Option<bool>,
}

impl GetRecordRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            table_id: String::new(),
            record_id: String::new(),
            text_field_as_array: None,
            user_id_type: None,
            display_formula_ref: None,
            with_shared_url: None,
            automatic_fields: None,
        }
    }

    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    pub fn table_id(mut self, table_id: String) -> Self {
        self.table_id = table_id;
        self
    }

    pub fn record_id(mut self, record_id: String) -> Self {
        self.record_id = record_id;
        self
    }

    /// 多行文本字段数据是否以数组形式返回
    pub fn text_field_as_array(mut self, text_field_as_array: bool) -> Self {
        self.text_field_as_array = Some(text_field_as_array);
        self
    }

    /// 用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 控制公式、查找引用是否显示完整原样的返回结果
    pub fn display_formula_ref(mut self, display_formula_ref: bool) -> Self {
        self.display_formula_ref = Some(display_formula_ref);
        self
    }

    /// 控制是否返回该记录的链接（record_url）
    pub fn with_shared_url(mut self, with_shared_url: bool) -> Self {
        self.with_shared_url = Some(with_shared_url);
        self
    }

    /// 控制是否返回自动计算字段
    pub fn automatic_fields(mut self, automatic_fields: bool) -> Self {
        self.automatic_fields = Some(automatic_fields);
        self
    }

    pub async fn execute(self) -> SDKResult<GetRecordResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetRecordResponse> {
        // === 必填字段验证 ===
        validate_required!(self.app_token.trim(), "app_token");
        validate_required!(self.table_id.trim(), "table_id");
        validate_required!(self.record_id.trim(), "record_id");

        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::RecordGet(
            self.app_token.clone(),
            self.table_id.clone(),
            self.record_id,
        );

        let mut api_request: ApiRequest<GetRecordResponse> =
            ApiRequest::get(&api_endpoint.to_url());

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
            "with_shared_url",
            self.with_shared_url.map(|v| v.to_string()),
        );
        api_request = api_request.query_opt(
            "automatic_fields",
            self.automatic_fields.map(|v| v.to_string()),
        );

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

/// 获取记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetRecordResponse {
    /// 记录
    pub record: Record,
}

impl ApiResponseTrait for GetRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_record_request_builder() {
        let config = Config::default();
        let request = GetRecordRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string())
            .record_id("record_id".to_string())
            .user_id_type("open_id".to_string());

        assert_eq!(request.app_token, "app_token");
        assert_eq!(request.table_id, "table_id");
        assert_eq!(request.record_id, "record_id");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_optional_fields() {
        let config = Config::default();
        let request = GetRecordRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string())
            .record_id("record_id".to_string())
            .text_field_as_array(true)
            .display_formula_ref(true)
            .with_shared_url(true)
            .automatic_fields(true);

        assert_eq!(request.text_field_as_array, Some(true));
        assert_eq!(request.display_formula_ref, Some(true));
        assert_eq!(request.with_shared_url, Some(true));
        assert_eq!(request.automatic_fields, Some(true));
    }

    #[test]
    fn test_empty_app_token_validation() {
        let config = Config::default();
        let request = GetRecordRequest::new(config)
            .app_token("".to_string())
            .table_id("table_id".to_string())
            .record_id("record_id".to_string());
        assert_eq!(request.app_token, "");
    }

    #[test]
    fn test_empty_table_id_validation() {
        let config = Config::default();
        let request = GetRecordRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("".to_string())
            .record_id("record_id".to_string());
        assert_eq!(request.table_id, "");
    }

    #[test]
    fn test_empty_record_id_validation() {
        let config = Config::default();
        let request = GetRecordRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string())
            .record_id("".to_string());
        assert_eq!(request.record_id, "");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(GetRecordResponse::data_format(), ResponseFormat::Data);
    }
}
