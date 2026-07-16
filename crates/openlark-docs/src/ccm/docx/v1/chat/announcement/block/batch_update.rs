/// 批量更新群公告块的内容
///
/// 批量更新群公告块的富文本内容。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement-block/batch_update
/// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement-block/batch_update>
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::ccm::docx::models::block_update::BlockUpdateOperation;
use crate::ccm::docx::models::common_types::DocxBlock;
use crate::common::{api_endpoints::DocxApiV1, api_utils::*};

/// 批量更新群公告块内容请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateChatAnnouncementBlocksParams {
    /// 群聊ID
    #[serde(skip_serializing)]
    pub chat_id: String,
    /// 批量请求
    pub requests: Vec<BatchUpdateRequest>,
}

/// 单个批量更新请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateRequest {
    /// 块 ID。
    pub block_id: String,
    /// 操作内容（update_text_elements / merge_table_cells 等 15 种之一）
    #[serde(flatten)]
    pub operation: BlockUpdateOperation,
}

/// 批量更新群公告块内容响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateChatAnnouncementBlocksResponse {
    /// 更新后的块列表。
    #[serde(default)]
    pub blocks: Vec<DocxBlock>,
    /// 群公告版本号（操作后的版本）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<i32>,
    /// 幂等标记（请求时传入的 client_token 原样回传）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

impl ApiResponseTrait for BatchUpdateChatAnnouncementBlocksResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量更新群公告块内容请求
///
/// 用于批量修改群公告中的块内容。
pub struct BatchUpdateChatAnnouncementBlocksRequest {
    config: Config,
}

impl BatchUpdateChatAnnouncementBlocksRequest {
    /// 创建批量更新群公告块内容请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement-block/batch_update
    pub async fn execute(
        self,
        params: BatchUpdateChatAnnouncementBlocksParams,
    ) -> SDKResult<BatchUpdateChatAnnouncementBlocksResponse> {
        self.execute_with_options(params, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    /// 执行请求（带请求选项）
    ///
    /// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement-block/batch_update
    pub async fn execute_with_options(
        self,
        params: BatchUpdateChatAnnouncementBlocksParams,
        option: RequestOption,
    ) -> SDKResult<BatchUpdateChatAnnouncementBlocksResponse> {
        validate_required!(params.chat_id, "群聊ID不能为空");
        validate_required!(params.requests, "批量请求不能为空");

        let api_endpoint = DocxApiV1::ChatAnnouncementBlockBatchUpdate(params.chat_id.clone());

        let api_request: ApiRequest<BatchUpdateChatAnnouncementBlocksResponse> = api_endpoint
            .to_request()
            .body(serialize_params(&params, "批量更新群公告块的内容")?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "批量更新群公告块的内容")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    use crate::ccm::docx::models::block_update::BlockUpdateOperation;

    /// 端到端：PATCH .../announcement/blocks/batch_update → BatchUpdateChatAnnouncementBlocksResponse（blocks）。
    #[tokio::test]
    async fn test_batch_update_chat_announcement_blocks_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path(
                "/open-apis/docx/v1/chats/chat001/announcement/blocks/batch_update",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": { "blocks": [], "revision_id": 2 }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = BatchUpdateChatAnnouncementBlocksRequest::new(config)
            .execute(BatchUpdateChatAnnouncementBlocksParams {
                chat_id: "chat001".into(),
                requests: vec![BatchUpdateRequest {
                    block_id: "blk1".into(),
                    operation: BlockUpdateOperation::Raw(json!({})),
                }],
            })
            .await
            .expect("批量更新群公告块应成功");
        assert!(resp.blocks.is_empty());
        assert_eq!(resp.revision_id, Some(2));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/docx/v1/chats/chat001/announcement/blocks/batch_update"
        );
    }
}
