//! 获取单个审批实例详情（用户级，v4）
//!
//! docPath: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/instance/detail

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use serde::Deserialize;
use std::sync::Arc;

use crate::common::api_utils::missing_response_data_error;

/// 审批实例任务项（用户级，v4）
#[derive(Debug, Clone, Deserialize)]
pub struct DetailInstanceTaskV4 {
    /// 任务 ID
    pub id: String,
    /// 任务处理人用户 ID
    pub user_id: String,
    /// 任务状态
    pub status: String,
    /// 节点 ID
    pub node_id: String,
    /// 节点名称
    pub node_name: String,
    /// 任务类型（AND/OR）
    #[serde(rename = "type")]
    pub task_type: String,
    /// 任务开始时间（毫秒）
    pub start_time: String,
    /// 任务结束时间（毫秒）
    pub end_time: String,
}

/// 审批实例详情响应 data（用户级，v4）
#[derive(Debug, Clone, Deserialize)]
pub struct DetailInstanceResponseV4 {
    /// 审批定义名称
    pub definition_name: String,
    /// 审批实例开始时间（毫秒）
    pub start_time: String,
    /// 审批实例结束时间（毫秒）
    pub end_time: String,
    /// 发起人用户 ID
    pub user_id: String,
    /// 审批流水号
    pub serial_number: String,
    /// 发起人部门 ID
    #[serde(default)]
    pub department_id: Option<String>,
    /// 审批实例状态
    pub status: String,
    /// 表单数据（JSON 字符串）
    #[serde(default)]
    pub form: Option<String>,
    /// 任务列表
    #[serde(default)]
    pub tasks: Vec<DetailInstanceTaskV4>,
}

/// 获取单个审批实例详情请求（用户级，v4）
#[derive(Debug, Clone)]
pub struct DetailInstanceRequestV4 {
    config: Arc<Config>,
    instance_code: String,
    user_id_type: Option<String>,
    locale: Option<String>,
}

impl DetailInstanceRequestV4 {
    /// 创建请求实例
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            instance_code: String::new(),
            user_id_type: None,
            locale: None,
        }
    }

    /// 设置审批实例 Code
    pub fn instance_code(mut self, instance_code: impl Into<String>) -> Self {
        self.instance_code = instance_code.into();
        self
    }

    /// 设置用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置语言
    pub fn locale(mut self, locale: impl Into<String>) -> Self {
        self.locale = Some(locale.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DetailInstanceResponseV4> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DetailInstanceResponseV4> {
        validate_required!(self.instance_code.trim(), "审批实例 Code 不能为空");

        let api_endpoint = crate::common::api_endpoints::ApprovalApiV4::InstanceDetail;
        let mut request = ApiRequest::<DetailInstanceResponseV4>::get(api_endpoint.to_url())
            .query("instance_code", self.instance_code);

        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }
        if let Some(locale) = self.locale {
            request = request.query("locale", locale);
        }

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            missing_response_data_error(
                "获取审批实例详情（用户级）",
                response.raw_response.request_id.clone(),
            )
        })
    }
}

impl ApiResponseTrait for DetailInstanceResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_instance_detail_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::InstanceDetail;
        assert_eq!(endpoint.to_url(), "/open-apis/approval/v4/instances/detail");
    }

    #[test]
    fn test_detail_instance_request_builder() {
        let config = Arc::new(
            Config::builder()
                .app_id("test_app")
                .app_secret("test_secret")
                .build(),
        );
        let request = DetailInstanceRequestV4::new(config)
            .instance_code("instance_code")
            .user_id_type("open_id")
            .locale("zh-CN");

        assert_eq!(request.instance_code, "instance_code");
        assert_eq!(request.user_id_type.as_deref(), Some("open_id"));
        assert_eq!(request.locale.as_deref(), Some("zh-CN"));
    }

    #[test]
    fn test_detail_instance_response_deserialize() {
        let json = serde_json::json!({
            "definition_name": "请假",
            "start_time": "1564590532967",
            "end_time": "1564590532967",
            "user_id": "ou_xxx",
            "serial_number": "202102060002",
            "department_id": "123456",
            "status": "PENDING",
            "form": "[]",
            "tasks": [{
                "id": "1234",
                "user_id": "12345",
                "status": "PENDING",
                "node_id": "node_1",
                "node_name": "START",
                "type": "AND",
                "start_time": "1564590532967",
                "end_time": "0"
            }]
        });
        let resp: DetailInstanceResponseV4 =
            serde_json::from_value(json).expect("detail response should deserialize");
        assert_eq!(resp.definition_name, "请假");
        assert_eq!(resp.status, "PENDING");
        assert_eq!(resp.tasks.len(), 1);
        assert_eq!(resp.tasks[0].task_type, "AND");
        assert_eq!(resp.tasks[0].node_name, "START");
    }
}
