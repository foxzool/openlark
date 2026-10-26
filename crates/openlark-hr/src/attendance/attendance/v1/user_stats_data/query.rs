//! 查询统计数据
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/user_stats_data/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};

use super::models::{QueryRequestBody, QueryResponse};

/// 查询统计数据请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 员工 ID 类型（必填，query 参数，可选值：employee_id、employee_no）
    employee_type: String,
    /// 语言类型（必填，可选值：en、ja、zh）
    locale: String,
    /// 统计类型（必填，可选值：daily-日度统计、month-月度统计）
    stats_type: String,
    /// 开始时间（必填，格式 `yyyyMMdd`）
    start_date: i32,
    /// 结束时间（必填，格式 `yyyyMMdd`，与开始时间间隔不超过 31 天）
    end_date: i32,
    /// 查询的用户 ID 列表（可选，不超过 200 个，对应 employee_type）
    user_ids: Option<Vec<String>>,
    /// 是否包含离职人员和转出人员（可选，默认 false）
    need_history: Option<bool>,
    /// 是否只展示员工当前所属考勤组数据（可选，默认 false）
    current_group_only: Option<bool>,
    /// 操作者的用户 ID（可选，对应 employee_type；新系统要求必填）
    user_id: Option<String>,
    /// 配置信息
    config: Config,
}

impl QueryRequest {
    /// 创建查询统计数据请求
    pub fn new(config: Config) -> Self {
        Self {
            employee_type: String::new(),
            locale: String::new(),
            stats_type: String::new(),
            start_date: 0,
            end_date: 0,
            user_ids: None,
            need_history: None,
            current_group_only: None,
            user_id: None,
            config,
        }
    }

    /// 设置员工 ID 类型（必填，query 参数）
    pub fn employee_type(mut self, employee_type: String) -> Self {
        self.employee_type = employee_type;
        self
    }

    /// 设置语言类型（必填）
    pub fn locale(mut self, locale: String) -> Self {
        self.locale = locale;
        self
    }

    /// 设置统计类型（必填）
    pub fn stats_type(mut self, stats_type: String) -> Self {
        self.stats_type = stats_type;
        self
    }

    /// 设置开始时间（必填，`yyyyMMdd`）
    pub fn start_date(mut self, start_date: i32) -> Self {
        self.start_date = start_date;
        self
    }

    /// 设置结束时间（必填，`yyyyMMdd`）
    pub fn end_date(mut self, end_date: i32) -> Self {
        self.end_date = end_date;
        self
    }

    /// 设置查询的用户 ID 列表（可选，不超过 200 个）
    pub fn user_ids(mut self, user_ids: Vec<String>) -> Self {
        self.user_ids = Some(user_ids);
        self
    }

    /// 设置是否包含离职人员和转出人员（可选）
    pub fn need_history(mut self, need_history: bool) -> Self {
        self.need_history = Some(need_history);
        self
    }

    /// 设置是否只展示员工当前所属考勤组数据（可选）
    pub fn current_group_only(mut self, current_group_only: bool) -> Self {
        self.current_group_only = Some(current_group_only);
        self
    }

