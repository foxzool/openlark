//! 批量查询补充信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/performance-v2/additional_information/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use openlark_core::{validate_required, validate_required_list};
use serde::{Deserialize, Serialize};

/// 批量查询补充信息请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 绩效周期 ID（必填）
    cycle_id: String,
    /// 用户 ID 列表（必填）
    user_ids: Vec<String>,
    /// 配置信息
    config: Config,
}

impl QueryRequest {
    /// 创建请求
    pub fn new(config: Config, cycle_id: String) -> Self {
        Self {
            cycle_id,
            user_ids: Vec::new(),
            config,
        }
    }

    /// 添加用户 ID
    pub fn add_user_id(mut self, user_id: String) -> Self {
        self.user_ids.push(user_id);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<QueryResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryResponse> {
        use crate::common::api_endpoints::PerformanceApiV1;

        validate_required!(self.cycle_id.trim(), "cycle_id");
        validate_required_list!(self.user_ids, 50, "user_ids 不能为空且不能超过 50 个");

        // 1. 构建端点
        let api_endpoint = PerformanceApiV1::AdditionalInformationQuery;
        let request = ApiRequest::<QueryResponse>::post(api_endpoint.to_url());

        // 2. 构建请求体
        let request_body = QueryRequestBody {
            cycle_id: self.cycle_id,
            user_ids: self.user_ids,
        };
        let request_body_json = serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?;
        let request = request.body(request_body_json);

        // 3. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "批量查询补充信息响应数据为空",
        )
        .await
    }
}

/// 查询请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequestBody {
    /// 绩效周期 ID
    pub cycle_id: String,
    /// 用户 ID 列表
    pub user_ids: Vec<String>,
}

/// 批量查询补充信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponse {
    /// 补充信息列表
    pub items: Vec<AdditionalInformation>,
}

/// 补充信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdditionalInformation {
    /// 用户 ID
    pub user_id: String,
    /// 补充内容
    pub content: String,
}

impl ApiResponseTrait for QueryResponse {
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
    async fn test_performance_v2_additional_information_query_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value = serde_json::from_str(r#"{"items": []}"#).unwrap();
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/performance/v2/additional_informations/query",
            ))
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

        let data = QueryRequest::new(config, "cycle_001".to_string())
            .add_user_id("user_001".to_string())
            .execute()
            .await
            .expect("performance_v2_additional_information_query 应成功");

        assert!(data.items.is_empty());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/performance/v2/additional_informations/query"
        );
    }
}
