//! 修改 OKR 周期状态
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v1/period/patch>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 修改 OKR 周期状态请求
#[derive(Debug, Clone)]
pub struct PatchRequest {
    /// 周期 ID（必填）
    period_id: String,
    /// 周期状态（必填）
    /// - 1: 未开始
    /// - 2: 进行中
    /// - 3: 已结束
    status: i32,
    /// 配置信息
    config: Config,
}

impl PatchRequest {
    /// 创建请求
    pub fn new(config: Config, period_id: String, status: i32) -> Self {
        Self {
            period_id,
            status,
            config,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<PatchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<PatchResponse> {
        use crate::common::api_endpoints::OkrApiV1;

        validate_required!(self.period_id.trim(), "period_id");
        if !(1..=3).contains(&self.status) {
            return Err(openlark_core::error::validation_error(
                "status 无效",
                "status 必须为 1、2 或 3",
            ));
        }

        // 1. 构建端点
        let api_endpoint = OkrApiV1::PeriodPatch(self.period_id.clone());
        let request = ApiRequest::<PatchResponse>::patch(api_endpoint.to_url());

        // 2. 序列化请求体
        let request_body = PatchRequestBody {
            status: self.status,
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
            "修改 OKR 周期状态响应数据为空",
        )
        .await
    }
}

/// 修改 OKR 周期状态请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchRequestBody {
    /// 周期状态（必填）
    pub status: i32,
}

/// 修改 OKR 周期状态响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchResponse {
    /// 周期 ID
    pub period_id: String,
    /// 周期状态
    pub status: i32,
    /// 更新时间
    pub updated_at: i64,
}

impl ApiResponseTrait for PatchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;

    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_okr_v1_period_patch_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value =
            serde_json::from_str(r#"{"period_id": "test", "status": 0, "updated_at": 0}"#).unwrap();
        Mock::given(method("PATCH"))
            .and(path("/open-apis/okr/v1/periods/period_001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": data_body
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = PatchRequest::new(config, "period_001".to_string(), 1)
            .execute()
            .await
            .expect("okr_v1_period_patch 应成功");

        assert_eq!(data.period_id, "test");
        let _ = data.status;
        let _ = data.updated_at;

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/okr/v1/periods/period_001"
        );
    }
}
