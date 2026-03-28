//! 获取消息中的资源文件
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-resource/get>

use openlark_core::{
    api::ApiRequest, config::Config, error, http::Transport, validate_required, SDKResult,
};

use crate::{common::api_utils::extract_response_data, endpoints::IM_V1_MESSAGES};

/// 消息资源类型
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageResourceType {
    Image,
    File,
}

impl MessageResourceType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Image => "image",
            Self::File => "file",
        }
    }
}

/// 获取消息中的资源文件请求
///
/// 用于获取指定消息中的资源文件（图片或文件）。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `message_id`: 消息 ID，必填
/// - `file_key`: 资源 Key，必填
/// - `resource_type`: 资源类型，必填
///
/// # 示例
///
/// ```rust,ignore
/// let request = GetMessageResourceRequest::new(config)
///     .message_id("msg_xxx")
///     .file_key("file_key_xxx")
///     .resource_type(MessageResourceType::Image)
///     .execute().await?;
/// ```
pub struct GetMessageResourceRequest {
    config: Config,
    message_id: String,
    file_key: String,
    resource_type: Option<MessageResourceType>,
}

impl GetMessageResourceRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            message_id: String::new(),
            file_key: String::new(),
            resource_type: None,
        }
    }

    /// 消息 ID（路径参数）
    pub fn message_id(mut self, message_id: impl Into<String>) -> Self {
        self.message_id = message_id.into();
        self
    }

    /// 资源 Key（路径参数）
    pub fn file_key(mut self, file_key: impl Into<String>) -> Self {
        self.file_key = file_key.into();
        self
    }

    /// 资源类型（查询参数，必填）
    pub fn resource_type(mut self, resource_type: MessageResourceType) -> Self {
        self.resource_type = Some(resource_type);
        self
    }

    /// 执行请求（返回二进制内容）
    ///
    /// docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/im-v1/message-resource/get>
    pub async fn execute(self) -> SDKResult<Vec<u8>> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<Vec<u8>> {
        // === 必填字段验证 ===
        validate_required!(self.message_id, "message_id 不能为空");
        validate_required!(self.file_key, "file_key 不能为空");
        let resource_type = self.resource_type.ok_or_else(|| {
            error::validation_error(
                "type 不能为空".to_string(),
                "获取消息资源需要指定资源类型（type=image/file）".to_string(),
            )
        })?;

        // url: GET:/open-apis/im/v1/messages/:message_id/resources/:file_key
        let req: ApiRequest<Vec<u8>> = ApiRequest::get(format!(
            "{}/{}/resources/{}",
            IM_V1_MESSAGES, self.message_id, self.file_key
        ))
        .query("type", resource_type.as_str());

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取消息中的资源文件")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_get_message_resource_request_builder() {
        let config = Config::default();
        let request = GetMessageResourceRequest::new(config)
            .message_id("msg_xxx")
            .file_key("file_key_xxx")
            .resource_type(MessageResourceType::Image);
        assert_eq!(request.message_id, "msg_xxx");
        assert_eq!(request.file_key, "file_key_xxx");
        assert_eq!(request.resource_type, Some(MessageResourceType::Image));
    }

    #[test]
    fn test_get_message_resource_request_default_values() {
        let config = Config::default();
        let request = GetMessageResourceRequest::new(config);
        assert_eq!(request.message_id, "");
        assert_eq!(request.file_key, "");
        assert!(request.resource_type.is_none());
    }

    #[test]
    fn test_get_message_resource_request_with_file_type() {
        let config = Config::default();
        let request = GetMessageResourceRequest::new(config)
            .message_id("msg_xxx")
            .file_key("file_key_xxx")
            .resource_type(MessageResourceType::File);
        assert_eq!(request.resource_type, Some(MessageResourceType::File));
    }

    #[test]
    fn test_message_resource_type_as_str() {
        assert_eq!(MessageResourceType::Image.as_str(), "image");
        assert_eq!(MessageResourceType::File.as_str(), "file");
    }
}
