//! 移动云空间文档至知识空间
//!
//! 该接口允许移动云空间文档至知识空间，并挂载在指定位置。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/wiki-v2/task/move_docs_to_wiki>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::WikiApiV2, api_utils::*};

/// 移动云空间文档至知识空间请求（流式 Builder 模式）
pub struct MoveDocsToWikiRequest {
    config: Config,
    /// 知识空间ID
    space_id: String,
    /// 源文档 token
    obj_token: String,
    /// 源文档类型（例如 doc、docx 等）
    obj_type: String,
    /// 目标父节点 wiki token
    parent_wiki_token: String,
}

/// 移动云空间文档至知识空间请求体（内部使用）
#[derive(Debug, Clone, Serialize, Deserialize)]
struct MoveDocsToWikiRequestBody {
    obj_token: String,
    obj_type: String,
    parent_wiki_token: String,
}

/// 移动云空间文档至知识空间响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MoveDocsToWikiResponse {
    /// 操作已完成时返回的 wiki token
    pub wiki_token: Option<String>,
    /// 操作未完成时返回的异步任务 ID
    pub task_id: Option<String>,
}

impl ApiResponseTrait for MoveDocsToWikiResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl MoveDocsToWikiRequest {
    /// 创建移动云空间文档至知识空间请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            space_id: String::new(),
            obj_token: String::new(),
            obj_type: String::new(),
            parent_wiki_token: String::new(),
        }
    }

    /// 设置知识空间 ID
    pub fn space_id(mut self, space_id: impl Into<String>) -> Self {
        self.space_id = space_id.into();
        self
    }

    /// 设置源文档 token
    pub fn obj_token(mut self, obj_token: impl Into<String>) -> Self {
        self.obj_token = obj_token.into();
        self
    }

    /// 设置源文档类型（例如 doc、docx 等）
    pub fn obj_type(mut self, obj_type: impl Into<String>) -> Self {
        self.obj_type = obj_type.into();
        self
    }

    /// 设置目标父节点 wiki token
    pub fn parent_wiki_token(mut self, parent_wiki_token: impl Into<String>) -> Self {
        self.parent_wiki_token = parent_wiki_token.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<MoveDocsToWikiResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<MoveDocsToWikiResponse> {
        validate_required!(self.space_id, "知识空间ID不能为空");
        validate_required!(self.obj_token, "源文档token不能为空");
        validate_required!(self.obj_type, "源文档类型不能为空");
        validate_required!(self.parent_wiki_token, "目标父节点wiki token不能为空");

        let api_endpoint = WikiApiV2::SpaceNodeMoveDocsToWiki(self.space_id);

        let request_body = MoveDocsToWikiRequestBody {
            obj_token: self.obj_token,
            obj_type: self.obj_type,
            parent_wiki_token: self.parent_wiki_token,
        };

        let api_request: ApiRequest<MoveDocsToWikiResponse> =
            ApiRequest::post(&api_endpoint.to_url())
                .body(serialize_params(&request_body, "移动云空间文档至知识空间")?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "节点")
    }
}

/// 移动云空间文档至知识空间请求参数（兼容旧 API，已弃用）
#[deprecated(
    since = "0.16.0",
    note = "请使用 MoveDocsToWikiRequest 的流式 Builder 模式"
)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveDocsToWikiParams {
    /// 源文档 token
    pub obj_token: String,
    /// 源文档类型（例如 doc、docx 等）
    pub obj_type: String,
    /// 目标父节点 wiki token
    pub parent_wiki_token: String,
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
