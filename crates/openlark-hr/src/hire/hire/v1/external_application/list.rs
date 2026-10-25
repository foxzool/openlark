//! 查询外部投递列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/external_application/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::hire::hire::common_models::ExternalApplication;

/// 查询外部投递列表请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 配置信息
    config: Config,
    request_body: Option<Value>,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request_body: None,
        }
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
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
        let mut request =
            ApiRequest::<ListResponse>::get("/open-apis/hire/v1/external_applications");
        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "查询外部投递列表响应数据为空",
        )
        .await
    }
}

/// 查询外部投递列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListResponse {
    #[serde(default, alias = "records")]
    /// 结果项列表（官方 `external_application[]`）。
    pub items: Vec<ExternalApplication>,
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

    /// 端到端：GET /open-apis/hire/v1/external_applications
    #[tokio::test]
    async fn test_list_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/hire/v1/external_applications"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "ok",
                "data": {
                    "items": [
                        {
                            "id": "6989202908470446380",
                            "job_recruitment_type": 1,
                            "job_title": "高级Java",
                            "resume_source": "lagou",
                            "stage": "简历初筛",
                            "talent_id": "6960663240925956459",
                            "termination_reason": "不合适",
                            "delivery_type": 1,
                            "modify_time": 1618500278645i64,
                            "create_time": 1618500278644i64,
                            "termination_type": "HR 主动终止"
                        }
                    ],
                    "has_more": true,
                    "page_token": "cursor_ext_app"
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
        assert_eq!(resp.items.len(), 1);
        let item = &resp.items[0];
        assert_eq!(item.id.as_deref(), Some("6989202908470446380"));
        assert_eq!(item.job_recruitment_type, Some(1));
        assert_eq!(item.job_title.as_deref(), Some("高级Java"));
        assert_eq!(item.resume_source.as_deref(), Some("lagou"));
        assert_eq!(item.stage.as_deref(), Some("简历初筛"));
        assert_eq!(item.delivery_type, Some(1));
        assert_eq!(item.modify_time, Some(1618500278645));
        assert_eq!(item.create_time, Some(1618500278644));
        assert_eq!(item.termination_type.as_deref(), Some("HR 主动终止"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
