//! 查询支行信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/basic_info.bank_branch/search>

use crate::feishu_people::corehr::v2::basic_info::I18nText;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 查询支行信息请求
#[derive(Debug, Clone)]
pub struct SearchRequest {
    /// 配置信息
    config: Config,
    request_body: Value,
}

impl SearchRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request_body: Value::Object(serde_json::Map::new()),
        }
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = request_body;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<SearchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<SearchResponse> {
        use crate::common::api_endpoints::CorehrApiV2;

        let api_endpoint = CorehrApiV2::BasicInfoBankBranchSearch;
        let endpoint_url = api_endpoint.to_url();
        validate_required!(endpoint_url.as_str(), "API 端点不能为空");
        let request = ApiRequest::<SearchResponse>::post(endpoint_url).body(self.request_body);
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "查询支行信息响应数据为空",
        )
        .await
    }
}

/// 查询支行信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResponse {
    /// 响应数据
    /// 数据列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<BankBranchItem>>,
    /// 分页令牌
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 支行条目信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BankBranchItem {
    /// 支行 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_branch_id: Option<String>,
    /// 支行名称（i18n 多语数组）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_branch_name: Option<Vec<I18nText>>,
    /// 所属银行 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_id: Option<String>,
    /// 金融分支机构编码（即银行联行号）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// SWIFT 银行代码（ISO 9362）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swift_code: Option<String>,
    /// 状态（1 生效 / 0 失效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// Bank Branch Code（特定国家地区使用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_branch_code: Option<String>,
    /// 注册地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub register_place: Option<String>,
    /// 银行地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_address: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 兼容保留字段
    #[serde(flatten)]
    pub extra: Value,
}

impl ApiResponseTrait for SearchResponse {
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

    /// 端到端：POST /open-apis/corehr/v2/basic_info/bank_branchs/search
    #[tokio::test]
    async fn test_search_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/basic_info/bank_branchs/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": {
                    "items": [
                        {
                            "bank_branch_id": "MDBK00061194",
                            "bank_branch_name": [{ "lang": "zh-CN", "value": "中文示例" }],
                            "bank_id": "MDBH00000080",
                            "code": "308100005019",
                            "swift_code": "CMBCCNBS201",
                            "status": 1,
                            "bank_branch_code": "B04",
                            "register_place": "北京市朝阳区",
                            "bank_address": "北京市朝阳区",
                            "create_time": "2020-01-01 00:00:00",
                            "update_time": "2024-01-01 00:00:00"
                        }
                    ],
                    "page_token": "MDBK00000120",
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

        let resp = SearchRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let item = resp
            .items
            .as_ref()
            .and_then(|items| items.first())
            .expect("应返回支行条目");
        assert_eq!(item.bank_branch_id.as_deref(), Some("MDBK00061194"));
        assert_eq!(item.bank_id.as_deref(), Some("MDBH00000080"));
        assert_eq!(item.code.as_deref(), Some("308100005019"));
        assert_eq!(item.swift_code.as_deref(), Some("CMBCCNBS201"));
        assert_eq!(item.bank_branch_code.as_deref(), Some("B04"));
        assert_eq!(
            item.bank_branch_name
                .as_ref()
                .and_then(|n| n.first())
                .and_then(|n| n.value.as_deref()),
            Some("中文示例")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
