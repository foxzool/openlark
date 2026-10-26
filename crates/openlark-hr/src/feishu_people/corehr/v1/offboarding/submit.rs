//! 操作员工离职
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/offboarding/submit>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 操作员工离职请求
#[derive(Debug, Clone)]
pub struct SubmitRequest {
    /// 配置信息
    config: Config,
    /// 用户 ID 类型（查询参数，可选）
    user_id_type: Option<String>,
    /// 请求体
    body: SubmitBody,
}

impl SubmitRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_id_type: None,
            body: SubmitBody::default(),
        }
    }

    /// 设置用户 ID 类型（可选：open_id / union_id / user_id / people_corehr_id）。
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 设置离职方式 `offboarding_mode`（必填，目前只支持 1：直接离职）。
    pub fn offboarding_mode(mut self, offboarding_mode: i32) -> Self {
        self.body.offboarding_mode = offboarding_mode;
        self
    }

    /// 设置离职员工 ID `employment_id`（必填）。
    pub fn employment_id(mut self, employment_id: String) -> Self {
        self.body.employment_id = employment_id;
        self
    }

    /// 设置离职日期 `offboarding_date`（必填，格式 `YYYY-MM-DD`）。
    pub fn offboarding_date(mut self, offboarding_date: String) -> Self {
        self.body.offboarding_date = offboarding_date;
        self
    }

    /// 设置离职原因 `offboarding_reason_unique_identifier`（必填）。
    pub fn offboarding_reason_unique_identifier(mut self, v: String) -> Self {
        self.body.offboarding_reason_unique_identifier = v;
        self
    }

    /// 设置离职原因说明 `offboarding_reason_explanation`（可选，最长 6000 字符）。
    pub fn offboarding_reason_explanation(mut self, v: String) -> Self {
        self.body.offboarding_reason_explanation = Some(v);
        self
    }

    /// 设置操作发起人 ID `initiator_id`（可选，为空时默认系统发起人）。
    pub fn initiator_id(mut self, initiator_id: String) -> Self {
        self.body.initiator_id = Some(initiator_id);
        self
    }

    /// 设置是否加入离职屏蔽名单 `add_block_list`（可选；为 true 时 block_reason 必填）。
    pub fn add_block_list(mut self, add_block_list: bool) -> Self {
        self.body.add_block_list = Some(add_block_list);
        self
    }

    /// 设置屏蔽原因 `block_reason`（可选）。
    pub fn block_reason(mut self, block_reason: String) -> Self {
        self.body.block_reason = Some(block_reason);
        self
    }

    /// 设置屏蔽原因说明 `block_reason_explanation`（可选）。
    pub fn block_reason_explanation(mut self, v: String) -> Self {
        self.body.block_reason_explanation = Some(v);
        self
    }

    /// 设置离职自定义字段 `custom_fields`（可选）。
    pub fn custom_fields(mut self, custom_fields: Vec<CustomFieldData>) -> Self {
        self.body.custom_fields = Some(custom_fields);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<SubmitResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<SubmitResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        validate_required!(self.body.employment_id, "employment_id 不能为空");
        validate_required!(self.body.offboarding_date, "offboarding_date 不能为空");
        validate_required!(
            self.body.offboarding_reason_unique_identifier,
            "offboarding_reason_unique_identifier 不能为空"
        );
        if self.body.offboarding_mode != 1 {
            return Err(openlark_core::error::validation_error(
                "offboarding_mode",
                "offboarding_mode 目前只支持 1（直接离职）",
            ));
        }

        let api_endpoint = FeishuPeopleApiV1::OffboardingSubmit;
        let mut request = ApiRequest::<SubmitResponse>::post(api_endpoint.to_url())
            .body(serde_json::to_value(&self.body)?);
        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "操作员工离职响应数据为空",
        )
        .await
    }
}

/// 操作员工离职请求体（`offboarding_mode`/`employment_id`/`offboarding_date`/`offboarding_reason_unique_identifier` 必填）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SubmitBody {
    /// 离职方式，目前只支持 1：直接离职
    pub offboarding_mode: i32,
    /// 离职员工 ID，ID 类型与查询参数 user_id_type 取值一致
    pub employment_id: String,
    /// 离职日期，格式 `YYYY-MM-DD`
    pub offboarding_date: String,
    /// 离职原因唯一标识，可通过【查询员工离职原因列表】接口获取
    pub offboarding_reason_unique_identifier: String,
    /// 离职原因说明，长度限制 6000 个字符
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offboarding_reason_explanation: Option<String>,
    /// 操作发起人 ID，为空时默认系统发起人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator_id: Option<String>,
    /// 是否加入离职屏蔽名单（为 true 时 block_reason 必填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_block_list: Option<bool>,
    /// 屏蔽原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_reason: Option<String>,
    /// 屏蔽原因说明
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_reason_explanation: Option<String>,
    /// 离职自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<CustomFieldData>>,
}

