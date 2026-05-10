//! 获取白板节点列表（v1）
//! docPath: https://open.feishu.cn/document/docs/board-v1/whiteboard-node/list

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Deserialize)]
/// 白板节点信息。
pub struct WhiteboardNode {
    /// 节点 ID。
    pub node_id: String,
    /// 节点标题。
    pub title: String,
    /// 节点类型。
    pub node_type: String,
    #[serde(default)]
    /// 父节点 ID。
    pub parent_id: String,
    #[serde(default)]
    /// 节点内容。
    pub content: serde_json::Value,
    #[serde(default)]
    /// 节点位置。
    pub position: NodePosition,
    /// 创建时间。
    pub create_time: i64,
    #[serde(default)]
    /// 更新时间。
    pub update_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// 节点位置信息。
pub struct NodePosition {
    #[serde(default)]
    /// 横坐标。
    pub x: f64,
    #[serde(default)]
    /// 纵坐标。
    pub y: f64,
    #[serde(default)]
    /// 宽度。
    pub width: f64,
    #[serde(default)]
    /// 高度。
    pub height: f64,
}

#[derive(Debug, Clone, Deserialize)]
/// 白板节点列表响应。
pub struct ListWhiteboardNodeResponseV1 {
    /// 节点列表。
    pub nodes: Vec<WhiteboardNode>,
    #[serde(default)]
    /// 是否还有更多数据。
    pub has_more: bool,
    #[serde(default)]
    /// 分页令牌。
    pub page_token: String,
}

#[derive(Debug, Clone)]
/// 获取白板节点列表请求构建器。
pub struct ListWhiteboardNodeRequestV1 {
    config: Arc<Config>,
    board_id: String,
    parent_id: Option<String>,
    page_size: Option<u32>,
    page_token: Option<String>,
}

impl ListWhiteboardNodeRequestV1 {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, board_id: impl Into<String>) -> Self {
        Self {
            config,
            board_id: board_id.into(),
            parent_id: None,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置父节点过滤条件。
    pub fn parent_id(mut self, parent_id: impl Into<String>) -> Self {
        self.parent_id = Some(parent_id.into());
        self
    }

    /// 设置分页大小。
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页令牌。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<ListWhiteboardNodeResponseV1> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListWhiteboardNodeResponseV1> {
        validate_required!(self.board_id.trim(), "白板 ID 不能为空");

        let api_endpoint =
            crate::common::api_endpoints::BoardApiV1::WhiteboardNodeList(self.board_id);
        let mut request = ApiRequest::<ListWhiteboardNodeResponseV1>::get(api_endpoint.to_url());

        if let Some(parent_id) = self.parent_id {
            request = request.query_param("parent_id", parent_id);
        }
        if let Some(page_size) = self.page_size {
            request = request.query_param("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            request = request.query_param("page_token", page_token);
        }

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

impl ApiResponseTrait for ListWhiteboardNodeResponseV1 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

    #[test]
    fn test_whiteboard_node_list_v1_url() {
        let endpoint = crate::common::api_endpoints::BoardApiV1::WhiteboardNodeList(
            "test_board_id".to_string(),
        );
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/board/v1/whiteboards/test_board_id/nodes"
        );
    }
}
