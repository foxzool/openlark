//! 获取会话历史消息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/im-v1/message/list>

use openlark_core::{
    api::ApiRequest, config::Config, error, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::api_utils::extract_response_data,
    endpoints::IM_V1_MESSAGES,
    im::im::v1::message::models::{ContainerIdType, SortType},
};

/// 获取会话历史消息请求
pub struct ListMessagesRequest {
    config: Config,
    container_id_type: Option<ContainerIdType>,
    container_id: String,
    start_time: Option<String>,
    end_time: Option<String>,
    sort_type: Option<SortType>,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl ListMessagesRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            container_id_type: None,
            container_id: String::new(),
            start_time: None,
            end_time: None,
            sort_type: None,
            page_size: None,
            page_token: None,
        }
    }

    /// 容器类型（查询参数，必填）
    pub fn container_id_type(mut self, container_id_type: ContainerIdType) -> Self {
        self.container_id_type = Some(container_id_type);
        self
    }

    /// 容器 ID（查询参数，必填）
    pub fn container_id(mut self, container_id: impl Into<String>) -> Self {
        self.container_id = container_id.into();
        self
    }

    /// 起始时间（秒级时间戳，可选）
    pub fn start_time(mut self, start_time: impl Into<String>) -> Self {
        self.start_time = Some(start_time.into());
        self
    }

    /// 结束时间（秒级时间戳，可选）
    pub fn end_time(mut self, end_time: impl Into<String>) -> Self {
        self.end_time = Some(end_time.into());
        self
    }

    /// 消息排序方式（可选，默认 ByCreateTimeAsc）
    pub fn sort_type(mut self, sort_type: SortType) -> Self {
        self.sort_type = Some(sort_type);
        self
    }

    /// 分页大小（可选，默认 20，范围 1~50）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 分页标记（可选）
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/im-v1/message/list>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        validate_required!(self.container_id, "container_id 不能为空");
        let container_id_type = self.container_id_type.ok_or_else(|| {
            error::validation_error(
                "container_id_type 不能为空".to_string(),
                "获取会话历史消息需要指定 container_id_type".to_string(),
            )
        })?;

        // url: GET:/open-apis/im/v1/messages
        let mut req: ApiRequest<serde_json::Value> = ApiRequest::get(IM_V1_MESSAGES)
            .query("container_id_type", container_id_type.as_str())
            .query("container_id", self.container_id);

        if let Some(start_time) = self.start_time {
            req = req.query("start_time", start_time);
        }
        if let Some(end_time) = self.end_time {
            req = req.query("end_time", end_time);
        }
        if let Some(sort_type) = self.sort_type {
            req = req.query("sort_type", sort_type.as_str());
        }
        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            req = req.query("page_token", page_token);
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取会话历史消息")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_list_messages_request_builder() {
        let request = ListMessagesRequest::new(Config::default())
            .container_id_type(ContainerIdType::Chat)
            .container_id("oc_123")
            .start_time("1700000000")
            .end_time("1700003600")
            .sort_type(SortType::ByCreateTimeDesc)
            .page_size(50)
            .page_token("next_page");

        assert_eq!(request.container_id_type, Some(ContainerIdType::Chat));
        assert_eq!(request.container_id, "oc_123");
        assert_eq!(request.start_time, Some("1700000000".to_string()));
        assert_eq!(request.end_time, Some("1700003600".to_string()));
        assert_eq!(request.sort_type, Some(SortType::ByCreateTimeDesc));
        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token, Some("next_page".to_string()));
    }

    #[tokio::test]
    async fn test_list_messages_request_validate_container_id() {
        let result = ListMessagesRequest::new(Config::default())
            .container_id_type(ContainerIdType::Chat)
            .execute_with_options(openlark_core::req_option::RequestOption::default())
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_list_messages_request_validate_container_id_type() {
        let result = ListMessagesRequest::new(Config::default())
            .container_id("oc_123")
            .execute_with_options(openlark_core::req_option::RequestOption::default())
            .await;

        assert!(result.is_err());
    }
}
