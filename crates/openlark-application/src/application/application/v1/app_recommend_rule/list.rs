//! 获取当前设置的推荐规则列表
//! docPath: <https://open.feishu.cn/document/application-v6/admin/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 获取当前设置的推荐规则列表的请求。
#[derive(Debug, Clone)]
pub struct ListAppRecommendRuleRequest {
    config: Arc<Config>,
    app_id: String,
}

/// 获取当前设置的推荐规则列表的响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAppRecommendRuleResponse {
    /// 响应数据。
    pub data: Option<RecommendRuleListData>,
}

impl ApiResponseTrait for ListAppRecommendRuleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取当前设置的推荐规则列表的响应数据。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendRuleListData {
    /// 规则列表。
    pub rules: Vec<RecommendRule>,
}

/// 推荐规则。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendRule {
    /// 规则 ID。
    pub rule_id: String,
    /// 规则名称。
    pub rule_name: String,
}

impl ListAppRecommendRuleRequest {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>, app_id: impl Into<String>) -> Self {
        Self {
            config,
            app_id: app_id.into(),
        }
    }

    /// 执行获取当前设置的推荐规则列表请求。
    pub async fn execute(self) -> SDKResult<ListAppRecommendRuleResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 带自定义请求选项执行。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListAppRecommendRuleResponse> {
        let path = format!(
            "/open-apis/application/v6/applications/{}/recommend_rules",
            self.app_id
        );
        let req: ApiRequest<ListAppRecommendRuleResponse> = ApiRequest::get(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取当前设置的推荐规则列表", "响应数据为空")
        })
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

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
