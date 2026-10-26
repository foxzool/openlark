//! 删除审批实例评论（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/instance_comment/delete>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use serde::Deserialize;
use std::sync::Arc;

/// 删除审批实例评论响应（v4）
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteInstanceCommentResponseV4 {}

/// 删除审批实例评论请求（v4）
#[derive(Debug, Clone)]
pub struct DeleteInstanceCommentRequestV4 {
    config: Arc<Config>,
    instance_id: String,
    comment_id: String,
    /// 用户 ID（必填，ID 类型与 user_id_type 取值一致）
    user_id: String,
    /// 用户 ID 类型（open_id / union_id / user_id，默认 open_id）
    user_id_type: Option<String>,
}

impl DeleteInstanceCommentRequestV4 {
    /// 创建请求实例。
    pub fn new(
        config: Arc<Config>,
        instance_id: impl Into<String>,
        comment_id: impl Into<String>,
        user_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            instance_id: instance_id.into(),
            comment_id: comment_id.into(),
            user_id: user_id.into(),
            user_id_type: None,
        }
    }

    /// 设置用户 ID 类型（open_id / union_id / user_id，默认 open_id）
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DeleteInstanceCommentResponseV4> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteInstanceCommentResponseV4> {
        validate_required!(self.instance_id.trim(), "审批实例 ID 不能为空");
        validate_required!(self.comment_id.trim(), "评论 ID 不能为空");
        validate_required!(self.user_id.trim(), "用户 ID 不能为空");

        let api_endpoint = crate::common::api_endpoints::ApprovalApiV4::InstanceCommentDelete(
            self.instance_id,
            self.comment_id,
        );
        let mut request =
            ApiRequest::<DeleteInstanceCommentResponseV4>::delete(api_endpoint.to_url())
                .query("user_id", self.user_id);
        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "响应数据为空",
        )
        .await
    }
}

impl ApiResponseTrait for DeleteInstanceCommentResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

    #[test]
    fn test_instance_comment_delete_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::InstanceCommentDelete(
            "test_instance_id".to_string(),
            "test_comment_id".to_string(),
        );
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/approval/v4/instances/test_instance_id/comments/test_comment_id"
        );
    }
}
