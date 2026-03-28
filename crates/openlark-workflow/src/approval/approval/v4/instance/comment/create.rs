//! 创建审批实例评论（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/instance_comment/create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 创建审批实例评论请求体（v4）
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateInstanceCommentBodyV4 {
    /// 评论内容
    pub content: String,
    /// 是否为私密评论
    #[serde(default)]
    pub is_private: bool,
}

/// 创建审批实例评论响应（v4）
#[derive(Debug, Clone, Deserialize)]
pub struct CreateInstanceCommentResponseV4 {
    /// 评论 ID
    pub comment_id: String,
    /// 创建时间（Unix 时间戳）
    pub create_time: i64,
}

/// 创建审批实例评论请求（v4）
#[derive(Debug, Clone)]
pub struct CreateInstanceCommentRequestV4 {
    config: Arc<Config>,
    instance_id: String,
    body: CreateInstanceCommentBodyV4,
}

impl CreateInstanceCommentRequestV4 {
    pub fn new(config: Arc<Config>, instance_id: impl Into<String>) -> Self {
        Self {
            config,
            instance_id: instance_id.into(),
            body: CreateInstanceCommentBodyV4::default(),
        }
    }

    /// 设置评论内容
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.body.content = content.into();
        self
    }

    /// 设置是否为私密评论
    pub fn is_private(mut self, is_private: bool) -> Self {
        self.body.is_private = is_private;
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
        validate_required!(self.body.content.trim(), "评论内容不能为空");

        let api_endpoint =
            crate::common::api_endpoints::ApprovalApiV4::InstanceCommentCreate(self.instance_id);
        let mut request =
            ApiRequest::<CreateInstanceCommentResponseV4>::post(api_endpoint.to_url());

        let body_json = serde_json::to_value(&self.body).map_err(|e| {
            openlark_core::error::validation_error("序列化请求体失败", e.to_string().as_str())
        })?;

        request = request.body(body_json);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
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
}
