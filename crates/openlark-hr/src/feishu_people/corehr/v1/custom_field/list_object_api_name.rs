//! 获取飞书人事对象列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/custom_field/list_object_api_name>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 获取飞书人事对象列表请求
#[derive(Debug, Clone)]
pub struct ListObjectApiNameRequest {
    /// 配置信息
    config: Config,
    /// 分页大小（必填）
    page_size: i32,
    /// 分页标记（可选，第一次请求不填表示从头开始遍历）
    page_token: Option<String>,
}

impl ListObjectApiNameRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: 0,
            page_token: None,
        }
    }

    /// 设置分页大小（必填，示例值 100）。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = page_size;
        self
    }

    /// 设置分页标记（可选，第一次请求不填）。
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListObjectApiNameResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListObjectApiNameResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        if self.page_size < 1 {
            return Err(openlark_core::error::validation_error(
                "page_size",
                "page_size 不能为空且须为正整数",
            ));
        }

        let api_endpoint = FeishuPeopleApiV1::CustomFieldListObjectName;
        let mut request = ApiRequest::<ListObjectApiNameResponse>::get(api_endpoint.to_url())
            .query("page_size", self.page_size.to_string());
        if let Some(page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取飞书人事对象列表响应数据为空",
        )
        .await
    }
}

/// 获取飞书人事对象列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListObjectApiNameResponse {
    /// 响应数据
    /// 数据列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ObjectApiNameItem>>,
    /// 分页令牌
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// 飞书人事对象条目
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ObjectApiNameItem {
    /// 对象的唯一标识
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub object_api_name: Option<String>,
    /// 对象名称（`{zh_cn, en_us}`）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<crate::common::shared_models::I18nText>,
    /// 是否启用
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_open: Option<bool>,
    /// 创建时间，秒级时间戳
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间，秒级时间戳
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 透传的扩展字段
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

impl ApiResponseTrait for ListObjectApiNameResponse {
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

    /// 端到端：GET /open-apis/corehr/v1/custom_fields/list_object_api_name
    #[tokio::test]
    async fn test_list_object_api_name_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/corehr/v1/custom_fields/list_object_api_name",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "object_api_name": "offboarding_info",
                            "name": {"zh_cn": "cn", "en_us": "en"},
                            "is_open": false,
                            "create_time": "1644992809",
                            "update_time": "1644992809"
                        }
                    ],
                    "has_more": true,
                    "page_token": "3"
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = ListObjectApiNameRequest::new(config)
            .page_size(100)
            .execute()
            .await
            .expect("请求应成功");

        let items = resp.items.expect("items 应解析出对象列表");
        assert_eq!(items.len(), 1);
        assert_eq!(
            items[0].object_api_name.as_deref(),
            Some("offboarding_info")
        );
        assert_eq!(items[0].is_open, Some(false));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        // page_size 为官方必填查询参数
        let query = received[0].url.query().unwrap_or_default();
        assert!(
            query.contains("page_size=100"),
            "查询串应含 page_size=100，实际 {query}"
        );
    }
}
