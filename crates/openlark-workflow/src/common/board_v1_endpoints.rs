//! Board v1 新增端点。

/// Board v1 API 端点。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoardV1Endpoint {
    /// 批量删除白板节点。
    WhiteboardNodeBatchDelete(String),
}

impl BoardV1Endpoint {
    /// 生成对应的 URL。
    pub fn to_url(&self) -> String {
        match self {
            BoardV1Endpoint::WhiteboardNodeBatchDelete(whiteboard_id) => {
                format!("/open-apis/board/v1/whiteboards/{whiteboard_id}/nodes/batch_delete")
            }
        }
    }
}
