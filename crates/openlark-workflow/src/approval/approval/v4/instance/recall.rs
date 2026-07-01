//! 撤回审批实例（用户级，v4）
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/recall>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_utils::{missing_response_data_error, request_serialization_error};

/// 撤回审批实例请求体（用户级，v4）
#[derive(Debug, Clone, Serialize, Default)]
pub struct RecallInstanceBodyV4 {
    /// 审批实例 Code
    pub instance_code: String,
}

/// 撤回审批实例响应（用户级，v4）
#[derive(Debug, Clone, Deserialize, Default)]
pub struct RecallInstanceResponseV4 {}

/// 撤回审批实例请求（用户级，v4）
#[derive(Debug, Clone)]
pub struct RecallInstanceRequestV4 {
    config: Arc<Config>,
    body: RecallInstanceBodyV4,
    user_id_type: Option<String>,
}

impl RecallInstanceRequestV4 {
    /// 创建请求实例
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: RecallInstanceBodyV4::default(),
            user_id_type: None,
        }
    }

    /// 设置审批实例 Code
    pub fn instance_code(mut self, instance_code: impl Into<String>) -> Self {
        self.body.instance_code = instance_code.into();
        self
    }

    /// 设置用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<RecallInstanceResponseV4> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<RecallInstanceResponseV4> {
        validate_required!(self.body.instance_code.trim(), "审批实例 Code 不能为空");

        let api_endpoint = crate::common::api_endpoints::ApprovalApiV4::InstanceRecall;
        let mut request = ApiRequest::<RecallInstanceResponseV4>::post(api_endpoint.to_url());

        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        let body_json = serde_json::to_value(&self.body)
            .map_err(|e| request_serialization_error("撤回审批实例（用户级）", e))?;

        request = request.body(body_json);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            missing_response_data_error(
                "撤回审批实例（用户级）",
                response.raw_response.request_id.clone(),
            )
        })
    }
}

impl ApiResponseTrait for RecallInstanceResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_instance_recall_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::InstanceRecall;
        assert_eq!(endpoint.to_url(), "/open-apis/approval/v4/instances/recall");
    }

    #[test]
    fn test_recall_instance_request_builder() {
        let config = Arc::new(
            Config::builder()
                .app_id("test_app")
                .app_secret("test_secret")
                .build(),
        );
        let request = RecallInstanceRequestV4::new(config)
            .instance_code("instance_code")
            .user_id_type("open_id");

        assert_eq!(request.body.instance_code, "instance_code");
        assert_eq!(request.user_id_type.as_deref(), Some("open_id"));
    }
}
