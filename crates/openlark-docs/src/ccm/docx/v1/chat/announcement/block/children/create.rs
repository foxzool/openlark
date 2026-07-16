/// 在群公告中创建块
///
/// 在指定块的子块列表中，新创建一批子块，并放置到指定位置。如果操作成功，接口将返回新创建子块的富文本内容。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement-block-children/create
/// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement-block-children/create>
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

/// 在群公告中创建块请求参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChatAnnouncementBlockChildrenParams {
    /// 群聊ID
    #[serde(skip_serializing)]
    pub chat_id: String,
    /// 父块ID
    #[serde(skip_serializing)]
    pub block_id: String,
    /// 插入位置索引（可选，默认插入到末尾）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    /// 新建的子块列表（按文档定义传入）
    pub children: Vec<serde_json::Value>,
}

/// 在群公告中创建块响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateChatAnnouncementBlockChildrenResponse {
    /// 新建子块列表。
    #[serde(default)]
    pub children: Vec<DocxBlock>,
    /// 群公告版本号（操作后的版本）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<i32>,
    /// 幂等标记（请求时传入的 client_token 原样回传）。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_token: Option<String>,
}

impl ApiResponseTrait for CreateChatAnnouncementBlockChildrenResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 在群公告中创建块请求
///
/// 用于在群公告块下插入一批子块。
pub struct CreateChatAnnouncementBlockChildrenRequest {
    config: Config,
}

impl CreateChatAnnouncementBlockChildrenRequest {
    /// 创建在群公告中创建块请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement-block-children/create
    pub async fn execute(
        self,
        params: CreateChatAnnouncementBlockChildrenParams,
    ) -> SDKResult<CreateChatAnnouncementBlockChildrenResponse> {
        self.execute_with_options(params, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    /// 执行请求（带请求选项）
    ///
    /// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement-block-children/create
    pub async fn execute_with_options(
        self,
        params: CreateChatAnnouncementBlockChildrenParams,
        option: RequestOption,
    ) -> SDKResult<CreateChatAnnouncementBlockChildrenResponse> {
        validate_required!(params.chat_id, "群聊ID不能为空");
        validate_required!(params.block_id, "父块ID不能为空");
        validate_required!(params.children, "子块列表不能为空");

        let api_endpoint = DocxApiV1::ChatAnnouncementBlockChildrenCreate(
            params.chat_id.clone(),
            params.block_id.clone(),
        );

        let api_request: ApiRequest<CreateChatAnnouncementBlockChildrenResponse> = api_endpoint
            .to_request()
            .body(serialize_params(&params, "在群公告中创建块")?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "在群公告中创建块")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST .../blocks/{block_id}/children → CreateChatAnnouncementBlockChildrenResponse（children）。
    #[tokio::test]
    async fn test_create_chat_announcement_block_children_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/docx/v1/chats/chat001/announcement/blocks/blk1/children",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": { "children": [{ "block_id": "newBlk", "block_type": 2 }] }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = CreateChatAnnouncementBlockChildrenRequest::new(config)
            .execute(CreateChatAnnouncementBlockChildrenParams {
                chat_id: "chat001".into(),
                block_id: "blk1".into(),
                index: None,
                children: vec![json!({ "block_id": "newBlk", "block_type": 2 })],
            })
            .await
            .expect("创建群公告子块应成功");
        assert_eq!(resp.children.len(), 1);
        assert_eq!(resp.children[0].block_id, "newBlk");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/docx/v1/chats/chat001/announcement/blocks/blk1/children"
        );
    }
}
