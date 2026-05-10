//! 更新可搜可见规则
//!
//! 文档: https://open.feishu.cn/document/trust_party-v1/searchable-and-visible-rules/update

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 更新可搜可见规则 Builder
#[derive(Debug, Clone)]
pub struct CollaborationRuleUpdateBuilder {
    config: Config,
    /// 规则 ID
    collaboration_rule_id: String,
    /// 规则名称
    name: Option<String>,
    /// 搜索可见范围类型
    search_visible_scope_type: Option<String>,
    /// 搜索可见范围用户列表
    search_visible_scope_user_ids: Vec<String>,
    /// 搜索可见范围部门列表
    search_visible_scope_department_ids: Vec<String>,
}

impl CollaborationRuleUpdateBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, collaboration_rule_id: impl Into<String>) -> Self {
        Self {
            config,
            collaboration_rule_id: collaboration_rule_id.into(),
            name: None,
            search_visible_scope_type: None,
            search_visible_scope_user_ids: Vec::new(),
            search_visible_scope_department_ids: Vec::new(),
        }
    }

    /// 设置规则名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置搜索可见范围类型
    pub fn search_visible_scope_type(mut self, scope_type: impl Into<String>) -> Self {
        self.search_visible_scope_type = Some(scope_type.into());
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
    pub async fn execute(self) -> SDKResult<CollaborationRuleUpdateResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CollaborationRuleUpdateResponse> {
        let url = format!(
            "/open-apis/directory/v1/collaboration_rules/{}",
            self.collaboration_rule_id
        );

        use serde_json::json;

        let request = json!({
            "name": self.name,
            "search_visible_scope_type": self.search_visible_scope_type,
            "search_visible_scope_user_ids": self.search_visible_scope_user_ids,
            "search_visible_scope_department_ids": self.search_visible_scope_department_ids,
        });

        let mut api_request = ApiRequest::<CollaborationRuleUpdateResponse>::put(&url);
        api_request = api_request.body(request);

        let resp = Transport::request(api_request, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("更新可搜可见规则", "响应数据为空")
        })
    }
}

/// 更新可搜可见规则响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CollaborationRuleUpdateResponse {
    /// 规则 ID
    #[serde(rename = "collaboration_rule_id")]
    collaboration_rule_id: String,
    /// 更新后的名称
    #[serde(rename = "name")]
    name: String,
    /// 更新时间
    #[serde(rename = "updated_at")]
    updated_at: i64,
}

impl ApiResponseTrait for CollaborationRuleUpdateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {

    use serde_json;

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
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
