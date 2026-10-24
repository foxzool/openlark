//! 创建评论（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/instance-comment/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_utils::request_serialization_error;

/// 评论中艾特人信息（comment_at_info）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CommentAtInfoV4 {
    /// 被艾特人的 ID
    pub user_id: String,
    /// 被艾特人的姓名
    pub name: String,
    /// 被艾特人在评论中的位置，从 0 开始
    pub offset: String,
}

/// 创建评论请求体（v4）
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateInstanceCommentBodyV4 {
    /// 评论内容（JSON 压缩转义字符串：text / files 等）
    pub content: String,
    /// 评论中艾特人信息列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_info_list: Option<Vec<CommentAtInfoV4>>,
    /// 父评论 ID（回复评论时传入）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_comment_id: Option<String>,
    /// 评论 ID（编辑、删除一条评论时传入）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_id: Option<String>,
    /// 是否不启用 Bot（true 时只同步数据，不触发 Bot）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_bot: Option<bool>,
    /// 附加字段（JSON 压缩转义字符串）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
}

/// 创建评论响应（v4）
#[derive(Debug, Clone, Deserialize, Default)]
pub struct CreateInstanceCommentResponseV4 {
    /// 评论 ID
    pub comment_id: String,
}

/// 创建评论请求（v4）
#[derive(Debug, Clone)]
pub struct CreateInstanceCommentRequestV4 {
    config: Arc<Config>,
    instance_id: String,
    body: CreateInstanceCommentBodyV4,
}

impl CreateInstanceCommentRequestV4 {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>, instance_id: impl Into<String>) -> Self {
        Self {
            config,
            instance_id: instance_id.into(),
            body: CreateInstanceCommentBodyV4::default(),
        }
    }

    /// 设置评论内容（JSON 压缩转义字符串）
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.body.content = content.into();
        self
    }

    /// 设置评论中艾特人信息列表
    pub fn at_info_list(mut self, at_info_list: Vec<CommentAtInfoV4>) -> Self {
        self.body.at_info_list = Some(at_info_list);
        self
    }

    /// 添加一条艾特人信息
    pub fn add_at_info(mut self, at_info: CommentAtInfoV4) -> Self {
        let list = self.body.at_info_list.get_or_insert_with(Vec::new);
        list.push(at_info);
        self
    }

    /// 设置父评论 ID（回复评论）
    pub fn parent_comment_id(mut self, parent_comment_id: impl Into<String>) -> Self {
        self.body.parent_comment_id = Some(parent_comment_id.into());
        self
    }

    /// 设置评论 ID（编辑、删除评论）
    pub fn comment_id(mut self, comment_id: impl Into<String>) -> Self {
        self.body.comment_id = Some(comment_id.into());
        self
    }

    /// 设置是否不启用 Bot
    pub fn disable_bot(mut self, disable_bot: bool) -> Self {
        self.body.disable_bot = Some(disable_bot);
        self
    }

    /// 设置附加字段（JSON 压缩转义字符串）
    pub fn extra(mut self, extra: impl Into<String>) -> Self {
        self.body.extra = Some(extra.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateInstanceCommentResponseV4> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateInstanceCommentResponseV4> {
        validate_required!(self.instance_id, "审批实例 Code 不能为空");
        validate_required!(self.body.content, "评论内容不能为空");

        let api_endpoint =
            crate::common::api_endpoints::ApprovalApiV4::InstanceCommentCreate(self.instance_id);
        let mut request =
            ApiRequest::<CreateInstanceCommentResponseV4>::post(api_endpoint.to_url());

        let body_json = serde_json::to_value(&self.body)
            .map_err(|e| request_serialization_error("创建评论", e))?;

        request = request.body(body_json);

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "创建评论",
        )
        .await
    }
}

impl ApiResponseTrait for CreateInstanceCommentResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_instance_comment_create_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::InstanceCommentCreate(
            "test_instance_id".to_string(),
        );
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/approval/v4/instances/test_instance_id/comments"
        );
    }

    #[test]
    fn test_create_comment_request_builder() {
        let config = Arc::new(
            Config::builder()
                .app_id("test_app")
                .app_secret("test_secret")
                .build(),
        );
        let request =
            CreateInstanceCommentRequestV4::new(config, "6A123516-FB88-470D-A428-9AF58B71B3C0")
                .content("{\"text\":\"@username艾特展示\"}")
                .add_at_info(CommentAtInfoV4 {
                    user_id: "579fd9c4".to_string(),
                    name: "张敏".to_string(),
                    offset: "0".to_string(),
                })
                .parent_comment_id("7081516627711524883")
                .disable_bot(false);

        assert_eq!(request.instance_id, "6A123516-FB88-470D-A428-9AF58B71B3C0");
        assert_eq!(request.body.content, "{\"text\":\"@username艾特展示\"}");
        assert_eq!(request.body.at_info_list.as_ref().unwrap().len(), 1);
        assert_eq!(
            request.body.parent_comment_id.as_deref(),
            Some("7081516627711524883")
        );
        assert_eq!(request.body.disable_bot, Some(false));
    }
}
