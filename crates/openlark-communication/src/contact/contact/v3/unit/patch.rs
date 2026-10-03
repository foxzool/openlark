//! 修改单位信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/unit/patch>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::{api_utils::serialize_params, models::EmptyData},
    endpoints::CONTACT_V3_UNIT,
};

/// 修改单位信息请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchUnitBody {
    /// 单位名字
    ///
    /// 注意：文档中标注“必填：否”，但实际请求该字段必填。
    pub name: String,
}

/// 修改单位信息请求
///
/// 用于更新指定单位的基本属性。
pub struct PatchUnitRequest {
    config: Config,
    unit_id: String,
}

impl PatchUnitRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            unit_id: String::new(),
        }
    }

    /// 单位 ID（路径参数）
    pub fn unit_id(mut self, unit_id: impl Into<String>) -> Self {
        self.unit_id = unit_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/unit/patch>
    pub async fn execute(self, body: PatchUnitBody) -> SDKResult<EmptyData> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: PatchUnitBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EmptyData> {
        validate_required!(self.unit_id, "unit_id 不能为空");
        validate_required!(body.name, "name 不能为空");

        // url: PATCH:/open-apis/contact/v3/unit/:unit_id
        let req: ApiRequest<EmptyData> =
            ApiRequest::patch(format!("{}/{}", CONTACT_V3_UNIT, self.unit_id))
                .body(serialize_params(&body, "修改单位信息")?);

        Transport::request_typed(req, &self.config, Some(option), "修改单位信息").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：PATCH /open-apis/contact/v3/unit/test001
    #[tokio::test]
    async fn test_patch_unit_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PATCH"))
            .and(path("/open-apis/contact/v3/unit/test001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": {}
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let body: PatchUnitBody =
            serde_json::from_value(json!({ "name": "test001" })).expect("body 构造");
        PatchUnitRequest::new(config)
            .unit_id("test001".to_string())
            .execute(body)
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
