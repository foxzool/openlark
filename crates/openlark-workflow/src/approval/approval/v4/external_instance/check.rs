//! 校验三方审批实例（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/external_instance/check>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_utils::request_serialization_error;

/// 校验的任务信息（external_instance_task）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExternalInstanceCheckTaskV4 {
    /// 审批实例内的审批任务 ID
    pub task_id: String,
    /// 任务最近更新时间（Unix 毫秒时间戳）
    pub update_time: String,
}

/// 校验的实例信息（exteranl_instance_check）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExternalInstanceCheckItemV4 {
    /// 审批实例 ID
    pub instance_id: String,
    /// 审批实例最近更新时间（Unix 毫秒时间戳）
    pub update_time: String,
    /// 任务信息
    pub tasks: Vec<ExternalInstanceCheckTaskV4>,
}

/// 校验三方审批实例请求体（v4）
#[derive(Debug, Clone, Serialize, Default)]
pub struct CheckExternalInstanceBodyV4 {
    /// 校验的实例信息列表
    pub instances: Vec<ExternalInstanceCheckItemV4>,
}

/// 校验三方审批实例响应（v4）
#[derive(Debug, Clone, Deserialize, Default)]
pub struct CheckExternalInstanceResponseV4 {
    /// 更新时间不一致的实例信息列表
    #[serde(default)]
    pub diff_instances: Vec<ExternalInstanceCheckItemV4>,
}

/// 校验三方审批实例请求（v4）
#[derive(Debug, Clone)]
pub struct CheckExternalInstanceRequestV4 {
    config: Arc<Config>,
    body: CheckExternalInstanceBodyV4,
}

impl CheckExternalInstanceRequestV4 {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: CheckExternalInstanceBodyV4::default(),
        }
    }

    /// 设置校验的实例信息列表
    pub fn instances(mut self, instances: Vec<ExternalInstanceCheckItemV4>) -> Self {
        self.body.instances = instances;
        self
    }

    /// 添加一条校验的实例信息
    pub fn add_instance(mut self, instance: ExternalInstanceCheckItemV4) -> Self {
        self.body.instances.push(instance);
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
        validate_required!(self.body.instances, "校验的实例信息列表不能为空");

        let api_endpoint = crate::common::api_endpoints::ApprovalApiV4::ExternalInstanceCheck;
        let mut request =
            ApiRequest::<CheckExternalInstanceResponseV4>::post(api_endpoint.to_url());

        let body_json = serde_json::to_value(&self.body)
            .map_err(|e| request_serialization_error("校验三方审批实例", e))?;

        request = request.body(body_json);

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "校验三方审批实例",
        )
        .await
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
    use super::*;

    #[test]
    fn test_external_instance_check_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::ExternalInstanceCheck;
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/approval/v4/external_instances/check"
        );
    }

    #[test]
    fn test_check_external_instance_request_builder() {
        let config = Arc::new(
            Config::builder()
                .app_id("test_app")
                .app_secret("test_secret")
                .build(),
        );
        let request =
            CheckExternalInstanceRequestV4::new(config).add_instance(ExternalInstanceCheckItemV4 {
                instance_id: "1234234234242423".to_string(),
                update_time: "1591603040000".to_string(),
                tasks: vec![ExternalInstanceCheckTaskV4 {
                    task_id: "112253".to_string(),
                    update_time: "1591603040000".to_string(),
                }],
            });

        assert_eq!(request.body.instances.len(), 1);
        assert_eq!(request.body.instances[0].instance_id, "1234234234242423");
        assert_eq!(request.body.instances[0].tasks[0].task_id, "112253");
    }
}
