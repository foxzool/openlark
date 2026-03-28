//! 批量撤回消息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/im-v1/batch_message/delete>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::{api_utils::extract_response_data, models::EmptyData},
    endpoints::IM_V1_BATCH_MESSAGES,
};

/// 批量撤回消息请求
///
/// 用于撤回指定的批量消息任务。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `batch_message_id`: 批量消息任务 ID，必填
///
/// # 示例
///
/// ```rust,ignore
/// let request = DeleteBatchMessageRequest::new(config)
///     .batch_message_id("batch_xxx")
///     .execute().await?;
/// ```
pub struct DeleteBatchMessageRequest {
    config: Config,
    batch_message_id: String,
}

impl DeleteBatchMessageRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            batch_message_id: String::new(),
        }
    }

    /// 批量消息任务 ID（路径参数）
    pub fn batch_message_id(mut self, batch_message_id: impl Into<String>) -> Self {
        self.batch_message_id = batch_message_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/im-v1/batch_message/delete>
    pub async fn execute(self) -> SDKResult<EmptyData> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EmptyData> {
        // === 必填字段验证 ===
        validate_required!(self.batch_message_id, "batch_message_id 不能为空");

        // url: DELETE:/open-apis/im/v1/batch_messages/:batch_message_id
        let req: ApiRequest<EmptyData> = ApiRequest::delete(format!(
            "{}/{}",
            IM_V1_BATCH_MESSAGES, self.batch_message_id
        ));

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "批量撤回消息")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_batch_message_request_builder() {
        let config = Config::default();
        let request = DeleteBatchMessageRequest::new(config).batch_message_id("batch_xxx");
        assert_eq!(request.batch_message_id, "batch_xxx");
    }

    #[test]
    fn test_delete_batch_message_request_default_values() {
        let config = Config::default();
        let request = DeleteBatchMessageRequest::new(config);
        assert_eq!(request.batch_message_id, "");
    }

    #[test]
    fn test_delete_batch_message_request_with_different_ids() {
        let config = Config::default();
        let request1 = DeleteBatchMessageRequest::new(config.clone()).batch_message_id("batch_111");
        let request2 = DeleteBatchMessageRequest::new(config).batch_message_id("batch_222");
        assert_eq!(request1.batch_message_id, "batch_111");
        assert_eq!(request2.batch_message_id, "batch_222");
    }

    #[test]
    fn test_delete_batch_message_request_chaining() {
        let config = Config::default();
        let request = DeleteBatchMessageRequest::new(config).batch_message_id("batch_xxx");
        assert_eq!(request.batch_message_id, "batch_xxx");
    }
}
