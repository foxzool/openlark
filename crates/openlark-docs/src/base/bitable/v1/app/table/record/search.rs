//! Bitable 查询记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-record/search>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use super::models::Record;

/// 查询记录请求
#[derive(Debug, Clone)]
pub struct SearchRecordRequest {
    config: Config,
    app_token: String,
    table_id: String,
    user_id_type: Option<String>,
    page_token: Option<String>,
    page_size: Option<i32>,
    body: SearchRecordRequestBody,
}

impl SearchRecordRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            table_id: String::new(),
            user_id_type: None,
            page_token: None,
            page_size: None,
            body: SearchRecordRequestBody::default(),
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

    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size.min(500));
        self
    }

    /// 视图 ID（当 filter 或 sort 不为空时，view_id 会被忽略）
    pub fn view_id(mut self, view_id: String) -> Self {
        self.body.view_id = Some(view_id);
        self
    }

    /// 指定返回字段名
    pub fn field_names(mut self, field_names: Vec<String>) -> Self {
        self.body.field_names = Some(field_names);
        self
    }

    pub fn sort(mut self, sort: Vec<SortCondition>) -> Self {
        self.body.sort = Some(sort);
        self
    }

    pub fn filter(mut self, filter: FilterInfo) -> Self {
        self.body.filter = Some(filter);
        self
    }

    pub fn automatic_fields(mut self, automatic_fields: bool) -> Self {
        self.body.automatic_fields = Some(automatic_fields);
        self
    }

    pub async fn execute(self) -> SDKResult<SearchRecordResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<SearchRecordResponse> {
        use crate::common::{api_endpoints::BitableApiV1, api_utils::*};

        validate_required!(self.app_token.trim(), "app_token 不能为空");
        validate_required!(self.table_id.trim(), "table_id 不能为空");

        if let Some(page_size) = self.page_size {
            if page_size <= 0 {
                return Err(validation_error("page_size", "page_size 必须大于 0"));
            }
        }

        let api_endpoint =
            BitableApiV1::RecordSearch(self.app_token.clone(), self.table_id.clone());

        let request = ApiRequest::<SearchRecordResponse>::post(&api_endpoint.to_url())
            .query_opt("user_id_type", self.user_id_type)
            .query_opt("page_token", self.page_token)
            .query_opt("page_size", self.page_size.map(|v| v.to_string()))
            .body(serialize_params(&self.body, "查询记录")?);

        let response = Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "查询记录")
    }

    /// 获取所有记录（自动处理分页）
    pub async fn fetch_all(mut self) -> SDKResult<Vec<Record>> {
        let mut all_records = Vec::new();
        let mut has_more = true;
        let mut page_token: Option<String> = None;

        while has_more {
            if let Some(token) = page_token {
                self = self.page_token(token);
            }

            let response = self.clone().execute().await?;
            all_records.extend(response.items);

            has_more = response.has_more;
            page_token = response.page_token;
        }

        Ok(all_records)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SearchRecordRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    view_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    field_names: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sort: Option<Vec<SortCondition>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filter: Option<FilterInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    automatic_fields: Option<bool>,
}

/// 排序条件
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SortCondition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<bool>,
}

/// 筛选条件
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FilterInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conjunction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<FilterCondition>>,
}

/// 单个筛选条件
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FilterCondition {
    pub field_name: String,
    pub operator: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<String>>,
}

/// 查询记录响应
///
/// 包含查询结果列表以及分页信息。
///
/// # 示例
/// ```json
/// {
///   "items": [
///     {
///       "record_id": "recxxxxxxxxxxxx",
///       "fields": {
///         "姓名": "张三",
///         "年龄": 25
///       }
///     },
///     {
///       "record_id": "recyyyyyyyyyyyy",
///       "fields": {
///         "姓名": "李四",
///         "年龄": 30
///       }
///     }
///   ],
///   "has_more": true,
///   "page_token": "page_token_xxxxxxxxx",
///   "total": 100
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchRecordResponse {
    /// 查询结果列表
    ///
    /// 符合查询条件的记录列表，每个记录包含：
    /// - `record_id`: 记录的唯一标识符
    /// - `fields`: 字段名到字段值的映射
    /// - `created_by` (可选): 记录创建者信息
    /// - `created_time` (可选): 创建时间戳
    /// - `last_modified_by` (可选): 最后更新者信息
    /// - `last_modified_time` (可选): 最后更新时间戳
    pub items: Vec<Record>,
    /// 是否还有更多数据
    ///
    /// - `true`: 还有更多记录，可以使用 page_token 继续查询
    /// - `false`: 已查询到最后一条记录
    pub has_more: bool,
    /// 分页标记
    ///
    /// 用于获取下一页数据的令牌。仅在 has_more 为 true 时有效。
    /// 将此值作为 page_token 参数传递给下一个请求即可获取下一页数据。
    pub page_token: Option<String>,
    /// 符合条件的记录总数
    ///
    /// 注意：此字段表示符合查询条件的总记录数，而非当前返回的记录数。
    /// 当设置了查询条件或过滤器时，此值可能远大于 items.length。
    pub total: i32,
}

impl ApiResponseTrait for SearchRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {

    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
