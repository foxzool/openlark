//! 查询部门 HRBP
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/bp/get_by_department>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 查询部门 HRBP请求
#[derive(Debug, Clone)]
pub struct GetByDepartmentRequest {
    /// 配置信息
    config: Config,
    body: Option<Value>,
}

impl GetByDepartmentRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self { config, body: None }
    }

    /// 设置请求体。
    pub fn body(mut self, body: Value) -> Self {
        self.body = Some(body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetByDepartmentResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetByDepartmentResponse> {
        let mut request = ApiRequest::<GetByDepartmentResponse>::post(
            "/open-apis/corehr/v2/bps/get_by_department",
        );

        if let Some(body) = self.body {
            request = request.body(body);
        }

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 查询部门 HRBP响应
///
/// 响应 `data` 子树形状：`{items: department_hrbp[]}`（由 decode 层解包）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetByDepartmentResponse {
    /// 部门 HRBP 信息，依次为部门及各层级上级部门
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DepartmentHrbp>>,
}

/// 部门 HRBP 信息（官方 `department_hrbp` 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DepartmentHrbp {
    /// 部门 ID，ID 类型与 department_id_type 的取值意义一致
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 部门 HRBP 员工 ID，ID 类型与 user_id_type 的取值意义一致
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hrbp_ids: Option<Vec<String>>,
}

impl ApiResponseTrait for GetByDepartmentResponse {
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

    /// 端到端：POST /open-apis/corehr/v2/bps/get_by_department
    #[tokio::test]
    async fn test_get_by_department_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/bps/get_by_department"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": {
                    "items": [
                        {
                            "department_id": "4719456877659520852",
                            "hrbp_ids": ["4719456877659520852"]
                        }
                    ]
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

        let resp = GetByDepartmentRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let item = resp
            .items
            .as_ref()
            .and_then(|items| items.first())
            .expect("items 应包含部门 HRBP");
        assert_eq!(item.department_id.as_deref(), Some("4719456877659520852"));
        assert_eq!(
            item.hrbp_ids.as_ref().and_then(|ids| ids.first()),
            Some(&"4719456877659520852".to_string())
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
