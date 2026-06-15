//! 多实体搜索
//!
//! docPath: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/multi_entity/search

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 多实体搜索请求。
#[derive(Debug, Clone)]
pub struct MultiEntitySearchRequest {
    config: Arc<Config>,
    body: MultiEntitySearchRequestBody,
}

/// 多实体搜索请求体。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiEntitySearchRequestBody {
    /// 搜索关键词（1-50 字符，必填）。
    pub query: String,
    /// 获取的数据条数（默认 20，支持 1-20）。
    pub size: Option<i32>,
}

/// 多实体搜索响应。
#[derive(Debug, Clone, Deserialize)]
pub struct MultiEntitySearchResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<MultiEntitySearchData>,
}

impl ApiResponseTrait for MultiEntitySearchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 多实体搜索响应数据。
#[derive(Debug, Clone, Deserialize)]
pub struct MultiEntitySearchData {
    /// 搜索结果列表。
    pub items: Option<Vec<MultiEntitySearchItem>>,
}

/// 多实体搜索结果项。
#[derive(Debug, Clone, Deserialize)]
pub struct MultiEntitySearchItem {
    /// 实体类型（如 user、chat 等）。
    #[serde(rename = "type")]
    pub entity_type: Option<String>,
    /// 唯一标识 ID。
    pub id: Option<String>,
    /// 名称。
    pub name: Option<String>,
    /// 邮箱地址。
    pub email: Option<String>,
}

impl MultiEntitySearchRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: MultiEntitySearchRequestBody {
                query: String::new(),
                size: None,
            },
        }
    }

    /// 设置搜索关键词（必填，1-50 字符）。
    pub fn query(mut self, query: impl Into<String>) -> Self {
        self.body.query = query.into();
        self
    }

    /// 设置返回条数（默认 20，1-20）。
    pub fn size(mut self, size: i32) -> Self {
        self.body.size = Some(size);
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<MultiEntitySearchResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<MultiEntitySearchResponse> {
        validate_required!(self.body.query, "query 不能为空");

        let req: ApiRequest<MultiEntitySearchResponse> =
            ApiRequest::post("/open-apis/mail/v1/multi_entity/search");
        let body = serde_json::to_value(&self.body).map_err(|e| {
            openlark_core::error::validation_error("多实体搜索", format!("请求体序列化失败: {e}"))
        })?;
        let req = req.body(body);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("多实体搜索", "响应数据为空"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn response_deserializes() {
        let json = r#"{
            "code": 0,
            "msg": "success",
            "data": { "items": [{ "type": "user", "id": "691", "name": "张三", "email": "z@b.com" }] }
        }"#;
        let resp: MultiEntitySearchResponse = serde_json::from_str(json).unwrap();
        let item = resp.data.unwrap().items.unwrap().pop().unwrap();
        assert_eq!(item.entity_type, Some("user".to_string()));
    }
}
