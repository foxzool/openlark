//! 启用 / 停用成本中心
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/cost_center/patch>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use super::CostCenter;

/// 启用 / 停用成本中心请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PatchBody {
    /// 版本生效日期（必填，格式：YYYY-MM-DD，范围 1900-01-01～9999-12-31）
    pub effective_time: String,
    /// 启用停用状态（必填，true 为启用，false 为停用；默认 false 即停用，调用方必须显式设置）
    pub active: bool,
    /// 操作原因（必填）
    pub operation_reason: String,
}

/// PatchRequest
#[derive(Debug, Clone)]
pub struct PatchRequest {
    /// 配置信息
    config: Config,
    cost_center_id: Option<String>,
    /// 请求体
    body: PatchBody,
    /// 用户 ID 类型
    user_id_type: Option<String>,
}

impl PatchRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            cost_center_id: None,
            body: PatchBody::default(),
            user_id_type: None,
        }
    }

    /// 设置 `cost_center_id`。
    pub fn cost_center_id(mut self, cost_center_id: impl Into<String>) -> Self {
        self.cost_center_id = Some(cost_center_id.into());
        self
    }

    /// 设置请求体
    pub fn body(mut self, body: PatchBody) -> Self {
        self.body = body;
        self
    }

    /// 设置版本生效日期（格式：YYYY-MM-DD）。
    pub fn effective_time(mut self, effective_time: impl Into<String>) -> Self {
        self.body.effective_time = effective_time.into();
        self
    }

    /// 设置启用停用状态（true 为启用，false 为停用；必填）。
    pub fn active(mut self, active: bool) -> Self {
        self.body.active = active;
        self
    }

    /// 设置操作原因（必填）。
    pub fn operation_reason(mut self, operation_reason: impl Into<String>) -> Self {
        self.body.operation_reason = operation_reason.into();
        self
    }

    /// 设置用户 ID 类型（open_id/union_id/user_id/people_corehr_id）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<PatchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<PatchResponse> {
        let cost_center_id = self.cost_center_id.unwrap_or_default();
        validate_required!(cost_center_id.trim(), "cost_center_id 不能为空");
        validate_required!(self.body.effective_time, "effective_time 不能为空");
        validate_required!(self.body.operation_reason, "operation_reason 不能为空");

        let request = ApiRequest::<PatchResponse>::patch(format!(
            "/open-apis/corehr/v2/cost_centers/{cost_center_id}"
        ))
        .query_opt("user_id_type", self.user_id_type)
        .body(serde_json::to_value(&self.body)?);

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// PatchResponse
///
/// 响应 `data` 子树形状：`{cost_center: {...}}`（由 decode 层解包）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchResponse {
    /// 成本中心结果（官方 `cost_center` 结构）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center: Option<CostCenter>,
}

impl ApiResponseTrait for PatchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{body_json, method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：PATCH /open-apis/corehr/v2/cost_centers/test001
    #[tokio::test]
    async fn test_patch_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/corehr/v2/cost_centers/test001"))
            .and(body_json(json!({
                "effective_time": "2020-01-01",
                "active": true,
                "operation_reason": "强行操作"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "cost_center": {
                        "cost_center_id": "6969828847121885087",
                        "name": [
                            { "lang": "zh-CN", "value": "张三" }
                        ],
                        "code": "MDPD00000023",
                        "parent_cost_center_id": "6862995757234914824",
                        "managers": ["6862995757234914824"],
                        "description": [
                            { "lang": "zh-CN", "value": "基础架构相关" }
                        ],
                        "effective_time": "2020-01-01",
                        "expiration_time": "2020-01-01",
                        "active": true
                    }
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

        let resp = PatchRequest::new(config)
            .cost_center_id("test001".to_string())
            .effective_time("2020-01-01")
            .active(true)
            .operation_reason("强行操作")
            .execute()
            .await
            .expect("请求应成功");

        let cost_center = resp.cost_center.as_ref().expect("cost_center 应存在");
        assert_eq!(
            cost_center.cost_center_id.as_deref(),
            Some("6969828847121885087")
        );
        assert_eq!(
            cost_center.parent_cost_center_id.as_deref(),
            Some("6862995757234914824")
        );
        assert_eq!(
            cost_center.managers.as_deref(),
            Some(&["6862995757234914824".to_string()][..])
        );
        assert_eq!(cost_center.active, Some(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
