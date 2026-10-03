//! Bitable 列出字段
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-field/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::SDKResult,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 重用 `Field` 类型。
pub use super::create::Field;

/// 列出字段请求
#[derive(Debug, Clone)]
pub struct ListFieldRequest {
    /// 配置信息
    config: Config,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
    /// 视图 ID
    view_id: Option<String>,
    /// 控制字段描述数据的返回格式
    text_field_as_array: Option<bool>,
    /// 分页标记
    page_token: Option<String>,
    /// 分页大小
    page_size: Option<i32>,
}

impl ListFieldRequest {
    /// 创建新的字段列表请求。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            table_id: String::new(),
            view_id: None,
            text_field_as_array: None,
            page_token: None,
            page_size: None,
        }
    }

    /// 设置多维表格 token。
    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    /// 设置数据表 ID。
    pub fn table_id(mut self, table_id: String) -> Self {
        self.table_id = table_id;
        self
    }

    /// 设置视图 ID。
    pub fn view_id(mut self, view_id: String) -> Self {
        self.view_id = Some(view_id);
        self
    }

    /// 设置文本字段返回为数组格式。
    pub fn text_field_as_array(mut self, text_field_as_array: bool) -> Self {
        self.text_field_as_array = Some(text_field_as_array);
        self
    }

    /// 设置分页标记。
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 设置分页大小。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size.min(100)); // 限制最大100
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<ListFieldResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListFieldResponse> {
        // 参数验证
        validate_required!(self.app_token.trim(), "app_token");

        validate_required!(self.table_id.trim(), "table_id");

        // 验证分页大小
        if let Some(page_size) = self.page_size
            && page_size <= 0
        {
            return Err(openlark_core::error::validation_error(
                "page_size",
                "分页大小必须大于0",
            ));
        }

        // 🚀 使用新的enum+builder系统生成API端点
        // 替代传统的字符串拼接方式，提供类型安全和IDE自动补全
        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::FieldList(self.app_token.clone(), self.table_id.clone());
        // #439: method 来自 catalog
        let mut api_request: ApiRequest<ListFieldResponse> = api_endpoint.to_request();

        // 构建查询参数
        if let Some(ref view_id) = self.view_id {
            api_request = api_request.query("view_id", view_id);
        }

        if let Some(text_field_as_array) = self.text_field_as_array {
            api_request =
                api_request.query("text_field_as_array", &text_field_as_array.to_string());
        }

        if let Some(ref page_token) = self.page_token {
            api_request = api_request.query("page_token", page_token);
        }

        if let Some(page_size) = self.page_size {
            api_request = api_request.query("page_size", &page_size.to_string());
        }

        // 发送请求
        Transport::request_typed(api_request, &self.config, Some(option), "Bitable 列出字段").await
    }
}

/// 列出字段响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListFieldResponse {
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token，否则不返回 page_token
    pub page_token: Option<String>,
    /// 总数
    pub total: i32,
    /// 字段信息
    pub items: Vec<Field>,
}

impl ApiResponseTrait for ListFieldResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET .../fields → ListFieldResponse。
    #[tokio::test]
    async fn test_list_field_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/bitable/v1/apps/app001/tables/tbl001/fields",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "has_more": false, "total": 0, "items": [] }
            })))
            .mount(&server)
            .await;
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();
        ListFieldRequest::new(config)
            .app_token("app001".into())
            .table_id("tbl001".into())
            .execute()
            .await
            .expect("列出字段应成功");
        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/bitable/v1/apps/app001/tables/tbl001/fields"
        );
    }
}
