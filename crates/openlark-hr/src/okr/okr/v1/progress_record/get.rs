//! 获取 OKR 进展记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v1/progress_record/get>

use super::create::{ContentBlock, ProgressRateNew};
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 获取 OKR 进展记录请求
#[derive(Debug, Clone)]
pub struct GetRequest {
    /// 进展记录 ID（必填）
    progress_id: String,
    /// 用户 ID 类型（可选 query 参数，默认 open_id）
    user_id_type: Option<String>,
    /// 配置信息
    config: Config,
}

impl GetRequest {
    /// 创建请求
    pub fn new(config: Config, progress_id: String) -> Self {
        Self {
            progress_id,
            user_id_type: None,
            config,
        }
    }

    /// 设置用户 ID 类型（可选 query 参数，open_id/union_id/user_id）
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetResponse> {
        use crate::common::api_endpoints::OkrApiV1;

        // 1. 验证必填字段
        validate_required!(self.progress_id.trim(), "进展记录 ID 不能为空");

        // 2. 构建端点
        let api_endpoint = OkrApiV1::ProgressRecordGet(self.progress_id.clone());
        let request = ApiRequest::<GetResponse>::get(api_endpoint.to_url())
            .query_opt("user_id_type", self.user_id_type);

        // 3. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取 OKR 进展记录响应数据为空",
        )
        .await
    }
}

/// 获取 OKR 进展记录响应（progress_record）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetResponse {
    /// OKR 进展 ID
    pub progress_id: String,
    /// 进展更新时间（毫秒）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<String>,
    /// 进展对应的 Content 详细内容（content_block 富文本）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<ContentBlock>,
    /// 进展，包括百分比和状态（progress_rate_new）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_rate: Option<ProgressRateNew>,
}

impl ApiResponseTrait for GetResponse {
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
    async fn test_okr_v1_progress_record_get_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 官方响应示例形状：progress_id + modify_time + content 富文本 + progress_rate
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{
                "progress_id": "7046518160811425812",
                "modify_time": "1640677213095",
                "content": {
                    "blocks": [
                        {
                            "type": "paragraph",
                            "paragraph": {
                                "elements": [
                                    {"type": "textRun", "textRun": {"text": " ", "style": {}}},
                                    {"type": "person", "person": {"openId": "ou_b1ba99a5340289d7af30839fd95ce1ee", "userId": "7012194140645721644"}}
                                ]
                            }
                        }
                    ]
                },
                "progress_rate": {"percent": 50.21, "status": 0}
            }"#,
        )
        .unwrap();
        Mock::given(method("GET"))
            .and(path("/open-apis/okr/v1/progress_records/progress_001"))
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

        let data = GetRequest::new(config, "progress_001".to_string())
            .execute()
            .await
            .expect("okr_v1_progress_record_get 应成功");

        assert_eq!(data.progress_id, "7046518160811425812");
        assert_eq!(data.modify_time.as_deref(), Some("1640677213095"));
        let content = data.content.expect("content 应存在");
        assert_eq!(content.blocks.len(), 1);
        let elements = content.blocks[0]
            .paragraph
            .as_ref()
            .expect("paragraph 应存在")
            .elements
            .clone();
        assert_eq!(elements.len(), 2);
        assert_eq!(
            elements[1]
                .person
                .as_ref()
                .and_then(|p| p.open_id.as_deref()),
            Some("ou_b1ba99a5340289d7af30839fd95ce1ee")
        );
        let rate = data.progress_rate.expect("progress_rate 应存在");
        assert_eq!(rate.percent, Some(50.21));
        assert_eq!(rate.status, Some(0));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/okr/v1/progress_records/progress_001"
        );
    }
}
