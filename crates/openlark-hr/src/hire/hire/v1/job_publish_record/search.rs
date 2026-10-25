//! 获取职位广告发布记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/job_publish_record/search>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::hire::hire::v1::referral_website::job_post::list::ReferralWebsiteJobPostItem;

/// 获取职位广告发布记录请求
#[derive(Debug, Clone)]
pub struct SearchRequest {
    request_body: SearchRequestBody,
    /// 配置信息
    config: Config,
}

impl SearchRequest {
    /// 创建请求
    pub fn new(config: Config, request_body: SearchRequestBody) -> Self {
        Self {
            request_body,
            config,
        }
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: SearchRequestBody) -> Self {
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
        use crate::common::api_endpoints::HireApiV1;

        self.request_body.validate()?;

        let api_endpoint = HireApiV1::JobPublishRecordSearch;
        let request = ApiRequest::<SearchResponse>::post(api_endpoint.to_url());
        let request = request.body(serde_json::to_value(&self.request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?);
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取职位广告发布记录响应数据为空",
        )
        .await
    }
}

/// `SearchRequestBody`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchRequestBody {
    /// 招聘渠道 ID（必填）：官网 ID 可通过获取招聘官网列表接口获取。
    pub job_channel_id: String,
}

impl SearchRequestBody {
    /// 创建新的请求实例（`job_channel_id` 为官方必填字段）。
    pub fn new(job_channel_id: impl Into<String>) -> Self {
        Self {
            job_channel_id: job_channel_id.into(),
        }
    }

    fn validate(&self) -> SDKResult<()> {
        validate_required!(self.job_channel_id, "job_channel_id 不能为空");

        Ok(())
    }
}

/// `SearchResponse` 响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SearchResponse {
    #[serde(default)]
    /// 结果项列表（官方 `website_job_post[]`）。
    pub items: Vec<ReferralWebsiteJobPostItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 下一页分页标记。
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 是否还有更多结果。
    pub has_more: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
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

    /// 端到端：POST /open-apis/hire/v1/job_publish_records/search
    #[tokio::test]
    async fn test_search_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/hire/v1/job_publish_records/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": {
                    "items": [
                        {
                            "id": "6791698585114724616",
                            "title": "后端开发",
                            "job_id": "6791698585114724616",
                            "job_code": "A106043",
                            "job_expire_time": "1722441599999",
                            "job_active_status": 1,
                            "job_process_type": 1,
                            "job_recruitment_type": {
                                "id": "101",
                                "name": { "zh_cn": "全职", "en_us": "Full-time" }
                            }
                        }
                    ],
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

        let resp = SearchRequest::new(config, SearchRequestBody::new("7047318856652261676"))
            .execute()
            .await
            .expect("请求应成功");
        assert_eq!(resp.items.len(), 1);
        let item = &resp.items[0];
        assert_eq!(item.title.as_deref(), Some("后端开发"));
        assert_eq!(item.job_code.as_deref(), Some("A106043"));
        assert_eq!(item.job_expire_time.as_deref(), Some("1722441599999"));
        assert_eq!(item.job_active_status, Some(1));
        assert_eq!(item.job_process_type, Some(1));
        assert_eq!(
            item.job_recruitment_type
                .as_ref()
                .and_then(|t| t.id.as_deref()),
            Some("101")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let wire_body: serde_json::Value =
            serde_json::from_slice(&received[0].body).expect("请求体应为合法 JSON");
        assert_eq!(
            wire_body,
            json!({ "job_channel_id": "7047318856652261676" })
        );
    }

    /// `job_channel_id` 缺失时应快速失败，不发起请求。
    #[tokio::test]
    async fn test_search_rejects_blank_job_channel_id() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build();

        let result = SearchRequest::new(config, SearchRequestBody::new("  "))
            .execute()
            .await;
        assert!(result.is_err());
    }
}
