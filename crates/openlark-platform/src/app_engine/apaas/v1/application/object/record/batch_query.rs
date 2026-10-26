//! 查询记录列表
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/application-object-record/batch_query>
//! docPath: <https://open.feishu.cn/document/apaas-v1/application-object-record/batch_query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 查询记录列表 Builder
#[derive(Debug, Clone)]
pub struct RecordBatchQueryRequestBuilder {
    config: Config,
    /// 应用命名空间
    namespace: String,
    /// 对象 API 名称
    object_api_name: String,
    /// 请求体
    body: RecordBatchQueryBody,
}

impl RecordBatchQueryRequestBuilder {
    /// 创建新的 Builder
    pub fn new(
        config: Config,
        namespace: impl Into<String>,
        object_api_name: impl Into<String>,
    ) -> Self {
        Self {
            config,
            namespace: namespace.into(),
            object_api_name: object_api_name.into(),
            body: RecordBatchQueryBody {
                select: Vec::new(),
                filter: None,
                order_by: None,
                group_by: None,
                page_token: None,
                use_page_token: None,
                page_size: None,
                offset: None,
                need_total_count: None,
            },
        }
    }

    /// 添加需要获取的字段（必填，可多次调用）
    pub fn select(mut self, field: impl Into<String>) -> Self {
        self.body.select.push(field.into());
        self
    }

    /// 设置需要获取的字段列表（必填，一次设置多个）
    pub fn select_fields(mut self, fields: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.body.select.extend(fields.into_iter().map(Into::into));
        self
    }

    /// 设置筛选条件（官方 criterion 对象，如 json!({...})）
    pub fn filter(mut self, filter: serde_json::Value) -> Self {
        self.body.filter = Some(filter);
        self
    }

    /// 设置排序参数
    pub fn order_by(mut self, order_by: Vec<RecordSortItem>) -> Self {
        self.body.order_by = Some(order_by);
        self
    }

    /// 设置聚合参数
    pub fn group_by(mut self, group_by: Vec<RecordGroupByItem>) -> Self {
        self.body.group_by = Some(group_by);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.body.page_token = Some(page_token.into());
        self
    }

    /// 设置是否使用 page_token 功能
    pub fn use_page_token(mut self, use_page_token: bool) -> Self {
        self.body.use_page_token = Some(use_page_token);
        self
    }

    /// 设置期望返回的记录条数（上限 500）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.body.page_size = Some(page_size);
        self
    }

    /// 设置返回记录的偏移量
    pub fn offset(mut self, offset: i64) -> Self {
        self.body.offset = Some(offset);
        self
    }

    /// 设置是否返回符合条件的记录总数
    pub fn need_total_count(mut self, need_total_count: bool) -> Self {
        self.body.need_total_count = Some(need_total_count);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<RecordBatchQueryResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<RecordBatchQueryResponse> {
        validate_required!(self.body.select, "查询字段列表不能为空");

        let url = format!(
            "/open-apis/apaas/v1/applications/{}/objects/{}/records/batch_query",
            self.namespace, self.object_api_name
        );

        let req: ApiRequest<RecordBatchQueryResponse> =
            ApiRequest::post(&url).body(serde_json::to_value(&self.body)?);
        Transport::request_typed(req, &self.config, Some(option), "查询记录列表").await
    }
}

/// 查询记录列表请求体（官方字段：select 必填，其余可选）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordBatchQueryBody {
    /// 需要获取的字段列表
    pub select: Vec<String>,
    /// 筛选条件（criterion 对象）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<serde_json::Value>,
    /// 排序参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<Vec<RecordSortItem>>,
    /// 聚合参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_by: Option<Vec<RecordGroupByItem>>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否使用 page_token 功能
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_page_token: Option<bool>,
    /// 期望服务端返回的记录条数（上限 500）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 返回记录的偏移量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// 是否返回符合条件的记录总数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_total_count: Option<bool>,
}

/// 排序参数（field 为排序字段，direction 为 ASC/DESC）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordSortItem {
    /// 字段唯一标识
    pub field: String,
    /// 排序方向：ASC / DESC
    pub direction: String,
}

/// 聚合参数
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordGroupByItem {
    /// 字段唯一标识
    pub field: String,
}

/// 查询记录列表响应（官方字段：items 为 JSON 字符串）
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecordBatchQueryResponse {
    /// 符合条件的记录列表（JSON 字符串）
    pub items: String,
    /// 符合条件的记录数
    pub total: i64,
    /// 下一页的起始位置 Token（访问至末尾时不返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    /// 是否还有更多项
    pub has_more: bool,
}

impl ApiResponseTrait for RecordBatchQueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to RecordBatchQueryRequestBuilder, will be removed in v1.0 (#271)")]
pub type RecordBatchQueryBuilder = RecordBatchQueryRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../records/batch_query → 官方响应形状（items/total/next_page_token/has_more）。
    #[tokio::test]
    async fn test_batch_query_records_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/apaas/v1/applications/ns_test/objects/_user/records/batch_query",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": "[{}]",
                    "total": 0,
                    "next_page_token": "null",
                    "has_more": true
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

        let resp = RecordBatchQueryRequestBuilder::new(config, "ns_test", "_user")
            .select("_name")
            .filter(json!({
                "conditions": [
                    {
                        "index": "1",
                        "left": {
                            "type": "metadataVariable",
                            "settings": "{\"fieldPath\":[{\"fieldApiName\": \"_id\",\"objectApiName\": \"_user\"}]}"
                        },
                        "right": {
                            "type": "constant",
                            "settings": "{\"data\":\"123456\"}"
                        },
                        "operator": "equals"
                    }
                ],
                "logic_expression": "1 and 2"
            }))
            .order_by(vec![RecordSortItem {
                field: "_id".to_string(),
                direction: "ASC".to_string(),
            }])
            .page_size(100)
            .need_total_count(true)
            .execute()
            .await
            .expect("批量查询记录应成功");
        assert_eq!(resp.items, "[{}]");
        assert_eq!(resp.total, 0);
        assert_eq!(resp.next_page_token.as_deref(), Some("null"));
        assert!(resp.has_more);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/applications/ns_test/objects/_user/records/batch_query"
        );
        // 请求体须与官方文档一致：select + filter + order_by + 分页字段
        let sent: serde_json::Value = received[0].body_json().expect("请求体应为 JSON");
        assert_eq!(
            sent,
            json!({
                "select": ["_name"],
                "filter": {
                    "conditions": [
                        {
                            "index": "1",
                            "left": {
                                "type": "metadataVariable",
                                "settings": "{\"fieldPath\":[{\"fieldApiName\": \"_id\",\"objectApiName\": \"_user\"}]}"
                            },
                            "right": {
                                "type": "constant",
                                "settings": "{\"data\":\"123456\"}"
                            },
                            "operator": "equals"
                        }
                    ],
                    "logic_expression": "1 and 2"
                },
                "order_by": [ { "field": "_id", "direction": "ASC" } ],
                "page_size": 100,
                "need_total_count": true
            })
        );
    }
}
