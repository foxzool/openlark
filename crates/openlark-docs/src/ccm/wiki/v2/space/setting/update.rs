//! 更新知识空间设置
//!
//! 根据 space_id 更新知识空间公共设置。
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/wiki-v2/space-setting/update>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::ccm::wiki::v2::models::WikiSpaceSetting;
use crate::common::{api_endpoints::WikiApiV2, api_utils::*};

/// 更新知识空间设置请求
pub struct UpdateWikiSpaceSettingRequest {
    space_id: String,
    config: Config,
}

/// 更新知识空间设置响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWikiSpaceSettingResponse {
    /// 知识空间设置信息
    pub setting: Option<WikiSpaceSetting>,
}

impl ApiResponseTrait for UpdateWikiSpaceSettingResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl UpdateWikiSpaceSettingRequest {
    /// 创建更新知识空间设置请求
    pub fn new(config: Config) -> Self {
        Self {
            space_id: String::new(),
            config,
        }
    }

    /// 设置知识空间ID
    pub fn space_id(mut self, space_id: impl Into<String>) -> Self {
        self.space_id = space_id.into();
        self
    }

    /// 执行请求
    pub async fn execute(
        self,
        setting: WikiSpaceSetting,
    ) -> SDKResult<UpdateWikiSpaceSettingResponse> {
        self.execute_with_options(setting, RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        setting: WikiSpaceSetting,
        option: RequestOption,
    ) -> SDKResult<UpdateWikiSpaceSettingResponse> {
        // 验证必填字段
        validate_required!(self.space_id, "知识空间ID不能为空");

        // 使用新的enum+builder系统生成API端点
        let api_endpoint = WikiApiV2::SpaceSettingUpdate(self.space_id.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<UpdateWikiSpaceSettingResponse> = api_endpoint
            .to_request()
            .body(serialize_params(&setting, "更新知识空间设置")?);

        // 发送请求
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "更新知识空间设置")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    use crate::ccm::wiki::v2::models::WikiSpaceSetting;

    /// 端到端：PUT .../spaces/{space_id}/setting → UpdateWikiSpaceSettingResponse。
    #[tokio::test]
    async fn test_update_wiki_space_setting_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path("/open-apis/wiki/v2/spaces/sp001/setting"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": { "setting": { "create_setting": "admin", "security_setting": "allow", "comment_setting": "allow" } }
            })))
            .mount(&server).await;
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();
        let resp = UpdateWikiSpaceSettingRequest::new(config)
            .space_id("sp001")
            .execute(WikiSpaceSetting {
                create_setting: "admin".into(),
                security_setting: "allow".into(),
                comment_setting: "allow".into(),
            })
            .await
            .expect("更新知识空间设置应成功");
        let setting = resp.setting.expect("响应应包含 setting");
        assert_eq!(setting.create_setting, "admin");
        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/wiki/v2/spaces/sp001/setting"
        );
    }
}
