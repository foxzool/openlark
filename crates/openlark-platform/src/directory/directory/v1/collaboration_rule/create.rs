//! 新增可搜可见规则
//!
//! 文档: <https://open.feishu.cn/document/trust_party-v1/searchable-and-visible-rules/create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 新增可搜可见规则 Builder
#[derive(Debug, Clone)]
pub struct CollaborationRuleCreateBuilder {
    config: Config,
    /// 规则名称
    name: String,
    /// 搜索可见范围类型
    search_visible_scope_type: String,
    /// 搜索可见范围用户列表
    search_visible_scope_user_ids: Vec<String>,
    /// 搜索可见范围部门列表
    search_visible_scope_department_ids: Vec<String>,
}

impl CollaborationRuleCreateBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, name: impl Into<String>) -> Self {
        Self {
            config,
            name: name.into(),
            search_visible_scope_type: String::new(),
            search_visible_scope_user_ids: Vec::new(),
            search_visible_scope_department_ids: Vec::new(),
        }
    }

    /// 设置搜索可见范围类型
    pub fn search_visible_scope_type(mut self, scope_type: impl Into<String>) -> Self {
        self.search_visible_scope_type = scope_type.into();
        self
    }

    /// 添加搜索可见范围用户 ID
    pub fn search_visible_scope_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.search_visible_scope_user_ids.push(user_id.into());
        self
    }

    /// 添加搜索可见范围部门 ID
    pub fn search_visible_scope_department_id(mut self, department_id: impl Into<String>) -> Self {
        self.search_visible_scope_department_ids
            .push(department_id.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CollaborationRuleCreateResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CollaborationRuleCreateResponse> {
        let url = "/open-apis/directory/v1/collaboration_rules".to_string();

        let request = CollaborationRuleCreateRequest {
            name: self.name,
            search_visible_scope_type: self.search_visible_scope_type,
            search_visible_scope_user_ids: self.search_visible_scope_user_ids,
            search_visible_scope_department_ids: self.search_visible_scope_department_ids,
        };

        let req: ApiRequest<CollaborationRuleCreateResponse> =
            ApiRequest::post(&url).body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 新增可搜可见规则请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct CollaborationRuleCreateRequest {
    /// 规则名称
    #[serde(rename = "name")]
    name: String,
    /// 搜索可见范围类型
    #[serde(rename = "search_visible_scope_type")]
    search_visible_scope_type: String,
    /// 搜索可见范围用户列表
    #[serde(rename = "search_visible_scope_user_ids")]
    search_visible_scope_user_ids: Vec<String>,
    /// 搜索可见范围部门列表
    #[serde(rename = "search_visible_scope_department_ids")]
    search_visible_scope_department_ids: Vec<String>,
}

/// 新增可搜可见规则响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CollaborationRuleCreateResponse {
    /// 规则 ID
    #[serde(rename = "collaboration_rule_id")]
    collaboration_rule_id: String,
    /// 规则名称
    #[serde(rename = "name")]
    name: String,
    /// 创建时间
    #[serde(rename = "created_at")]
    created_at: i64,
}

impl ApiResponseTrait for CollaborationRuleCreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let request = CollaborationRuleCreateBuilder::new(config.clone(), "test".to_string())
            .search_visible_scope_type("test".to_string())
            .search_visible_scope_user_id("test".to_string());
        let _ = request;
    }
}
