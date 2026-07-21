//! 获取部门树
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/department/tree>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};

use super::models::{TreeRequestBody, TreeResponse};

/// 获取部门树请求
#[derive(Debug, Clone)]
pub struct TreeRequest {
    /// 配置信息
    config: Config,
    /// 根部门 ID（不传则从顶层开始）
    department_id: Option<String>,
    /// 是否包含已停用部门
    include_inactive: Option<bool>,
}

impl TreeRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            department_id: None,
            include_inactive: None,
        }
    }

    /// 设置根部门 ID（不传则从顶层开始）
    pub fn department_id(mut self, department_id: String) -> Self {
        self.department_id = Some(department_id);
        self
    }

    /// 设置是否包含已停用部门
    pub fn include_inactive(mut self, include_inactive: bool) -> Self {
        self.include_inactive = Some(include_inactive);
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
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        // 1. 构建端点
        let api_endpoint = FeishuPeopleApiV1::DepartmentTree;
        let request = ApiRequest::<TreeResponse>::post(api_endpoint.to_url());

        // 2. 序列化请求体
        let request_body = TreeRequestBody {
            department_id: self.department_id,
            include_inactive: self.include_inactive,
        };
        let request = request.body(serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?);

        // 3. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取部门树响应数据为空",
        )
        .await
    }
}

impl ApiResponseTrait for TreeResponse {
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

    /// 端到端：POST /open-apis/corehr/v1/departments/tree
    #[tokio::test]
    async fn test_tree_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v1/departments/tree"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "items": [] }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        TreeRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
