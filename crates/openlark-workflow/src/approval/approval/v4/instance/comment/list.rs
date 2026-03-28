//! 获取审批实例评论列表（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/instance_comment/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 审批实例评论
#[derive(Debug, Clone, Deserialize)]
pub struct InstanceComment {
    /// 评论 ID
    pub comment_id: String,
    /// 评论内容
    pub content: String,
    /// 发起人用户 ID
    pub user_id: String,
    /// 是否为私密评论
    #[serde(default)]
    pub is_private: bool,
    /// 创建时间（Unix 时间戳）
    pub create_time: i64,
    /// 更新时间（Unix 时间戳）
    #[serde(default)]
    pub update_time: i64,
}

/// 获取审批实例评论列表响应（v4）
#[derive(Debug, Clone, Deserialize)]
pub struct ListInstanceCommentResponseV4 {
    /// 评论列表
    pub comments: Vec<InstanceComment>,
    /// 是否有更多数据
    #[serde(default)]
    pub has_more: bool,
    /// 游标
    #[serde(default)]
    pub page_token: String,
}

/// 获取审批实例评论列表请求（v4）
#[derive(Debug, Clone)]
pub struct ListInstanceCommentRequestV4 {
    config: Arc<Config>,
    instance_id: String,
    /// 页大小
    page_size: Option<u32>,
    /// 游标
    page_token: Option<String>,
}

impl ListInstanceCommentRequestV4 {
    pub fn new(config: Arc<Config>, instance_id: impl Into<String>) -> Self {
        Self {
            config,
            instance_id: instance_id.into(),
            page_size: None,
            page_token: None,
        }
    }

    /// 设置页大小
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置游标
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListInstanceCommentResponseV4> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListInstanceCommentResponseV4> {
        validate_required!(self.instance_id.trim(), "审批实例 ID 不能为空");

        let api_endpoint =
            crate::common::api_endpoints::ApprovalApiV4::InstanceCommentList(self.instance_id);
        let mut request =
            ApiRequest::<ListInstanceCommentResponseV4>::get(api_endpoint.to_url());

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

impl ApiResponseTrait for ListInstanceCommentResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    

    #[test]
    fn test_instance_comment_list_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::InstanceCommentList(
            "test_instance_id".to_string(),
        );
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/approval/v4/instances/test_instance_id/comments"
        );
    }
}
