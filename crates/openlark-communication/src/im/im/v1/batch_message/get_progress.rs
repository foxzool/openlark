//! 查询批量消息整体进度
//!
//! docPath: <https://open.feishu.cn/document/server-docs/im-v1/batch_message/get_progress>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{common::api_utils::extract_response_data, endpoints::IM_V1_BATCH_MESSAGES};

/// 查询批量消息整体进度请求
///
/// 用于查询指定批量消息任务的整体发送进度信息。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `batch_message_id`: 批量消息任务 ID，必填
///
/// # 示例
///
/// ```rust,ignore
/// let request = GetBatchMessageProgressRequest::new(config)
///     .batch_message_id("batch_xxx")
///     .execute().await?;
/// ```
pub struct GetBatchMessageProgressRequest {
    config: Config,
    batch_message_id: String,
}

impl GetBatchMessageProgressRequest {
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
    /// docPath: <https://open.feishu.cn/document/server-docs/im-v1/batch_message/get_progress>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // === 必填字段验证 ===
        validate_required!(self.batch_message_id, "batch_message_id 不能为空");

        // url: GET:/open-apis/im/v1/batch_messages/:batch_message_id/get_progress
        let req: ApiRequest<serde_json::Value> = ApiRequest::get(format!(
            "{}/{}/get_progress",
            IM_V1_BATCH_MESSAGES, self.batch_message_id
        ));

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "查询批量消息整体进度")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_get_batch_message_progress_request_builder() {
        let config = Config::default();
        let request = GetBatchMessageProgressRequest::new(config).batch_message_id("batch_xxx");
        assert_eq!(request.batch_message_id, "batch_xxx");
    }

    #[test]
    fn test_get_batch_message_progress_request_default_values() {
        let config = Config::default();
        let request = GetBatchMessageProgressRequest::new(config);
        assert_eq!(request.batch_message_id, "");
    }

    #[test]
    fn test_get_batch_message_progress_request_with_different_ids() {
        let config = Config::default();
        let request1 =
            GetBatchMessageProgressRequest::new(config.clone()).batch_message_id("batch_111");
        let request2 = GetBatchMessageProgressRequest::new(config).batch_message_id("batch_222");
        assert_eq!(request1.batch_message_id, "batch_111");
        assert_eq!(request2.batch_message_id, "batch_222");
    }

    #[test]
    fn test_get_batch_message_progress_request_chaining() {
        let config = Config::default();
        let request = GetBatchMessageProgressRequest::new(config).batch_message_id("batch_xxx");
        assert_eq!(request.batch_message_id, "batch_xxx");
    }
}
