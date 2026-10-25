//! 查询编制规划方案
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/workforce_plan/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 查询编制规划方案请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 配置信息
    config: Config,
    body: Option<Value>,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self { config, body: None }
    }

    /// 设置请求体。
    pub fn body(mut self, body: Value) -> Self {
        self.body = Some(body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListResponse> {
        let mut request = ApiRequest::<ListResponse>::get("/open-apis/corehr/v2/workforce_plans");

        if let Some(body) = self.body {
            request = request.body(body);
        }

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 查询编制规划方案响应
///
/// 官方响应字段直接平铺在 `data` 顶层（`items` + `total`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 响应数据
    /// 编制规划方案列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<WorkforcePlan>>,
    /// 满足条件的方案总数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
}

/// 编制规划方案
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkforcePlan {
    /// 编制规划方案 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workforce_plan_id: Option<String>,
    /// 编制规划方案名称（i18n 多语数组）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workforce_plan_name: Option<Vec<I18nText>>,
    /// 开始日期（YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// 结束日期（YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// 是否启用（true 启用 / false 停用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
}

/// 官方 i18n 多语文本结构：{lang, value}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nText {
    /// 语言信息（中文为 zh-CN，英文为 en-US）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 文本内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ApiResponseTrait for ListResponse {
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

    /// 端到端：GET /open-apis/corehr/v2/workforce_plans
    #[tokio::test]
    async fn test_list_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v2/workforce_plans"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": {
                    "items": [
                        {
                            "workforce_plan_id": "123456",
                            "workforce_plan_name": [{ "lang": "zh-CN", "value": "张三" }],
                            "start_date": "2020-10-01",
                            "end_date": "2020-10-31",
                            "active": true
                        }
                    ],
                    "total": 100
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

        let resp = ListRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        assert_eq!(resp.total, Some(100));
        let plan = resp
            .items
            .as_ref()
            .and_then(|items| items.first())
            .expect("应返回编制规划方案条目");
        assert_eq!(plan.workforce_plan_id.as_deref(), Some("123456"));
        assert_eq!(plan.active, Some(true));
        assert_eq!(
            plan.workforce_plan_name
                .as_ref()
                .and_then(|n| n.first())
                .and_then(|n| n.value.as_deref()),
            Some("张三")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
