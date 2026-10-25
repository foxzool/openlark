//! 根据生效日期分页查询参保方案
//!
//! docPath: <https://open.feishu.cn/document/server-docs/compensation-v1/social_plan/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::validation_error,
    http::Transport,
};
use serde::{Deserialize, Serialize};

use super::query::SocialPlan;

/// 根据生效日期分页查询参保方案请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 生效日期（必填，查询在该日期生效的参保方案数据，格式 YYYY-mm-dd，长度 10 字符）
    effective_date: String,
    /// 分页大小（可选，默认 100，最大 200）
    page_size: Option<i32>,
    /// 分页标记（可选，第一次请求不填）
    page_token: Option<String>,
    /// 配置信息
    config: Config,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config, effective_date: String) -> Self {
        Self {
            effective_date,
            page_size: None,
            page_token: None,
            config,
        }
    }

    /// 设置分页大小（可选）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记（可选）
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    fn validate(&self) -> SDKResult<()> {
        // 官方要求：日期格式为 YYYY-mm-dd，长度为 10 字符
        let date = self.effective_date.trim();
        let bytes = date.as_bytes();
        if date.len() != 10 || bytes[4] != b'-' || bytes[7] != b'-' {
            return Err(validation_error(
                "effective_date 无效",
                "effective_date 必须为 YYYY-mm-dd 格式的 10 字符日期，如 2024-01-01",
            ));
        }
        Ok(())
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
        use crate::common::api_endpoints::CompensationApiV1;

        self.validate()?;

        // 2. 构建端点
        let api_endpoint = CompensationApiV1::SocialPlanList;
        let mut request = ApiRequest::<ListResponse>::get(api_endpoint.to_url());

        // 3. 添加查询参数
        request = request.query("effective_date", self.effective_date);
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(ref page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        // 4. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "查询参保方案响应数据为空",
        )
        .await
    }
}

/// 根据生效日期分页查询参保方案响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 方案列表
    #[serde(default)]
    pub plans: Vec<SocialPlan>,
    /// 是否有更多数据
    #[serde(default)]
    pub has_more: bool,
    /// 分页标记（has_more 为 true 时返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
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
    async fn test_list_social_plans_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/compensation/v1/social_plans"))
            .and(query_param("effective_date", "2024-01-01"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "plans": [{
                        "plan_id": "7198027289165088300",
                        "plan_tid": "7198027289165055532",
                        "name": { "zh_cn": "2024年度社保方案", "en_us": "2024 Social Insurance Plan" },
                        "effective_date": "2024-01-01",
                        "active": true,
                        "insurance_type": "social_insurance",
                        "scope": {
                            "is_all": true,
                            "rules": [[{ "left_type": 1, "operator": 1, "right_values": ["7283712673258931756"] }]]
                        },
                        "item_detail": [],
                        "remark": { "zh_cn": "中文备注", "en_us": "english remark" }
                    }],
                    "has_more": false
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

        let data = ListRequest::new(config, "2024-01-01".to_string())
            .execute()
            .await
            .expect("查询参保方案应成功");

        assert!(!data.has_more);
        assert_eq!(data.plans.len(), 1);
        assert_eq!(data.plans[0].plan_id, "7198027289165088300");
        assert_eq!(
            data.plans[0]
                .scope
                .as_ref()
                .unwrap()
                .rules
                .as_ref()
                .unwrap()[0][0]
                .left_type,
            1
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/compensation/v1/social_plans"
        );
        assert!(
            received[0]
                .url
                .query()
                .unwrap_or("")
                .contains("effective_date=2024-01-01"),
            "查询参数缺 effective_date: {:?}",
            received[0].url.query()
        );
    }
}
