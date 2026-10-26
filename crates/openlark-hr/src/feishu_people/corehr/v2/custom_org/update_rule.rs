//! 更新自定义组织的匹配规则
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/custom_org/update_rule>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::query::MatchRuleGroup;

/// 更新自定义组织的匹配规则请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateRuleBody {
    /// 组织类型编码（必填，可在「飞书人事-设置-组织设置」中相应的自定义组织目录下查看）
    pub object_api_name: String,
    /// 自定义组织 ID（必填，可从批量查询自定义组织的 org_id 字段中获取）
    pub org_id: String,
    /// 自动匹配的规则组（组间并集、组内交集，最多 16 组）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_rule_groups: Option<Vec<MatchRuleGroup>>,
}

/// 更新自定义组织的匹配规则请求
#[derive(Debug, Clone)]
pub struct UpdateRuleRequest {
    /// 配置信息
    config: Config,
    /// 请求体
    body: UpdateRuleBody,
}

impl UpdateRuleRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: UpdateRuleBody::default(),
        }
    }

    /// 设置请求体
    pub fn body(mut self, body: UpdateRuleBody) -> Self {
        self.body = body;
        self
    }

    /// 设置组织类型编码（必填）。
    pub fn object_api_name(mut self, object_api_name: impl Into<String>) -> Self {
        self.body.object_api_name = object_api_name.into();
        self
    }

    /// 设置自定义组织 ID（必填）。
    pub fn org_id(mut self, org_id: impl Into<String>) -> Self {
        self.body.org_id = org_id.into();
        self
    }

    /// 设置自动匹配的规则组。
    pub fn match_rule_groups(mut self, match_rule_groups: Vec<MatchRuleGroup>) -> Self {
        self.body.match_rule_groups = Some(match_rule_groups);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UpdateRuleResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateRuleResponse> {
        validate_required!(self.body.object_api_name, "object_api_name 不能为空");
        validate_required!(self.body.org_id, "org_id 不能为空");

        let request =
            ApiRequest::<UpdateRuleResponse>::post("/open-apis/corehr/v2/custom_orgs/update_rule")
                .body(serde_json::to_value(&self.body)?);

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 更新自定义组织的匹配规则响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateRuleResponse {
    /// 响应数据
    pub data: Value,
}

impl ApiResponseTrait for UpdateRuleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{body_json, method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v2/custom_orgs/update_rule
    #[tokio::test]
    async fn test_update_rule_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/custom_orgs/update_rule"))
            .and(body_json(json!({
                "object_api_name": "custom_org_01",
                "org_id": "6862995757234914824",
                "match_rule_groups": [{
                    "match_rules": [{
                        "left_value": "department",
                        "operator": "contains",
                        "right_values": ["6862995757234914824"]
                    }]
                }]
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "data": {} }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        UpdateRuleRequest::new(config)
            .object_api_name("custom_org_01")
            .org_id("6862995757234914824")
            .match_rule_groups(vec![MatchRuleGroup {
                match_rules: Some(vec![super::super::query::MatchRule {
                    left_value: Some("department".to_string()),
                    operator: Some("contains".to_string()),
                    right_values: Some(vec!["6862995757234914824".to_string()]),
                }]),
            }])
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
