/// 删除群公告中的块
///
/// 删除指定块的子块。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement-block-children/batch_delete
/// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement-block-children/batch_delete>
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DocxApiV1, api_utils::*};

/// 删除群公告中的块请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDeleteChatAnnouncementBlockChildrenParams {
    /// 群聊ID
    #[serde(skip_serializing)]
    pub chat_id: String,
    /// 父块ID
    #[serde(skip_serializing)]
    pub block_id: String,
    /// 幂等 token（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
    /// 删除的起始索引（左闭右开）
    pub start_index: i32,
    /// 删除的末尾索引（左闭右开）
    pub end_index: i32,
}

/// 删除群公告中的块响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDeleteChatAnnouncementBlockChildrenResponse {
    /// 文档修订版本。
    pub revision_id: i64,
    /// 幂等 token。
    pub client_token: String,
}

impl ApiResponseTrait for BatchDeleteChatAnnouncementBlockChildrenResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除群公告中的块请求
///
/// 用于删除群公告块下的一段子块。
pub struct BatchDeleteChatAnnouncementBlockChildrenRequest {
    config: Config,
}

impl BatchDeleteChatAnnouncementBlockChildrenRequest {
    /// 创建删除群公告中的块请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement-block-children/batch_delete
    pub async fn execute(
        self,
        params: BatchDeleteChatAnnouncementBlockChildrenParams,
    ) -> SDKResult<BatchDeleteChatAnnouncementBlockChildrenResponse> {
        self.execute_with_options(params, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    /// 执行请求（带请求选项）
    ///
    /// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement-block-children/batch_delete
    pub async fn execute_with_options(
        self,
        params: BatchDeleteChatAnnouncementBlockChildrenParams,
        option: RequestOption,
    ) -> SDKResult<BatchDeleteChatAnnouncementBlockChildrenResponse> {
        validate_required!(params.chat_id, "群聊ID不能为空");
        validate_required!(params.block_id, "父块ID不能为空");

        let api_endpoint = DocxApiV1::ChatAnnouncementBlockChildrenBatchDelete(
            params.chat_id.clone(),
            params.block_id.clone(),
        );

        let api_request: ApiRequest<BatchDeleteChatAnnouncementBlockChildrenResponse> =
            api_endpoint
                .to_request()
                .body(serialize_params(&params, "删除群公告中的块")?);

        Transport::request_typed(api_request, &self.config, Some(option), "删除群公告中的块").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：DELETE .../blocks/{block_id}/children/batch_delete → BatchDeleteChatAnnouncementBlockChildrenResponse（revision_id）。
    #[tokio::test]
    async fn test_batch_delete_chat_announcement_block_children_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path(
                "/open-apis/docx/v1/chats/chat001/announcement/blocks/blk1/children/batch_delete",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": { "revision_id": 2, "client_token": "ct001" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = BatchDeleteChatAnnouncementBlockChildrenRequest::new(config)
            .execute(BatchDeleteChatAnnouncementBlockChildrenParams {
                chat_id: "chat001".into(),
                block_id: "blk1".into(),
                client_token: None,
                start_index: 0,
                end_index: 1,
            })
            .await
            .expect("删除群公告子块应成功");
        assert_eq!(resp.revision_id, 2);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/docx/v1/chats/chat001/announcement/blocks/blk1/children/batch_delete"
        );
    }
}
