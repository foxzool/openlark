//! 获取人才库列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/talent_pool/search>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::hire::hire::common_models::I18nText;

/// `SearchRequest` 请求。
#[derive(Debug, Clone)]
pub struct SearchRequest {
    config: Config,
    page_size: Option<i32>,
    page_token: Option<String>,
    id_list: Option<Vec<String>>,
}

impl SearchRequest {
    /// 创建新的请求实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            id_list: None,
        }
    }

    /// 设置分页大小。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置 `id_list`。
    pub fn id_list(mut self, id_list: Vec<String>) -> Self {
        self.id_list = Some(id_list);
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<SearchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<SearchResponse> {
        if let Some(page_size) = self.page_size
            && !(1..=100).contains(&page_size)
        {
            return Err(error::validation_error(
                "page_size",
                "page_size 必须在 1-100 之间",
            ));
        }

        if let Some(ref id_list) = self.id_list
            && id_list.len() > 50
        {
            return Err(error::validation_error("id_list", "id_list 不能超过 50 个"));
        }

        let mut request = ApiRequest::<SearchResponse>::get("/open-apis/hire/v1/talent_pools/");
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }
        if let Some(id_list) = self.id_list {
            request = request.query(
                "id_list",
                serde_json::to_string(&id_list).map_err(|e| {
                    error::validation_error("id_list", format!("无法序列化数组查询参数: {e}"))
                })?,
            );
        }
        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            error::validation_error("获取人才库列表响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

/// `TalentPoolItem`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TalentPoolItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `i18n_name` 字段。
    pub i18n_name: Option<I18nText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `i18n_description` 字段。
    pub i18n_description: Option<I18nText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `parent_id` 字段。
    pub parent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `is_private` 字段。
    pub is_private: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `create_time` 字段。
    pub create_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `modify_time` 字段。
    pub modify_time: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `SearchResponse` 响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SearchResponse {
    #[serde(default)]
    /// 结果项列表。
    pub items: Vec<TalentPoolItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 下一页分页标记。
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 是否还有更多结果。
    pub has_more: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for SearchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    #[test]
    fn test_serialization_roundtrip() {
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
