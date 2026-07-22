//! 获取人才详情
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v2/talent/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::common::shared_models::I18nText;
use crate::hire::hire::common_models::AttachmentMeta;

/// 获取人才详情请求
#[derive(Debug, Clone)]
pub struct GetRequest {
    /// 配置信息
    config: Config,
    talent_id: String,
    user_id_type: Option<String>,
}

impl GetRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            talent_id: String::new(),
            user_id_type: None,
        }
    }

    /// 设置 `talent_id`。
    pub fn talent_id(mut self, talent_id: String) -> Self {
        self.talent_id = talent_id;
        self
    }

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetResponse> {
        use crate::common::api_endpoints::HireApiV2;

        validate_required!(self.talent_id.trim(), "人才 ID 不能为空");

        let api_endpoint = HireApiV2::TalentGet(self.talent_id);
        let mut request = ApiRequest::<GetResponse>::get(api_endpoint.to_url());
        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取人才详情响应数据为空",
        )
        .await
    }
}

/// 获取人才详情响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TalentValueOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 键。
    pub key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<I18nText>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `TalentTimeRange`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TalentTimeRange {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 开始时间。
    pub start_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 结束时间。
    pub end_time: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `TalentCustomizedValue`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TalentCustomizedValue {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 内容。
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 选项。
    pub option: Option<TalentValueOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 选项列表。
    pub option_list: Option<Vec<TalentValueOption>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 时间范围。
    pub time_range: Option<TalentTimeRange>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 时间值。
    pub time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 数值。
    pub number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 自定义附件列表。
    pub customized_attachment: Option<Vec<AttachmentMeta>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `TalentCustomizedData`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TalentCustomizedData {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 对象 ID。
    pub object_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<I18nText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 对象类型。
    pub object_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 值。
    pub value: Option<TalentCustomizedValue>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `TalentBasicInfo` 信息。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TalentBasicInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 手机号。
    pub mobile_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 手机号区号。
    pub mobile_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 邮箱地址。
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 工作年限。
    pub experience_years: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 年龄。
    pub age: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 国籍代码。
    pub nationality_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 性别。
    pub gender: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 当前位置代码。
    pub current_location_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 籍贯位置代码。
    pub hometown_location_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 期望工作地点代码列表。
    pub preferred_location_code_list: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 家庭住址。
    pub home_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 证件类型。
    pub identification_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 证件号码。
    pub identification_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 生日时间戳。
    pub birthday: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 婚姻状态。
    pub marital_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 自定义信息列表。
    pub customized_data_list: Option<Vec<TalentCustomizedData>>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `GetResponse` 响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 基础信息。
    pub basic_info: Option<TalentBasicInfo>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for GetResponse {
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

    /// 端到端：GET /open-apis/hire/v2/talents/test001
    #[tokio::test]
    async fn test_get_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/hire/v2/talents/test001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": {  }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        GetRequest::new(config)
            .talent_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