    /// 设置操作者的用户 ID（可选）
    pub fn user_id(mut self, user_id: String) -> Self {
        self.user_id = Some(user_id);
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

        // 1. 验证必填字段
        validate_required!(self.employee_type, "employee_type 不能为空");
        validate_required!(self.locale, "locale 不能为空");
        validate_required!(self.stats_type, "stats_type 不能为空");
        if self.start_date <= 0 || self.end_date <= 0 {
            return Err(openlark_core::error::validation_error(
                "查询日期无效",
                "start_date / end_date 必须为 yyyyMMdd 格式的非零日期",
            ));
        }
        if self.end_date < self.start_date {
            return Err(openlark_core::error::validation_error(
                "日期范围无效",
                "end_date 不能早于 start_date",
            ));
        }

        // 验证用户 ID 数量（不超过 200 个）
        if let Some(ref user_ids) = self.user_ids
            && user_ids.len() > 200
        {
            return Err(openlark_core::error::validation_error(
                "查询用户 ID 列表超出限制",
                "最多支持 200 个用户",
            ));
        }

        // 2. 构建端点并附加必填 query 参数 employee_type
        let api_endpoint = AttendanceApiV1::UserStatsDataQuery;
        let mut request = ApiRequest::<QueryResponse>::post(api_endpoint.to_url())
            .query("employee_type", &self.employee_type);

        // 3. 序列化请求体
        let request_body = QueryRequestBody {
            locale: self.locale,
            stats_type: self.stats_type,
            start_date: self.start_date,
            end_date: self.end_date,
            user_ids: self.user_ids,
            need_history: self.need_history,
            current_group_only: self.current_group_only,
            user_id: self.user_id,
        };
        request = request.body(serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?);

        // 4. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "查询统计数据响应数据为空",
        )
        .await
    }
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
        let request = QueryRequest::new(TestConfigBuilder::new().build())
            .employee_type("employee_id".to_string())
            .locale("zh".to_string())
            .start_date(20_210_316);
        let _ = request;
    }

    #[test]
    fn test_query_request_validation_fails_on_default_request() {
        let request = QueryRequest::new(TestConfigBuilder::new().build());
        let rt = tokio::runtime::Runtime::new().expect("创建 tokio runtime 失败");
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
    }

    #[test]
    fn test_query_request_validation_requires_locale() {
        let request = QueryRequest::new(TestConfigBuilder::new().build())
            .employee_type("employee_id".to_string())
            .locale("   ".to_string())
            .stats_type("month".to_string())
            .start_date(20_210_316)
            .end_date(20_210_323);
        let rt = tokio::runtime::Runtime::new().expect("创建 tokio runtime 失败");
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
        let err = result.err().unwrap().to_string();
        assert!(err.contains("locale 不能为空"), "unexpected err: {err}");
    }

    #[test]
    fn test_query_request_validation_rejects_too_many_user_ids() {
        let user_ids: Vec<String> = (0..201).map(|i| format!("user_{i}")).collect();
        let request = QueryRequest::new(TestConfigBuilder::new().build())
            .employee_type("employee_id".to_string())
            .locale("zh".to_string())
            .stats_type("month".to_string())
            .start_date(20_210_316)
            .end_date(20_210_323)
            .user_ids(user_ids);
        let rt = tokio::runtime::Runtime::new().expect("创建 tokio runtime 失败");
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
        let err = result.err().unwrap().to_string();
        assert!(err.contains("最多支持 200 个用户"), "unexpected err: {err}");
    }
    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_attendance_v1_user_stats_data_query_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value =
            serde_json::from_str(r#"{"items": [], "has_more": false}"#).unwrap();
        Mock::given(method("POST"))
            .and(path("/open-apis/attendance/v1/user_stats_datas/query"))
            .and(query_param("employee_type", "employee_id"))
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
            .employee_type("employee_id".to_string())
            .locale("zh".to_string())
            .stats_type("month".to_string())
            .start_date(20_210_316)
            .end_date(20_210_323)
            .user_ids(vec!["user_001".to_string()])
            .user_id("user_001".to_string())
            .execute()
            .await
            .expect("attendance_v1_user_stats_data_query 应成功");

        let _ = &data;

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/attendance/v1/user_stats_datas/query"
        );
        let body = String::from_utf8(received[0].body.clone()).unwrap();
        assert!(body.contains("\"locale\""), "请求体缺 locale: {body}");
        assert!(
            body.contains("\"stats_type\""),
            "请求体缺 stats_type: {body}"
        );
        assert!(
            !body.contains("\"group_ids\""),
            "请求体不应含旧字段 group_ids: {body}"
        );
        assert!(
            !body.contains("\"page_size\""),
            "请求体不应含旧字段 page_size: {body}"
        );
    }
}
