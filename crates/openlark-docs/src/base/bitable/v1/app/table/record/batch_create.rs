//! Bitable 新增多条记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/batch_create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::SDKResult,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::models::Record;

/// 批量新增记录请求
///
/// 用于在多维表格中批量新增多条记录。
///
/// # 字段说明
///
/// - `app_token`: 多维表格的 app token，不能为空
/// - `table_id`: 表格的 ID，不能为空
/// - `user_id_type`: 用户 ID 类型（可选）
/// - `client_token`: 幂等令牌（可选）
/// - `ignore_consistency_check`: 是否忽略一致性检查（可选）
/// - `records`: 要新增的记录列表，最多 500 条
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_core::config::Config;
/// use openlark_docs::base::bitable::v1::app::table::record::BatchCreateRecordRequest;
/// use serde_json::json;
///
/// let config = Config::builder().app_id("app_id").app_secret("app_secret").build();
/// let record = json!({"字段名": "字段值"});
/// let request = BatchCreateRecordRequest::new(config)
///     .app_token("app_token".to_string())
///     .table_id("table_id".to_string())
///     .records(vec![BatchCreateRecordRequest::CreateRecordItem { fields: record }]);
/// let response = request.execute().await?;
/// ```
#[derive(Debug, Clone)]
pub struct BatchCreateRecordRequest {
    config: Config,
    app_token: String,
    table_id: String,
    user_id_type: Option<String>,
    client_token: Option<String>,
    ignore_consistency_check: Option<bool>,
    records: Vec<CreateRecordItem>,
}

impl BatchCreateRecordRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            table_id: String::new(),
            user_id_type: None,
            client_token: None,
            ignore_consistency_check: None,
            records: Vec::new(),
        }
    }

    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    pub fn table_id(mut self, table_id: String) -> Self {
        self.table_id = table_id;
        self
    }

    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    pub fn client_token(mut self, client_token: String) -> Self {
        self.client_token = Some(client_token);
        self
    }

    pub fn ignore_consistency_check(mut self, ignore_consistency_check: bool) -> Self {
        self.ignore_consistency_check = Some(ignore_consistency_check);
        self
    }

    pub fn records(mut self, records: Vec<CreateRecordItem>) -> Self {
        self.records = records;
        self
    }

    pub async fn execute(self) -> SDKResult<BatchCreateRecordResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchCreateRecordResponse> {
        // === 必填字段验证 ===
        validate_required!(self.app_token.trim(), "app_token");
        validate_required!(self.table_id.trim(), "table_id");
        validate_required!(self.records, "records");

        // === 业务规则验证 ===
        if self.records.len() > 500 {
            return Err(openlark_core::error::validation_error(
                "records",
                "单次最多新增 500 条记录",
            ));
        }

        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint =
            BitableApiV1::RecordBatchCreate(self.app_token.clone(), self.table_id.clone());

        let mut api_request: ApiRequest<BatchCreateRecordResponse> = ApiRequest::post(
            &api_endpoint.to_url(),
        )
        .body(serde_json::to_vec(&BatchCreateRecordRequestBody {
            records: self.records,
        })?);

        api_request = api_request.query_opt("user_id_type", self.user_id_type);
        api_request = api_request.query_opt("client_token", self.client_token);
        api_request = api_request.query_opt(
            "ignore_consistency_check",
            self.ignore_consistency_check.map(|v| v.to_string()),
        );

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

/// 新增记录条目
///
/// 表示单条要新增的记录。
///
/// # 字段说明
///
/// - `fields`: 记录的字段键值对
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateRecordItem {
    /// 记录字段
    pub fields: Value,
}

#[derive(Serialize)]
struct BatchCreateRecordRequestBody {
    records: Vec<CreateRecordItem>,
}

/// 批量新增记录响应
///
/// 包含新增成功的记录列表。
///
/// # 字段说明
///
/// - `records`: 新增成功的记录列表
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchCreateRecordResponse {
    /// 创建的记录列表
    pub records: Vec<Record>,
}

impl ApiResponseTrait for BatchCreateRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_create_record_request_builder() {
        let config = Config::default();
        let request = BatchCreateRecordRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string())
            .records(vec![CreateRecordItem {
                fields: serde_json::json!({"name": "test"}),
            }]);

        assert_eq!(request.app_token, "app_token");
        assert_eq!(request.table_id, "table_id");
        assert_eq!(request.records.len(), 1);
    }

    #[test]
    fn test_batch_create_with_empty_app_token() {
        let config = Config::default();
        let request = BatchCreateRecordRequest::new(config)
            .app_token("".to_string())
            .table_id("table_id".to_string())
            .records(vec![]);
        assert_eq!(request.app_token, "");
        // 空 app_token 应在 execute_with_options 时被校验拦截
    }

    #[test]
    fn test_batch_create_with_empty_table_id() {
        let config = Config::default();
        let request = BatchCreateRecordRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("".to_string())
            .records(vec![]);
        assert_eq!(request.table_id, "");
        // 空 table_id 应在 execute_with_options 时被校验拦截
    }

    #[test]
    fn test_batch_create_with_empty_records() {
        let config = Config::default();
        let request = BatchCreateRecordRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string())
            .records(vec![]);
        assert_eq!(request.records.len(), 0);
        // 空 records 应在 execute_with_options 时被校验拦截
    }

    #[test]
    fn test_batch_create_with_exceed_max_records() {
        let config = Config::default();
        let records: Vec<CreateRecordItem> = (0..501)
            .map(|_| CreateRecordItem {
                fields: serde_json::json!({}),
            })
            .collect();
        let request = BatchCreateRecordRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string())
            .records(records);
        assert_eq!(request.records.len(), 501);
        // 超过 500 条记录应在 execute_with_options 时被校验拦截
    }

    #[test]
    fn test_batch_create_with_max_records() {
        let config = Config::default();
        let records: Vec<CreateRecordItem> = (0..500)
            .map(|_| CreateRecordItem {
                fields: serde_json::json!({}),
            })
            .collect();
        let request = BatchCreateRecordRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string())
            .records(records);
        assert_eq!(request.records.len(), 500);
    }

    #[test]
    fn test_create_record_item() {
        let item = CreateRecordItem {
            fields: serde_json::json!({"name": "test", "value": 123}),
        };
        assert_eq!(item.fields["name"], "test");
        assert_eq!(item.fields["value"], 123);
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            BatchCreateRecordResponse::data_format(),
            ResponseFormat::Data
        );
    }
}
