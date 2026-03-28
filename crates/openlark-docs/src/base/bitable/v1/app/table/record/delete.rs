//! Bitable 删除记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/delete>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::SDKResult,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 删除记录请求
///
/// 用于删除多维表格中的单条记录。
///
/// # 字段说明
///
/// - `app_token`: 多维表格的 app token，不能为空
/// - `table_id`: 表格的 ID，不能为空
/// - `record_id`: 记录的 ID，不能为空
///
/// # 示例
///
/// ```rust,ignore
/// use openlark_core::config::Config;
/// use openlark_docs::base::bitable::v1::app::table::record::DeleteRecordRequest;
///
/// let config = Config::builder().app_id("app_id").app_secret("app_secret").build();
/// let request = DeleteRecordRequest::new(config)
///     .app_token("app_token".to_string())
///     .table_id("table_id".to_string())
///     .record_id("record_id".to_string());
/// let response = request.execute().await?;
/// println!("记录已删除: {}", response.deleted);
/// ```
#[derive(Debug, Clone)]
pub struct DeleteRecordRequest {
    config: Config,
    app_token: String,
    table_id: String,
    record_id: String,
}

impl DeleteRecordRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            table_id: String::new(),
            record_id: String::new(),
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

    pub async fn execute(self) -> SDKResult<DeleteRecordResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteRecordResponse> {
        // === 必填字段验证 ===
        validate_required!(self.app_token.trim(), "app_token 不能为空");
        validate_required!(self.table_id.trim(), "table_id 不能为空");
        validate_required!(self.record_id.trim(), "record_id 不能为空");

        use crate::common::{api_endpoints::BitableApiV1, api_utils::*};

        let api_endpoint =
            BitableApiV1::RecordDelete(self.app_token, self.table_id, self.record_id);
        let request = ApiRequest::<DeleteRecordResponse>::delete(&api_endpoint.to_url());

        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "删除记录")
    }
}

/// 删除记录响应
///
/// 表示记录是否成功删除以及被删除记录的ID。
///
/// # 示例
/// ```json
/// {
///   "deleted": true,
///   "record_id": "recxxxxxxxxxxxx"
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeleteRecordResponse {
    /// 是否成功删除
    ///
    /// - `true`: 删除成功
    /// - `false`: 删除失败（通常不会出现，失败会抛出异常）
    pub deleted: bool,
    /// 删除的记录 ID
    ///
    /// 被删除记录的唯一标识符
    pub record_id: String,
}

impl ApiResponseTrait for DeleteRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_record_request_builder() {
        let config = Config::default();
        let request = DeleteRecordRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string())
            .record_id("record_id".to_string());

        assert_eq!(request.app_token, "app_token");
        assert_eq!(request.table_id, "table_id");
        assert_eq!(request.record_id, "record_id");
    }

    #[test]
    fn test_delete_record_with_empty_app_token() {
        let config = Config::default();
        let request = DeleteRecordRequest::new(config)
            .app_token("".to_string())
            .table_id("table_id".to_string())
            .record_id("record_id".to_string());
        assert_eq!(request.app_token, "");
        // 空 app_token 应在 execute_with_options 时被校验拦截
    }

    #[test]
    fn test_delete_record_with_empty_table_id() {
        let config = Config::default();
        let request = DeleteRecordRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("".to_string())
            .record_id("record_id".to_string());
        assert_eq!(request.table_id, "");
        // 空 table_id 应在 execute_with_options 时被校验拦截
    }

    #[test]
    fn test_delete_record_with_empty_record_id() {
        let config = Config::default();
        let request = DeleteRecordRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string())
            .record_id("".to_string());
        assert_eq!(request.record_id, "");
        // 空 record_id 应在 execute_with_options 时被校验拦截
    }

    #[test]
    fn test_delete_record_request_builder_build() {
        let config = Config::default();
        let request = DeleteRecordRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string())
            .record_id("record_id".to_string());

        assert_eq!(request.app_token, "app_token");
        assert_eq!(request.table_id, "table_id");
        assert_eq!(request.record_id, "record_id");
    }

    #[test]
    fn test_delete_record_response() {
        let response = DeleteRecordResponse {
            deleted: true,
            record_id: "rec123".to_string(),
        };
        assert!(response.deleted);
        assert_eq!(response.record_id, "rec123");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(DeleteRecordResponse::data_format(), ResponseFormat::Data);
    }
}
