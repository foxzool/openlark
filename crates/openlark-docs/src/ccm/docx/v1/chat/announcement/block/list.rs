/// 获取群公告所有块
///
/// 获取群公告所有块的富文本内容并分页返回。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement-block/list
/// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement-block/list>
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

/// 获取群公告所有块请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatAnnouncementBlocksParams {
    /// 群聊ID
    pub chat_id: String,
    /// 分页大小
    pub page_size: Option<u32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 公告版本号（可选，-1 表示最新版本）
    pub revision_id: Option<i64>,
}

/// 获取群公告所有块响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatAnnouncementBlocksResponse {
    /// 块列表。
    #[serde(default)]
    pub items: Vec<DocxBlock>,
    /// 下一页分页标记。
    pub page_token: Option<String>,
    /// 是否还有更多数据。
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for GetChatAnnouncementBlocksResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取群公告所有块请求
///
/// 用于分页查询群公告中的全部块。
pub struct GetChatAnnouncementBlocksRequest {
    config: Config,
}

impl GetChatAnnouncementBlocksRequest {
    /// 创建获取群公告所有块请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement-block/list
    pub async fn execute(
        self,
        params: GetChatAnnouncementBlocksParams,
    ) -> SDKResult<GetChatAnnouncementBlocksResponse> {
        self.execute_with_options(params, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    /// 执行请求（带请求选项）
    ///
    /// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement-block/list
    pub async fn execute_with_options(
        self,
        params: GetChatAnnouncementBlocksParams,
        option: RequestOption,
    ) -> SDKResult<GetChatAnnouncementBlocksResponse> {
        validate_required!(params.chat_id, "群聊ID不能为空");

        let api_endpoint = DocxApiV1::ChatAnnouncementBlockList(params.chat_id.clone());

        let mut api_request: ApiRequest<GetChatAnnouncementBlocksResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        if let Some(page_size) = params.page_size {
            api_request = api_request.query("page_size", &page_size.to_string());
        }
        if let Some(page_token) = params.page_token {
            api_request = api_request.query("page_token", &page_token);
        }
        if let Some(revision_id) = params.revision_id {
            api_request = api_request.query("revision_id", &revision_id.to_string());
        }

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取群公告所有块")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET .../announcement/blocks → GetChatAnnouncementBlocksResponse（items）。
    #[tokio::test]
    async fn test_list_chat_announcement_blocks_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/docx/v1/chats/chat001/announcement/blocks"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": { "items": [{ "block_id": "blk1", "block_type": 1 }] }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = GetChatAnnouncementBlocksRequest::new(config)
            .execute(GetChatAnnouncementBlocksParams {
                chat_id: "chat001".into(),
                page_size: None,
                page_token: None,
                revision_id: None,
            })
            .await
            .expect("获取群公告所有块应成功");
        assert_eq!(resp.items.len(), 1);
        assert_eq!(resp.items[0].block_id, "blk1");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/docx/v1/chats/chat001/announcement/blocks"
        );
    }
}
