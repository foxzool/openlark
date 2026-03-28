//! 检查外部审批实例状态（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/external_instance/check>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 检查外部审批实例状态请求体（v4）
#[derive(Debug, Clone, Serialize, Default)]
pub struct CheckExternalInstanceBodyV4 {
    /// 审批实例 ID
    pub instance_id: String,
}

/// 检查外部审批实例状态响应（v4）
#[derive(Debug, Clone, Deserialize)]
pub struct CheckExternalInstanceResponseV4 {
    /// 审批实例 ID
    pub instance_id: String,
    /// 审批实例状态
    pub status: String,
    /// 当前审批节点
    #[serde(default)]
    pub current_node: String,
    /// 审批结果
    #[serde(default)]
    pub result: String,
}

/// 检查外部审批实例状态请求（v4）
#[derive(Debug, Clone)]
pub struct CheckExternalInstanceRequestV4 {
    config: Arc<Config>,
    body: CheckExternalInstanceBodyV4,
}

impl CheckExternalInstanceRequestV4 {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: CheckExternalInstanceBodyV4::default(),
        }
    }

    /// 设置审批实例 ID
    pub fn instance_id(mut self, instance_id: impl Into<String>) -> Self {
        self.body.instance_id = instance_id.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CheckExternalInstanceResponseV4> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CheckExternalInstanceResponseV4> {
        validate_required!(
            self.body.instance_id.trim(),
            "审批实例 ID 不能为空"
        );

        let api_endpoint = crate::common::api_endpoints::ApprovalApiV4::ExternalInstanceCheck;
        let mut request =
            ApiRequest::<CheckExternalInstanceResponseV4>::post(api_endpoint.to_url());

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

impl ApiResponseTrait for CheckExternalInstanceResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    

    #[test]
    fn test_external_instance_check_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::ExternalInstanceCheck;
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/approval/v4/external_instances/check"
        );
    }
}
