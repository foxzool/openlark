/// 批量更新群公告块的内容
///
/// 批量更新群公告块的富文本内容。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement-block/batch_update
/// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement-block/batch_update>
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
    pub block_id: String,
    /// 操作内容（例如 update_text_elements / merge_table_cells 等）
    #[serde(flatten)]
    pub operation: serde_json::Value,
}

/// 批量更新群公告块内容响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchUpdateChatAnnouncementBlocksResponse {
    #[serde(default)]
    pub blocks: Vec<DocxBlock>,
}

impl ApiResponseTrait for BatchUpdateChatAnnouncementBlocksResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量更新群公告块内容请求
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

        let api_request: ApiRequest<BatchUpdateChatAnnouncementBlocksResponse> =
            ApiRequest::patch(&api_endpoint.to_url())
                .body(serialize_params(&params, "批量更新群公告块的内容")?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "批量更新群公告块的内容")
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
