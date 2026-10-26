//! 查询数据变更日志列表
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/application-audit_log/data_change_logs_list>
//! docPath: <https://open.feishu.cn/document/apaas-v1/application-audit_log/data_change_logs_list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 查询数据变更日志列表 Builder
#[derive(Debug, Clone)]
pub struct DataChangeLogsListRequestBuilder {
    config: Config,
    /// 应用命名空间
    namespace: String,
    /// 查询参数
    query: DataChangeLogsListQuery,
}

impl DataChangeLogsListRequestBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, namespace: impl Into<String>) -> Self {
        Self {
            config,
            namespace: namespace.into(),
            query: DataChangeLogsListQuery {
                quick_query: None,
                page_size: 0,
                offset: 0,
                from: None,
                to: None,
                log_type: String::new(),
                filter: None,
                columns: None,
                sort_by: None,
                sort_order: None,
                app_type: None,
            },
        }
    }

    /// 设置模糊查询
    pub fn quick_query(mut self, quick_query: impl Into<String>) -> Self {
        self.query.quick_query = Some(quick_query.into());
        self
    }

    /// 设置分页大小（范围 0～1000）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.query.page_size = page_size;
        self
    }

    /// 设置翻页数量（范围 0～1000）
    pub fn offset(mut self, offset: i32) -> Self {
        self.query.offset = offset;
        self
    }

    /// 设置查询时间范围：开始时间（毫秒时间戳）
    pub fn from(mut self, from: i64) -> Self {
        self.query.from = Some(from);
        self
    }

    /// 设置查询时间范围：结束时间（毫秒时间戳）
    pub fn to(mut self, to: i64) -> Self {
        self.query.to = Some(to);
        self
    }

    /// 设置日志类型（10007-数据变更日志）
    pub fn log_type(mut self, log_type: impl Into<String>) -> Self {
        self.query.log_type = log_type.into();
        self
    }

    /// 设置日志查询筛选能力（JSON 字符串）
    pub fn filter(mut self, filter: impl Into<String>) -> Self {
        self.query.filter = Some(filter.into());
        self
    }

    /// 设置展示行信息（如 ["opTime","appName","eventName"]）
    pub fn columns(mut self, columns: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.query.columns = Some(columns.into_iter().map(Into::into).collect());
        self
    }

    /// 设置查询排序字段（可选项为操作时间 opTime）
    pub fn sort_by(mut self, sort_by: impl Into<String>) -> Self {
        self.query.sort_by = Some(sort_by.into());
        self
    }

    /// 设置查询排序（按时间从小到大使用 asc）
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
    pub async fn execute(self) -> SDKResult<DataChangeLogsListResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DataChangeLogsListResponse> {
        validate_required!(self.query.log_type, "日志类型不能为空");

        let url = format!(
            "/open-apis/apaas/v1/applications/{}/audit_log/data_change_logs_list",
            self.namespace
        );

        let mut req: ApiRequest<DataChangeLogsListResponse> = ApiRequest::get(&url);
        if let Some(quick_query) = self.query.quick_query {
            req = req.query("quick_query", quick_query);
        }
        req = req.query("page_size", self.query.page_size.to_string());
        req = req.query("offset", self.query.offset.to_string());
        if let Some(from) = self.query.from {
            req = req.query("from", from.to_string());
        }
        if let Some(to) = self.query.to {
            req = req.query("to", to.to_string());
        }
        req = req.query("log_type", self.query.log_type.clone());
        if let Some(filter) = self.query.filter {
            req = req.query("filter", filter);
        }
        if let Some(columns) = self.query.columns {
            req = req.query("columns", columns.join(","));
        }
        if let Some(sort_by) = self.query.sort_by {
            req = req.query("sort_by", sort_by);
        }
        if let Some(sort_order) = self.query.sort_order {
            req = req.query("sort_order", sort_order);
        }
        if let Some(app_type) = self.query.app_type {
            req = req.query("app_type", app_type);
        }
        Transport::request_typed(req, &self.config, Some(option), "查询数据变更日志列表").await
    }
}

