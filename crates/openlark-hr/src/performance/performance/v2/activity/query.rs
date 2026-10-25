//! 获取项目列表
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/performance-v2/activity/query>

use crate::common::shared_models::I18nText;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 获取项目列表请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 评估周期 ID 列表（可选，≤10；填写 activity_ids 时无效）
    semester_ids: Option<Vec<String>>,
    /// 项目 ID 列表（可选，≤50）
    activity_ids: Option<Vec<String>>,
    /// 用户 ID 类型（可选 query 参数，默认 open_id）
    user_id_type: Option<String>,
    /// 配置信息
    config: Config,
}

impl QueryRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            semester_ids: None,
            activity_ids: None,
            user_id_type: None,
            config,
        }
    }

    /// 设置评估周期 ID 列表（可选，≤10）
    pub fn semester_ids(mut self, semester_ids: Vec<String>) -> Self {
        self.semester_ids = Some(semester_ids);
        self
    }

    /// 设置项目 ID 列表（可选，≤50）
    pub fn activity_ids(mut self, activity_ids: Vec<String>) -> Self {
        self.activity_ids = Some(activity_ids);
        self
    }

    /// 设置用户 ID 类型（可选 query 参数，默认 open_id）
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<QueryResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryResponse> {
        use crate::common::api_endpoints::PerformanceApiV1;

        // 1. 构建端点
        let api_endpoint = PerformanceApiV1::ActivityQuery;
        let request = ApiRequest::<QueryResponse>::post(api_endpoint.to_url())
            .query_opt("user_id_type", self.user_id_type);

        // 2. 构建请求体（semester_ids 与 activity_ids 均可不填，都不填时返回空数据）
        let request_body = QueryRequestBody {
            semester_ids: self.semester_ids,
            activity_ids: self.activity_ids,
        };
        let request_body_json = serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?;
        let request = request.body(request_body_json);

        // 3. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取项目列表响应数据为空",
        )
        .await
    }
}

/// 请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequestBody {
    /// 评估周期 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semester_ids: Option<Vec<String>>,
    /// 项目 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activity_ids: Option<Vec<String>>,
}

/// 获取项目列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct QueryResponse {
    /// 项目列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activities: Option<Vec<Activity>>,
}

/// 项目信息（官方 activity 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Activity {
    /// 项目 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 项目名称（i18n）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    /// 项目描述（i18n）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<I18nText>,
    /// 周期 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semester_id: Option<String>,
    /// 项目模式：metric_development / performance_review / metric_development_and_performance_review
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// 项目状态：configurable / unable / initiating / enabled / finished
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
    /// 项目创建时间，毫秒时间戳
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 项目更新时间，毫秒时间戳
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<String>,
    /// 项目创建人 ID，与入参 user_id_type 类型一致（需字段权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_user_id: Option<String>,
    /// 项目更新人 ID，与入参 user_id_type 类型一致（需字段权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modify_user_id: Option<String>,
}

impl ApiResponseTrait for QueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;

    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_performance_v2_activity_query_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{body_json, method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 使用官方响应示例的真实 shape（activities 容器 + i18n name/description）
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{
                "activities": [
                    {
                        "id": "7343513161666707459",
                        "name": { "zh_cn": "项目一", "en_us": "Activity 1" },
                        "description": { "zh_cn": "体验", "en_us": "Interactive experience" },
                        "semester_id": "7343513161666707459",
                        "mode": "performance_review",
                        "progress": "configurable",
                        "create_time": "1691951256000",
                        "modify_time": "1691951256000",
                        "create_user_id": "6924187793321444877",
                        "modify_user_id": "6924187793321444877"
                    }
                ]
            }"#,
        )
        .unwrap();
        Mock::given(method("POST"))
            .and(path("/open-apis/performance/v2/activity/query"))
            .and(body_json(json!({
                "semester_ids": ["6992035450862224940"],
                "activity_ids": ["7343513161666707459"]
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": data_body
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = QueryRequest::new(config)
            .semester_ids(vec!["6992035450862224940".to_string()])
            .activity_ids(vec!["7343513161666707459".to_string()])
            .user_id_type("open_id".to_string())
            .execute()
            .await
            .expect("performance_v2_activity_query 应成功");

        let activities = data.activities.as_ref().expect("activities 应存在");
        assert_eq!(activities.len(), 1);
        assert_eq!(activities[0].id.as_deref(), Some("7343513161666707459"));
        assert_eq!(
            activities[0].name.as_ref().and_then(|n| n.zh_cn.as_deref()),
            Some("项目一")
        );
        assert_eq!(
            activities[0]
                .description
                .as_ref()
                .and_then(|d| d.en_us.as_deref()),
            Some("Interactive experience")
        );
        assert_eq!(activities[0].mode.as_deref(), Some("performance_review"));
        assert_eq!(activities[0].progress.as_deref(), Some("configurable"));
        assert_eq!(activities[0].create_time.as_deref(), Some("1691951256000"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/performance/v2/activity/query"
        );
    }
}
