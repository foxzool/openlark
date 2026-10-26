//! 获取部门维度的用户活跃和功能使用数据 API
//! docPath: <https://open.feishu.cn/document/server-docs/admin-v1/data-report-management/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 获取部门维度用户活跃和功能使用数据的请求构建器。
pub struct ListAdminDeptStatRequestBuilder {
    /// 部门 ID 类型：department_id / open_department_id
    department_id_type: String,
    /// 起始日期（包含），格式 YYYY-mm-dd
    start_date: String,
    /// 终止日期（包含），与 start_date 相差不超过 91 天
    end_date: String,
    /// 部门的 ID，取决于 department_id_type
    department_id: String,
    /// 是否包含子部门
    contains_child_dept: bool,
    /// 每页返回数据量，取值范围 1～20，默认 10
    page_size: Option<u32>,
    /// 分页标记
    page_token: Option<String>,
    /// 跨域访问的 Geo 数据（cn、sg、jp、us）
    target_geo: Option<String>,
    /// 是否返回分产品版本数据，默认 false
    with_product_version: Option<bool>,
    config: Config,
}

impl ListAdminDeptStatRequestBuilder {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            department_id_type: String::new(),
            start_date: String::new(),
            end_date: String::new(),
            department_id: String::new(),
            contains_child_dept: false,
            page_size: None,
            page_token: None,
            target_geo: None,
            with_product_version: None,
            config,
        }
    }

    /// 设置部门 ID 类型（department_id / open_department_id）。
    pub fn department_id_type(mut self, department_id_type: impl Into<String>) -> Self {
        self.department_id_type = department_id_type.into();
        self
    }

    /// 设置统计开始日期。
    pub fn start_date(mut self, start_date: impl Into<String>) -> Self {
        self.start_date = start_date.into();
        self
    }

    /// 设置统计结束日期。
    pub fn end_date(mut self, end_date: impl Into<String>) -> Self {
        self.end_date = end_date.into();
        self
    }

    /// 设置部门的 ID（取决于 department_id_type）。
    pub fn department_id(mut self, department_id: impl Into<String>) -> Self {
        self.department_id = department_id.into();
        self
    }

    /// 设置是否包含子部门。
    pub fn contains_child_dept(mut self, contains_child_dept: bool) -> Self {
        self.contains_child_dept = contains_child_dept;
        self
    }

    /// 设置分页大小（1～20，默认 10）。
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页游标。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置跨域访问的 Geo 数据（cn、sg、jp、us）。
    pub fn target_geo(mut self, target_geo: impl Into<String>) -> Self {
        self.target_geo = Some(target_geo.into());
        self
    }

    /// 设置是否返回分产品版本数据。
    pub fn with_product_version(mut self, with_product_version: bool) -> Self {
        self.with_product_version = Some(with_product_version);
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<ListAdminDeptStatResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListAdminDeptStatResponse> {
        validate_required!(self.department_id_type, "department_id_type 不能为空");
        validate_required!(self.start_date, "start_date 不能为空");
        validate_required!(self.end_date, "end_date 不能为空");
        validate_required!(self.department_id, "department_id 不能为空");

        let req: ApiRequest<ListAdminDeptStatResponse> =
            ApiRequest::get("/open-apis/admin/v1/admin_dept_stats")
                .query("department_id_type", self.department_id_type)
                .query("start_date", self.start_date)
                .query("end_date", self.end_date)
                .query("department_id", self.department_id)
                .query("contains_child_dept", self.contains_child_dept.to_string())
                .query_opt("page_size", self.page_size.map(|size| size.to_string()))
                .query_opt("page_token", self.page_token)
                .query_opt("target_geo", self.target_geo)
                .query_opt(
                    "with_product_version",
                    self.with_product_version
                        .map(|with_product_version| with_product_version.to_string()),
                );

        Transport::request_typed(req, &self.config, Some(option), "获取部门统计数据").await
    }
}

/// 获取部门维度统计数据的响应。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListAdminDeptStatResponse {
    /// 统计条目列表。
    pub items: Vec<AdminDeptStatItem>,
    /// 下一页分页游标。
    pub page_token: Option<String>,
    /// 是否还有更多数据。
    pub has_more: bool,
}

/// 单个部门的统计信息。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AdminDeptStatItem {
    /// 部门 ID。
    pub dept_id: String,
    /// 即时消息使用次数。
    pub im_count: u32,
    /// 日历使用次数。
    pub calendar_count: u32,
    /// 文档使用次数。
    pub doc_count: u32,
    /// 视频会议使用次数。
    pub vc_count: u32,
    /// 邮件使用次数。
    pub mail_count: u32,
}

impl ApiResponseTrait for ListAdminDeptStatResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to ListAdminDeptStatRequestBuilder, will be removed in v1.0 (#271)")]
pub type ListAdminDeptStatBuilder = ListAdminDeptStatRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let config = openlark_core::config::Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();
        let request = ListAdminDeptStatRequestBuilder::new(config.clone())
            .department_id_type("open_department_id")
            .start_date("2020-02-15")
            .end_date("2020-02-15")
            .department_id("od-382e2793cfc9471f892e8a672987654c")
            .contains_child_dept(false)
            .page_size(10)
            .page_token("2");
        let _ = request;
    }

    /// 端到端：GET .../admin/v1/admin_dept_stats → 必填查询参数按官方文档发送。
    #[tokio::test]
    async fn test_list_admin_dept_stats_sends_required_query_params() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/admin/v1/admin_dept_stats"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "has_more": false,
                    "items": []
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = ListAdminDeptStatRequestBuilder::new(config)
            .department_id_type("open_department_id")
            .start_date("2020-02-15")
            .end_date("2020-02-15")
            .department_id("od-382e2793cfc9471f892e8a672987654c")
            .contains_child_dept(true)
            .page_size(20)
            .with_product_version(true)
            .execute()
            .await
            .expect("获取部门统计数据应成功");
        assert!(resp.items.is_empty());
        assert!(!resp.has_more);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let query = received[0].url.query().unwrap_or("");
        // 官方必填查询参数：department_id_type/start_date/end_date/department_id/contains_child_dept
        assert!(query.contains("department_id_type=open_department_id"));
        assert!(query.contains("start_date=2020-02-15"));
        assert!(query.contains("end_date=2020-02-15"));
        assert!(query.contains("department_id=od-382e2793cfc9471f892e8a672987654c"));
        assert!(query.contains("contains_child_dept=true"));
        assert!(query.contains("page_size=20"));
        assert!(query.contains("with_product_version=true"));
    }
}
