//! 抄送审批实例（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/instance/cc>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 抄送审批实例请求体（v4）
#[derive(Debug, Clone, Serialize, Default)]
pub struct CcInstanceBodyV4 {
    /// 审批实例 Code
    pub instance_code: String,
    /// 被抄送人用户 ID 列表
    pub cc_user_ids: Vec<String>,
}

/// 抄送审批实例响应（v4）
#[derive(Debug, Clone, Deserialize)]
pub struct CcInstanceResponseV4 {
    /// 是否成功
    pub success: bool,
}

/// 抄送审批实例请求（v4）
#[derive(Debug, Clone)]
pub struct CcInstanceRequestV4 {
    config: Arc<Config>,
    body: CcInstanceBodyV4,
}

impl CcInstanceRequestV4 {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: CcInstanceBodyV4::default(),
        }
    }

    /// 设置审批实例 Code
    pub fn instance_code(mut self, instance_code: impl Into<String>) -> Self {
        self.body.instance_code = instance_code.into();
        self
    }

    /// 添加被抄送人
    pub fn add_cc_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.body.cc_user_ids.push(user_id.into());
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
        validate_required!(self.body.instance_code.trim(), "审批实例 Code 不能为空");

        let api_endpoint = crate::common::api_endpoints::ApprovalApiV4::InstanceCc;
        let mut request = ApiRequest::<CcInstanceResponseV4>::post(api_endpoint.to_url());

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

impl ApiResponseTrait for CcInstanceResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    

    #[test]
    fn test_instance_cc_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::InstanceCc;
        assert_eq!(endpoint.to_url(), "/open-apis/approval/v4/instances/cc");
    }
}
