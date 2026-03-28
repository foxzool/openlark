//! 批量获取评论

//!

//! docPath: <https://open.feishu.cn/document/server-docs/docs/CommentAPI/batch_query>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

use super::models::Comment;

/// 批量获取评论请求
///
/// 根据评论 ID 批量获取评论详情。
///
/// # 字段说明
///
/// - `file_token`: 文件的唯一标识符
/// - `file_type`: 文件类型，支持 docx/sheet/bitable/file 等
/// - `comment_ids`: 评论 ID 列表
/// - `user_id_type`: 用户 ID 类型，默认为 open_id
///
/// # 示例
///
/// ```rust,no_run
/// use openlark_docs::ccm::drive::v1::file::comment::BatchQueryCommentRequest;
///
/// let comment_ids = vec!["comment_1".to_string(), "comment_2".to_string()];
/// let request = BatchQueryCommentRequest::new("file_token", "docx", comment_ids);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchQueryCommentRequest {
    /// 文件 token
    pub file_token: String,

    /// 文件类型（必填）
    pub file_type: String,

    /// 评论 ID 列表
    pub comment_ids: Vec<String>,

    /// 用户 ID 类型（默认 open_id）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl BatchQueryCommentRequest {
    pub fn new(
        file_token: impl Into<String>,

        file_type: impl Into<String>,

        comment_ids: Vec<String>,
    ) -> Self {
        Self {
            file_token: file_token.into(),

            file_type: file_type.into(),

            comment_ids,

            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());

        self
    }
}

#[derive(Debug, Serialize)]

struct BatchQueryCommentRequestBody {
    comment_ids: Vec<String>,
}

/// 批量获取评论响应（data）
///
/// 包含请求的评论详情列表。
///
/// # 字段说明
///
/// - `items`: 评论详情列表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchQueryCommentResponse {
    #[serde(default)]
    pub items: Vec<Comment>,
}

impl ApiResponseTrait for BatchQueryCommentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量获取评论
///
/// 根据评论 ID 批量获取评论详情。
///
/// # 参数
///
/// - `request`: 批量获取评论请求
/// - `config`: SDK 配置
/// - `option`: 可选的请求选项
///
/// # 返回
///
/// 返回评论详情列表。
///
/// # 错误
///
/// - `file_token` 为空
/// - `file_type` 为空或不受支持
/// - `comment_ids` 为空
pub async fn batch_query_comment(
    request: BatchQueryCommentRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<BatchQueryCommentResponse> {
    // ===== 参数校验 =====
    if request.file_token.trim().is_empty() {
        return Err(openlark_core::error::validation_error(
            "file_token",
            "file_token 不能为空",
        ));
    }

    if request.file_type.trim().is_empty() {
        return Err(openlark_core::error::validation_error(
            "file_type",
            "file_type 不能为空",
        ));
    }

    super::validate_comment_file_type_for_list_like(&request.file_type)?;

    if request.comment_ids.is_empty() {
        return Err(openlark_core::error::validation_error(
            "comment_ids",
            "comment_ids 不能为空",
        ));
    }

    // ===== 构建请求 =====
    let api_endpoint = DriveApi::BatchQueryComments(request.file_token.clone());

    let mut api_request: ApiRequest<BatchQueryCommentResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(
            &BatchQueryCommentRequestBody {
                comment_ids: request.comment_ids,
            },
            "批量获取评论",
        )?);

    api_request = api_request.query("file_type", &request.file_type);

    if let Some(user_id_type) = &request.user_id_type {
        api_request = api_request.query("user_id_type", user_id_type);
    }

    // ===== 发送请求 =====
    let response = Transport::request(api_request, config, option).await?;

    extract_response_data(response, "批量获取评论")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_query_comment_request_builder() {
        let comment_ids = vec!["comment_1".to_string(), "comment_2".to_string()];

        let request = BatchQueryCommentRequest::new("file_token", "docx", comment_ids);

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.file_type, "docx");
        assert_eq!(request.comment_ids.len(), 2);
    }

    #[test]
    fn test_batch_query_comment_with_user_id_type() {
        let comment_ids = vec!["comment_1".to_string()];
        let request = BatchQueryCommentRequest::new("file_token", "sheet", comment_ids)
            .user_id_type("union_id");

        assert_eq!(request.user_id_type, Some("union_id".to_string()));
    }

    #[test]
    fn test_batch_query_comment_single_id() {
        let comment_ids = vec!["comment_123".to_string()];
        let request = BatchQueryCommentRequest::new("file_token", "docx", comment_ids);

        assert_eq!(request.comment_ids.len(), 1);
        assert_eq!(request.comment_ids[0], "comment_123");
    }

    #[test]
    fn test_batch_query_response_structure() {
        let response = BatchQueryCommentResponse { items: vec![] };

        assert!(response.items.is_empty());
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            BatchQueryCommentResponse::data_format(),
            ResponseFormat::Data
        );
    }

    #[test]
    fn test_batch_query_multiple_comments() {
        let comment_ids = vec![
            "c1".to_string(),
            "c2".to_string(),
            "c3".to_string(),
            "c4".to_string(),
            "c5".to_string(),
        ];
        let request = BatchQueryCommentRequest::new("file", "bitable", comment_ids);

        assert_eq!(request.comment_ids.len(), 5);
    }
}
