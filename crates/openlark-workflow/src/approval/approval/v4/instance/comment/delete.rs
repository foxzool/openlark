//! 删除审批实例评论（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/instance_comment/delete>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
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
}

impl DeleteInstanceCommentRequestV4 {
    pub fn new(
        config: Arc<Config>,
        instance_id: impl Into<String>,
        comment_id: impl Into<String>,
    ) -> Self {
        Self {
            config,
            instance_id: instance_id.into(),
            comment_id: comment_id.into(),
        }
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

        let api_endpoint = crate::common::api_endpoints::ApprovalApiV4::InstanceCommentDelete(
            self.instance_id,
            self.comment_id,
        );
        let request =
            ApiRequest::<DeleteInstanceCommentResponseV4>::delete(api_endpoint.to_url());

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
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
