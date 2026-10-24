//! 抄送审批实例（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/instance/cc>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, validate_required_list,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_utils::request_serialization_error;

/// 抄送审批实例请求体（v4）
#[derive(Debug, Clone, Serialize, Default)]
pub struct CcInstanceBodyV4 {
    /// 审批定义 Code
    pub approval_code: String,
    /// 审批实例 Code
    pub instance_code: String,
    /// 发起当前操作的用户 ID
    pub user_id: String,
    /// 被抄送人用户 ID 列表
    pub cc_user_ids: Vec<String>,
    /// 抄送留言
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
}

/// 抄送审批实例响应（v4）
///
/// 官方响应仅包含 `code` / `msg`，`data` 为空对象。
#[derive(Debug, Clone, Deserialize, Default)]
pub struct CcInstanceResponseV4 {}

/// 抄送审批实例请求（v4）
#[derive(Debug, Clone)]
pub struct CcInstanceRequestV4 {
    config: Arc<Config>,
    body: CcInstanceBodyV4,
    user_id_type: Option<String>,
}

impl CcInstanceRequestV4 {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: CcInstanceBodyV4::default(),
            user_id_type: None,
        }
    }

    /// 设置审批定义 Code
    pub fn approval_code(mut self, approval_code: impl Into<String>) -> Self {
        self.body.approval_code = approval_code.into();
        self
    }

    /// 设置审批实例 Code
    pub fn instance_code(mut self, instance_code: impl Into<String>) -> Self {
        self.body.instance_code = instance_code.into();
        self
    }

    /// 设置发起当前操作的用户 ID
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.body.user_id = user_id.into();
        self
    }

    /// 添加被抄送人用户 ID
    pub fn add_cc_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.body.cc_user_ids.push(user_id.into());
        self
    }

    /// 设置被抄送人用户 ID 列表
    pub fn cc_user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.body.cc_user_ids = user_ids;
        self
    }

    /// 设置抄送留言
    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.body.comment = Some(comment.into());
        self
    }

    /// 设置用户 ID 类型（open_id/union_id/user_id，默认 open_id）
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CcInstanceResponseV4> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CcInstanceResponseV4> {
        validate_required!(self.body.approval_code, "审批定义 Code 不能为空");
        validate_required!(self.body.instance_code, "审批实例 Code 不能为空");
        validate_required!(self.body.user_id, "发起当前操作的用户 ID 不能为空");
        validate_required_list!(
            self.body.cc_user_ids,
            1000,
            "被抄送人用户 ID 列表不能为空且不能超过 1000 个"
        );

        let api_endpoint = crate::common::api_endpoints::ApprovalApiV4::InstanceCc;
        let mut request = ApiRequest::<CcInstanceResponseV4>::post(api_endpoint.to_url());

        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        let body_json = serde_json::to_value(&self.body)
            .map_err(|e| request_serialization_error("抄送审批实例", e))?;

        request = request.body(body_json);

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "抄送审批实例",
        )
        .await
    }
}

impl ApiResponseTrait for CcInstanceResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }

    /// 官方成功响应 `data` 为空对象（或缺省），显式给出空成功值
    fn empty_success() -> Option<Self> {
        Some(Self::default())
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_instance_cc_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::InstanceCc;
        assert_eq!(endpoint.to_url(), "/open-apis/approval/v4/instances/cc");
    }

    #[test]
    fn test_cc_request_builder() {
        let config = Arc::new(
            Config::builder()
                .app_id("test_app")
                .app_secret("test_secret")
                .build(),
        );
        let request = CcInstanceRequestV4::new(config)
            .approval_code("7C468A54-8745-2245-9675-08B7C63E7A85")
            .instance_code("81D31358-93AF-92D6-7425-01A5D67C4E71")
            .user_id("f7cb567e")
            .cc_user_ids(vec!["f7cb567e".to_string()])
            .comment("ok");

        assert_eq!(
            request.body.approval_code,
            "7C468A54-8745-2245-9675-08B7C63E7A85"
        );
        assert_eq!(
            request.body.instance_code,
            "81D31358-93AF-92D6-7425-01A5D67C4E71"
        );
        assert_eq!(request.body.user_id, "f7cb567e");
        assert_eq!(request.body.cc_user_ids, vec!["f7cb567e"]);
        assert_eq!(request.body.comment.as_deref(), Some("ok"));
    }
}
