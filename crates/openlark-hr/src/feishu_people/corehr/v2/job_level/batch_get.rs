//! 根据条件批量获取职级信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/job_level/batch_get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// BatchGetRequest
#[derive(Debug, Clone)]
pub struct BatchGetRequest {
    /// 配置信息
    config: Config,
    /// 请求体（可选）
    body: Option<Value>,
}

impl BatchGetRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self { config, body: None }
    }

    /// 设置请求体
    pub fn body(mut self, body: Value) -> Self {
        self.body = Some(body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchGetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchGetResponse> {
        let mut request =
            ApiRequest::<BatchGetResponse>::post("/open-apis/corehr/v2/job_levels/batch_get");

        if let Some(body) = self.body {
            request = request.body(body);
        }

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// BatchGetResponse
///
/// 响应 `data` 子树形状：`{items: job_level[]}`（由 decode 层解包）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchGetResponse {
    /// 查询的职级信息列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<JobLevel>>,
}

/// 职级信息（官方 `job_level` 结构，v2 形状与 v1 `JobLevel` 不同）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JobLevel {
    /// 职级 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_level_id: Option<String>,
    /// 职级数值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub level_order: Option<i64>,
    /// 编码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 名称（i18n 多语）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<I18nText>>,
    /// 描述（i18n 多语）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<I18nText>>,
    /// 停启用状态。停用为 false，启用为 true
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// 自定义字段（目前职级暂不支持该功能）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldData>>,
    /// 职等 ID 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_grade: Option<Vec<String>>,
    /// 通道 ID 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pathway_ids: Option<Vec<String>>,
    /// 排序，数值越小排序越靠前
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub view_order: Option<i64>,
}

/// 官方 i18n 多语文本结构：`{lang, value}`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nText {
    /// 语言编码（IETF BCP 47）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 文本内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 自定义字段名称：`{zh_cn, en_us}`
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomFieldName {
    /// 中文
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    /// 英文
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
}

/// 官方 `custom_field_data` 结构
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomFieldData {
    /// 自定义字段 apiname，即自定义字段的唯一标识
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_api_name: Option<String>,
    /// 自定义字段名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<CustomFieldName>,
    /// 自定义字段类型
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i64>,
    /// 字段值，是 json 转义后的字符串
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ApiResponseTrait for BatchGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v2/job_levels/batch_get
    #[tokio::test]
    async fn test_batch_get_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/job_levels/batch_get"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": {
                    "items": [
                        {
                            "job_level_id": "4692446793125560154",
                            "level_order": 9999,
                            "code": "VQzo/BSonp8l6PmcZ+VlDhkd2595LMkhyBAGX6HAlCY=",
                            "name": [{ "lang": "zh-CN", "value": "高级开发工程师" }],
                            "description": [{ "lang": "zh-CN", "value": "负责团队技术研发" }],
                            "active": true,
                            "custom_fields": [
                                {
                                    "custom_api_name": "name",
                                    "name": { "zh_cn": "自定义姓名", "en_us": "Custom Name" },
                                    "type": 1,
                                    "value": "\"231\""
                                }
                            ],
                            "job_grade": ["4692446793125560154"],
                            "pathway_ids": ["4719519211875096301"],
                            "view_order": 9999
                        }
                    ]
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

        let resp = BatchGetRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let item = resp
            .items
            .as_ref()
            .and_then(|items| items.first())
            .expect("items 应包含职级");
        assert_eq!(item.job_level_id.as_deref(), Some("4692446793125560154"));
        assert_eq!(item.level_order, Some(9999));
        assert_eq!(item.active, Some(true));
        assert_eq!(
            item.name
                .as_ref()
                .and_then(|n| n.first())
                .and_then(|n| n.value.as_deref()),
            Some("高级开发工程师")
        );
        assert_eq!(item.pathway_ids.as_ref().map(|p| p.len()), Some(1));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
