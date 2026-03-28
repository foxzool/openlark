//! 审批实例加签（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/instance/add_sign>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 审批实例加签请求体（v4）
#[derive(Debug, Clone, Serialize, Default)]
pub struct AddSignBodyV4 {
    /// 加签类型
    #[serde(rename = "sign_type")]
    pub sign_type: String,
    /// 加签节点名称
    #[serde(skip_serializing_if = "String::is_empty")]
    pub node_name: String,
    /// 加签处理人用户 ID 列表
    pub user_ids: Vec<String>,
    /// 是否允许转交
    #[serde(default)]
    pub allow_transfer: bool,
}

/// 审批实例加签响应（v4）
#[derive(Debug, Clone, Deserialize)]
pub struct AddSignResponseV4 {
    /// 加签任务 ID
    pub task_id: String,
}

/// 审批实例加签请求（v4）
#[derive(Debug, Clone)]
pub struct AddSignRequestV4 {
    config: Arc<Config>,
    body: AddSignBodyV4,
}

impl AddSignRequestV4 {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: AddSignBodyV4::default(),
        }
    }

    /// 设置加签类型
    pub fn sign_type(mut self, sign_type: impl Into<String>) -> Self {
        self.body.sign_type = sign_type.into();
        self
    }

    /// 设置加签节点名称
    pub fn node_name(mut self, node_name: impl Into<String>) -> Self {
        self.body.node_name = node_name.into();
        self
    }

    /// 添加加签处理人用户 ID
    pub fn add_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.body.user_ids.push(user_id.into());
        self
    }

    /// 设置处理人用户 ID 列表
    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.body.user_ids = user_ids;
        self
    }

    /// 设置是否允许转交
    pub fn allow_transfer(mut self, allow_transfer: bool) -> Self {
        self.body.allow_transfer = allow_transfer;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<AddSignResponseV4> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<AddSignResponseV4> {
        validate_required!(self.body.sign_type.trim(), "加签类型不能为空");
        validate_required!(
            !self.body.user_ids.is_empty(),
            "加签处理人用户 ID 列表不能为空"
        );

        let api_endpoint = crate::common::api_endpoints::ApprovalApiV4::InstanceAddSign;
        let mut request = ApiRequest::<AddSignResponseV4>::post(api_endpoint.to_url());

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

impl ApiResponseTrait for AddSignResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    

    #[test]
    fn test_instance_add_sign_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::InstanceAddSign;
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/approval/v4/instances/add_sign"
        );
    }
}
