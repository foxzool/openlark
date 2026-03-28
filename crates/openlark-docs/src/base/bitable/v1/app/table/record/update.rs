//! Bitable 更新记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/update>

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

/// 更新记录请求
///
/// 用于更新多维表格中的单条记录。
///
/// # 字段说明
///
/// - `app_token`: 多维表格的 app token，不能为空
/// - `table_id`: 表格的 ID，不能为空
/// - `record_id`: 记录的 ID，不能为空
/// - `user_id_type`: 用户 ID 类型（可选）
/// - `ignore_consistency_check`: 是否忽略一致性检查（可选）
/// - `fields`: 要更新的字段键值对
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_core::config::Config;
/// use openlark_docs::base::bitable::v1::app::table::record::UpdateRecordRequest;
/// use serde_json::json;
///
/// let config = Config::builder().app_id("app_id").app_secret("app_secret").build();
/// let fields = json!({"字段名": "新值"});
/// let request = UpdateRecordRequest::new(config)
///     .app_token("app_token".to_string())
///     .table_id("table_id".to_string())
///     .record_id("record_id".to_string())
///     .fields(fields);
/// let response = request.execute().await?;
/// ```
#[derive(Debug, Clone)]
pub struct UpdateRecordRequest {
    config: Config,
    app_token: String,
    table_id: String,
    record_id: String,
    user_id_type: Option<String>,
    ignore_consistency_check: Option<bool>,
    fields: Value,
}

impl UpdateRecordRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            table_id: String::new(),
            record_id: String::new(),
            user_id_type: None,
            ignore_consistency_check: None,
            fields: Value::Object(Default::default()),
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

    pub fn record_id(mut self, record_id: String) -> Self {
        self.record_id = record_id;
        self
    }

    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 是否忽略一致性读写检查
    pub fn ignore_consistency_check(mut self, ignore_consistency_check: bool) -> Self {
        self.ignore_consistency_check = Some(ignore_consistency_check);
        self
    }

    pub fn fields(mut self, fields: Value) -> Self {
        self.fields = fields;
        self
    }

    pub async fn execute(self) -> SDKResult<UpdateRecordResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateRecordResponse> {
        // === 必填字段验证 ===
        validate_required!(self.app_token.trim(), "app_token 不能为空");
        validate_required!(self.table_id.trim(), "table_id 不能为空");
        validate_required!(self.record_id.trim(), "record_id 不能为空");

        use crate::common::{api_endpoints::BitableApiV1, api_utils::*};

        let api_endpoint = BitableApiV1::RecordUpdate(
            self.app_token.clone(),
            self.table_id.clone(),
            self.record_id,
        );

        let request = ApiRequest::<UpdateRecordResponse>::put(&api_endpoint.to_url())
            .query_opt("user_id_type", self.user_id_type)
            .query_opt(
                "ignore_consistency_check",
                self.ignore_consistency_check.map(|v| v.to_string()),
            )
            .body(serialize_params(
                &UpdateRecordRequestBody {
                    fields: self.fields,
                },
                "更新记录",
            )?);

        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "更新记录")
    }
}

#[derive(Serialize)]
struct UpdateRecordRequestBody {
    fields: Value,
}

/// 更新记录响应
///
/// 包含更新后记录的完整信息，包括记录ID、更新后的字段值以及更新元数据。
///
/// # 示例
/// ```json
/// {
///   "record": {
///     "record_id": "recxxxxxxxxxxxx",
///     "fields": {
///       "姓名": "张三",
///       "邮箱": "new_email@example.com"
///     },
///     "last_modified_by": {
///       "id": "ou_xxxxxxxxxxxxxxxx",
///       "name": "李四",
///       "en_name": "Li Si"
///     },
///     "last_modified_time": 1234567890000,
///     "record_url": "<https://example.feishu.cn/base/xxxxxxxxxxxxx">
///   }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateRecordResponse {
    /// 更新记录的内容
    ///
    /// 包含记录ID、更新后的所有字段值以及元数据信息。
    /// - `record_id`: 记录的唯一标识符
    /// - `fields`: 字段名到更新后字段值的映射
    /// - `last_modified_by` (可选): 最后更新者信息，需要开启 automatic_fields 参数
    /// - `last_modified_time` (可选): 最后更新时间戳（毫秒），需要开启 automatic_fields 参数
    /// - `shared_url` (可选): 记录分享链接
    /// - `record_url` (可选): 记录访问链接
    pub record: Record,
}

impl ApiResponseTrait for UpdateRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_record_request_builder() {
        let config = Config::default();
        let fields = serde_json::json!({"name": "updated"});
        let request = UpdateRecordRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string())
            .record_id("record_id".to_string())
            .fields(fields);

        assert_eq!(request.app_token, "app_token");
        assert_eq!(request.table_id, "table_id");
        assert_eq!(request.record_id, "record_id");
    }

    #[test]
    fn test_update_record_with_empty_app_token() {
        let config = Config::default();
        let request = UpdateRecordRequest::new(config)
            .app_token("".to_string())
            .table_id("table_id".to_string())
            .record_id("record_id".to_string());
        assert_eq!(request.app_token, "");
        // 空 app_token 应在 execute_with_options 时被校验拦截
    }

    #[test]
    fn test_update_record_with_empty_table_id() {
        let config = Config::default();
        let request = UpdateRecordRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("".to_string())
            .record_id("record_id".to_string());
        assert_eq!(request.table_id, "");
        // 空 table_id 应在 execute_with_options 时被校验拦截
    }

    #[test]
    fn test_update_record_with_empty_record_id() {
        let config = Config::default();
        let request = UpdateRecordRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string())
            .record_id("".to_string());
        assert_eq!(request.record_id, "");
        // 空 record_id 应在 execute_with_options 时被校验拦截
    }

    #[test]
    fn test_update_record_with_ignore_consistency_check() {
        let config = Config::default();
        let request = UpdateRecordRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string())
            .record_id("record_id".to_string())
            .ignore_consistency_check(true);
        assert_eq!(request.ignore_consistency_check, Some(true));
    }

    #[test]
    fn test_update_record_with_user_id_type() {
        let config = Config::default();
        let request = UpdateRecordRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string())
            .record_id("record_id".to_string())
            .user_id_type("open_id".to_string());
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_update_record_request_builder_build() {
        let config = Config::default();
        let fields = serde_json::json!({"status": "active"});
        let request = UpdateRecordRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string())
            .record_id("record_id".to_string())
            .fields(fields);

        assert_eq!(request.app_token, "app_token");
        assert_eq!(request.table_id, "table_id");
        assert_eq!(request.record_id, "record_id");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(UpdateRecordResponse::data_format(), ResponseFormat::Data);
    }
}
