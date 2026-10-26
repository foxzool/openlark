//! 更新默认成本中心
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/default_cost_center/update_version>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 更新默认成本中心请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateVersionBody {
    /// 员工雇佣 ID（必填）
    pub employment_id: String,
    /// 默认成本中心信息（wk_id 与 wk_tid 必填）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_cost_center: Option<UpdateDefaultCostCenter>,
}

/// 更新默认成本中心的成本中心信息（官方 employment_default_cost_center 结构）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpdateDefaultCostCenter {
    /// 默认成本中心 ID（必填）
    pub wk_id: String,
    /// 默认成本中心版本 ID（必填）
    pub wk_tid: String,
    /// 生效日期（示例值格式："2024-12-01"）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 成本中心 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
    /// 是否继承自岗位/部门的默认成本中心
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_inherit: Option<bool>,
    /// 变更原因
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// 更新默认成本中心请求
#[derive(Debug, Clone)]
pub struct UpdateVersionRequest {
    /// 配置信息
    config: Config,
    version_id: Option<String>,
    /// 请求体
    body: UpdateVersionBody,
    /// 幂等标识，服务端会忽略 client_token 重复的请求
    client_token: Option<String>,
    /// 用户 ID 类型（open_id/union_id/user_id/people_corehr_id）
    user_id_type: Option<String>,
}

impl UpdateVersionRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            version_id: None,
            body: UpdateVersionBody::default(),
            client_token: None,
            user_id_type: None,
        }
    }

    /// 设置 `version_id`。
    pub fn version_id(mut self, version_id: impl Into<String>) -> Self {
        self.version_id = Some(version_id.into());
        self
    }

    /// 设置请求体。
    pub fn body(mut self, body: UpdateVersionBody) -> Self {
        self.body = body;
        self
    }

    /// 设置员工雇佣 ID（必填）。
    pub fn employment_id(mut self, employment_id: impl Into<String>) -> Self {
        self.body.employment_id = employment_id.into();
        self
    }

    /// 设置默认成本中心信息（wk_id 与 wk_tid 必填）。
    pub fn default_cost_center(mut self, default_cost_center: UpdateDefaultCostCenter) -> Self {
        self.body.default_cost_center = Some(default_cost_center);
        self
    }

    /// 设置幂等标识。
    pub fn client_token(mut self, client_token: impl Into<String>) -> Self {
        self.client_token = Some(client_token.into());
        self
    }

    /// 设置用户 ID 类型（open_id/union_id/user_id/people_corehr_id）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UpdateVersionResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateVersionResponse> {
        validate_required!(self.body.employment_id, "employment_id 不能为空");
        if let Some(default_cost_center) = &self.body.default_cost_center {
            validate_required!(default_cost_center.wk_id, "wk_id 不能为空");
            validate_required!(default_cost_center.wk_tid, "wk_tid 不能为空");
        }

        let request = ApiRequest::<UpdateVersionResponse>::post(
            "/open-apis/corehr/v2/default_cost_centers/update_version",
        )
        .query_opt("client_token", self.client_token)
        .query_opt("user_id_type", self.user_id_type)
        .body(serde_json::to_value(&self.body)?);

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 更新默认成本中心响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateVersionResponse {
    /// 响应数据
    pub data: Value,
}

impl ApiResponseTrait for UpdateVersionResponse {
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

    /// 端到端：POST /open-apis/corehr/v2/default_cost_centers/update_version
    #[tokio::test]
    async fn test_update_version_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/corehr/v2/default_cost_centers/update_version",
            ))
            .and(body_json(json!({
                "employment_id": "6862995757234914821",
                "default_cost_center": {
                    "wk_id": "703912325303191204",
                    "wk_tid": "703912321231239801",
                    "effective_time": "2024-12-01",
                    "cost_center_id": "7039123253031711012",
                    "is_inherit": false,
                    "reason": "异动"
                }
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "data": {} }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        UpdateVersionRequest::new(config)
            .employment_id("6862995757234914821")
            .default_cost_center(UpdateDefaultCostCenter {
                wk_id: "703912325303191204".to_string(),
                wk_tid: "703912321231239801".to_string(),
                effective_time: Some("2024-12-01".to_string()),
                cost_center_id: Some("7039123253031711012".to_string()),
                is_inherit: Some(false),
                reason: Some("异动".to_string()),
            })
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
