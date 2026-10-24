//! 更新 OKR 进展记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v1/progress_record/update>

use super::create::{ContentBlock, ProgressRateNew};
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 更新 OKR 进展记录请求
#[derive(Debug, Clone)]
pub struct UpdateRequest {
    /// 进展记录 ID（必填，路径参数）
    progress_id: String,
    /// 进展详情 富文本格式（必填，content_block）
    content: ContentBlock,
    /// 进展，包括百分比和状态（可选，progress_rate_new）
    progress_rate: Option<ProgressRateNew>,
    /// 用户 ID 类型（可选 query 参数，默认 open_id）
    user_id_type: Option<String>,
    /// 配置信息
    config: Config,
}

impl UpdateRequest {
    /// 创建请求
    pub fn new(config: Config, progress_id: String, content: ContentBlock) -> Self {
        Self {
            progress_id,
            content,
            progress_rate: None,
            user_id_type: None,
            config,
        }
    }

    /// 设置进展百分比与状态（可选）
    pub fn progress_rate(mut self, progress_rate: ProgressRateNew) -> Self {
        self.progress_rate = Some(progress_rate);
        self
    }

    /// 设置用户 ID 类型（可选 query 参数，open_id/union_id/user_id）
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UpdateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateResponse> {
        use crate::common::api_endpoints::OkrApiV1;

        // 1. 验证必填字段
        validate_required!(self.progress_id.trim(), "进展记录 ID 不能为空");
        validate_required!(self.content.blocks, "进展内容不能为空");

        // 2. 构建端点
        let api_endpoint = OkrApiV1::ProgressRecordUpdate(self.progress_id.clone());
        let request = ApiRequest::<UpdateResponse>::put(api_endpoint.to_url())
            .query_opt("user_id_type", self.user_id_type);

        // 3. 序列化请求体
        let request_body = UpdateRequestBody {
            content: self.content,
            progress_rate: self.progress_rate,
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
            "更新 OKR 进展记录响应数据为空",
        )
        .await
    }
}

/// 更新 OKR 进展记录请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRequestBody {
    /// 进展详情 富文本格式（必填）
    pub content: ContentBlock,
    /// 进展，包括百分比和状态（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_rate: Option<ProgressRateNew>,
}

/// 更新 OKR 进展记录响应（progress_record）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateResponse {
    /// OKR 进展 ID
    pub progress_id: String,
    /// 进展更新时间（毫秒）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<String>,
    /// 进展对应的 Content 详细内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<ContentBlock>,
    /// 进展，包括百分比和状态
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_rate: Option<ProgressRateNew>,
}

impl ApiResponseTrait for UpdateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::super::create::{
        ContentBlockElement, ContentParagraph, ContentParagraphElement, ContentTextRun,
    };
    use super::*;
    use openlark_core::config::Config;

    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_okr_v1_progress_record_update_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{body_json, method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{"progress_id": "test", "modify_time": "1640677213095", "content": {"blocks": []}}"#,
        )
        .unwrap();
        Mock::given(method("PUT"))
            .and(path("/open-apis/okr/v1/progress_records/progress_001"))
            .and(body_json(json!({
                "content": {"blocks": [{"type": "paragraph", "paragraph": {"elements": [{"type": "textRun", "textRun": {"text": "更新后的进展"}}]}}]},
                "progress_rate": {"percent": 70.5, "status": 1}
            })))
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

        let data = UpdateRequest::new(
            config,
            "progress_001".to_string(),
            ContentBlock {
                blocks: vec![ContentBlockElement {
                    block_element_type: Some("paragraph".to_string()),
                    paragraph: Some(ContentParagraph {
                        style: None,
                        elements: vec![ContentParagraphElement {
                            paragraph_element_type: Some("textRun".to_string()),
                            text_run: Some(ContentTextRun {
                                text: Some("更新后的进展".to_string()),
                                style: None,
                            }),
                            docs_link: None,
                            person: None,
                        }],
                    }),
                    gallery: None,
                }],
            },
        )
        .progress_rate(ProgressRateNew {
            percent: Some(70.5),
            status: Some(1),
        })
        .execute()
        .await
        .expect("okr_v1_progress_record_update 应成功");

        assert_eq!(data.progress_id, "test");
        assert_eq!(data.modify_time.as_deref(), Some("1640677213095"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/okr/v1/progress_records/progress_001"
        );
    }

    /// 官方请求体仅 content + progress_rate，无 description
    #[test]
    fn test_update_request_body_shape() {
        let body = UpdateRequestBody {
            content: ContentBlock { blocks: Vec::new() },
            progress_rate: None,
        };
        let value = serde_json::to_value(&body).expect("序列化请求体失败");
        assert!(value.get("content").is_some());
        assert!(value.get("progress_rate").is_none());
        assert!(value.get("description").is_none());
    }
}
