//! 获取自定义枚举详细信息
//!
//! URL: GET:/open-apis/apaas/v1/workspaces/:workspace_id/enums/:enum_name
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/apaas-v1/workspace-enum/enum_get>
//!
//! URL: GET:/open-apis/apaas/v1/workspaces/:workspace_id/enums/:enum_name

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 获取枚举详情 Builder
#[derive(Debug, Clone)]
pub struct EnumGetRequestBuilder {
    config: Config,
    /// 工作空间 ID
    workspace_id: String,
    /// 枚举名称
    enum_name: String,
}

impl EnumGetRequestBuilder {
    /// 创建新的 Builder
    pub fn new(
        config: Config,
        workspace_id: impl Into<String>,
        enum_name: impl Into<String>,
    ) -> Self {
        Self {
            config,
            workspace_id: workspace_id.into(),
            enum_name: enum_name.into(),
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<EnumGetResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<EnumGetResponse> {
        let url = format!(
            "/open-apis/apaas/v1/workspaces/{}/enums/{}",
            self.workspace_id, self.enum_name
        );

        let req: ApiRequest<EnumGetResponse> = ApiRequest::get(&url);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 枚举详情响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EnumGetResponse {
    /// 枚举名称
    #[serde(rename = "enum_name")]
    pub enum_name: String,
    /// 枚举描述
    #[serde(rename = "description", skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 枚举值列表
    #[serde(rename = "values")]
    pub values: Vec<EnumValue>,
    /// 创建时间
    #[serde(rename = "created_time")]
    pub created_time: i64,
    /// 更新时间
    #[serde(rename = "updated_time")]
    pub updated_time: i64,
}

/// 枚举值
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct EnumValue {
    /// 值 ID
    #[serde(rename = "value_id")]
    value_id: String,
    /// 值名称
    #[serde(rename = "value_name")]
    value_name: String,
    /// 是否为默认值
    #[serde(rename = "is_default")]
    is_default: bool,
}

impl ApiResponseTrait for EnumGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to EnumGetRequestBuilder, will be removed in v1.0 (#271)")]
pub type EnumGetBuilder = EnumGetRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：GET .../apaas/v1/workspaces/{ws}/enums/{enum_name} → 强类型 EnumGetResponse。
    #[tokio::test]
    async fn test_get_enum_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/apaas/v1/workspaces/ws_001/enums/priority"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "enum_name": "priority",
                    "description": "优先级枚举",
                    "values": [
                        {"value_id": "v1", "value_name": "高", "is_default": true},
                        {"value_id": "v2", "value_name": "低", "is_default": false}
                    ],
                    "created_time": 1700000000,
                    "updated_time": 1700000100
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

        let resp = EnumGetRequestBuilder::new(config, "ws_001", "priority")
            .execute()
            .await
            .expect("获取枚举详情应成功");
        assert_eq!(resp.enum_name, "priority");
        assert_eq!(resp.description.as_deref(), Some("优先级枚举"));
        assert_eq!(resp.values.len(), 2);
        assert_eq!(resp.values[0].value_id, "v1");
        assert_eq!(resp.created_time, 1700000000);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/workspaces/ws_001/enums/priority"
        );
        assert_eq!(received[0].method, "GET");
    }
}
