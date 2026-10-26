//! 创建职等
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/job_grade/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 官方 i18n 多语文本结构：`{lang, value}`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nText {
    /// 语言编码（IETF BCP 47，如 zh-CN）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 文本内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 创建职等请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateBody {
    /// 职等数值（必填，取值范围 0～99999）
    pub grade_order: i32,
    /// 编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 名称（必填，i18n 多语，长度 0～2，不能包含「/」「；」「;」「\」、「'」字符）
    pub names: Vec<I18nText>,
    /// 描述（i18n 多语，长度 0～2）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<Vec<I18nText>>,
}

/// CreateRequest
#[derive(Debug, Clone)]
pub struct CreateRequest {
    /// 配置信息
    config: Config,
    /// 请求体
    body: CreateBody,
    /// 幂等标记（根据 client_token 是否一致判断是否为同一请求）
    client_token: Option<String>,
}

impl CreateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: CreateBody::default(),
            client_token: None,
        }
    }

    /// 设置请求体
    pub fn body(mut self, body: CreateBody) -> Self {
        self.body = body;
        self
    }

    /// 设置职等数值（必填，取值范围 0～99999）。
    pub fn grade_order(mut self, grade_order: i32) -> Self {
        self.body.grade_order = grade_order;
        self
    }

    /// 设置编码。
    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.body.code = Some(code.into());
        self
    }

    /// 设置名称（必填，i18n 多语）。
    pub fn names(mut self, names: Vec<I18nText>) -> Self {
        self.body.names = names;
        self
    }

    /// 设置描述（i18n 多语）。
    pub fn descriptions(mut self, descriptions: Vec<I18nText>) -> Self {
        self.body.descriptions = Some(descriptions);
        self
    }

    /// 设置幂等标记。
    pub fn client_token(mut self, client_token: impl Into<String>) -> Self {
        self.client_token = Some(client_token.into());
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
        if !(0..=99999).contains(&self.body.grade_order) {
            return Err(openlark_core::error::CoreError::validation_msg(
                "grade_order 取值范围为 0～99999",
            ));
        }
        validate_required!(self.body.names, "names 不能为空");

        let request = ApiRequest::<CreateResponse>::post("/open-apis/corehr/v2/job_grades")
            .query_opt("client_token", self.client_token)
            .body(serde_json::to_value(&self.body)?);

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// CreateResponse
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponse {
    /// 职等 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grade_id: Option<String>,
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
    use wiremock::matchers::{body_json, method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v2/job_grades
    #[tokio::test]
    async fn test_create_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/job_grades"))
            .and(query_param("client_token", "12454646"))
            .and(body_json(json!({
                "grade_order": 9999,
                "code": "A01234",
                "names": [
                    { "lang": "zh-CN", "value": "中文示例" }
                ],
                "descriptions": [
                    { "lang": "zh-CN", "value": "中文示例" }
                ]
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "grade_id": "12345678" }
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
            .grade_order(9999)
            .code("A01234")
            .names(vec![I18nText {
                lang: Some("zh-CN".to_string()),
                value: Some("中文示例".to_string()),
            }])
            .descriptions(vec![I18nText {
                lang: Some("zh-CN".to_string()),
                value: Some("中文示例".to_string()),
            }])
            .client_token("12454646")
            .execute()
            .await
            .expect("请求应成功");

        assert_eq!(resp.grade_id, Some("12345678".to_string()));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }

    /// names 缺失时快速失败
    #[tokio::test]
    async fn test_create_rejects_empty_names() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url("http://127.0.0.1:1")
            .enable_token_cache(false)
            .build();

        let err = CreateRequest::new(config).grade_order(9999).execute().await;
        assert!(err.is_err());
    }
}
