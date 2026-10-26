//! 查询指定生效日期的部门架构树
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/department/tree>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};

use serde::{Deserialize, Serialize};

/// 查询指定生效日期的部门架构树请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreeBody {
    /// 部门 ID，默认根部门
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 是否包含失效部门，默认 false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_inactive: Option<bool>,
    /// 日期，格式 yyyy-mm-dd，默认当前日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
}

/// 查询指定生效日期的部门架构树响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TreeResponse {
    /// 部门树节点
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DepartmentNode>>,
    /// 分页标记，当 has_more 为 true 时返回
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多项
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// `DepartmentNode`（官方 department_tree 结构）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DepartmentNode {
    /// 部门 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 部门绝对层级，根部门层级为 0
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    /// 下级部门 ID 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<String>>,
}

impl ApiResponseTrait for TreeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询指定生效日期的部门架构树请求构建器
#[derive(Debug, Clone)]
pub struct TreeRequestBuilder {
    /// 配置信息
    config: Config,
    /// 请求体
    body: TreeBody,
    /// 分页大小（必填，官方类型 int，取值范围 1 ～ 2000，经 query string 以字符串发送）
    page_size: String,
    /// 分页标记，第一次请求不填
    page_token: Option<String>,
    /// 此次调用中使用的部门 ID 类型
    department_id_type: Option<String>,
}

impl TreeRequestBuilder {
    /// 创建请求构建器
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: TreeBody::default(),
            page_size: String::new(),
            page_token: None,
            department_id_type: None,
        }
    }

    /// 设置请求体。
    pub fn body(mut self, body: TreeBody) -> Self {
        self.body = body;
        self
    }

    /// 设置部门 ID（默认根部门）。
    pub fn department_id(mut self, department_id: impl Into<String>) -> Self {
        self.body.department_id = Some(department_id.into());
        self
    }

    /// 设置是否包含失效部门（默认 false）。
    pub fn need_inactive(mut self, need_inactive: bool) -> Self {
        self.body.need_inactive = Some(need_inactive);
        self
    }

    /// 设置日期（格式 yyyy-mm-dd，默认当前日期）。
    pub fn effective_date(mut self, effective_date: impl Into<String>) -> Self {
        self.body.effective_date = Some(effective_date.into());
        self
    }

    /// 设置分页大小（必填，取值范围 1 ～ 2000）。
    pub fn page_size(mut self, page_size: impl Into<String>) -> Self {
        self.page_size = page_size.into();
        self
    }

    /// 设置分页标记。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置部门 ID 类型（open_department_id/department_id/people_corehr_department_id）。
    pub fn department_id_type(mut self, department_id_type: impl Into<String>) -> Self {
        self.department_id_type = Some(department_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<TreeResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<TreeResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        validate_required!(self.page_size, "page_size 不能为空");

        // 构建端点
        let api_endpoint = FeishuPeopleApiV2::DepartmentTree;
        let request = ApiRequest::<TreeResponse>::post(api_endpoint.to_url())
            .query("page_size", self.page_size)
            .query_opt("page_token", self.page_token)
            .query_opt("department_id_type", self.department_id_type)
            .body(serde_json::to_value(&self.body).map_err(|e| {
                openlark_core::error::validation_error(
                    "请求体序列化失败",
                    format!("无法序列化请求参数: {e}"),
                )
            })?);

        // 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "查询指定生效日期的部门架构树响应数据为空",
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{body_json, method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v2/departments/tree
    /// （官方查询参数 page_size 必填；请求体 department_id/need_inactive/effective_date 可选；
    ///   data 内层键为 items / page_token / has_more，children 为部门 ID 字符串列表）
    #[tokio::test]
    async fn test_tree_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/departments/tree"))
            .and(query_param("page_size", "100"))
            .and(query_param(
                "department_id_type",
                "people_corehr_department_id",
            ))
            .and(body_json(json!({
                "department_id": "6893014062142064111",
                "need_inactive": false,
                "effective_date": "2024-01-01"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "id": "4719456877659520852",
                            "level": 1,
                            "children": [
                                "7094136522860922111"
                            ]
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

        let resp = TreeRequestBuilder::new(config)
            .department_id("6893014062142064111")
            .need_inactive(false)
            .effective_date("2024-01-01")
            .page_size("100")
            .department_id_type("people_corehr_department_id")
            .execute()
            .await
            .expect("请求应成功");

        let items = resp.items.expect("items 应解码");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id.as_deref(), Some("4719456877659520852"));
        assert_eq!(items[0].level, Some(1));
        assert_eq!(
            items[0].children.as_deref(),
            Some(&["7094136522860922111".to_string()][..])
        );
        assert_eq!(resp.has_more, Some(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
