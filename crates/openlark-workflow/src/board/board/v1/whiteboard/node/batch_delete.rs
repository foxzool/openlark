//! 批量删除白板节点。
//!
//! docPath: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard-node/batch_delete>

use crate::common::{BoardV1Endpoint, api_utils::*};
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, validate_required_list,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 批量删除白板节点请求体。
#[derive(Debug, Clone, Serialize, Default)]
pub struct BatchDeleteWhiteboardNodeBodyV1 {
    /// 需要删除的节点 ID 列表。
    pub ids: Vec<String>,
}

/// 批量删除白板节点响应。
#[derive(Debug, Clone, Deserialize)]
pub struct BatchDeleteWhiteboardNodeResponseV1 {
    /// 操作的唯一标识，用于幂等更新。
    pub client_token: String,
}

/// 批量删除白板节点请求。
#[derive(Debug, Clone)]
pub struct BatchDeleteWhiteboardNodeRequestV1 {
    config: Arc<Config>,
    whiteboard_id: String,
    client_token: Option<String>,
    body: BatchDeleteWhiteboardNodeBodyV1,
}

impl BatchDeleteWhiteboardNodeRequestV1 {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, whiteboard_id: impl Into<String>) -> Self {
        Self {
            config,
            whiteboard_id: whiteboard_id.into(),
            client_token: None,
            body: BatchDeleteWhiteboardNodeBodyV1::default(),
        }
    }

    /// 设置幂等操作标识。
    pub fn client_token(mut self, client_token: impl Into<String>) -> Self {
        self.client_token = Some(client_token.into());
        self
    }

    /// 设置要删除的节点 ID 列表。
    pub fn ids(mut self, ids: Vec<String>) -> Self {
        self.body.ids = ids;
        self
    }

    /// 添加单个要删除的节点 ID。
    pub fn add_id(mut self, id: impl Into<String>) -> Self {
        self.body.ids.push(id.into());
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<BatchDeleteWhiteboardNodeResponseV1> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchDeleteWhiteboardNodeResponseV1> {
        validate_required!(self.whiteboard_id.trim(), "白板 ID 不能为空");
        validate_required_list!(self.body.ids, 100, "节点 ID 列表不能为空且不能超过 100 个");

        let api_endpoint = BoardV1Endpoint::WhiteboardNodeBatchDelete(self.whiteboard_id.clone());
        let mut request =
            ApiRequest::<BatchDeleteWhiteboardNodeResponseV1>::delete(api_endpoint.to_url());

        if let Some(client_token) = self.client_token {
            request = request.query("client_token", client_token);
        }

        request = request.body(serialize_params(&self.body, "批量删除白板节点")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "批量删除白板节点")
    }
}

impl ApiResponseTrait for BatchDeleteWhiteboardNodeResponseV1 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn issue_194_batch_delete_whiteboard_node_builder() {
        let request =
            BatchDeleteWhiteboardNodeRequestV1::new(Arc::new(Config::default()), "whiteboard_123")
                .client_token("token_123")
                .add_id("o1:1")
                .add_id("o1:2");

        assert_eq!(request.whiteboard_id, "whiteboard_123");
        assert_eq!(request.client_token.as_deref(), Some("token_123"));
        assert_eq!(request.body.ids, vec!["o1:1", "o1:2"]);
    }
}
