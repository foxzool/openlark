//! Bitable 批量新增数据表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table/batch_create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::BitableApiV1;

use super::create::TableData;

/// 批量新增数据表请求。
#[derive(Debug, Clone)]
pub struct BatchCreateTableRequest {
    config: Config,
    app_token: String,
    user_id_type: Option<String>,
    client_token: Option<String>,
    tables: Vec<TableData>,
}

impl BatchCreateTableRequest {
    /// 创建新的批量新增数据表请求。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            user_id_type: None,
            client_token: None,
            tables: vec![],
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

    /// 设置幂等标识。
    pub fn client_token(mut self, client_token: impl Into<String>) -> Self {
        self.client_token = Some(client_token.into());
        self
    }

    /// 设置要新增的数据表列表（单次最多 50 个）。
    pub fn tables(mut self, tables: Vec<TableData>) -> Self {
        self.tables = tables;
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<BatchCreateTableResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchCreateTableResponse> {
        validate_required!(self.app_token, "app_token 不能为空");
        if self.tables.is_empty() {
            return Err(openlark_core::error::validation_error(
                "tables",
                "数据表列表不能为空",
            ));
        }
        if self.tables.len() > 50 {
            return Err(openlark_core::error::validation_error(
                "tables",
                "批量创建数据表数量不能超过 50 个",
            ));
        }

        let api_endpoint = BitableApiV1::TableBatchCreate(self.app_token);
        // #439: method 来自 catalog
        let mut api_request: ApiRequest<BatchCreateTableResponse> = api_endpoint
            .to_request::<BatchCreateTableResponse>()
            .body(serde_json::to_vec(&BatchCreateTableRequestBody {
                tables: self.tables,
            })?);

        api_request = api_request.query_opt("user_id_type", self.user_id_type);
        api_request = api_request.query_opt("client_token", self.client_token);

        Transport::request_typed(
            api_request,
            &self.config,
            Some(option),
            "Bitable 批量新增数据表",
        )
        .await
    }
}

/// 批量新增数据表请求体（内部使用）。
#[derive(Debug, Serialize)]
struct BatchCreateTableRequestBody {
    tables: Vec<TableData>,
}

/// 批量新增数据表响应。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateTableResponse {
    /// 新增的数据表 ID 列表
    pub table_ids: Vec<String>,
}

impl ApiResponseTrait for BatchCreateTableResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST .../tables/batch_create → BatchCreateTableResponse。
    #[tokio::test]
    async fn test_batch_create_table_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/bitable/v1/apps/app001/tables/batch_create",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "table_ids": [] }
            })))
            .mount(&server)
            .await;
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();
        BatchCreateTableRequest::new(config)
            .app_token("app001")
            .tables(vec![TableData::new("表1")])
            .execute()
            .await
            .expect("批量创建数据表应成功");
        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/bitable/v1/apps/app001/tables/batch_create"
        );
    }
}
