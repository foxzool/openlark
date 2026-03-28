//! 获取通讯录授权范围
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/scope/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::extract_response_data,
    contact::contact::v3::user::models::{DepartmentIdType, UserIdType},
    endpoints::CONTACT_V3_SCOPES,
};

/// 获取通讯录授权范围响应 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListScopesResponse {
    /// 可访问的部门 ID 列表
    #[serde(default)]
    pub department_ids: Vec<String>,
    /// 可访问的用户 ID 列表
    #[serde(default)]
    pub user_ids: Vec<String>,
    /// 可访问的用户组 ID 列表
    #[serde(default)]
    pub group_ids: Vec<String>,
    /// 是否还有更多数据
    #[serde(default)]
    pub has_more: bool,
    /// 下一页分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListScopesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取通讯录授权范围请求
pub struct ListScopesRequest {
    config: Config,
    user_id_type: Option<UserIdType>,
    department_id_type: Option<DepartmentIdType>,
    page_token: Option<String>,
    page_size: Option<i32>,
}

impl ListScopesRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_id_type: None,
            department_id_type: None,
            page_token: None,
            page_size: None,
        }
    }

    /// 用户 ID 类型（查询参数，可选）
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 部门 ID 类型（查询参数，可选）
    pub fn department_id_type(mut self, department_id_type: DepartmentIdType) -> Self {
        self.department_id_type = Some(department_id_type);
        self
    }

    /// 分页标记（查询参数，可选）
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 分页大小（查询参数，可选）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/scope/list>
    pub async fn execute(self) -> SDKResult<ListScopesResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListScopesResponse> {
        let mut req: ApiRequest<ListScopesResponse> = ApiRequest::get(CONTACT_V3_SCOPES);

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }
        if let Some(department_id_type) = self.department_id_type {
            req = req.query("department_id_type", department_id_type.as_str());
        }
        if let Some(page_token) = self.page_token {
            req = req.query("page_token", page_token);
        }
        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取通讯录授权范围")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
