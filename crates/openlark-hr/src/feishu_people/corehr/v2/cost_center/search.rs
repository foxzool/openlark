//! 搜索成本中心信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/cost_center/search>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::CoreError,
    http::Transport,
};
use serde::{Deserialize, Serialize};

use super::CostCenterVersion;

/// 搜索成本中心信息请求体（官方字段全部可选，不填写默认为空，不参与筛选）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SearchBody {
    /// 成本中心 ID 列表（一次性最多传入 100 个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_center_id_list: Option<Vec<String>>,
    /// 成本中心名称列表，精确匹配
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_list: Option<Vec<String>>,
    /// 成本中心编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 上级成本中心 ID，可用于查询直接下级成本中心
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_cost_center_id: Option<String>,
    /// 是否获取所有成本中心版本（true 为获取所有版本记录，false 为仅获取当前生效记录，默认 false）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get_all_version: Option<bool>,
}

/// SearchRequest
#[derive(Debug, Clone)]
pub struct SearchRequest {
    /// 配置信息
    config: Config,
    /// 请求体
    body: SearchBody,
    /// 分页大小（必填，取值范围 1～100）
    page_size: Option<i32>,
    /// 分页标记
    page_token: Option<String>,
    /// 用户 ID 类型
    user_id_type: Option<String>,
}

impl SearchRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: SearchBody::default(),
            page_size: None,
            page_token: None,
            user_id_type: None,
        }
    }

    /// 设置请求体
    pub fn body(mut self, body: SearchBody) -> Self {
        self.body = body;
        self
    }

    /// 设置成本中心 ID 列表。
    pub fn cost_center_id_list(mut self, cost_center_id_list: Vec<String>) -> Self {
        self.body.cost_center_id_list = Some(cost_center_id_list);
        self
    }

    /// 设置成本中心名称列表（精确匹配）。
    pub fn name_list(mut self, name_list: Vec<String>) -> Self {
        self.body.name_list = Some(name_list);
        self
    }

    /// 设置成本中心编码。
    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.body.code = Some(code.into());
        self
    }

    /// 设置上级成本中心 ID。
    pub fn parent_cost_center_id(mut self, parent_cost_center_id: impl Into<String>) -> Self {
        self.body.parent_cost_center_id = Some(parent_cost_center_id.into());
        self
    }

    /// 设置是否获取所有成本中心版本。
    pub fn get_all_version(mut self, get_all_version: bool) -> Self {
        self.body.get_all_version = Some(get_all_version);
        self
    }

    /// 设置分页大小（必填，取值范围 1～100）。
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

    /// 执行请求
    pub async fn execute(self) -> SDKResult<SearchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<SearchResponse> {
        let page_size = self
            .page_size
            .ok_or_else(|| CoreError::validation_msg("page_size 不能为空"))?;
        if !(1..=100).contains(&page_size) {
            return Err(CoreError::validation_msg("page_size 取值范围为 1～100"));
        }

        let request =
            ApiRequest::<SearchResponse>::post("/open-apis/corehr/v2/cost_centers/search")
                .query("page_size", page_size.to_string())
                .query_opt("page_token", self.page_token)
                .query_opt("user_id_type", self.user_id_type)
                .body(serde_json::to_value(&self.body)?);

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// SearchResponse
///
/// 响应 `data` 子树形状：`{items, page_token, has_more}`（由 decode 层解包）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResponse {
    /// 成本中心信息列表（元素为官方 `cost_center.version` 结构）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<CostCenterVersion>>,
    /// 分页标记，当 has_more 为 true 时返回
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多项
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for SearchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{body_json, method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v2/cost_centers/search
    #[tokio::test]
    async fn test_search_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/cost_centers/search"))
            .and(query_param("page_size", "100"))
            .and(body_json(json!({
                "cost_center_id_list": ["7140964208476371111"],
                "get_all_version": true
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "cost_center_id": "6969828847121885087",
                            "version_id": "6969828847121885087",
                            "name": [
                                { "lang": "zh-CN", "value": "技术部成本中心" }
                            ],
                            "code": "MDPD00000023",
                            "parent_cost_center_id": "6862995757234914824",
                            "managers": ["6862995757234914824"],
                            "description": [
                                { "lang": "zh-CN", "value": "基础架构相关业务" }
                            ],
                            "effective_time": "2020-01-01",
                            "expiration_time": "2020-01-01",
                            "active": true
                        }
                    ],
                    "page_token": "6969828847121885087",
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

        let resp = SearchRequest::new(config)
            .page_size(100)
            .cost_center_id_list(vec!["7140964208476371111".to_string()])
            .get_all_version(true)
            .execute()
            .await
            .expect("请求应成功");

        let item = resp
            .items
            .as_ref()
            .and_then(|items| items.first())
            .expect("items 应包含成本中心版本");
        assert_eq!(item.cost_center_id.as_deref(), Some("6969828847121885087"));
        assert_eq!(item.version_id.as_deref(), Some("6969828847121885087"));
        assert_eq!(
            item.parent_cost_center_id.as_deref(),
            Some("6862995757234914824")
        );
        assert_eq!(
            item.managers.as_deref(),
            Some(&["6862995757234914824".to_string()][..])
        );
        assert_eq!(resp.has_more, Some(true));
        assert_eq!(resp.page_token.as_deref(), Some("6969828847121885087"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
