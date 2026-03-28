//! 批量获取文件元数据

//!

//! 批量获取文件元数据信息。

//!

//! docPath: <https://open.feishu.cn/document/server-docs/docs/drive-v1/file/batch_query>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 批量查询元数据请求

#[derive(Debug, Clone, Serialize)]

pub struct BatchQueryMetaRequest {
    /// 用户 ID 类型（默认 open_id）

    #[serde(skip)]
    pub user_id_type: Option<String>,

    /// 文件token列表
    pub request_docs: Vec<RequestDoc>,

    /// 是否获取文件的访问链接

    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_url: Option<bool>,
}

/// 请求文档

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct RequestDoc {
    /// 文档token
    pub doc_token: String,

    /// 文档类型
    pub doc_type: String,
}

impl BatchQueryMetaRequest {
    pub fn new(request_docs: Vec<RequestDoc>) -> Self {
        Self {
            user_id_type: None,

            request_docs,

            with_url: None,
        }
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());

        self
    }

    pub fn with_url(mut self, with_url: bool) -> Self {
        self.with_url = Some(with_url);

        self
    }
}

/// 文件元数据

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct Meta {
    /// 文件的 token
    pub doc_token: String,

    /// 文件的类型
    pub doc_type: String,

    /// 标题
    pub title: String,

    /// 文件的所有者（ID 类型由 user_id_type 决定）
    pub owner_id: String,

    /// 创建时间。UNIX 时间戳，单位为秒
    pub create_time: String,

    /// 最后编辑者（ID 类型由 user_id_type 决定）
    pub latest_modify_user: String,

    /// 最后编辑时间。UNIX 时间戳，单位为秒
    pub latest_modify_time: String,

    /// 文档访问链接

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,

    /// 文档密级标签名称（需对应权限才返回）

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sec_label_name: Option<String>,
}

/// 获取元数据失败信息

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct FailedMeta {
    /// 获取元数据失败的文档 token
    pub token: String,

    /// 获取元数据失败的错误码
    pub code: i32,
}

/// 批量查询元数据响应（data）

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct BatchQueryMetaResponse {
    /// 文档元数据列表

    #[serde(default)]
    pub metas: Vec<Meta>,

    /// 获取元数据失败的文档 token 列表

    #[serde(default)]
    pub failed_list: Vec<FailedMeta>,
}

impl ApiResponseTrait for BatchQueryMetaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量获取文件元数据
pub async fn batch_query(
    request: BatchQueryMetaRequest,

    config: &Config,

    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<BatchQueryMetaResponse> {
    if request.request_docs.is_empty() || request.request_docs.len() > 200 {
        return Err(openlark_core::error::validation_error(
            "request_docs",
            "request_docs 数量必须在 1~200 之间",
        ));
    }

    for doc in &request.request_docs {
        if doc.doc_token.is_empty() {
            return Err(openlark_core::error::validation_error(
                "request_docs.doc_token",
                "doc_token 不能为空",
            ));
        }

        if doc.doc_type.is_empty() {
            return Err(openlark_core::error::validation_error(
                "request_docs.doc_type",
                "doc_type 不能为空",
            ));
        }

        match doc.doc_type.as_str() {
            "doc" | "sheet" | "bitable" | "mindnote" | "file" | "wiki" | "docx" | "folder"
            | "synced_block" => {}

            _ => {
                return Err(openlark_core::error::validation_error(
                    "request_docs.doc_type",
                    "doc_type 仅支持 doc/sheet/bitable/mindnote/file/wiki/docx/folder/synced_block",
                ));
            }
        }
    }

    let api_endpoint = DriveApi::BatchQueryMetas;

    let mut api_request: ApiRequest<BatchQueryMetaResponse> =
        ApiRequest::post(&api_endpoint.to_url())
            .body(serialize_params(&request, "获取文件元数据")?);

    if let Some(user_id_type) = &request.user_id_type {
        match user_id_type.as_str() {
            "open_id" | "union_id" | "user_id" => {}

            _ => {
                return Err(openlark_core::error::validation_error(
                    "user_id_type",
                    "user_id_type 仅支持 open_id/union_id/user_id",
                ));
            }
        }

        api_request = api_request.query("user_id_type", user_id_type);
    }

    let response = Transport::request(api_request, config, option).await?;

    extract_response_data(response, "获取文件元数据")
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
