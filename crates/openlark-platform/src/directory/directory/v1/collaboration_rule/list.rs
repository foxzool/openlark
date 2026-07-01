//! 查询可搜可见规则
//!
//! 文档: <https://open.feishu.cn/document/trust_party-v1/searchable-and-visible-rules/list>
//! docPath: <https://open.feishu.cn/document/trust_party-v1/searchable-and-visible-rules/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 查询可搜可见规则 Builder
#[derive(Debug, Clone)]
pub struct CollaborationRuleListRequestBuilder {
    config: Config,
}

impl CollaborationRuleListRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CollaborationRuleListResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CollaborationRuleListResponse> {
        let url = "/open-apis/directory/v1/collaboration_rules".to_string();

        let req: ApiRequest<CollaborationRuleListResponse> = ApiRequest::get(&url);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 可搜可见规则信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CollaborationRuleInfo {
    /// 规则 ID
    #[serde(rename = "collaboration_rule_id")]
    collaboration_rule_id: String,
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
    /// 创建时间
    #[serde(rename = "created_at")]
    created_at: i64,
    /// 更新时间
    #[serde(rename = "updated_at")]
    updated_at: i64,
}

/// 查询可搜可见规则响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CollaborationRuleListResponse {
    /// 可搜可见规则列表
    #[serde(rename = "items")]
    pub items: Vec<CollaborationRuleInfo>,
}

impl ApiResponseTrait for CollaborationRuleListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(
    note = "renamed to CollaborationRuleListRequestBuilder, will be removed in v1.0 (#271)"
)]
pub type CollaborationRuleListBuilder = CollaborationRuleListRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let request = CollaborationRuleListRequestBuilder::new(config.clone());
        let _ = request;
    }
}
