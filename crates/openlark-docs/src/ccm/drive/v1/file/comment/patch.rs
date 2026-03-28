//! 解决/恢复评论

//!

//! docPath: <https://open.feishu.cn/document/server-docs/docs/CommentAPI/patch>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 解决/恢复评论请求
///
/// 用于标记评论为已解决或恢复未解决状态。
///
/// # 字段说明
///
/// - `file_token`: 文件的唯一标识符
/// - `comment_id`: 评论的唯一标识符
/// - `is_solved`: true 表示解决评论，false 表示恢复评论
/// - `file_type`: 文件类型，支持 docx/sheet/bitable/file 等
/// - `user_id_type`: 用户 ID 类型，默认为 open_id
///
/// # 示例
///
/// ```rust,no_run
/// use openlark_docs::ccm::drive::v1::file::comment::PatchCommentRequest;
///
/// let request = PatchCommentRequest::new("file_token", "comment_123", true, "docx");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchCommentRequest {
    /// 文件 token
    pub file_token: String,

    /// 评论 ID
    pub comment_id: String,

    /// 评论解决标志：true=解决，false=恢复
    pub is_solved: bool,

    /// 文件类型（必填）
    pub file_type: String,

    /// 用户 ID 类型（默认 open_id）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl PatchCommentRequest {
    pub fn new(
        file_token: impl Into<String>,

        comment_id: impl Into<String>,

        is_solved: bool,

        file_type: impl Into<String>,
    ) -> Self {
        Self {
            file_token: file_token.into(),

            comment_id: comment_id.into(),

            is_solved,

            file_type: file_type.into(),

            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());

        self
    }
}

#[derive(Debug, Serialize)]

struct PatchCommentRequestBody {
    is_solved: bool,
}

/// 解决/恢复评论响应（data 为空对象）
///
/// 成功时响应体 data 字段为空对象 `{}`。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchCommentResponse {}

impl ApiResponseTrait for PatchCommentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 解决/恢复评论
///
/// 将指定评论标记为已解决或恢复为未解决状态。
///
/// # 参数
///
/// - `request`: 解决/恢复评论请求
/// - `config`: SDK 配置
/// - `option`: 可选的请求选项
///
/// # 返回
///
/// 成功时返回空响应对象。
///
/// # 错误
///
/// - `file_token` 为空或格式错误
/// - `comment_id` 为空
/// - `file_type` 为空或不受支持
pub async fn patch_comment(
    request: PatchCommentRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<PatchCommentResponse> {
    // ===== 参数校验 =====
    if request.file_token.trim().is_empty() {
        return Err(openlark_core::error::validation_error(
            "file_token",
            "file_token 不能为空",
        ));
    }

    if request.comment_id.trim().is_empty() {
        return Err(openlark_core::error::validation_error(
            "comment_id",
            "comment_id 不能为空",
        ));
    }

    if request.file_type.trim().is_empty() {
        return Err(openlark_core::error::validation_error(
            "file_type",
            "file_type 不能为空",
        ));
    }

    super::validate_comment_file_type_for_list_like(&request.file_type)?;

    // ===== 构建请求 =====
    let api_endpoint =
        DriveApi::PatchComment(request.file_token.clone(), request.comment_id.clone());

    let mut api_request: ApiRequest<PatchCommentResponse> =
        ApiRequest::patch(&api_endpoint.to_url()).body(serialize_params(
            &PatchCommentRequestBody {
                is_solved: request.is_solved,
            },
            "解决/恢复评论",
        )?);

    api_request = api_request.query("file_type", &request.file_type);

    if let Some(user_id_type) = &request.user_id_type {
        api_request = api_request.query("user_id_type", user_id_type);
    }

    // ===== 发送请求 =====
    let response = Transport::request(api_request, config, option).await?;

    extract_response_data(response, "解决/恢复评论")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_patch_comment_request_builder() {
        let request = PatchCommentRequest::new("file_token", "comment_123", true, "docx");

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.comment_id, "comment_123");
        assert!(request.is_solved);
        assert_eq!(request.file_type, "docx");
    }

    #[test]
    fn test_patch_comment_request_with_user_id_type() {
        let request = PatchCommentRequest::new("file_token", "comment_123", false, "sheet")
            .user_id_type("union_id");

        assert_eq!(request.user_id_type, Some("union_id".to_string()));
        assert!(!request.is_solved);
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(PatchCommentResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_patch_comment_response_empty() {
        let response = PatchCommentResponse {};
        // 验证空结构体可以正常创建
        let _ = response;
    }

    #[test]
    fn test_patch_comment_solve_flag() {
        let solve_request = PatchCommentRequest::new("file", "comment", true, "docx");
        let unsolve_request = PatchCommentRequest::new("file", "comment", false, "docx");

        assert!(solve_request.is_solved);
        assert!(!unsolve_request.is_solved);
    }

    #[test]
    fn test_patch_comment_file_types() {
        let types = vec!["docx", "sheet", "bitable", "file"];
        for file_type in types {
            let request = PatchCommentRequest::new("file", "comment", true, file_type);
            assert_eq!(request.file_type, file_type);
        }
    }
}
