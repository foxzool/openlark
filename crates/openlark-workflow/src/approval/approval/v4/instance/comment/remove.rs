//! 清空评论（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/instance-comment/remove>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 清空评论响应（v4）
#[derive(Debug, Clone, Deserialize)]
pub struct RemoveInstanceCommentResponseV4 {
    /// 是否成功
    pub success: bool,
}

/// 清空评论请求（v4）
#[derive(Debug, Clone)]
pub struct RemoveInstanceCommentRequestV4 {
    config: Arc<Config>,
    instance_id: String,
}

impl RemoveInstanceCommentRequestV4 {
    pub fn new(config: Arc<Config>, instance_id: impl Into<String>) -> Self {
        Self {
            config,
            instance_id: instance_id.into(),
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<RemoveInstanceCommentResponseV4> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<RemoveInstanceCommentResponseV4> {
        validate_required!(self.instance_id.trim(), "审批实例 ID 不能为空");

        let url = format!(
            "/open-apis/approval/v4/instances/{}/comments/remove",
            self.instance_id
        );
        let request = ApiRequest::<RemoveInstanceCommentResponseV4>::post(url);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

impl ApiResponseTrait for RemoveInstanceCommentResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    

    #[test]
    fn test_instance_comment_remove_v4_url() {
        let request = RemoveInstanceCommentRequestV4::new(
            openlark_core::config::Config::builder()
                .app_id("test_app_id")
                .app_secret("test_app_secret")
                .build()
                .unwrap(),
            "test_instance_id".to_string(),
        );
        // Just verify it doesn't panic
        let _ = request.clone();
    }
}
