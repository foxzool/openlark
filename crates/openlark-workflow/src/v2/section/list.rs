//! 获取分组列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/section/list>

use crate::common::api_endpoints::TaskApiV2;
use crate::v2::section::models::ListSectionsResponse;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use std::sync::Arc;

/// 获取分组列表请求
#[derive(Debug, Clone)]
pub struct ListSectionsRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 自定义分组所属的资源类型（必填，"my_tasks" 我负责的 / "tasklist" 清单）
    resource_type: String,
    /// 清单 GUID（resource_type 为 "tasklist" 时填写）
    resource_id: Option<String>,
    /// 用户 ID 类型（open_id / union_id / user_id，默认 open_id）
    user_id_type: Option<String>,
    /// 分页大小
    page_size: Option<i32>,
    /// 分页标记
    page_token: Option<String>,
}

impl ListSectionsRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            // 官方默认资源类型为 tasklist（清单）
            resource_type: "tasklist".to_string(),
            resource_id: None,
            user_id_type: None,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置资源类型（必填，"my_tasks" 我负责的 / "tasklist" 清单）
    pub fn resource_type(mut self, resource_type: impl Into<String>) -> Self {
        self.resource_type = resource_type.into();
        self
    }

    /// 设置清单 GUID（resource_type 为 "tasklist" 时填写）
    pub fn resource_id(mut self, resource_id: impl Into<String>) -> Self {
        self.resource_id = Some(resource_id.into());
        self
    }

    /// 设置用户 ID 类型（open_id / union_id / user_id，默认 open_id）
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListSectionsResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListSectionsResponse> {
        validate_required!(self.resource_type.trim(), "资源类型不能为空");

        let api_endpoint = TaskApiV2::SectionList;
        let mut request = ApiRequest::<ListSectionsResponse>::get(api_endpoint.to_url())
            .query("resource_type", self.resource_type);

        // 构建查询参数
        if let Some(resource_id) = &self.resource_id {
            request = request.query("resource_id", resource_id);
        }
        if let Some(user_id_type) = &self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = &self.page_token {
            request = request.query("page_token", page_token);
        }

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取分组列表",
        )
        .await
    }
}

impl ApiResponseTrait for ListSectionsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use std::sync::Arc;

    use super::*;

    #[test]
    fn test_list_sections_request() {
        let config = openlark_core::config::Config::builder()
            .app_id("test")
            .app_secret("test")
            .build();

        let request = ListSectionsRequest::new(Arc::new(config))
            .resource_id("tasklist_123")
            .page_size(20);

        assert_eq!(request.resource_type, "tasklist");
        assert_eq!(request.resource_id.as_deref(), Some("tasklist_123"));
        assert_eq!(request.page_size, Some(20));
    }
}
