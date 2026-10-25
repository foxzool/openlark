//! 获取人才面试信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/interview/get_by_talent>

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

use crate::common::shared_models::{CodeNameObject, IdNameObject};
use crate::hire::hire::v1::interview_record::list::InterviewRecordItem;

/// 获取人才面试信息请求
#[derive(Debug, Clone)]
pub struct GetByTalentRequest {
    /// 配置信息
    config: Config,
    talent_id: String,
}

impl GetByTalentRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            talent_id: String::new(),
        }
    }

    /// 设置 `talent_id`。
    pub fn talent_id(mut self, talent_id: String) -> Self {
        self.talent_id = talent_id;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetByTalentResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetByTalentResponse> {
        use crate::common::api_endpoints::HireApiV1;

        validate_required!(self.talent_id.trim(), "候选人 ID 不能为空");

        let api_endpoint = HireApiV1::InterviewGetByTalent(self.talent_id.clone());
        let request = ApiRequest::<GetByTalentResponse>::get(api_endpoint.to_url());
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取人才面试信息响应数据为空",
        )
        .await
    }
}

/// 面试地址（官方 `interview_address`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewAddress {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 地址 ID，可通过获取地址列表接口获取。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 地址名称。
    pub name: Option<crate::common::shared_models::I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 区域。
    pub district: Option<CodeNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 城市。
    pub city: Option<CodeNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 省。
    pub state: Option<CodeNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 国家。
    pub country: Option<CodeNameObject>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 面试会议室（官方 `interview_meeting_room`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewMeetingRoom {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 会议室 ID。
    pub room_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 会议室名称。
    pub room_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 建筑名称。
    pub building_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 会议室预定状态：1 预约中 / 2 预约成功 / 3 预约失败。
    pub reserved_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 楼层。
    pub floor_name: Option<String>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 面试详情（官方 `interview_extend`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewExtendItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试 ID。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试开始时间，毫秒时间戳（int64）。
    pub begin_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试结束时间，毫秒时间戳（int64）。
    pub end_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试轮次。
    pub round: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试评价信息。
    pub interview_record_list: Option<Vec<InterviewRecordItem>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试关联的投递阶段。
    pub stage_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递 ID。
    pub application_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 阶段信息。
    pub stage: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 创建人信息。
    pub creator: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 创建时间，毫秒时间戳（int64）。
    pub biz_create_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 最近更新时间，毫秒时间戳（int64）。
    pub biz_modify_time: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试状态（2-18 枚举，详见官方文档）。
    pub interview_round_summary: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试安排 ID。
    pub interview_arrangement_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试类型：1 现场面试 / 2 电话面试 / 3 视频面试。
    pub interview_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 候选人时区。
    pub talent_time_zone: Option<CodeNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试联系人。
    pub contact_user: Option<IdNameObject>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试联系人电话。
    pub contact_mobile: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 备注。
    pub remark: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试地址。
    pub address: Option<InterviewAddress>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 视频面试工具（1/2/3/4/5/8/9/100 枚举，详见官方文档）。
    pub video_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 集中面试的安排状态：1 未开始 / 2 进行中 / 3 已结束。
    pub arrangement_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 安排类型：1 社招单面 / 2 集中面试。
    pub arrangement_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 安排方式：1 直接安排 / 2 自助约面。
    pub arrangement_appointment_kind: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试会议室。
    pub meeting_room_list: Option<Vec<InterviewMeetingRoom>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试轮次类型。
    pub interview_round_type: Option<IdNameObject>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 投递面试（官方 `talent_interview`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct TalentInterviewItem {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 投递 ID。
    pub application_id: Option<String>,
    #[serde(default)]
    /// 面试列表。
    pub interview_list: Vec<InterviewExtendItem>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 获取人才面试信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetByTalentResponse {
    #[serde(default)]
    /// 投递面试列表。
    pub items: Vec<TalentInterviewItem>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for GetByTalentResponse {
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

    /// 端到端：GET /open-apis/hire/v1/interviews/get_by_talent
    #[tokio::test]
    async fn test_get_by_talent_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/hire/v1/interviews/get_by_talent"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "ok", "data": {
                    "items": [
                        {
                            "application_id": "6949805467799537964",
                            "interview_list": [
                                {
                                    "id": "6949805467799537964",
                                    "begin_time": 1618899376474i64,
                                    "end_time": 1618999376474i64,
                                    "round": 0,
                                    "interview_record_list": [
                                        {
                                            "id": "6949805467799537964",
                                            "user_id": "1618899376474",
                                            "content": "逻辑严密，代码质量高",
                                            "commit_status": 1,
                                            "conclusion": 1,
                                            "interview_score": {
                                                "id": "6949805467799537964",
                                                "level": 3,
                                                "zh_name": "3",
                                                "zh_description": "通过, 能力达到要求, 建议录用",
                                                "en_name": "three",
                                                "en_description": "Pass, ability to meet the requirements, suggest to hire"
                                            },
                                            "interviewer": {
                                                "id": "ou_266ed047ed37f24554e5a0afdbb15556",
                                                "name": { "zh_cn": "段誉", "en_us": "Yu Duan" }
                                            }
                                        }
                                    ],
                                    "stage_id": "634324253532232",
                                    "application_id": "634324253532232",
                                    "stage": {
                                        "id": "634324253532232",
                                        "name": { "zh_cn": "待入职", "en_us": "Prehire" }
                                    },
                                    "creator": {
                                        "id": "ou_266ed047ed37f24554e5a0afdbb15556",
                                        "name": { "zh_cn": "段誉", "en_us": "Yu Duan" }
                                    },
                                    "biz_create_time": 1618999376474i64,
                                    "biz_modify_time": 1618999376474i64,
                                    "interview_round_summary": 1,
                                    "interview_arrangement_id": "634324253532232",
                                    "interview_type": 1,
                                    "talent_time_zone": {
                                        "code": "Asia/Shanghai",
                                        "name": {
                                            "zh_cn": "(GMT+08:00) 中国标准时间 - 北京",
                                            "en_us": "(GMT+08:00) China Standard Time - Beijing"
                                        }
                                    },
                                    "contact_user": {
                                        "id": "ou_266ed047ed37f24554e5a0afdbb15556",
                                        "name": { "zh_cn": "段誉", "en_us": "Yu Duan" }
                                    },
                                    "contact_mobile": "13333333333",
                                    "remark": "test",
                                    "address": {
                                        "id": "6949805467799537964",
                                        "name": {
                                            "zh_cn": "中国四川成都天府二街155号",
                                            "en_us": "No. 155, Tianfu 2nd Street, Chengdu, Sichuan, China"
                                        },
                                        "district": { "code": "DS_1", "name": { "zh_cn": "武侯区", "en_us": "Wuhou" } },
                                        "city": { "code": "CT_1", "name": { "zh_cn": "成都", "en_us": "Chengdu" } },
                                        "state": { "code": "ST_1", "name": { "zh_cn": "四川省", "en_us": "SiChuan" } },
                                        "country": { "code": "CN_1", "name": { "zh_cn": "中国", "en_us": "China" } }
                                    },
                                    "video_type": 1,
                                    "arrangement_status": 1,
                                    "arrangement_type": 1,
                                    "arrangement_appointment_kind": 1,
                                    "meeting_room_list": [
                                        {
                                            "room_id": "6949805467799537964",
                                            "room_name": "OCG111",
                                            "building_name": "OCG",
                                            "reserved_status": 1,
                                            "floor_name": "17"
                                        }
                                    ],
                                    "interview_round_type": {
                                        "id": "6949805467799537964",
                                        "name": { "zh_cn": "一面", "en_us": "Round one" }
                                    }
                                }
                            ]
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

        let resp = GetByTalentRequest::new(config)
            .talent_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");
        assert_eq!(resp.items.len(), 1);
        assert_eq!(
            resp.items[0].application_id.as_deref(),
            Some("6949805467799537964")
        );
        let interview = resp.items[0]
            .interview_list
            .first()
            .expect("interview_list 应非空");
        assert_eq!(interview.id.as_deref(), Some("6949805467799537964"));
        assert_eq!(interview.begin_time, Some(1618899376474));
        assert_eq!(interview.end_time, Some(1618999376474));
        assert_eq!(interview.interview_type, Some(1));
        let record = interview
            .interview_record_list
            .as_ref()
            .expect("interview_record_list 应存在")
            .first()
            .unwrap();
        assert_eq!(record.content.as_deref(), Some("逻辑严密，代码质量高"));
        assert_eq!(record.commit_status, Some(1));
        let score = record
            .interview_score
            .as_ref()
            .expect("interview_score 应存在");
        assert_eq!(score.level, Some(3));
        assert_eq!(score.en_name.as_deref(), Some("three"));
        assert_eq!(
            interview
                .address
                .as_ref()
                .expect("address 应存在")
                .city
                .as_ref()
                .unwrap()
                .code
                .as_deref(),
            Some("CT_1")
        );
        assert_eq!(
            interview
                .meeting_room_list
                .as_ref()
                .unwrap()
                .first()
                .unwrap()
                .room_name
                .as_deref(),
            Some("OCG111")
        );
        assert_eq!(
            interview
                .interview_round_type
                .as_ref()
                .unwrap()
                .id
                .as_deref(),
            Some("6949805467799537964")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
