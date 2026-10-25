//! 根据条件批量获取职务
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/job/batch_get>

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
            ApiRequest::<BatchGetResponse>::post("/open-apis/corehr/v2/jobs/batch_get");

        if let Some(body) = self.body {
            request = request.body(body);
        }

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// BatchGetResponse
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchGetResponse {
    /// 查询的职务信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Job>>,
}

/// 职务信息（job）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Job {
    /// 职务ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 编码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<I18nText>>,
    /// 描述
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<I18nText>>,
    /// 启用状态，true 为启用，false 为停用
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// 职务头衔
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_title: Option<Vec<I18nText>>,
    /// 通道ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pathway_id: Option<String>,
    /// 关联的序列ID列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_family_id_list: Option<Vec<String>>,
    /// 关联的职级ID列表（字段权限：获取职务中的职级信息 corehr:job.job_level:read）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_level_id_list: Option<Vec<String>>,
    /// 工时制度 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub working_hours_type_id: Option<String>,
    /// 当前版本生效日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 当前版本失效日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    /// 自定义字段
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<ObjectFieldData>>,
    /// 创建人
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// 当前版本创建日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// 更新人
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    /// 当前版本更新日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
}

/// 官方 i18n 多语文本结构：{lang, value}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nText {
    /// 语言编码（中文用 zh-CN，英文用 en-US）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 文本内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 自定义字段（object_field_data）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ObjectFieldData {
    /// 字段名
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    /// 字段值，是 json 转义后的字符串，根据元数据定义不同格式不同
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

    /// 端到端：POST /open-apis/corehr/v2/jobs/batch_get
    #[tokio::test]
    async fn test_batch_get_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/jobs/batch_get"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": {
                    "items": [
                        {
                            "id": "4698040628992333549",
                            "code": "JP422119",
                            "name": [ { "lang": "zh-CN", "value": "高级 Java 开发工程师" } ],
                            "description": [ { "lang": "zh-CN", "value": "负责公司核心业务系统的 Java 后端开发" } ],
                            "active": true,
                            "job_title": [ { "lang": "zh-CN", "value": "高级工程师" } ],
                            "pathway_id": "4719519211875096301",
                            "job_family_id_list": ["4719519211875096301"],
                            "job_level_id_list": ["4719519212005299950"],
                            "working_hours_type_id": "6890452208593372679",
                            "effective_time": "2020-01-01 00:00:00",
                            "expiration_time": "2021-01-01 00:00:00",
                            "custom_fields": [ { "field_name": "name", "value": "Sandy" } ],
                            "created_by": "4719519211875096301",
                            "created_time": "2021-01-01 00:00:00",
                            "updated_by": "2415132452875096301",
                            "updated_time": "2021-01-01 00:00:00"
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

        let items = resp.items.expect("items 应存在");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id, Some("4698040628992333549".to_string()));
        let job_title = items[0].job_title.as_ref().expect("job_title 应存在");
        assert_eq!(job_title[0].value, Some("高级工程师".to_string()));
        assert_eq!(
            items[0].job_level_id_list,
            Some(vec!["4719519212005299950".to_string()])
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
