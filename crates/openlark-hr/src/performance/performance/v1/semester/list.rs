//! 获取周期列表
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/performance-v1/semester/list>

use crate::performance::performance::v1::models::I18nName;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 获取周期列表请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 周期开始时间最小值，毫秒时间戳（可选）
    start_time: Option<String>,
    /// 周期结束时间最大值，毫秒时间戳（可选）
    end_time: Option<String>,
    /// 周期年份（可选，0～9999）
    year: Option<i32>,
    /// 周期类型分组（可选：Annual/Semi-annual/Quarter/Bimonth/Month/Non-standard）
    type_group: Option<String>,
    /// 周期类型（可选：Annual/H1/H2/Q1~Q4/January~December/Custom 等）
    r#type: Option<String>,
    /// 用户 ID 类型（可选，默认 open_id）
    user_id_type: Option<String>,
    /// 配置信息
    config: Config,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            start_time: None,
            end_time: None,
            year: None,
            type_group: None,
            r#type: None,
            user_id_type: None,
            config,
        }
    }

    /// 设置周期开始时间最小值，毫秒时间戳（可选）
    pub fn start_time(mut self, start_time: String) -> Self {
        self.start_time = Some(start_time);
        self
    }

    /// 设置周期结束时间最大值，毫秒时间戳（可选）
    pub fn end_time(mut self, end_time: String) -> Self {
        self.end_time = Some(end_time);
        self
    }

    /// 设置周期年份（可选，0～9999）
    pub fn year(mut self, year: i32) -> Self {
        self.year = Some(year);
        self
    }

    /// 设置周期类型分组（可选）
    pub fn type_group(mut self, type_group: String) -> Self {
        self.type_group = Some(type_group);
        self
    }

    /// 设置周期类型（可选）
    pub fn type_(mut self, type_: String) -> Self {
        self.r#type = Some(type_);
        self
    }

    /// 设置用户 ID 类型（可选，open_id/union_id/user_id，默认 open_id）
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListResponse> {
        use crate::common::api_endpoints::PerformanceApiV1;

        // 1. 构建端点
        let api_endpoint = PerformanceApiV1::SemesterList;
        let request = ApiRequest::<ListResponse>::get(api_endpoint.to_url())
            .query_opt("start_time", self.start_time)
            .query_opt("end_time", self.end_time)
            .query_opt("year", self.year.map(|v| v.to_string()))
            .query_opt("type_group", self.type_group)
            .query_opt("type", self.r#type)
            .query_opt("user_id_type", self.user_id_type);

        // 2. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取周期列表响应数据为空",
        )
        .await
    }
}

/// 获取周期列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 周期列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Semester>>,
    /// 是否还有更多项（官方响应体未定义该字段，兼容保留）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记（官方响应体未定义该字段，兼容保留）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 周期信息（官方 semester 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Semester {
    /// 周期 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 周期年份
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
    /// 周期类型分组
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_group: Option<String>,
    /// 周期类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// 周期名称（i18n，键为 zh-CN/en-US）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nName>,
    /// 周期状态：initiating 初始化 / enabled 已启动
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress: Option<String>,
    /// 周期开始时间，毫秒时间戳
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 周期结束时间，毫秒时间戳
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 周期创建时间，毫秒时间戳
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 周期更新时间，毫秒时间戳
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<String>,
    /// 周期创建人 ID，与入参 user_id_type 类型一致（需字段权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_user_id: Option<String>,
    /// 周期更新人 ID，与入参 user_id_type 类型一致（需字段权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modify_user_id: Option<String>,
}

impl ApiResponseTrait for ListResponse {
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
    async fn test_performance_v1_semester_list_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 使用官方响应示例的真实 shape（year/type_group/type/i18n name/progress 等）
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{
                "items": [
                    {
                        "id": "6992035450862224940",
                        "year": 2024,
                        "type_group": "Quarter",
                        "type": "Q1",
                        "name": { "zh-CN": "2024 全年绩效周期", "en-US": "2024 full year semester" },
                        "progress": "enabled",
                        "start_time": "1625068800000",
                        "end_time": "1640966399999",
                        "create_time": "1625068800000",
                        "modify_time": "1625068800000",
                        "create_user_id": "ou_ce613028fe74745421f5dc320bb9c709",
                        "modify_user_id": "ou_ce613028fe74745421f5dc320bb9c709"
                    }
                ]
            }"#,
        )
        .unwrap();
        Mock::given(method("GET"))
            .and(path("/open-apis/performance/v1/semesters"))
            .and(query_param("year", "2024"))
            .and(query_param("type_group", "Quarter"))
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

        let data = ListRequest::new(config)
            .year(2024)
            .type_group("Quarter".to_string())
            .type_("Q1".to_string())
            .start_time("1625068800000".to_string())
            .end_time("1640966399999".to_string())
            .user_id_type("open_id".to_string())
            .execute()
            .await
            .expect("performance_v1_semester_list 应成功");

        let items = data.items.expect("items 应存在");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id.as_deref(), Some("6992035450862224940"));
        assert_eq!(items[0].year, Some(2024));
        assert_eq!(items[0].type_group.as_deref(), Some("Quarter"));
        assert_eq!(items[0].r#type.as_deref(), Some("Q1"));
        assert_eq!(
            items[0].name.as_ref().and_then(|n| n.zh_cn.as_deref()),
            Some("2024 全年绩效周期")
        );
        assert_eq!(items[0].progress.as_deref(), Some("enabled"));
        assert_eq!(items[0].start_time.as_deref(), Some("1625068800000"));
        assert_eq!(items[0].end_time.as_deref(), Some("1640966399999"));
        assert_eq!(items[0].create_time.as_deref(), Some("1625068800000"));
        assert_eq!(items[0].modify_time.as_deref(), Some("1625068800000"));
        assert_eq!(
            items[0].create_user_id.as_deref(),
            Some("ou_ce613028fe74745421f5dc320bb9c709")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/performance/v1/semesters"
        );
    }
}
