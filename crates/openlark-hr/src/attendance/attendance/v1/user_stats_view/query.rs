//! 查询统计设置
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/user_stats_view/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 查询统计设置请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 考勤组 ID（可选）
    group_id: Option<String>,
    /// 配置信息
    config: Config,
}

impl QueryRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            group_id: None,
            config,
        }
    }

    /// 设置考勤组 ID（可选）
    pub fn group_id(mut self, group_id: String) -> Self {
        self.group_id = Some(group_id);
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
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 构建端点
        let api_endpoint = AttendanceApiV1::UserStatsViewQuery;
        let mut request = ApiRequest::<QueryResponse>::post(api_endpoint.to_url());

        // 2. 添加查询参数（可选）
        if let Some(ref group_id) = self.group_id {
            request = request.query("group_id", group_id);
        }

        // 3. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "查询统计设置响应数据为空",
        )
        .await
    }
}

/// 查询统计设置响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponse {
    /// 统计视图（官方 `data.view`）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub view: Option<UserStatsViewDetail>,
}

/// 统计视图详情（官方 `user_stats_view`，响应体形态）
///
/// 与 [`super::update::UserStatsView`]（请求体方向，仅 code+value）不同，
/// 响应侧的 child_item 额外携带 title/column_type/read_only/min_value/max_value。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserStatsViewDetail {
    /// 视图 ID，可用于更新统计设置
    pub view_id: String,
    /// 视图类型（daily: 日度统计 / month: 月度统计）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats_type: Option<String>,
    /// 操作者的用户 ID，对应 employee_type
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户设置字段
    #[serde(default)]
    pub items: Vec<ViewItem>,
}

/// 统计视图字段项（官方 `item`）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ViewItem {
    /// 标题编号
    pub code: String,
    /// 标题名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 子标题
    #[serde(default)]
    pub child_items: Vec<ViewDetailChildItem>,
}

/// 统计视图子标题项（官方 `child_item`）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ViewDetailChildItem {
    /// 子标题编号
    pub code: String,
    /// 开关字段（0: 关闭，1: 开启）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 子标题名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 列类型（0: 未知（默认），1: 复选框，2: 文本）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub column_type: Option<i32>,
    /// 是否只读
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
    /// 最小值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_value: Option<String>,
    /// 最大值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_value: Option<String>,
}

impl ApiResponseTrait for QueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use openlark_core::config::Config;
    use openlark_core::testing::prelude::TestConfigBuilder;

    #[test]
    fn test_query_request_builder_new() {
        let request =
            QueryRequest::new(TestConfigBuilder::new().build()).group_id("test".to_string());
        let _ = request;
    }
    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_attendance_v1_user_stats_view_query_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 官方响应示例形状：data.view.items[].child_items[]（含 column_type/read_only/min_value/max_value）
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{"view": {"view_id": "TmpZNU5qTTJORFF6T1RnNU5UTTNOakV6TWl0dGIyNTBhQT09", "stats_type": "daily", "user_id": "dd31248a", "items": [{"code": "522", "title": "考勤基础字段", "child_items": [{"code": "50101", "value": "1", "title": "姓名", "column_type": 0, "read_only": true, "min_value": "", "max_value": ""}]}]}}"#,
        )
        .unwrap();
        Mock::given(method("POST"))
            .and(path("/open-apis/attendance/v1/user_stats_views/query"))
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
            .execute()
            .await
            .expect("attendance_v1_user_stats_view_query 应成功");

        let view = data.view.expect("响应缺 view");
        assert_eq!(
            view.view_id,
            "TmpZNU5qTTJORFF6T1RnNU5UTTNOakV6TWl0dGIyNTBhQT09"
        );
        assert_eq!(view.stats_type.as_deref(), Some("daily"));
        assert_eq!(view.user_id.as_deref(), Some("dd31248a"));
        assert_eq!(view.items.len(), 1);
        let child = &view.items[0].child_items[0];
        assert_eq!(child.code, "50101");
        assert_eq!(child.value.as_deref(), Some("1"));
        assert_eq!(child.title.as_deref(), Some("姓名"));
        assert_eq!(child.column_type, Some(0));
        assert_eq!(child.read_only, Some(true));
        assert_eq!(child.min_value.as_deref(), Some(""));
        assert_eq!(child.max_value.as_deref(), Some(""));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/attendance/v1/user_stats_views/query"
        );
    }
}
