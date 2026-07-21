//! 批量获取 OKR
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v1/okr/batch_get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required_list,
};
use serde::{Deserialize, Serialize};

/// 批量获取 OKR请求
#[derive(Debug, Clone)]
pub struct BatchGetRequest {
    /// OKR ID 列表（必填，不超过 50 个）
    okr_ids: Vec<String>,
    /// 配置信息
    config: Config,
}

impl BatchGetRequest {
    /// 创建请求
    pub fn new(config: Config, okr_ids: Vec<String>) -> Self {
        Self { okr_ids, config }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchGetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchGetResponse> {
        use crate::common::api_endpoints::OkrApiV1;

        // 1. 验证必填字段
        validate_required_list!(self.okr_ids, 50, "OKR ID 列表不能为空且不能超过 50 个");

        // 2. 构建端点
        let api_endpoint = OkrApiV1::OkrBatchGet;
        let request = ApiRequest::<BatchGetResponse>::get(api_endpoint.to_url());

        // 3. 序列化请求体
        let request_body = BatchGetRequestBody {
            okr_ids: self.okr_ids.join(","),
        };
        let request = request.body(serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?);

        // 4. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "批量获取 OKR 响应数据为空",
        )
        .await
    }
}

/// 批量获取 OKR请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetRequestBody {
    /// OKR ID 列表（必填，不超过 50 个）
    pub okr_ids: String,
}

/// 批量获取 OKR响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchGetResponse {
    /// OKR 列表
    pub items: Vec<OkrInfo>,
}

/// OKR 信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OkrInfo {
    /// OKR ID
    pub okr_id: String,
    /// OKR 标题
    pub title: String,
    /// OKR 内容
    pub content: String,
    /// 周期 ID
    pub period_id: String,
    /// 进度百分比
    pub progress_rate: i32,
    /// 状态
    /// - 1: 进行中
    /// - 2: 已完成
    /// - 3: 已放弃
    pub status: i32,
    /// 创建者 ID
    pub creator_id: String,
    /// 创建时间
    pub created_at: i64,
    /// 更新时间
    pub updated_at: i64,
}

impl ApiResponseTrait for BatchGetResponse {
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
    async fn test_batch_get_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value = serde_json::from_str(r#"{"items": []}"#).unwrap();
        Mock::given(method("GET"))
            .and(path("/open-apis/okr/v1/okrs/batch_get"))
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

        let data = BatchGetRequest::new(config, vec!["id_001".to_string()])
            .execute()
            .await
            .expect("batch_get 应成功");

        assert!(data.items.is_empty());

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/okr/v1/okrs/batch_get");
    }
}
