//! 查询审计日志列表
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/application-audit_log/audit_log_list>
//! docPath: <https://open.feishu.cn/document/apaas-v1/application-audit_log/audit_log_list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 查询审计日志列表 Builder
#[derive(Debug, Clone)]
pub struct AuditLogListRequestBuilder {
    config: Config,
    /// 应用命名空间
    namespace: String,
    /// 查询参数（官方均为 string 类型）
    query: AuditLogListQuery,
}

impl AuditLogListRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, namespace: impl Into<String>) -> Self {
        Self {
            config,
            namespace: namespace.into(),
            query: AuditLogListQuery {
                page_size: String::new(),
                offset: String::new(),
                from: String::new(),
                to: String::new(),
                log_type: String::new(),
                quick_query: None,
                filter: None,
                columns: None,
                sort_by: None,
                sort_order: None,
                app_type: None,
            },
        }
    }

    /// 设置分页大小（必填，如 "10"）
    pub fn page_size(mut self, page_size: impl Into<String>) -> Self {
        self.query.page_size = page_size.into();
        self
    }

    /// 设置翻页数量（必填，如 "0"）
    pub fn offset(mut self, offset: impl Into<String>) -> Self {
        self.query.offset = offset.into();
        self
    }

    /// 设置查询时间范围开始时间（必填，毫秒级时间戳字符串）
    pub fn from(mut self, from: impl Into<String>) -> Self {
        self.query.from = from.into();
        self
    }

    /// 设置查询时间范围结束时间（必填，毫秒级时间戳字符串）
    pub fn to(mut self, to: impl Into<String>) -> Self {
        self.query.to = to.into();
        self
    }

    /// 设置日志类型（必填：10000 全部 / 10001 企业管理 / 10002 登录 / 10003 应用管理）
    pub fn log_type(mut self, log_type: impl Into<String>) -> Self {
        self.query.log_type = log_type.into();
        self
    }

    /// 设置模糊查询
    pub fn quick_query(mut self, quick_query: impl Into<String>) -> Self {
        self.query.quick_query = Some(quick_query.into());
        self
    }

    /// 设置日志筛选条件（JSON 字符串）
    pub fn filter(mut self, filter: impl Into<String>) -> Self {
        self.query.filter = Some(filter.into());
        self
    }

    /// 设置选择展示的行信息
    pub fn columns(mut self, columns: Vec<String>) -> Self {
        self.query.columns = Some(columns);
        self
    }

    /// 设置查询排序字段（可选项为 opTime）
    pub fn sort_by(mut self, sort_by: impl Into<String>) -> Self {
        self.query.sort_by = Some(sort_by.into());
        self
    }

    /// 设置查询排序（默认从大到小；从小到大使用 asc）
    pub fn sort_order(mut self, sort_order: impl Into<String>) -> Self {
        self.query.sort_order = Some(sort_order.into());
        self
    }

    /// 设置应用类型（0 为 apaas 类型，1 为 aily 类型）
    pub fn app_type(mut self, app_type: impl Into<String>) -> Self {
        self.query.app_type = Some(app_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<AuditLogListResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<AuditLogListResponse> {
        validate_required!(self.query.page_size, "分页大小不能为空");
        validate_required!(self.query.offset, "翻页数量不能为空");
        validate_required!(self.query.from, "开始时间不能为空");
        validate_required!(self.query.to, "结束时间不能为空");
        validate_required!(self.query.log_type, "日志类型不能为空");

        let url = format!(
            "/open-apis/apaas/v1/applications/{}/audit_log/audit_log_list",
            self.namespace
        );

        let q = self.query;
        let req: ApiRequest<AuditLogListResponse> = ApiRequest::get(&url)
            .query("page_size", q.page_size)
            .query("offset", q.offset)
            .query("from", q.from)
            .query("to", q.to)
            .query("log_type", q.log_type)
            .query_opt("quick_query", q.quick_query)
            .query_opt("filter", q.filter)
            .query_opt("columns", q.columns.map(|columns| columns.join(",")))
            .query_opt("sort_by", q.sort_by)
            .query_opt("sort_order", q.sort_order)
            .query_opt("app_type", q.app_type);
        Transport::request_typed(req, &self.config, Some(option), "查询审计日志列表").await
    }
}

/// 查询审计日志列表查询参数（官方均为 string；page_size/offset/from/to/log_type 必填）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditLogListQuery {
    /// 分页大小
    pub page_size: String,
    /// 翻页数量
    pub offset: String,
    /// 查询时间范围：开始时间（毫秒级时间戳）
    pub from: String,
    /// 查询时间范围：结束时间（毫秒级时间戳）
    pub to: String,
    /// 日志类型：10000 全部 / 10001 企业管理 / 10002 登录 / 10003 应用管理
    pub log_type: String,
    /// 模糊查询
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quick_query: Option<String>,
    /// 日志查询：筛选能力（JSON 字符串）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    /// 日志列表：选择展示行信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<String>>,
    /// 查询排序字段（可选项为 opTime）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// 查询排序（默认按时间从大到小；从小到大使用 asc）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// 应用类型（0 为 apaas 类型，1 为 aily 类型）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_type: Option<String>,
}

/// 查询审计日志列表响应（items 字段随日志类型差异较大，保留原始 JSON）
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuditLogListResponse {
    /// 审计日志查询结果列表详情信息
    pub items: Vec<serde_json::Value>,
    /// 审计日志查询总条数
    pub total: String,
}

impl ApiResponseTrait for AuditLogListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to AuditLogListRequestBuilder, will be removed in v1.0 (#271)")]
pub type AuditLogListBuilder = AuditLogListRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../audit_log/audit_log_list → 官方查询参数 + 响应形状（items + total）。
    #[tokio::test]
    async fn test_list_audit_log_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/apaas/v1/applications/ns_test/audit_log/audit_log_list",
            ))
            .and(query_param("page_size", "10"))
            .and(query_param("offset", "0"))
            .and(query_param("from", "1723691857002"))
            .and(query_param("to", "1724296657002"))
            .and(query_param("log_type", "10000"))
            .and(query_param("sort_order", "asc"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "log_id": "7376574450886557740",
                            "op_time": "1723634726874",
                            "log_type": "10000",
                            "operator": {
                                "id": "1768491480010814",
                                "name": "郭xx",
                                "tenant_id": "79888",
                                "email": "gxxx@bytedance.com"
                            },
                            "status": "18001"
                        }
                    ],
                    "total": "10"
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

        let resp = AuditLogListRequestBuilder::new(config, "ns_test")
            .page_size("10")
            .offset("0")
            .from("1723691857002")
            .to("1724296657002")
            .log_type("10000")
            .sort_order("asc")
            .execute()
            .await
            .expect("查询审计日志列表应成功");
        assert_eq!(resp.items.len(), 1);
        assert_eq!(resp.items[0]["log_id"], "7376574450886557740");
        assert_eq!(resp.total, "10");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/applications/ns_test/audit_log/audit_log_list"
        );
        let query = received[0].url.query().unwrap_or("");
        assert!(query.contains("page_size=10"));
        assert!(query.contains("offset=0"));
        assert!(query.contains("from=1723691857002"));
        assert!(query.contains("to=1724296657002"));
        assert!(query.contains("log_type=10000"));
        assert!(query.contains("sort_order=asc"));
    }
}
