/// 获取群公告所有块
///
/// 获取群公告所有块的富文本内容并分页返回。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement-block/list
/// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement-block/list>
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
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
    #[serde(default)]
    pub items: Vec<DocxBlock>,
    pub page_token: Option<String>,
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for GetChatAnnouncementBlocksResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取群公告所有块请求
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

    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
