//! 获取 OKR 周期规则
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v1/period_rule/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 获取 OKR 周期规则请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 周期 ID（可选）
    period_id: Option<String>,
    /// 配置信息
    config: Config,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            period_id: None,
            config,
        }
    }

    /// 设置周期 ID（可选）
    pub fn period_id(mut self, period_id: String) -> Self {
        self.period_id = Some(period_id);
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
        use crate::common::api_endpoints::OkrApiV1;

        // 1. 构建端点
        let api_endpoint = OkrApiV1::PeriodRuleList;
        let mut request = ApiRequest::<ListResponse>::get(api_endpoint.to_url());

        // 2. 添加查询参数（可选）
        if let Some(ref period_id) = self.period_id {
            request = request.query("period_id", period_id);
        }

        // 3. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取 OKR 周期规则响应数据为空",
        )
        .await
    }
}

/// 获取 OKR 周期规则响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 周期规则列表（period_rule[]）
    #[serde(default)]
    pub period_rules: Vec<PeriodRule>,
}

/// OKR 周期规则（官方 `period_rule`）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PeriodRule {
    /// 周期规则 ID
    pub period_rule_id: String,
    /// 周期类型（year: 年度周期 / month: 月度周期）
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub period_type: Option<String>,
    /// 周期长度（月）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub length: Option<i32>,
    /// 每年首个开始月份
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_month: Option<i32>,
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
    async fn test_okr_v1_period_rule_list_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 官方响应示例形状：data.period_rules[]（period_rule_id/type/length/first_month）
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{"period_rules": [{"period_rule_id": "134", "type": "year", "length": 12, "first_month": 12}]}"#,
        )
        .unwrap();
        Mock::given(method("GET"))
            .and(path("/open-apis/okr/v1/period_rules"))
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
            .execute()
            .await
            .expect("okr_v1_period_rule_list 应成功");

        assert_eq!(data.period_rules.len(), 1);
        assert_eq!(data.period_rules[0].period_rule_id, "134");
        assert_eq!(data.period_rules[0].period_type.as_deref(), Some("year"));
        assert_eq!(data.period_rules[0].length, Some(12));
        assert_eq!(data.period_rules[0].first_month, Some(12));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/okr/v1/period_rules");
    }
}
