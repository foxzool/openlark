//! 创建成本中心
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/cost_center/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

use super::{CostCenter, I18nText};

/// 创建成本中心请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateBody {
    /// 成本中心名称（必填，i18n 多语；名称不能包含「/」「；」「;」「\」「'」字符，且有全局唯一校验）
    pub name: Vec<I18nText>,
    /// 成本中心编码（不能与其他记录的编码重复；开启自动编码时传入值不生效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 上级成本中心 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_cost_center_id: Option<String>,
    /// 成本中心负责人 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managers: Option<Vec<String>>,
    /// 成本中心描述（i18n 多语）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<I18nText>>,
    /// 版本生效日期（必填，格式：YYYY-MM-DD，范围 1900-01-01～9999-12-31）
    pub effective_time: String,
}

/// CreateRequest
#[derive(Debug, Clone)]
pub struct CreateRequest {
    /// 配置信息
    config: Config,
    /// 请求体
    body: CreateBody,
    /// 用户 ID 类型
    user_id_type: Option<String>,
}

impl CreateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: CreateBody::default(),
            user_id_type: None,
        }
    }

    /// 设置请求体
    pub fn body(mut self, body: CreateBody) -> Self {
        self.body = body;
        self
    }

    /// 设置成本中心名称（i18n 多语）。
    pub fn name(mut self, name: Vec<I18nText>) -> Self {
        self.body.name = name;
        self
    }

    /// 设置成本中心编码。
    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.body.code = Some(code.into());
        self
    }

    /// 设置上级成本中心 ID。
    pub fn parent_cost_center_id(mut self, parent_cost_center_id: impl Into<String>) -> Self {
        self.body.parent_cost_center_id = Some(parent_cost_center_id.into());
        self
    }

    /// 设置成本中心负责人 ID 列表。
    pub fn managers(mut self, managers: Vec<String>) -> Self {
        self.body.managers = Some(managers);
        self
    }

    /// 设置成本中心描述（i18n 多语）。
    pub fn description(mut self, description: Vec<I18nText>) -> Self {
        self.body.description = Some(description);
        self
    }

    /// 设置版本生效日期（格式：YYYY-MM-DD）。
    pub fn effective_time(mut self, effective_time: impl Into<String>) -> Self {
        self.body.effective_time = effective_time.into();
        self
    }

    /// 设置用户 ID 类型（open_id/union_id/user_id/people_corehr_id）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateResponse> {
        validate_required!(self.body.name, "name 不能为空");
        validate_required!(self.body.effective_time, "effective_time 不能为空");

        let request = ApiRequest::<CreateResponse>::post("/open-apis/corehr/v2/cost_centers")
            .query_opt("user_id_type", self.user_id_type)
            .body(serde_json::to_value(&self.body)?);

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// CreateResponse
///
/// 响应 `data` 子树形状：`{cost_center: {...}}`（由 decode 层解包）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponse {
    /// 成本中心信息（官方 `cost_center` 结构）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center: Option<CostCenter>,
}

impl ApiResponseTrait for CreateResponse {
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

    /// 端到端：POST /open-apis/corehr/v2/cost_centers
    #[tokio::test]
    async fn test_create_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/cost_centers"))
            .and(body_json(json!({
                "name": [
                    { "lang": "zh-CN", "value": "基础架构" }
                ],
                "code": "MDPD00000023",
                "parent_cost_center_id": "6862995757234914824",
                "managers": ["6862995757234914824"],
                "description": [
                    { "lang": "zh-CN", "value": "基础架构相关业务" }
                ],
                "effective_time": "2020-01-01"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "cost_center": {
                        "cost_center_id": "6969828847121885087",
                        "cost_center_version_id": "6969828847121885087",
                        "name": [
                            { "lang": "zh-CN", "value": "基础架构" }
                        ],
                        "code": "MDPD00000023",
                        "parent_cost_center_id": "6862995757234914824",
                        "managers": ["6862995757234914824"],
                        "description": [
                            { "lang": "zh-CN", "value": "基础架构相关内容" }
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

        let resp = CreateRequest::new(config)
            .name(vec![I18nText {
                lang: Some("zh-CN".to_string()),
                value: Some("基础架构".to_string()),
            }])
            .code("MDPD00000023")
            .parent_cost_center_id("6862995757234914824")
            .managers(vec!["6862995757234914824".to_string()])
            .description(vec![I18nText {
                lang: Some("zh-CN".to_string()),
                value: Some("基础架构相关业务".to_string()),
            }])
            .effective_time("2020-01-01")
            .execute()
            .await
            .expect("请求应成功");

        let cost_center = resp.cost_center.as_ref().expect("cost_center 应存在");
        assert_eq!(
            cost_center.cost_center_id.as_deref(),
            Some("6969828847121885087")
        );
        assert_eq!(
            cost_center.cost_center_version_id.as_deref(),
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
