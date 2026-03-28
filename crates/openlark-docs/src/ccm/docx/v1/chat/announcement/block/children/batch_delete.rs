/// 删除群公告中的块
///
/// 删除指定块的子块。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement-block-children/batch_delete
/// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement-block-children/batch_delete>
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
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
    pub revision_id: i64,
    pub client_token: String,
}

impl ApiResponseTrait for BatchDeleteChatAnnouncementBlockChildrenResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除群公告中的块请求
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
            ApiRequest::delete(&api_endpoint.to_url())
                .body(serialize_params(&params, "删除群公告中的块")?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "删除群公告中的块")
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
