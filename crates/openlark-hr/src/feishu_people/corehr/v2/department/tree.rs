//! 查询指定生效日期的部门架构树
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/department/tree>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};

use serde::{Deserialize, Serialize};

/// 查询指定生效日期的部门架构树请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreeRequest {
    /// 生效日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 根部门 ID（可选，不传则返回全公司架构树）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_department_id: Option<String>,
}

impl TreeRequest {
    /// 创建请求
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置生效日期
    pub fn effective_time(mut self, effective_time: String) -> Self {
        self.effective_time = Some(effective_time);
        self
    }

    /// 设置根部门 ID
    pub fn root_department_id(mut self, root_department_id: String) -> Self {
        self.root_department_id = Some(root_department_id);
        self
    }
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
    config: Config,
    request: TreeRequest,
}

impl TreeRequestBuilder {
    /// 创建请求构建器
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request: TreeRequest::new(),
        }
    }

    /// 设置生效日期
    pub fn effective_time(mut self, effective_time: String) -> Self {
        self.request = self.request.effective_time(effective_time);
        self
    }

    /// 设置根部门 ID
    pub fn root_department_id(mut self, root_department_id: String) -> Self {
        self.request = self.request.root_department_id(root_department_id);
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

        // 构建端点
        let api_endpoint = FeishuPeopleApiV2::DepartmentTree;
        let request = ApiRequest::<TreeResponse>::post(api_endpoint.to_url());

        // 序列化请求体
        let request = request.body(serde_json::to_value(&self.request).map_err(|e| {
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
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v2/departments/tree
    /// （官方 data 内层键为 items / page_token / has_more，children 为部门 ID 字符串列表）
    #[tokio::test]
    async fn test_tree_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/departments/tree"))
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
