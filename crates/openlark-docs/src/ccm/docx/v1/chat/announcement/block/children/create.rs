/// 在群公告中创建块
///
/// 在指定块的子块列表中，新创建一批子块，并放置到指定位置。如果操作成功，接口将返回新创建子块的富文本内容。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement-block-children/create
/// doc: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/document-docx/docx-v1/chat-announcement-block-children/create>
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
    #[serde(default)]
    pub children: Vec<DocxBlock>,
}

impl ApiResponseTrait for CreateChatAnnouncementBlockChildrenResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 在群公告中创建块请求
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

        let api_request: ApiRequest<CreateChatAnnouncementBlockChildrenResponse> =
            ApiRequest::post(&api_endpoint.to_url())
                .body(serialize_params(&params, "在群公告中创建块")?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "在群公告中创建块")
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
