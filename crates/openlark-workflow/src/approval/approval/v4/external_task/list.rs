//! 获取三方审批任务状态（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/external_task/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::common::api_utils::request_serialization_error;

/// 三方审批实例下的审批任务（external_task_item）
#[derive(Debug, Clone, Deserialize, Default)]
pub struct ExternalTaskItemV4 {
    /// 审批任务 ID
    #[serde(default)]
    pub id: String,
    /// 审批任务状态（PENDING/APPROVED/REJECTED/TRANSFERRED/DONE）
    #[serde(default)]
    pub status: String,
    /// 审批任务最后更新时间（毫秒）
    #[serde(default)]
    pub update_time: String,
}

/// 三方审批任务（external_task_list 元素）
#[derive(Debug, Clone, Deserialize, Default)]
pub struct ExternalTask {
    /// 审批实例 ID
    #[serde(default)]
    pub instance_id: String,
    /// 审批的 ID
    #[serde(default)]
    pub approval_id: String,
    /// 审批对应的 approval_code
    #[serde(default)]
    pub approval_code: String,
    /// 审批实例当前的状态
    #[serde(default)]
    pub status: String,
    /// 审批实例最后更新时间（毫秒）
    #[serde(default)]
    pub update_time: String,
    /// 审批实例下的审批任务
    #[serde(default)]
    pub tasks: Vec<ExternalTaskItemV4>,
}

/// 获取三方审批任务状态响应（v4）
#[derive(Debug, Clone, Deserialize, Default)]
pub struct ListExternalTaskResponseV4 {
    /// 返回数据（官方响应 `data.data`）
    #[serde(default)]
    pub data: Vec<ExternalTask>,
    /// 分页标记（has_more 为 true 时返回）
    #[serde(default)]
    pub page_token: Option<String>,
    /// 是否还有更多项
    #[serde(default)]
    pub has_more: Option<bool>,
}

/// 获取三方审批任务状态请求体（v4）
///
/// 官方支持三种过滤模式：
/// 1. `instance_ids` 获取指定实例的任务状态；
/// 2. `approval_codes` + `user_ids` + `status` 获取指定用户的任务状态；
/// 3. `approval_codes` + `status` 获取指定状态的所有任务。
#[derive(Debug, Clone, Serialize, Default)]
pub struct ListExternalTaskBodyV4 {
    /// 审批定义 Code 列表（最大 20）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_codes: Option<Vec<String>>,
    /// 审批实例 ID 列表（最多 20 个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_ids: Option<Vec<String>>,
    /// 审批人 user_id 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    /// 审批任务状态（PENDING/APPROVED/REJECTED/TRANSFERRED/DONE）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// 获取三方审批任务状态请求（v4）
#[derive(Debug, Clone)]
pub struct ListExternalTaskRequestV4 {
    config: Arc<Config>,
    body: ListExternalTaskBodyV4,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl ListExternalTaskRequestV4 {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: ListExternalTaskBodyV4::default(),
            page_size: None,
            page_token: None,
        }
    }

    /// 设置审批定义 Code 列表
    pub fn approval_codes(mut self, approval_codes: Vec<String>) -> Self {
        self.body.approval_codes = Some(approval_codes);
        self
    }

    /// 添加审批实例 ID
    pub fn add_instance_id(mut self, instance_id: impl Into<String>) -> Self {
        let ids = self.body.instance_ids.get_or_insert_with(Vec::new);
        ids.push(instance_id.into());
        self
    }

    /// 设置审批实例 ID 列表
    pub fn instance_ids(mut self, instance_ids: Vec<String>) -> Self {
        self.body.instance_ids = Some(instance_ids);
        self
    }

    /// 设置审批人 user_id 列表
    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.body.user_ids = Some(user_ids);
        self
    }

    /// 设置审批任务状态
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.body.status = Some(status.into());
        self
    }

    /// 设置分页大小（最大 500，默认 50）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记（遍历下一批数据）
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
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
        // 官方三种过滤模式至少需要 instance_ids 或 approval_codes 之一
        if self.body.instance_ids.as_ref().is_none_or(Vec::is_empty)
            && self.body.approval_codes.as_ref().is_none_or(Vec::is_empty)
        {
            return Err(openlark_core::error::CoreError::validation_msg(
                "instance_ids 与 approval_codes 至少需要传入其中一个",
            ));
        }

        let api_endpoint = crate::common::api_endpoints::ApprovalApiV4::ExternalTaskList;
        let mut request = ApiRequest::<ListExternalTaskResponseV4>::get(api_endpoint.to_url());

        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }

        let body_json = serde_json::to_value(&self.body)
            .map_err(|e| request_serialization_error("获取三方审批任务状态", e))?;

        request = request.body(body_json);

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取三方审批任务状态",
        )
        .await
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
    use super::*;

    #[test]
    fn test_external_task_list_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::ExternalTaskList;
        assert_eq!(endpoint.to_url(), "/open-apis/approval/v4/external_tasks");
    }

    #[test]
    fn test_list_external_task_request_builder() {
        let config = Arc::new(
            Config::builder()
                .app_id("test_app")
                .app_secret("test_secret")
                .build(),
        );
        let request = ListExternalTaskRequestV4::new(config)
            .approval_codes(vec!["B7B65FFE-C2GC-452F-9F0F-9AA8352363D6".to_string()])
            .user_ids(vec!["112321".to_string()])
            .status("PENDING")
            .page_size(100)
            .page_token("nF1ZXJ5VGhlbkZldGNoCg==");

        assert_eq!(
            request.body.approval_codes,
            Some(vec!["B7B65FFE-C2GC-452F-9F0F-9AA8352363D6".to_string()])
        );
        assert_eq!(request.body.user_ids, Some(vec!["112321".to_string()]));
        assert_eq!(request.body.status.as_deref(), Some("PENDING"));
        assert_eq!(request.page_size, Some(100));
    }
}
