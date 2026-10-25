//! 查询员工 HRBP / 属地 BP
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/employees.bp/batch_get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// `BatchGetRequest` 请求。
#[derive(Debug, Clone)]
pub struct BatchGetRequest {
    config: Config,
    request_body: Option<Value>,
}

impl BatchGetRequest {
    /// 创建新的请求实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request_body: None,
        }
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<BatchGetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchGetResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        let api_endpoint = FeishuPeopleApiV2::EmployeesBpBatchGet;
        let mut request = ApiRequest::<BatchGetResponse>::post(api_endpoint.to_url());

        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "查询员工 HRBP / 属地 BP响应数据为空",
        )
        .await
    }
}

/// `BatchGetResponse` 响应。
///
/// 响应 `data` 子树形状：`{employment_direct_bps, employment_all_bps}`
/// （由 decode 层解包）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchGetResponse {
    /// 员工直属 BP 信息，当员工所在部门、属地无 BP 时，会上钻找到最近的 BP
    /// （需 corehr:employee.bp:read 权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_direct_bps: Option<Vec<EmploymentBp>>,
    /// 员工全部 BP 信息（需 corehr:employee.all_bp:read 字段权限，未授权时不返回）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_all_bps: Option<Vec<EmploymentBp>>,
}

/// 员工 BP 信息（官方 `employment_bp` 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmploymentBp {
    /// 员工雇佣 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    /// 员工直属 HRBP 雇佣 ID 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hrbp_ids: Option<Vec<String>>,
    /// 员工直属属地 BP 雇佣 ID 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_bp_ids: Option<Vec<String>>,
}

impl ApiResponseTrait for BatchGetResponse {
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

    /// 端到端：POST /open-apis/corehr/v2/employees/bps/batch_get
    #[tokio::test]
    async fn test_batch_get_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/employees/bps/batch_get"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": {
                    "employment_direct_bps": [
                        {
                            "employment_id": "6863326262618752123",
                            "hrbp_ids": ["6863326262618752123"],
                            "location_bp_ids": ["6863326262618752123"]
                        }
                    ],
                    "employment_all_bps": [
                        {
                            "employment_id": "6863326262618752123",
                            "hrbp_ids": ["6863326262618752123"],
                            "location_bp_ids": ["6863326262618752123"]
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

        let resp = BatchGetRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let direct = resp
            .employment_direct_bps
            .as_ref()
            .and_then(|bps| bps.first())
            .expect("employment_direct_bps 应包含直属 BP");
        assert_eq!(direct.employment_id.as_deref(), Some("6863326262618752123"));
        assert_eq!(
            direct.hrbp_ids.as_ref().and_then(|ids| ids.first()),
            Some(&"6863326262618752123".to_string())
        );
        assert_eq!(
            direct.location_bp_ids.as_ref().and_then(|ids| ids.first()),
            Some(&"6863326262618752123".to_string())
        );
        assert_eq!(
            resp.employment_all_bps.as_ref().map(|bps| bps.len()),
            Some(1)
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