/// 离职自定义字段（object_field_data：`field_name`/`value` 均必填）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomFieldData {
    /// 字段唯一标识
    pub field_name: String,
    /// 字段值（json 转义后的字符串）
    pub value: String,
}

/// 操作员工离职响应
///
/// 对应官方响应体 `data` 节点（`ResponseFormat::Data` 下直接反序列化），
/// 官方 `data` 为扁平对象，字段直接平铺在本结构体上。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubmitResponse {
    /// 离职记录 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_id: Option<String>,
    /// 雇员 id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    /// 离职原因唯一标识，可通过【查询员工离职原因列表】接口获取
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_reason_unique_identifier: Option<String>,
    /// 离职日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_date: Option<String>,
    /// 离职原因说明
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offboarding_reason_explanation: Option<String>,
    /// 是否加入离职屏蔽名单
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add_block_list: Option<bool>,
    /// 屏蔽原因
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_reason: Option<String>,
    /// 屏蔽原因说明
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub block_reason_explanation: Option<String>,
    /// 创建时间
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
}

impl ApiResponseTrait for SubmitResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value, json};
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v1/offboardings/submit
    #[tokio::test]
    async fn test_submit_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v1/offboardings/submit"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "offboarding_id": "7095671727698478604",
                    "employment_id": "6982509313466189342",
                    "offboarding_reason_unique_identifier": "reason_for_offboarding_option8",
                    "offboarding_date": "2022-05-18",
                    "offboarding_reason_explanation": "离职原因说明",
                    "add_block_list": false,
                    "block_reason": "红线",
                    "block_reason_explanation": "xx 年 xx 月 xx 日因 xx 原因红线",
                    "created_time": "2022-05-09 17:50:17"
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

        let resp = SubmitRequest::new(config)
            .offboarding_mode(1)
            .employment_id("6982509313466189342".to_string())
            .offboarding_date("2022-05-18".to_string())
            .offboarding_reason_unique_identifier("reason_for_offboarding_option8".to_string())
            .offboarding_reason_explanation("离职原因说明".to_string())
            .initiator_id("6982509313466189341".to_string())
            .add_block_list(false)
            .block_reason("红线".to_string())
            .block_reason_explanation("xx 年 xx 月 xx 日因 xx 原因红线".to_string())
            .custom_fields(vec![CustomFieldData {
                field_name: "name".to_string(),
                value: "\"Sandy\"".to_string(),
            }])
            .execute()
            .await
            .expect("请求应成功");

        assert_eq!(resp.offboarding_id.as_deref(), Some("7095671727698478604"));
        assert_eq!(
            resp.offboarding_reason_unique_identifier.as_deref(),
            Some("reason_for_offboarding_option8")
        );
        assert_eq!(resp.add_block_list, Some(false));
        assert_eq!(resp.block_reason.as_deref(), Some("红线"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        // 请求体须与官方文档一致（官方请求体示例形状）
        let sent: Value = received[0].body_json().expect("请求体应为 JSON");
        assert_eq!(
            sent,
            json!({
                "offboarding_mode": 1,
                "employment_id": "6982509313466189342",
                "offboarding_date": "2022-05-18",
                "offboarding_reason_unique_identifier": "reason_for_offboarding_option8",
                "offboarding_reason_explanation": "离职原因说明",
                "initiator_id": "6982509313466189341",
                "add_block_list": false,
                "block_reason": "红线",
                "block_reason_explanation": "xx 年 xx 月 xx 日因 xx 原因红线",
                "custom_fields": [{ "field_name": "name", "value": "\"Sandy\"" }]
            })
        );
    }

    /// 必填校验：缺必填字段 / 非法 offboarding_mode 快速失败
    #[tokio::test]
    async fn test_submit_missing_required_fields_rejected() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build();

        let err = SubmitRequest::new(config.clone())
            .execute()
            .await
            .expect_err("缺必填字段应被拒绝");
        assert!(err.to_string().contains("employment_id"), "实际: {err}");

        let err = SubmitRequest::new(config)
            .offboarding_mode(2)
            .employment_id("6982509313466189342".to_string())
            .offboarding_date("2022-05-18".to_string())
            .offboarding_reason_unique_identifier("reason_for_offboarding_option8".to_string())
            .execute()
            .await
            .expect_err("非法 offboarding_mode 应被拒绝");
        assert!(err.to_string().contains("offboarding_mode"), "实际: {err}");
    }
}
