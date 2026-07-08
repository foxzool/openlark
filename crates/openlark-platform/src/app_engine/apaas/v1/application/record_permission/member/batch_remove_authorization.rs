//! 批量删除记录权限用户授权
//!
//! 文档: <https://open.feishu.cn/document/apaas-v1/permission/application-record_permission-member/batch_remove_authorization>
//! docPath: <https://open.feishu.cn/document/apaas-v1/permission/application-record_permission-member/batch_remove_authorization>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 批量删除记录权限用户授权 Builder
#[derive(Debug, Clone)]
pub struct RecordPermissionBatchRemoveAuthRequestBuilder {
    config: Config,
    /// 应用命名空间
    namespace: String,
    /// 记录权限 API 名称
    record_permission_api_name: String,
    /// 用户 ID 列表
    user_ids: Vec<String>,
}

impl RecordPermissionBatchRemoveAuthRequestBuilder {
    /// 创建新的 Builder
    pub fn new(
        config: Config,
        namespace: impl Into<String>,
        record_permission_api_name: impl Into<String>,
    ) -> Self {
        Self {
            config,
            namespace: namespace.into(),
            record_permission_api_name: record_permission_api_name.into(),
            user_ids: Vec::new(),
        }
    }

    /// 添加用户 ID
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_ids.push(user_id.into());
        self
    }

    /// 添加多个用户 ID
    pub fn user_ids(mut self, user_ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.user_ids.extend(user_ids.into_iter().map(Into::into));
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<RecordPermissionBatchRemoveAuthResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<RecordPermissionBatchRemoveAuthResponse> {
        let url = format!(
            "/open-apis/apaas/v1/applications/{}/record_permissions/{}/member/batch_remove_authorization",
            self.namespace, self.record_permission_api_name
        );

        let request = RecordPermissionBatchRemoveAuthRequest {
            user_ids: self.user_ids,
        };

        let req = ApiRequest::<RecordPermissionBatchRemoveAuthResponse>::post(&url)
            .body(serde_json::to_value(&request)?);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 批量删除记录权限用户授权请求
#[derive(Debug, Clone, Deserialize, Serialize)]
struct RecordPermissionBatchRemoveAuthRequest {
    /// 用户 ID 列表
    #[serde(rename = "user_ids")]
    user_ids: Vec<String>,
}

/// 批量删除记录权限用户授权响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RecordPermissionBatchRemoveAuthResponse {
    /// 取消授权的用户数量
    #[serde(rename = "removed_count")]
    pub removed_count: u32,
    /// 结果消息
    #[serde(rename = "message")]
    pub message: String,
}

impl ApiResponseTrait for RecordPermissionBatchRemoveAuthResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(
    note = "renamed to RecordPermissionBatchRemoveAuthRequestBuilder, will be removed in v1.0 (#271)"
)]
pub type RecordPermissionBatchRemoveAuthBuilder = RecordPermissionBatchRemoveAuthRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    /// 端到端：POST .../record_permissions/{api_name}/member/batch_remove_authorization。
    #[tokio::test]
    async fn test_batch_remove_record_permission_member_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/apaas/v1/applications/ns_test/record_permissions/rp_001/member/batch_remove_authorization"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "removed_count": 2,
                    "message": "取消授权成功"
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

        let resp = RecordPermissionBatchRemoveAuthRequestBuilder::new(config, "ns_test", "rp_001")
            .user_ids(vec!["u_001".to_string(), "u_002".to_string()])
            .execute()
            .await
            .expect("批量删除记录权限用户授权应成功");
        assert_eq!(resp.removed_count, 2);
        assert_eq!(resp.message, "取消授权成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/apaas/v1/applications/ns_test/record_permissions/rp_001/member/batch_remove_authorization"
        );
    }
}
