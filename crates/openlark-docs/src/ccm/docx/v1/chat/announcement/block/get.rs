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

        let api_request: ApiRequest<GetChatAnnouncementBlockResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取群公告块的内容")
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
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
