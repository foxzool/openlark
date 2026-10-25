//! 创建假期发放记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/leave_granting_record/create>

use openlark_core::{
    SDKResult,
    api::ApiRequest,
    api::{ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 创建假期发放记录请求
#[derive(Debug, Clone)]
pub struct CreateRequest {
    config: Config,
    body: Value,
}

impl CreateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: Value::Object(serde_json::Map::new()),
        }
    }

    /// 设置请求体。
    pub fn body(mut self, body: Value) -> Self {
        self.body = body;
        self
    }

    /// 向请求体添加字段。
    pub fn field(mut self, key: impl Into<String>, value: Value) -> Self {
        if !self.body.is_object() {
            self.body = Value::Object(serde_json::Map::new());
        }
        if let Some(body) = self.body.as_object_mut() {
            body.insert(key.into(), value);
        }
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
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        let api_endpoint = FeishuPeopleApiV1::LeaveGrantingRecordCreate;
        let request = ApiRequest::<CreateResponse>::post(api_endpoint.to_url()).body(self.body);
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "创建假期发放记录响应数据为空",
        )
        .await
    }
}

/// 创建假期发放记录响应
///
/// 对应官方响应体 `data` 节点（`ResponseFormat::Data` 下直接反序列化）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponse {
    /// 假期发放记录
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_granting_record: Option<LeaveGrantingRecord>,
}

/// 假期发放记录
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LeaveGrantingRecord {
    /// 假期发放记录 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 员工 ID，对应 user_id_type
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    /// 假期类型 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leave_type_id: Option<String>,
    /// 发放数量
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granting_quantity: Option<String>,
    /// 发放时长单位（1 天 / 2 小时）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granting_unit: Option<i32>,
    /// 生效时间，格式为 yyyy-MM-dd
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    /// 失效时间（根据休假规则自动计算），格式为 yyyy-MM-dd
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    /// 发放来源（1 系统发放 / 2 手动发放 / 3 外部系统发放）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub granted_by: Option<i32>,
    /// 发放原因（多语言，`[{lang, value}]` 形态）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<Vec<crate::common::shared_models::I18nLangValue>>,
    /// 发放记录的创建时间，毫秒级时间戳
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// 发放记录的创建人，值为创建人的员工 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// 发放记录的更新时间，毫秒级时间戳
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// 发放记录的更新人，值为更新人的员工 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<String>,
    /// 是否参与折算（1 不参与折算 / 2 参与折算）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub section_type: Option<i32>,
    /// 透传的扩展字段
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
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
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v1/leave_granting_records
    #[tokio::test]
    async fn test_create_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v1/leave_granting_records"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "leave_granting_record": {
                        "id": "6893014062142064135",
                        "employment_id": "6893014062142064135",
                        "leave_type_id": "6893014062142064135",
                        "granting_quantity": "0.5",
                        "granting_unit": 1,
                        "effective_date": "2022-01-01",
                        "expiration_date": "2022-01-01",
                        "granted_by": 3,
                        "reason": [{"lang": "zh_CN", "value": "张三"}],
                        "created_at": "1608725989000",
                        "created_by": "646465654545",
                        "updated_at": "1608725989000",
                        "updated_by": "646465654545",
                        "section_type": 1
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
            .execute()
            .await
            .expect("请求应成功");

        let record = resp
            .leave_granting_record
            .expect("leave_granting_record 应解析");
        assert_eq!(record.id.as_deref(), Some("6893014062142064135"));
        assert_eq!(record.granting_quantity.as_deref(), Some("0.5"));
        assert_eq!(record.granting_unit, Some(1));
        assert_eq!(record.granted_by, Some(3));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
