//! 批量查询算薪项
//!
//! docPath: <https://open.feishu.cn/document/server-docs/payroll-v1/acct_item/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::validation_error,
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 批量查询算薪项请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 分页大小（必填，取值范围 1~100）
    page_size: Option<i32>,
    /// 分页标记（可选，第一次请求不填，表示从头开始遍历）
    page_token: Option<String>,
    /// 配置信息
    config: Config,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            page_size: None,
            page_token: None,
            config,
        }
    }

    /// 设置分页大小（必填，取值范围 1~100）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记（可选）
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListResponse> {
        use crate::common::api_endpoints::PayrollApiV1;

        // 1. 验证必填字段
        let page_size = self.page_size.ok_or_else(|| {
            validation_error("page_size 不能为空", "分页大小必填，取值范围 1~100")
        })?;
        if !(1..=100).contains(&page_size) {
            return Err(validation_error(
                "page_size 超出范围",
                "分页大小取值范围为 1~100",
            ));
        }

        // 2. 构建端点
        let api_endpoint = PayrollApiV1::AcctItemList;
        let mut request = ApiRequest::<ListResponse>::get(api_endpoint.to_url());

        // 3. 添加查询参数
        request = request.query("page_size", page_size.to_string());
        if let Some(ref page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        // 4. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "批量查询算薪项响应数据为空",
        )
        .await
    }
}

/// 批量查询算薪项响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 算薪项列表
    pub items: Vec<AcctItem>,
    /// 分页标记，当 has_more 为 true 时返回，否则不返回
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多项
    pub has_more: bool,
}

/// 算薪项信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AcctItem {
    /// 算薪项 ID
    pub id: String,
    /// 算薪项名称（多语种）
    pub i18n_names: Vec<I18nContent>,
    /// 算薪项分类 ID
    pub category_id: String,
    /// 算薪项数据类型：1-文本 2-金额 3-数值 4-百分数 5-日期 6-引用项
    pub data_type: i32,
    /// 小数位数
    pub decimal_places: i32,
    /// 启用状态：1-已启用 2-已停用
    pub active_status: i32,
}

/// 多语种内容
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nContent {
    /// 语种，如 zh_cn、en_us
    pub locale: String,
    /// 语种对应的值
    pub value: String,
}

impl ApiResponseTrait for ListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;

    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_list_acct_items_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 官方响应示例形状
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{
                "items": [{
                    "id": "7169773973790425132",
                    "i18n_names": [{"locale": "zh_cn", "value": "名称"}],
                    "category_id": "7169773973790425132",
                    "data_type": 1,
                    "decimal_places": 2,
                    "active_status": 1
                }],
                "page_token": "eVQrYzJBNDNONlk4VFZBZVlSdzlKdFJ4bVVHVExENDNKVHoxaVdiVnViQT0=",
                "has_more": true
            }"#,
        )
        .unwrap();
        Mock::given(method("GET"))
            .and(path("/open-apis/payroll/v1/acct_items"))
            .and(query_param("page_size", "20"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": data_body
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = ListRequest::new(config)
            .page_size(20)
            .execute()
            .await
            .expect("批量查询算薪项应成功");

        assert!(data.has_more);
        assert_eq!(data.items.len(), 1);
        assert_eq!(data.items[0].id, "7169773973790425132");
        assert_eq!(data.items[0].i18n_names[0].locale, "zh_cn");
        assert_eq!(data.items[0].i18n_names[0].value, "名称");
        assert_eq!(data.items[0].category_id, "7169773973790425132");
        assert_eq!(data.items[0].data_type, 1);
        assert_eq!(data.items[0].decimal_places, 2);
        assert_eq!(data.items[0].active_status, 1);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/payroll/v1/acct_items");
    }

    /// page_size 为官方必填参数，缺失时应校验失败
    #[tokio::test]
    async fn test_list_acct_items_requires_page_size() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url("https://127.0.0.1:9")
            .enable_token_cache(false)
            .build();

        let result = ListRequest::new(config).execute().await;
        assert!(result.is_err());
    }
}
