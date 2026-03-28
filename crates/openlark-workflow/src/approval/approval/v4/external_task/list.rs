//! 获取外部审批任务列表（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/external_task/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 获取外部审批任务列表请求体（v4）
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListExternalTaskBodyV4 {
    /// 审批实例 ID 列表
    pub instance_ids: Vec<String>,
}

/// 外部审批任务
#[derive(Debug, Clone, Deserialize)]
pub struct ExternalTask {
    /// 任务 ID
    pub task_id: String,
    /// 审批实例 ID
    pub instance_id: String,
    /// 任务状态
    pub status: String,
    /// 任务类型
    pub task_type: String,
    /// 处理人用户 ID
    #[serde(default)]
    pub user_id: String,
    /// 创建时间（Unix 时间戳）
    pub create_time: i64,
}

/// 获取外部审批任务列表响应（v4）
#[derive(Debug, Clone, Deserialize)]
pub struct ListExternalTaskResponseV4 {
    /// 任务列表
    pub tasks: Vec<ExternalTask>,
}

/// 获取外部审批任务列表请求（v4）
#[derive(Debug, Clone)]
pub struct ListExternalTaskRequestV4 {
    config: Arc<Config>,
    body: ListExternalTaskBodyV4,
}

impl ListExternalTaskRequestV4 {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: ListExternalTaskBodyV4::default(),
        }
    }

    /// 添加审批实例 ID
    pub fn add_instance_id(mut self, instance_id: impl Into<String>) -> Self {
        self.body.instance_ids.push(instance_id.into());
        self
    }

    /// 设置审批实例 ID 列表
    pub fn instance_ids(mut self, instance_ids: Vec<String>) -> Self {
        self.body.instance_ids = instance_ids;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListExternalTaskResponseV4> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListExternalTaskResponseV4> {
        validate_required!(
            !self.body.instance_ids.is_empty(),
            "审批实例 ID 列表不能为空"
        );

        let api_endpoint = crate::common::api_endpoints::ApprovalApiV4::ExternalTaskList;
        let mut request = ApiRequest::<ListExternalTaskResponseV4>::post(api_endpoint.to_url());

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

impl ApiResponseTrait for ListExternalTaskResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    

    #[test]
    fn test_external_task_list_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::ExternalTaskList;
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/approval/v4/external_tasks"
        );
    }
}
