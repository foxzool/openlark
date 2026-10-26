//! 获取 HRBP 列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/bp/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::CoreError,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 获取 HRBP 列表请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 配置信息
    config: Config,
    /// 分页大小（必填，取值范围 1～500）
    page_size: Option<i32>,
    /// 分页标记
    page_token: Option<String>,
    /// 用户 ID 类型
    user_id_type: Option<String>,
    /// 部门 ID 类型
    department_id_type: Option<String>,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            user_id_type: None,
            department_id_type: None,
        }
    }

    /// 设置分页大小（必填，取值范围 1～500）。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置用户 ID 类型（open_id/union_id/user_id/people_corehr_id）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置部门 ID 类型（open_department_id/department_id/people_corehr_department_id）。
    pub fn department_id_type(mut self, department_id_type: impl Into<String>) -> Self {
        self.department_id_type = Some(department_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListResponse> {
        let page_size = self
            .page_size
            .ok_or_else(|| CoreError::validation_msg("page_size 不能为空"))?;
        if !(1..=500).contains(&page_size) {
            return Err(CoreError::validation_msg("page_size 取值范围为 1～500"));
        }

        let request = ApiRequest::<ListResponse>::get("/open-apis/corehr/v2/bps")
            .query("page_size", page_size.to_string())
            .query_opt("page_token", self.page_token)
            .query_opt("user_id_type", self.user_id_type)
            .query_opt("department_id_type", self.department_id_type);

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 获取 HRBP 列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 响应数据
    /// 数据列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<BpItem>>,
    /// 分页令牌
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// HRBP 条目信息（官方 `bp` 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BpItem {
    /// 部门 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 部门 HRBP 的雇佣 ID，不包括上级部门的 HRBP
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hrbp_id: Option<String>,
    /// 兼容保留字段
    #[serde(flatten)]
    pub extra: Value,
}

impl ApiResponseTrait for ListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/corehr/v2/bps
    #[tokio::test]
    async fn test_list_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v2/bps"))
            .and(query_param("page_size", "500"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": {
                    "items": [
                        {
                            "department_id": "4719456877659520852",
                            "hrbp_id": "4719456877659520852"
                        }
                    ],
                    "page_token": "eVQrYzJBNDNONlk4VFZBZVlSdzlKdFJ4bVVHVExENDNKVHoxaVdiVnViQT0=",
                    "has_more": true
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

        let resp = ListRequest::new(config)
            .page_size(500)
            .execute()
            .await
            .expect("请求应成功");

        let item = resp
            .items
            .as_ref()
            .and_then(|items| items.first())
            .expect("items 应包含 HRBP");
        assert_eq!(item.department_id.as_deref(), Some("4719456877659520852"));
        assert_eq!(item.hrbp_id.as_deref(), Some("4719456877659520852"));
        assert_eq!(resp.has_more, Some(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
