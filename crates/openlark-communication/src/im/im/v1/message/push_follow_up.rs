//! 添加跟随气泡
//!
//! docPath: <https://open.feishu.cn/document/im-v1/message/push_follow_up>

use openlark_core::{
    api::ApiRequest, config::Config, error, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::{
        api_utils::{extract_response_data, serialize_params},
        models::EmptyData,
    },
    endpoints::IM_V1_MESSAGES,
};

/// 跟随气泡多语言内容
///
/// 表示跟随气泡的多语言内容。
///
/// # 字段说明
///
/// - `content`: 气泡内容文本
/// - `language`: 语言标识（如 zh_CN, en_US 等）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FollowUpI18nContent {
    /// 气泡内容文本
    pub content: String,
    /// 语言标识
    pub language: String,
}

/// 跟随气泡
///
/// 表示单个跟随气泡的内容。
///
/// # 字段说明
///
/// - `content`: 气泡内容文本
/// - `i18n_contents`: 多语言内容列表（可选）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FollowUp {
    /// 气泡内容文本
    pub content: String,
    /// 多语言内容列表（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_contents: Option<Vec<FollowUpI18nContent>>,
}

/// 添加跟随气泡请求体
///
/// 表示添加跟随气泡所需的请求参数。
///
/// # 字段说明
///
/// - `follow_ups`: 跟随气泡列表，至少包含 1 项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushFollowUpBody {
    /// 跟随气泡列表
    pub follow_ups: Vec<FollowUp>,
}

/// 添加跟随气泡请求
///
/// 用于为机器人发送的消息添加跟随气泡。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `message_id`: 机器人发送的消息 ID，必填
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_core::config::Config;
/// use openlark_communication::im::im::v1::message::{PushFollowUpRequest, PushFollowUpBody, FollowUp};
///
/// let config = Config::builder().app_id("app_id").app_secret("app_secret").build();
/// let body = PushFollowUpBody {
///     follow_ups: vec![
///         FollowUp {
///             content: "查看详情".to_string(),
///             i18n_contents: None,
///         },
///     ],
/// };
/// let request = PushFollowUpRequest::new(config)
///     .message_id("om_xxx");
/// let response = request.execute(body).await?;
/// ```
pub struct PushFollowUpRequest {
    config: Config,
    message_id: String,
}

impl PushFollowUpRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            message_id: String::new(),
        }
    }

    /// 机器人发送的消息 ID（路径参数）
    pub fn message_id(mut self, message_id: impl Into<String>) -> Self {
        self.message_id = message_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/im-v1/message/push_follow_up>
    pub async fn execute(self, body: PushFollowUpBody) -> SDKResult<EmptyData> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: PushFollowUpBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EmptyData> {
        // === 必填字段验证 ===
        validate_required!(self.message_id, "message_id 不能为空");
        if body.follow_ups.is_empty() {
            return Err(error::validation_error(
                "follow_ups 不能为空".to_string(),
                "跟随气泡列表至少包含 1 项".to_string(),
            ));
        }

        // url: POST:/open-apis/im/v1/messages/:message_id/push_follow_up
        let req: ApiRequest<EmptyData> = ApiRequest::post(format!(
            "{}/{}/push_follow_up",
            IM_V1_MESSAGES, self.message_id
        ))
        .body(serialize_params(&body, "添加跟随气泡")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "添加跟随气泡")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_push_follow_up_request_builder() {
        let config = Config::default();
        let request = PushFollowUpRequest::new(config).message_id("om_xxx");
        assert_eq!(request.message_id, "om_xxx");
    }

    #[test]
    fn test_follow_up() {
        let follow_up = FollowUp {
            content: "查看详情".to_string(),
            i18n_contents: None,
        };
        assert_eq!(follow_up.content, "查看详情");
        assert_eq!(follow_up.i18n_contents, None);
    }

    #[test]
    fn test_follow_up_with_i18n() {
        let follow_up = FollowUp {
            content: "查看详情".to_string(),
            i18n_contents: Some(vec![FollowUpI18nContent {
                content: "View Details".to_string(),
                language: "en_US".to_string(),
            }]),
        };
        assert_eq!(follow_up.i18n_contents.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_push_follow_up_body() {
        let body = PushFollowUpBody {
            follow_ups: vec![
                FollowUp {
                    content: "查看详情".to_string(),
                    i18n_contents: None,
                },
                FollowUp {
                    content: "了解更多".to_string(),
                    i18n_contents: None,
                },
            ],
        };
        assert_eq!(body.follow_ups.len(), 2);
    }

    #[test]
    fn test_empty_message_id() {
        let config = Config::default();
        let request = PushFollowUpRequest::new(config).message_id("");
        assert_eq!(request.message_id, "");
    }

    #[test]
    fn test_follow_up_i18n_content() {
        let i18n = FollowUpI18nContent {
            content: "View Details".to_string(),
            language: "en_US".to_string(),
        };
        assert_eq!(i18n.content, "View Details");
        assert_eq!(i18n.language, "en_US");
    }
}
