//! 指定回退审批实例（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/instance/specified_rollback>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 指定回退审批实例请求体（v4）
#[derive(Debug, Clone, Serialize, Default)]
pub struct SpecifiedRollbackBodyV4 {
    /// 回退节点 ID
    #[serde(rename = "node_id")]
    pub node_id: String,
    /// 回退原因
    #[serde(skip_serializing_if = "String::is_empty")]
    pub reason: String,
}

/// 指定回退审批实例响应（v4）
#[derive(Debug, Clone, Deserialize)]
pub struct SpecifiedRollbackResponseV4 {
    /// 是否成功
    pub success: bool,
}

/// 指定回退审批实例请求（v4）
#[derive(Debug, Clone)]
pub struct SpecifiedRollbackRequestV4 {
    config: Arc<Config>,
    instance_id: String,
    body: SpecifiedRollbackBodyV4,
}

impl SpecifiedRollbackRequestV4 {
    pub fn new(config: Arc<Config>, instance_id: impl Into<String>) -> Self {
        Self {
            config,
            instance_id: instance_id.into(),
            body: SpecifiedRollbackBodyV4::default(),
        }
    }

    /// 设置回退节点 ID
    pub fn node_id(mut self, node_id: impl Into<String>) -> Self {
        self.body.node_id = node_id.into();
        self
    }

    /// 设置回退原因
    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.body.reason = reason.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<SpecifiedRollbackResponseV4> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<SpecifiedRollbackResponseV4> {
        validate_required!(self.instance_id.trim(), "审批实例 ID 不能为空");
        validate_required!(self.body.node_id.trim(), "回退节点 ID 不能为空");

        let api_endpoint = crate::common::api_endpoints::ApprovalApiV4::InstanceSpecifiedRollback(
            self.instance_id,
        );
        let mut request =
            ApiRequest::<SpecifiedRollbackResponseV4>::post(api_endpoint.to_url());

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

impl ApiResponseTrait for SpecifiedRollbackResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    

    #[test]
    fn test_instance_specified_rollback_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::InstanceSpecifiedRollback(
            "test_instance_id".to_string(),
        );
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/approval/v4/instances/test_instance_id/specified_rollback"
        );
    }
}
