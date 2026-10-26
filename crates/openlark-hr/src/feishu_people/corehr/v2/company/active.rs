//! 启用/停用公司
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/company/active>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};

use serde::{Deserialize, Serialize};

/// 启用/停用公司请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActiveBody {
    /// 公司 ID（必填），可从批量查询公司接口的 id 字段中获取
    pub company_id: String,
    /// 公司启用/停用生效时间（必填，格式：YYYY-MM-DD，范围 1900-01-01 ～ 9999-12-31）
    pub effective_time: String,
    /// 启用/停用状态（true=启用，false=停用）
    pub active: bool,
    /// 操作原因（必填）
    pub operation_reason: String,
}

/// 启用/停用公司响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActiveResponse {
    /// 公司 ID
    pub data: Option<ActiveResponseData>,
}

/// `ActiveResponseData`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActiveResponseData {
    /// 公司 ID
    pub id: String,
}

impl ApiResponseTrait for ActiveResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 启用/停用公司请求
#[derive(Debug, Clone)]
pub struct ActiveRequest {
    /// 配置信息
    config: Config,
    /// 请求体
    body: ActiveBody,
}

impl ActiveRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: ActiveBody::default(),
        }
    }

    /// 设置请求体。
    pub fn body(mut self, body: ActiveBody) -> Self {
        self.body = body;
        self
    }

    /// 设置公司 ID（必填）。
    pub fn company_id(mut self, company_id: impl Into<String>) -> Self {
        self.body.company_id = company_id.into();
        self
    }

    /// 设置生效时间（必填，格式：YYYY-MM-DD）。
    pub fn effective_time(mut self, effective_time: impl Into<String>) -> Self {
        self.body.effective_time = effective_time.into();
        self
    }

    /// 设置启用/停用（true=启用，false=停用）。
    pub fn active(mut self, active: bool) -> Self {
        self.body.active = active;
        self
    }

    /// 设置操作原因（必填）。
    pub fn operation_reason(mut self, operation_reason: impl Into<String>) -> Self {
        self.body.operation_reason = operation_reason.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ActiveResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ActiveResponse> {
        validate_required!(self.body.company_id, "company_id 不能为空");
        validate_required!(self.body.effective_time, "effective_time 不能为空");
        validate_required!(self.body.operation_reason, "operation_reason 不能为空");

        let request = ApiRequest::<ActiveResponse>::post("/open-apis/corehr/v2/companies/active")
            .body(serde_json::to_value(&self.body)?);

        // 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "启用/停用公司响应数据为空",
        )
        .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{body_json, method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v2/companies/active
    /// （官方请求体必填 company_id/effective_time/active/operation_reason）
    #[tokio::test]
    async fn test_active_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/companies/active"))
            .and(body_json(json!({
                "company_id": "1616161616",
                "effective_time": "2020-01-01",
                "active": true,
                "operation_reason": "业务操作"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": {  }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        ActiveRequest::new(config)
            .company_id("1616161616")
            .effective_time("2020-01-01")
            .active(true)
            .operation_reason("业务操作")
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
