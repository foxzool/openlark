//! Bitable 新增记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::models::Record;

/// 创建记录请求
#[derive(Debug, Clone)]
pub struct CreateRecordRequest {
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
    /// 用户 ID 类型
    user_id_type: Option<String>,
    /// 操作的唯一标识，用于幂等的操作
    client_token: Option<String>,
    /// 是否忽略一致性读写检查
    ignore_consistency_check: Option<bool>,
    /// 记录数据
    fields: Value,
    /// 配置信息
    config: Config,
}

impl CreateRecordRequest {
    /// 创建记录请求
    pub fn new(config: Config) -> Self {
        Self {
            app_token: String::new(),
            table_id: String::new(),
            user_id_type: None,
            client_token: None,
            ignore_consistency_check: None,
            fields: Value::Object(Default::default()),
            config,
        }
    }

    /// 设置应用token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    /// 设置数据表ID
    pub fn table_id(mut self, table_id: String) -> Self {
        self.table_id = table_id;
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 设置客户端令牌
    pub fn client_token(mut self, client_token: String) -> Self {
        self.client_token = Some(client_token);
        self
    }

    /// 是否忽略一致性读写检查
    pub fn ignore_consistency_check(mut self, ignore_consistency_check: bool) -> Self {
        self.ignore_consistency_check = Some(ignore_consistency_check);
        self
    }

    /// 设置记录数据
    pub fn fields(mut self, fields: Value) -> Self {
        self.fields = fields;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateRecordResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateRecordResponse> {
        use crate::common::{api_endpoints::BitableApiV1, api_utils::*};

        // 验证必填字段
        validate_required!(self.app_token.trim(), "应用token不能为空");
        validate_required!(self.table_id.trim(), "数据表ID不能为空");

        let api_endpoint =
            BitableApiV1::RecordCreate(self.app_token.clone(), self.table_id.clone());
        let mut request = ApiRequest::<CreateRecordResponse>::post(&api_endpoint.to_url());

        if let Some(ref user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        if let Some(ref client_token) = self.client_token {
            request = request.query("client_token", client_token);
        }

        if let Some(ignore_consistency_check) = self.ignore_consistency_check {
            request = request.query(
                "ignore_consistency_check",
                &ignore_consistency_check.to_string(),
            );
        }

        let request_body = CreateRecordRequestBody {
            fields: self.fields,
        };
        request = request.body(serialize_params(&request_body, "新增记录")?);

        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "新增记录")
    }
}

/// 创建记录请求体（内部使用）
#[derive(Serialize)]
struct CreateRecordRequestBody {
    fields: Value,
}

/// 创建记录响应
///
/// 包含新增记录的完整信息，包括记录ID、字段值以及创建元数据。
///
/// # 示例
/// ```json
/// {
///   "record": {
///     "record_id": "recxxxxxxxxxxxx",
///     "fields": {
///       "姓名": "张三",
///       "年龄": 25,
///       "部门": ["optxxxxxxxxxxxx"]
///     },
///     "created_by": {
///       "id": "ou_xxxxxxxxxxxxxxxx",
///       "name": "张三",
///       "en_name": "Zhang San"
///     },
///     "created_time": 1234567890000,
///     "record_url": "<https://example.feishu.cn/base/xxxxxxxxxxxxx">
///   }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateRecordResponse {
    /// 新增记录的内容
    ///
    /// 包含记录ID、所有字段的值以及元数据信息。
    /// - `record_id`: 记录的唯一标识符
    /// - `fields`: 字段名到字段值的映射，字段值可以是简单类型、数组或对象
    /// - `created_by` (可选): 记录创建者信息，需要开启 automatic_fields 参数
    /// - `created_time` (可选): 创建时间戳（毫秒），需要开启 automatic_fields 参数
    /// - `shared_url` (可选): 记录分享链接
    /// - `record_url` (可选): 记录访问链接
    pub record: Record,
}

impl ApiResponseTrait for CreateRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_record_request_builder() {
        let config = Config::default();
        let fields = serde_json::json!({
            "姓名": "张三",
            "年龄": 25
        });

        let request = CreateRecordRequest::new(config)
            .app_token("app_123".to_string())
            .table_id("table_456".to_string())
            .user_id_type("open_id".to_string())
            .client_token("token_789".to_string())
            .ignore_consistency_check(true)
            .fields(fields);

        assert_eq!(request.app_token, "app_123");
        assert_eq!(request.table_id, "table_456");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
        assert_eq!(request.client_token, Some("token_789".to_string()));
        assert_eq!(request.ignore_consistency_check, Some(true));
    }

    #[test]
    fn test_create_record_request_minimal() {
        let config = Config::default();
        let request = CreateRecordRequest::new(config);

        assert_eq!(request.app_token, "");
        assert_eq!(request.table_id, "");
        assert!(request.user_id_type.is_none());
        assert!(request.client_token.is_none());
        assert!(request.ignore_consistency_check.is_none());
    }

    #[test]
    fn test_create_record_request_partial() {
        let config = Config::default();
        let request = CreateRecordRequest::new(config)
            .app_token("app_token".to_string())
            .table_id("table_id".to_string());

        assert_eq!(request.app_token, "app_token");
        assert_eq!(request.table_id, "table_id");
        assert!(request.user_id_type.is_none());
    }

    #[test]
    fn test_create_record_request_fields() {
        let config = Config::default();
        let fields = serde_json::json!({
            "字段1": "值1",
            "字段2": 123,
            "字段3": true
        });

        let request = CreateRecordRequest::new(config).fields(fields.clone());

        assert_eq!(request.fields, fields);
    }
}
