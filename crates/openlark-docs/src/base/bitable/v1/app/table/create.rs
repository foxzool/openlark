//! Bitable 新增一个数据表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table/create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::SDKResult,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 新增数据表请求
#[derive(Debug, Clone)]
pub struct CreateTableRequest {
    /// 配置信息
    config: Config,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表信息
    table: TableData,
}

/// 创建数据表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateTableResponse {
    /// 多维表格数据表的 ID
    pub table_id: String,
    /// 默认表格视图的 ID（仅在请求参数中填写了 `default_view_name` 或 `fields` 字段才会返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_view_id: Option<String>,
    /// 数据表初始字段的 ID 列表（仅在请求参数中填写了 `fields` 字段才会返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_id_list: Option<Vec<String>>,
}

impl ApiResponseTrait for CreateTableResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CreateTableRequest {
    /// 创建新增数据表请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            table: TableData::default(),
        }
    }

    /// 设置应用token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    /// 设置数据表信息
    pub fn table(mut self, table: TableData) -> Self {
        self.table = table;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateTableResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateTableResponse> {
        // === 必填字段验证 ===
        validate_required!(self.app_token.trim(), "app_token");

        // === 业务规则验证 ===
        if self.table.name.trim().is_empty() {
            return Err(openlark_core::error::validation_error(
                "name",
                "数据表名称不能为空",
            ));
        }

        if self.table.name.len() > 100 {
            return Err(openlark_core::error::validation_error(
                "name",
                "数据表名称长度不能超过100个字符",
            ));
        }

        // 名称不允许包含 `/ \\ ? * : [ ]`
        let name = self.table.name.as_str();
        if name.contains('/') {
            return Err(openlark_core::error::validation_error(
                "name",
                "数据表名称不能包含 '/'",
            ));
        }
        if name.contains('\\') {
            return Err(openlark_core::error::validation_error(
                "name",
                "数据表名称不能包含 '\\\\'",
            ));
        }
        if name.contains('?') {
            return Err(openlark_core::error::validation_error(
                "name",
                "数据表名称不能包含 '?'",
            ));
        }
        if name.contains('*') {
            return Err(openlark_core::error::validation_error(
                "name",
                "数据表名称不能包含 '*'",
            ));
        }
        if name.contains(':') {
            return Err(openlark_core::error::validation_error(
                "name",
                "数据表名称不能包含 ':'",
            ));
        }
        if name.contains('[') || name.contains(']') {
            return Err(openlark_core::error::validation_error(
                "name",
                "数据表名称不能包含 '[' 或 ']'",
            ));
        }

        // 如果传入了 default_view_name，则必须传入 fields
        if self.table.default_view_name.is_some() && self.table.fields.is_none() {
            return Err(openlark_core::error::validation_error(
                "fields",
                "当填写 default_view_name 时，必须同时填写 fields",
            ));
        }

        // default_view_name 名称中不允许包含 [ ]
        if let Some(ref default_view_name) = self.table.default_view_name {
            if default_view_name.contains('[') || default_view_name.contains(']') {
                return Err(openlark_core::error::validation_error(
                    "default_view_name",
                    "默认视图名称不能包含 '[' 或 ']'",
                ));
            }
        }

        // 🚀 使用新的enum+builder系统生成API端点
        // 替代传统的字符串拼接方式，提供类型安全和IDE自动补全
        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::TableCreate(self.app_token.clone());

        // 构建请求体
        let request_body = CreateTableRequestBody { table: self.table };

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<CreateTableResponse> =
            ApiRequest::post(&api_endpoint.to_url()).body(serde_json::to_vec(&request_body)?);

        // 发送请求
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

/// 数据表数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TableData {
    /// 数据表名称
    pub name: String,
    /// 数据表的默认视图名称，不填则默认为数据表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_view_name: Option<String>,
    /// 数据表初始字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<TableField>>,
}

impl TableData {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            default_view_name: None,
            fields: None,
        }
    }

    /// 设置默认视图名称
    pub fn with_default_view_name(mut self, view_name: impl Into<String>) -> Self {
        self.default_view_name = Some(view_name.into());
        self
    }

    /// 设置初始字段
    pub fn with_fields(mut self, fields: Vec<TableField>) -> Self {
        self.fields = Some(fields);
        self
    }
}

/// 字段信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TableField {
    /// 字段名称
    pub field_name: String,
    /// 字段类型
    #[serde(rename = "type")]
    pub field_type: i32,
    /// 字段属性，不同字段类型对应不同的属性结构
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<serde_json::Value>,
    /// 字段描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<FieldDescription>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::testing::prelude::test_runtime;

    #[test]
    fn test_empty_app_token() {
        let config = Config::default();
        let table = TableData::new("测试表");
        let request = CreateTableRequest::new(config)
            .app_token("".to_string())
            .table(table);
        let rt = test_runtime();
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_table_name() {
        let config = Config::default();
        let table = TableData::new("");
        let request = CreateTableRequest::new(config)
            .app_token("app_token".to_string())
            .table(table);
        let rt = test_runtime();
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("数据表名称不能为空"));
    }

    #[test]
    fn test_table_name_too_long() {
        let config = Config::default();
        let table = TableData::new("a".repeat(101));
        let request = CreateTableRequest::new(config)
            .app_token("app_token".to_string())
            .table(table);
        let rt = test_runtime();
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("数据表名称长度"));
    }

    #[test]
    fn test_table_name_with_invalid_chars() {
        let config = Config::default();
        let table = TableData::new("test/table");
        let request = CreateTableRequest::new(config)
            .app_token("app_token".to_string())
            .table(table);
        let rt = test_runtime();
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(CreateTableResponse::data_format(), ResponseFormat::Data);
    }
}

impl TableField {
    pub fn new(name: impl Into<String>, field_type: i32) -> Self {
        Self {
            field_name: name.into(),
            field_type,
            property: None,
            description: None,
        }
    }

    /// 创建文本字段
    pub fn text(name: impl Into<String>) -> Self {
        Self::new(name, 1) // 文本
    }

    /// 创建数字字段
    pub fn number(name: impl Into<String>) -> Self {
        Self::new(name, 2) // 数字
    }

    /// 创建单选字段
    pub fn single_select(name: impl Into<String>, options: Vec<String>) -> Self {
        let options_value: Vec<serde_json::Value> = options
            .into_iter()
            .map(|opt| serde_json::json!({"name": opt}))
            .collect();

        Self {
            field_name: name.into(),
            field_type: 3, // 单选
            property: Some(serde_json::json!({"options": options_value})),
            description: None,
        }
    }

    /// 创建多选字段
    pub fn multi_select(name: impl Into<String>, options: Vec<String>) -> Self {
        let options_value: Vec<serde_json::Value> = options
            .into_iter()
            .map(|opt| serde_json::json!({"name": opt}))
            .collect();

        Self {
            field_name: name.into(),
            field_type: 4, // 多选
            property: Some(serde_json::json!({"options": options_value})),
            description: None,
        }
    }

    /// 创建日期字段
    pub fn date(name: impl Into<String>) -> Self {
        Self::new(name, 5) // 日期
    }
}

/// 创建数据表请求体（内部使用）
#[derive(Serialize)]
struct CreateTableRequestBody {
    table: TableData,
}

/// 字段描述（用于创建数据表时的字段初始描述）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FieldDescription {
    /// 是否禁止同步到表单的问题描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_sync: Option<bool>,
    /// 描述文本内容（支持换行）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}
