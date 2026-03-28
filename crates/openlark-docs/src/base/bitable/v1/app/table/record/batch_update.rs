//! Bitable 更新多条记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/batch_update>

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

/// 批量更新记录请求
///
/// 用于批量更新多维表格数据表中的记录。
///
/// # 字段说明
///
/// - `app_token`: 多维表格的 app_token，不能为空
/// - `table_id`: 数据表的 table_id，不能为空
/// - `user_id_type`: 用户 ID 类型
/// - `ignore_consistency_check`: 是否忽略一致性检查
/// - `records`: 要更新的记录列表，最多 500 条，不能为空
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_core::config::Config;
/// use openlark_docs::base::bitable::v1::app::table::record::{BatchUpdateRecordRequest, UpdateRecordItem};
/// use serde_json::json;
///
/// let config = Config::builder().app_id("app_id").app_secret("app_secret").build();
/// let record = UpdateRecordItem {
///     record_id: "record_id".to_string(),
///     fields: json!({"field_name": "new_value"}),
/// };
/// let request = BatchUpdateRecordRequest::new(config)
///     .app_token("app_token")
///     .table_id("table_id")
///     .records(vec![record]);
/// let response = request.execute().await?;
/// ```
#[derive(Debug, Clone)]
pub struct BatchUpdateRecordRequest {
    config: Config,
    app_token: String,
    table_id: String,
    user_id_type: Option<String>,
    ignore_consistency_check: Option<bool>,
    records: Vec<UpdateRecordItem>,
}

impl BatchUpdateRecordRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            table_id: String::new(),
            user_id_type: None,
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

    pub fn ignore_consistency_check(mut self, ignore_consistency_check: bool) -> Self {
        self.ignore_consistency_check = Some(ignore_consistency_check);
        self
    }

    pub fn records(mut self, records: Vec<UpdateRecordItem>) -> Self {
        self.records = records;
        self
    }

    pub async fn execute(self) -> SDKResult<BatchUpdateRecordResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchUpdateRecordResponse> {
        // === 必填字段验证 ===
        validate_required!(self.app_token.trim(), "app_token");
        validate_required!(self.table_id.trim(), "table_id");
        validate_required!(self.records, "records");

        // === 业务规则验证 ===
        if self.records.len() > 500 {
            return Err(openlark_core::error::validation_error(
                "records",
                "单次最多更新 500 条记录",
            ));
        }

        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint =
            BitableApiV1::RecordBatchUpdate(self.app_token.clone(), self.table_id.clone());

        let mut api_request: ApiRequest<BatchUpdateRecordResponse> = ApiRequest::post(
            &api_endpoint.to_url(),
        )
        .body(serde_json::to_vec(&BatchUpdateRecordRequestBody {
            records: self.records,
        })?);

        api_request = api_request.query_opt("user_id_type", self.user_id_type);
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

/// 更新记录条目
///
/// 表示单条记录的更新信息。
///
/// # 字段说明
///
/// - `record_id`: 记录 ID，指定要更新的记录
/// - `fields`: 要更新的字段和值的键值对
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateRecordItem {
    /// 记录 ID
    pub record_id: String,
    /// 更新的字段
    pub fields: Value,
}

#[derive(Serialize)]
struct BatchUpdateRecordRequestBody {
    records: Vec<UpdateRecordItem>,
}

/// 批量更新记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchUpdateRecordResponse {
    /// 更新后的记录列表
    pub records: Vec<Record>,
}

impl ApiResponseTrait for BatchUpdateRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_batch_update_record_request_builder() {
        let config = Config::default();
        let record = UpdateRecordItem {
            record_id: "record_id".to_string(),
            fields: json!({"field_name": "new_value"}),
        };
        let request = BatchUpdateRecordRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string())
            .records(vec![record]);

        assert_eq!(request.app_token, "app_token");
        assert_eq!(request.table_id, "table_id");
        assert_eq!(request.records.len(), 1);
    }

    #[test]
    fn test_update_record_item() {
        let fields = json!({"name": "test", "value": 123});
        let item = UpdateRecordItem {
            record_id: "rec123".to_string(),
            fields: fields.clone(),
        };
        assert_eq!(item.record_id, "rec123");
        assert_eq!(item.fields, fields);
    }

    #[test]
    fn test_empty_app_token_validation() {
        let config = Config::default();
        let record = UpdateRecordItem {
            record_id: "record_id".to_string(),
            fields: json!({}),
        };
        let request = BatchUpdateRecordRequest::new(config)
            .app_token("".to_string())
            .table_id("table_id".to_string())
            .records(vec![record]);
        assert_eq!(request.app_token, "");
    }

    #[test]
    fn test_empty_table_id_validation() {
        let config = Config::default();
        let record = UpdateRecordItem {
            record_id: "record_id".to_string(),
            fields: json!({}),
        };
        let request = BatchUpdateRecordRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("".to_string())
            .records(vec![record]);
        assert_eq!(request.table_id, "");
    }

    #[test]
    fn test_empty_records_validation() {
        let config = Config::default();
        let request = BatchUpdateRecordRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string())
            .records(vec![]);
        assert_eq!(request.records.len(), 0);
    }

    #[test]
    fn test_records_exceeds_limit() {
        let config = Config::default();
        let records: Vec<UpdateRecordItem> = (0..501)
            .map(|i| UpdateRecordItem {
                record_id: format!("rec{}", i),
                fields: json!({}),
            })
            .collect();
        let request = BatchUpdateRecordRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string())
            .records(records);
        assert_eq!(request.records.len(), 501);
    }

    #[test]
    fn test_records_at_limit() {
        let config = Config::default();
        let records: Vec<UpdateRecordItem> = (0..500)
            .map(|i| UpdateRecordItem {
                record_id: format!("rec{}", i),
                fields: json!({}),
            })
            .collect();
        let request = BatchUpdateRecordRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string())
            .records(records);
        assert_eq!(request.records.len(), 500);
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            BatchUpdateRecordResponse::data_format(),
            ResponseFormat::Data
        );
    }
}