/// 查询数据变更日志列表查询参数
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DataChangeLogsListQuery {
    /// 模糊查询
    #[serde(rename = "quick_query", skip_serializing_if = "Option::is_none")]
    pub quick_query: Option<String>,
    /// 分页大小（范围 0～1000）
    #[serde(rename = "page_size")]
    pub page_size: i32,
    /// 翻页数量（范围 0～1000）
    #[serde(rename = "offset")]
    pub offset: i32,
    /// 查询时间范围：开始时间（毫秒时间戳）
    #[serde(rename = "from", skip_serializing_if = "Option::is_none")]
    pub from: Option<i64>,
    /// 查询时间范围：结束时间（毫秒时间戳）
    #[serde(rename = "to", skip_serializing_if = "Option::is_none")]
    pub to: Option<i64>,
    /// 日志类型：10007-数据变更日志
    #[serde(rename = "log_type")]
    pub log_type: String,
    /// 日志查询：筛选能力（JSON 字符串）
    #[serde(rename = "filter", skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    /// 日志列表：选择展示行信息
    #[serde(rename = "columns", skip_serializing_if = "Option::is_none")]
    pub columns: Option<Vec<String>>,
    /// 查询排序字段（可选项为 opTime）
    #[serde(rename = "sort_by", skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    /// 查询排序（asc 为从小到大）
    #[serde(rename = "sort_order", skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<String>,
    /// 应用类型（0 为 apaas，1 为 aily）
    #[serde(rename = "app_type", skip_serializing_if = "Option::is_none")]
    pub app_type: Option<String>,
}

/// 数据变更日志信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DataChangeLogInfo {
    /// 日志 ID
    #[serde(rename = "log_id")]
    log_id: String,
    /// 变更类型
    #[serde(rename = "change_type")]
    change_type: String,
    /// 变更对象
    #[serde(rename = "object_type")]
    object_type: String,
    /// 变更对象 ID
    #[serde(rename = "object_id")]
    object_id: String,
    /// 操作人
    #[serde(rename = "operator")]
    operator: String,
    /// 变更时间
    #[serde(rename = "change_time")]
    change_time: i64,
}

/// 查询数据变更日志列表响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DataChangeLogsListResponse {
    /// 数据变更日志列表
    #[serde(rename = "items")]
    pub items: Vec<DataChangeLogInfo>,
    /// 是否有更多
    #[serde(rename = "has_more")]
    pub has_more: bool,
    /// 页码
    #[serde(rename = "page")]
    pub page: u32,
    /// 每页数量
    #[serde(rename = "page_size")]
    pub page_size: u32,
}

impl ApiResponseTrait for DataChangeLogsListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to DataChangeLogsListRequestBuilder, will be removed in v1.0 (#271)")]
pub type DataChangeLogsListBuilder = DataChangeLogsListRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../audit_log/data_change_logs_list → 强类型 DataChangeLogsListResponse。
    #[tokio::test]
    async fn test_list_data_change_logs_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/apaas/v1/applications/ns_test/audit_log/data_change_logs_list",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "log_id": "log_001",
                            "change_type": "UPDATE",
                            "object_type": "record",
                            "object_id": "obj_001",
                            "operator": "u_001",
                            "change_time": 1717000000
                        }
                    ],
                    "has_more": false,
                    "page": 1,
                    "page_size": 20
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

        let resp = DataChangeLogsListRequestBuilder::new(config, "ns_test")
            .quick_query("Intel Mac OS")
            .page_size(20)
            .offset(0)
            .from(1716000000000)
            .to(1717000000000)
            .log_type("10007")
            .columns(vec!["opTime".to_string(), "operator".to_string()])
            .sort_by("opTime")
            .sort_order("asc")
            .execute()
            .await
            .expect("查询数据变更日志列表应成功");
        assert_eq!(resp.items.len(), 1);
        assert_eq!(resp.items[0].log_id, "log_001");
        assert_eq!(resp.items[0].change_type, "UPDATE");
        assert!(!resp.has_more);
        assert_eq!(resp.page, 1);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/applications/ns_test/audit_log/data_change_logs_list"
        );
        // 查询参数须与官方文档一致：page_size/offset/log_type 必填 + 可选筛选参数
        let query = received[0].url.query().unwrap_or("");
        assert!(
            query.contains("quick_query=Intel+Mac+OS")
                || query.contains("quick_query=Intel%20Mac%20OS")
        );
        assert!(query.contains("page_size=20"));
        assert!(query.contains("offset=0"));
        assert!(query.contains("from=1716000000000"));
        assert!(query.contains("to=1717000000000"));
        assert!(query.contains("log_type=10007"));
        // columns 数组按仓库惯例以逗号拼接；form-urlencoded 编码后 ',' 为 %2C
        assert!(query.contains("columns=opTime%2Coperator"));
        assert!(query.contains("sort_by=opTime"));
        assert!(query.contains("sort_order=asc"));
    }
}
