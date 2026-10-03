//! Bitable 批量删除数据表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table/batch_delete>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::BitableApiV1;

/// 批量删除数据表请求。
#[derive(Debug, Clone)]
pub struct BatchDeleteTableRequest {
    config: Config,
    app_token: String,
    user_id_type: Option<String>,
    client_token: Option<String>,
    table_ids: Vec<String>,
}

impl BatchDeleteTableRequest {
    /// 创建新的批量删除数据表请求。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            user_id_type: None,
            client_token: None,
            table_ids: vec![],
        }
    }

    /// 设置多维表格 token。
    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置客户端幂等标识。
    pub fn client_token(mut self, client_token: impl Into<String>) -> Self {
        self.client_token = Some(client_token.into());
        self
    }

    /// 设置待删除数据表 ID 列表。
    pub fn table_ids(mut self, table_ids: Vec<String>) -> Self {
        self.table_ids = table_ids;
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<BatchDeleteTableResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchDeleteTableResponse> {
        validate_required!(self.app_token, "app_token 不能为空");
        if self.table_ids.is_empty() {
            return Err(openlark_core::error::validation_error(
                "table_ids",
                "数据表 ID 列表不能为空",
            ));
        }
        if self.table_ids.len() > 50 {
            return Err(openlark_core::error::validation_error(
                "table_ids",
                "批量删除数据表数量不能超过 50 个",
            ));
        }

        let api_endpoint = BitableApiV1::TableBatchDelete(self.app_token);
        // #439: method 来自 catalog
        let mut api_request: ApiRequest<BatchDeleteTableResponse> = api_endpoint
            .to_request::<BatchDeleteTableResponse>()
            .body(serde_json::to_vec(&BatchDeleteTableRequestBody {
                table_ids: self.table_ids,
            })?);

        api_request = api_request.query_opt("user_id_type", self.user_id_type);
        api_request = api_request.query_opt("client_token", self.client_token);

        Transport::request_typed(
            api_request,
            &self.config,
            Some(option),
            "Bitable 批量删除数据表",
        )
        .await
    }
}

/// 批量删除数据表请求体（内部使用）。
#[derive(Debug, Serialize)]
struct BatchDeleteTableRequestBody {
    table_ids: Vec<String>,
}

/// 批量删除数据表响应（data 通常为空对象 `{}`）。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchDeleteTableResponse {}

impl ApiResponseTrait for BatchDeleteTableResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{header, method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST .../tables/batch_delete → BatchDeleteTableResponse。
    /// 完整断言 method、path、auth（来自 catalog #439）和响应。
    #[tokio::test]
    async fn test_batch_delete_table_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/bitable/v1/apps/app001/tables/batch_delete",
            ))
            .and(header("Authorization", "Bearer test-tenant-token"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": {}
            })))
            .mount(&server)
            .await;
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();
        let option = openlark_core::req_option::RequestOption::builder()
            .tenant_access_token("test-tenant-token")
            .build();
        let resp = BatchDeleteTableRequest::new(config)
            .app_token("app001")
            .user_id_type("open_id")
            .client_token("client_001")
            .table_ids(vec!["tbl001".into()])
            .execute_with_options(option)
            .await
            .expect("批量删除数据表应成功");
        let _response = resp;
        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].method, "POST");
        assert_eq!(
            received[0].url.path(),
            "/open-apis/bitable/v1/apps/app001/tables/batch_delete"
        );
        let body: serde_json::Value =
            serde_json::from_slice(&received[0].body).expect("请求体应为 JSON");
        assert_eq!(body["table_ids"], json!(["tbl001"]));
        let query: std::collections::HashMap<_, _> = received[0]
            .url
            .query_pairs()
            .map(|(key, value)| (key.into_owned(), value.into_owned()))
            .collect();
        assert_eq!(
            query.get("user_id_type").map(String::as_str),
            Some("open_id")
        );
        assert_eq!(
            query.get("client_token").map(String::as_str),
            Some("client_001")
        );
        assert_eq!(
            received[0]
                .headers
                .get("authorization")
                .and_then(|h| h.to_str().ok()),
            Some("Bearer test-tenant-token")
        );
    }
}
