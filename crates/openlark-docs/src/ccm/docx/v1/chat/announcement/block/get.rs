/// 获取群公告块的内容
///
/// 获取指定块的富文本内容。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement-block/get
/// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement-block/get>
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::ccm::docx::models::common_types::DocxBlock;
use crate::common::{api_endpoints::DocxApiV1, api_utils::*};

/// 获取群公告块内容请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatAnnouncementBlockParams {
    /// 群聊ID
    pub chat_id: String,
    /// 块ID
    pub block_id: String,
}

/// 获取群公告块内容响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatAnnouncementBlockResponse {
    /// 块列表。
    #[serde(default)]
    pub items: Vec<DocxBlock>,
    /// 单块内容（按 block_id 查询单块时返回）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block: Option<DocxBlock>,
    /// 下一页分页标记。
    pub page_token: Option<String>,
    /// 是否还有更多数据。
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for GetChatAnnouncementBlockResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取群公告块内容请求
///
/// 用于获取群公告中的指定块。
pub struct GetChatAnnouncementBlockRequest {
    config: Config,
}

impl GetChatAnnouncementBlockRequest {
    /// 创建获取群公告块内容请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement-block/get
    pub async fn execute(
        self,
        params: GetChatAnnouncementBlockParams,
    ) -> SDKResult<GetChatAnnouncementBlockResponse> {
        self.execute_with_options(params, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    /// 执行请求（带请求选项）
    ///
    /// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement-block/get
    pub async fn execute_with_options(
        self,
        params: GetChatAnnouncementBlockParams,
        option: RequestOption,
    ) -> SDKResult<GetChatAnnouncementBlockResponse> {
        validate_required!(params.chat_id, "群聊ID不能为空");
        validate_required!(params.block_id, "块ID不能为空");

        let api_endpoint =
            DocxApiV1::ChatAnnouncementBlockGet(params.chat_id.clone(), params.block_id.clone());

        let api_request: ApiRequest<GetChatAnnouncementBlockResponse> = api_endpoint.to_request();

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取群公告块的内容")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET .../announcement/blocks/{block_id} → GetChatAnnouncementBlockResponse（block）。
    #[tokio::test]
    async fn test_get_chat_announcement_block_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/docx/v1/chats/chat001/announcement/blocks/blk1",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": { "block": { "block_id": "blk1", "block_type": 1 } }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = GetChatAnnouncementBlockRequest::new(config)
            .execute(GetChatAnnouncementBlockParams {
                chat_id: "chat001".into(),
                block_id: "blk1".into(),
            })
            .await
            .expect("获取群公告块内容应成功");
        let block = resp.block.expect("响应应包含 block");
        assert_eq!(block.block_id, "blk1");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/docx/v1/chats/chat001/announcement/blocks/blk1"
        );
    }
}
