//! 获取任务关注者列表（v1）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v1/taskfollower/list>

use crate::v1::task::models::TaskFollowerV1;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
};
use serde::Deserialize;
use std::sync::Arc;

/// 获取任务关注者列表响应（v1）
#[derive(Debug, Clone, Deserialize)]
/// 任务关注者列表响应。
pub struct ListTaskFollowerResponseV1 {
    /// 关注者列表（官方 items 元素形状为 id + id_list，复用 TaskFollowerV1）
    pub items: Vec<TaskFollowerV1>,
    /// 是否有更多数据
    pub has_more: Option<bool>,
    /// 翻页标记
    pub page_token: Option<String>,
}

/// 获取任务关注者列表请求（v1）
#[derive(Debug, Clone)]
/// 获取任务关注者列表请求构建器。
pub struct ListTaskFollowerRequestV1 {
    config: Arc<Config>,
    task_id: String,
    /// 每页数量
    page_size: Option<i32>,
    /// 翻页标记
    page_token: Option<String>,
    /// 用户 ID 类型（open_id/union_id/user_id，默认 open_id）
    user_id_type: Option<String>,
}

impl ListTaskFollowerRequestV1 {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, task_id: impl Into<String>) -> Self {
        Self {
            config,
            task_id: task_id.into(),
            page_size: None,
            page_token: None,
            user_id_type: None,
        }
    }

    /// 设置每页数量
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置翻页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置用户 ID 类型（open_id/union_id/user_id，默认 open_id）
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListTaskFollowerResponseV1> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListTaskFollowerResponseV1> {
        let api_endpoint =
            crate::common::api_endpoints::TaskApiV1::TaskFollowerList(self.task_id.clone());
        let mut request = ApiRequest::<ListTaskFollowerResponseV1>::get(api_endpoint.to_url());

        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }
        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "响应数据为空",
        )
        .await
    }
}

impl ApiResponseTrait for ListTaskFollowerResponseV1 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_list_task_follower_v1_url() {
        let endpoint =
            crate::common::api_endpoints::TaskApiV1::TaskFollowerList("task_123".to_string());
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/task/v1/tasks/task_123/followers"
        );
    }

    #[test]
    fn test_list_task_follower_request_builder() {
        let config = Arc::new(
            Config::builder()
                .app_id("test_app")
                .app_secret("test_secret")
                .build(),
        );
        let request = ListTaskFollowerRequestV1::new(config, "task_123")
            .page_size(50)
            .page_token("token_1")
            .user_id_type("open_id");

        assert_eq!(request.task_id, "task_123");
        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token.as_deref(), Some("token_1"));
        assert_eq!(request.user_id_type.as_deref(), Some("open_id"));
    }

    /// 官方响应 items 元素形状为 id + id_list（不存在 follower_id/name 键）
    #[test]
    fn test_list_task_follower_response_official_shape() {
        let response: ListTaskFollowerResponseV1 = serde_json::from_value(serde_json::json!({
            "items": [
                {
                    "id": "ou_99e1a581b36ecc4862cbfbce473f3123",
                    "id_list": ["ou_550cc75233d8b7b9fcbdad65f34433f4"]
                }
            ],
            "page_token": "next_token",
            "has_more": false
        }))
        .expect("follower list response should deserialize");

        assert_eq!(response.items.len(), 1);
        assert_eq!(
            response.items[0].id.as_deref(),
            Some("ou_99e1a581b36ecc4862cbfbce473f3123")
        );
        assert_eq!(
            response.items[0].id_list,
            Some(vec!["ou_550cc75233d8b7b9fcbdad65f34433f4".to_string()])
        );
        assert_eq!(response.has_more, Some(false));
        assert_eq!(response.page_token.as_deref(), Some("next_token"));
    }
}
