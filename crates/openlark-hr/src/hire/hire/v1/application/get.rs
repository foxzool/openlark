//! 获取投递信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/application/get>

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

use crate::common::shared_models::{CodeNameObject, I18nText};
use crate::hire::hire::common_models::ApplicationJobInfo;

/// 获取投递信息请求
#[derive(Debug, Clone)]
pub struct GetRequest {
    /// 配置信息
    config: Config,
    application_id: String,
}

impl GetRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            application_id: String::new(),
        }
    }

    /// 设置 `application_id`。
    pub fn application_id(mut self, application_id: String) -> Self {
        self.application_id = application_id;
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
        use crate::common::api_endpoints::HireApiV1;

        validate_required!(self.application_id.trim(), "投递 ID 不能为空");

        let api_endpoint = HireApiV1::ApplicationGet(self.application_id.clone());
        let request = ApiRequest::<GetResponse>::get(api_endpoint.to_url());
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取投递信息响应数据为空",
        )
        .await
    }
}

/// 获取投递信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// `application` 字段（官方响应 data 下的投递数据容器）。
    pub application: Option<ApplicationDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_id` 字段。
    pub application_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_id` 字段。
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `application_status` 字段。
    pub application_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `stage_id` 字段。
    pub stage_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `stage_name` 字段。
    pub stage_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_info` 字段。
    pub job_info: Option<ApplicationJobInfo>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 投递详情（官方 `application` 对象，data 下唯一业务键）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ApplicationDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递的职位 ID。
    pub job_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 候选人 ID。
    pub talent_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 简历来源 ID。
    pub resume_source_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递处于的阶段。
    pub stage: Option<ApplicationStageInfo>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 活跃状态（1：活跃 2：非活跃，即为「已终止」）。
    pub active_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递方式（1：HR 寻访 2：候选人主动投递）。
    pub delivery_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递来源信息。
    pub resume_source_info: Option<ApplicationResumeSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 官网投递来源。
    pub website_resume_source: Option<ApplicationWebsiteResumeSource>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 简历附件 ID。
    pub talent_attachment_resume_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 创建时间。
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 修改时间。
    pub modify_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 阶段变更时间列表。
    pub stage_time_list: Option<Vec<ApplicationStageTime>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 终止原因的类型（1：我们拒绝了候选人 22：候选人拒绝了我们 27：其他）。
    pub termination_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 终止的具体原因的 ID 列表。
    pub termination_reason_list: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 终止备注。
    pub termination_reason_note: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 意向投递城市列表。
    pub application_preferred_city_list: Option<Vec<CodeNameObject>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递创建人 ID，仅当投递创建人为企业内部员工时可获取。
    pub creator_id: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 投递阶段信息（官方 `application_stage_info`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ApplicationStageInfo {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 阶段 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 阶段中文名字。
    pub zh_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 阶段英文名。
    pub en_name: Option<String>,
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    /// 阶段类型（1：筛选型 2：评估型 3：笔试型 4：面试型 5：Offer 型 6：待入职 7：已入职 8：其它类型 255：系统默认）。
    pub stage_type: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 投递来源信息（官方 `application_resume_source`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ApplicationResumeSource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递来源 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递来源名称。
    pub name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递来源类型（10000-10009，见官方枚举）。
    pub resume_source_type: Option<i32>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 官网投递来源（官方 `application_website_resume_source`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ApplicationWebsiteResumeSource {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 官网站点 ID。
    pub website_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 官网站点名称。
    pub website_name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 推广渠道来源。
    pub channel: Option<ApplicationWebsiteChannel>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 官网推广渠道来源（官方 `application_website_channel`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ApplicationWebsiteChannel {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 官网推广渠道 ID。
    pub channel_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 官网推广渠道名称。
    pub channel_name: Option<I18nText>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 阶段变更时间（官方 `application_stage_time`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ApplicationStageTime {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 阶段 ID。
    pub stage_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 最近一次进入该阶段的时间。
    pub enter_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 最后一次离开时间，如当前在该阶段，则为空。
    pub exit_time: Option<String>,
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

    /// 端到端：GET /open-apis/hire/v1/applications/test001
    #[tokio::test]
    async fn test_get_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/hire/v1/applications/test001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "application": {
                        "id": "6949805467799537964",
                        "job_id": "6843547872837273223",
                        "talent_id": "6843547872837273223",
                        "resume_source_id": "6583482347283472832",
                        "stage": {
                            "id": "614218419274131",
                            "zh_name": "面试",
                            "en_name": "Interview",
                            "type": 1
                        },
                        "active_status": 1,
                        "delivery_type": 1,
                        "resume_source_info": {
                            "id": "614218419274131",
                            "name": {"zh_cn": "测试", "en_us": "test"},
                            "resume_source_type": 10002
                        },
                        "talent_attachment_resume_id": "6960663240925956415",
                        "create_time": "1632990774278",
                        "modify_time": "1634801678103",
                        "creator_id": "ou_ce613028fe74745421f5dc320bb9c709"
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

        let resp = GetRequest::new(config)
            .application_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let application = resp.application.as_ref().expect("application 容器应存在");
        assert_eq!(application.id.as_deref(), Some("6949805467799537964"));
        assert_eq!(
            application.resume_source_id.as_deref(),
            Some("6583482347283472832")
        );
        assert_eq!(application.delivery_type, Some(1));
        assert_eq!(
            application
                .stage
                .as_ref()
                .and_then(|v| v.zh_name.as_deref()),
            Some("面试")
        );
        assert_eq!(
            application
                .resume_source_info
                .as_ref()
                .and_then(|v| v.resume_source_type),
            Some(10002)
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
