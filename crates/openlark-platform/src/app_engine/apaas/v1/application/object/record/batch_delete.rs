//! 批量删除记录
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/application-object-record/batch_delete>
//! docPath: <https://open.feishu.cn/document/apaas-v1/application-object-record/batch_delete>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required_list,
};
use serde::{Deserialize, Serialize};

/// 批量删除记录 Builder
#[derive(Debug, Clone)]
pub struct RecordBatchDeleteRequestBuilder {
    config: Config,
    /// 应用命名空间
    namespace: String,
    /// 对象 API 名称
    object_api_name: String,
    /// 请求体
    body: RecordBatchDeleteBody,
}

impl RecordBatchDeleteRequestBuilder {
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
            body: RecordBatchDeleteBody { ids: Vec::new() },
        }
    }

    /// 添加记录 ID
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.body.ids.push(id.into());
        self
    }

    /// 设置记录 ID 列表（最多 500 个）
    pub fn ids(mut self, ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.body.ids.extend(ids.into_iter().map(Into::into));
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<RecordBatchDeleteResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<RecordBatchDeleteResponse> {
        validate_required_list!(self.body.ids, 500, "记录ID列表不能为空且不能超过 500 个");

        let url = format!(
            "/open-apis/apaas/v1/applications/{}/objects/{}/records/batch_delete",
            self.namespace, self.object_api_name
        );

        let req: ApiRequest<RecordBatchDeleteResponse> =
            ApiRequest::delete(&url).body(serde_json::to_value(&self.body)?);
        Transport::request_typed(req, &self.config, Some(option), "批量删除记录").await
    }
}

/// 批量删除记录请求体（官方字段：ids 必填，上限 500）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordBatchDeleteBody {
    /// 记录 ID 列表
    pub ids: Vec<String>,
}

/// 批量删除记录响应（官方字段：items 为处理结果列表）
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecordBatchDeleteResponse {
    /// 处理结果
    pub items: Vec<RecordDeleteResult>,
}

/// 记录删除结果
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecordDeleteResult {
    /// 是否成功
    pub success: bool,
    /// 记录 ID
    pub id: String,
    /// 权限错误时的细分错误
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<RecordDeleteError>>,
}

/// 记录删除错误信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecordDeleteError {
    /// 错误码
    pub code: String,
    /// 错误消息
    pub message: String,
    /// 权限错误时的细分 code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_code: Option<String>,
    /// 权限错误时的涉及的字段 APIID 集合
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<String>>,
}

impl ApiResponseTrait for RecordBatchDeleteResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to RecordBatchDeleteRequestBuilder, will be removed in v1.0 (#271)")]
pub type RecordBatchDeleteBuilder = RecordBatchDeleteRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：DELETE .../records/batch_delete → 官方响应形状（items 处理结果）。
    #[tokio::test]
    async fn test_batch_delete_records_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("DELETE"))
            .and(path(
                "/open-apis/apaas/v1/applications/ns_test/objects/user/records/batch_delete",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "success": true,
                            "id": "1801464965461024",
                            "errors": [
                                {
                                    "code": "0",
                                    "message": "success",
                                    "sub_code": "k_ec_00001",
                                    "fields": ["_id"]
                                }
                            ]
                        }
                    ]
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

        let resp = RecordBatchDeleteRequestBuilder::new(config, "ns_test", "user")
            .ids(vec!["1801464965461024"])
            .execute()
            .await
            .expect("批量删除记录应成功");
        assert_eq!(resp.items.len(), 1);
        assert_eq!(resp.items[0].id, "1801464965461024");
        assert!(resp.items[0].success);
        assert_eq!(resp.items[0].errors.as_deref().map(|e| e.len()), Some(1));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/applications/ns_test/objects/user/records/batch_delete"
        );
        // 请求体须与官方文档一致：ids
        let sent: serde_json::Value = received[0].body_json().expect("请求体应为 JSON");
        assert_eq!(sent, json!({ "ids": ["1801464965461024"] }));
    }
}
