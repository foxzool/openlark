//! 写入审批结果
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/user_approval/create>
//!
//! 官方请求体为嵌套结构 `{"user_approval": {...}}`，必填 query 参数 `employee_type`；
//! 响应为 `data.user_approval`（各记录内含服务端生成的 `approval_id` 等回显字段）。

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 写入审批结果请求
#[derive(Debug, Clone)]
pub struct CreateRequest {
    /// 员工 ID 类型（必填，query 参数）
    employee_type: String,
    /// 审批信息
    user_approval: UserApproval,
    /// 配置信息
    config: Config,
}

impl CreateRequest {
    /// 创建请求
    pub fn new(config: Config, employee_type: String, user_approval: UserApproval) -> Self {
        Self {
            employee_type,
            user_approval,
            config,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.employee_type, "employee_type 不能为空");
        validate_required!(self.user_approval.user_id, "user_id 不能为空");
        validate_required!(self.user_approval.date, "date 不能为空");

        // 2. 构建端点并附加必填 query 参数 employee_type
        let api_endpoint = AttendanceApiV1::UserApprovalCreate;
        let request = ApiRequest::<CreateResponse>::post(api_endpoint.to_url())
            .query("employee_type", &self.employee_type);

        // 3. 构建请求体
        let request_body = CreateRequestBody {
            user_approval: self.user_approval,
        };
        let request_body_json = serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "构建请求体失败",
                format!("序列化请求体失败: {e}"),
            )
        })?;
        let request = request.body(request_body_json);

        // 4. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "写入审批结果响应数据为空",
        )
        .await
    }
}

/// 审批信息（请求体与响应体的 `user_approval` 同构）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct UserApproval {
    /// 审批提交人 ID，类型需与 employee_type 一致（必填）
    pub user_id: String,
    /// 审批作用日期，格式为 yyyyMMdd（必填）
    pub date: String,
    /// 外出信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outs: Option<Vec<UserOut>>,
    /// 请假信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leaves: Option<Vec<UserLeave>>,
    /// 加班信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overtime_works: Option<Vec<UserOvertimeWork>>,
    /// 出差信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trips: Option<Vec<UserTrip>>,
    /// 时区（此字段不再使用，以用户匹配的考勤组时区为准）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

/// 外出记录（user_out）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct UserOut {
    /// 审批实例 ID（仅响应返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_id: Option<String>,
    /// 外出类型唯一 ID，代表一种外出类型，长度小于 14（必填）
    pub uniq_id: String,
    /// 外出时长单位：1 天、2 小时、3 半天、4 半小时（必填）
    pub unit: i32,
    /// 关联审批单外出时长，单位为秒，与 unit 无关（必填）
    pub interval: i64,
    /// 开始时间，格式 yyyy-MM-dd HH:mm:ss（必填）
    pub start_time: String,
    /// 结束时间，格式 yyyy-MM-dd HH:mm:ss（必填）
    pub end_time: String,
    /// 外出多语言展示（必填）
    pub i18n_names: I18nNames,
    /// 默认语言类型（必填）
    pub default_locale: String,
    /// 外出理由（必填）
    pub reason: String,
    /// 唯一幂等键，用于避免记录重复创建
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotent_id: Option<String>,
    /// 审批通过时间（仅响应返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approve_pass_time: Option<String>,
    /// 审批申请时间（仅响应返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approve_apply_time: Option<String>,
    /// 更正流程实例 ID（系统自动生成，写入时无需传入）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_process_id: Option<Vec<String>>,
    /// 撤销流程实例 ID（系统自动生成，写入时无需传入）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_process_id: Option<Vec<String>>,
    /// 发起流程实例 ID（系统自动生成，写入时无需传入）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_id: Option<Vec<String>>,
}

/// 请假记录（user_leave）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct UserLeave {
    /// 审批实例 ID（仅响应返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_id: Option<String>,
    /// 假期类型唯一 ID，代表一种假期类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uniq_id: Option<String>,
    /// 假期时长单位：1 天、2 小时、3 半天、4 半小时（必填）
    pub unit: i32,
    /// 关联审批单假期时长，单位为秒，与 unit 无关（必填）
    pub interval: i64,
    /// 开始时间，格式 yyyy-MM-dd HH:mm:ss（必填）
    pub start_time: String,
    /// 结束时间，格式 yyyy-MM-dd HH:mm:ss（必填）
    pub end_time: String,
    /// 假期多语言展示（必填）
    pub i18n_names: I18nNames,
    /// 默认语言类型（必填）
    pub default_locale: String,
    /// 请假理由（必填）
    pub reason: String,
    /// 唯一幂等键，用于避免记录重复创建
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotent_id: Option<String>,
    /// 审批通过时间（仅响应返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approve_pass_time: Option<String>,
    /// 审批申请时间（仅响应返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approve_apply_time: Option<String>,
    /// 根据班次计算出来的请假具体时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leave_detail_range_objs: Option<Vec<TimeRangeList>>,
    /// 更正流程实例 ID（系统自动生成，写入时无需传入）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_process_id: Option<Vec<String>>,
    /// 撤销流程实例 ID（系统自动生成，写入时无需传入）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_process_id: Option<Vec<String>>,
    /// 发起流程实例 ID（系统自动生成，写入时无需传入）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_id: Option<Vec<String>>,
}

/// 加班记录（user_overtime_work）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct UserOvertimeWork {
    /// 审批实例 ID（仅响应返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_id: Option<String>,
    /// 加班时长；默认采用 start_time 和 end_time 计算（必填）
    pub duration: f64,
    /// 加班时长单位：1 天、2 小时、3 半天、4 半小时（必填）
    pub unit: i32,
    /// 加班日期类型：1 工作日、2 休息日、3 节假日（必填）
    pub category: i32,
    /// 加班规则类型：0 仅记录、1 调休、2 加班费（必填）
    pub r#type: i32,
    /// 开始时间，格式 yyyy-MM-dd HH:mm:ss（必填）
    pub start_time: String,
    /// 结束时间，格式 yyyy-MM-dd HH:mm:ss（必填）
    pub end_time: String,
    /// 加班事由
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// 唯一幂等键，用于避免记录重复创建
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotent_id: Option<String>,
    /// 更正流程实例 ID（系统自动生成，写入时无需传入）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_process_id: Option<Vec<String>>,
    /// 撤销流程实例 ID（系统自动生成，写入时无需传入）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_process_id: Option<Vec<String>>,
    /// 发起流程实例 ID（系统自动生成，写入时无需传入）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_id: Option<Vec<String>>,
}

/// 出差记录（user_trip）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct UserTrip {
    /// 审批实例 ID（仅响应返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_id: Option<String>,
    /// 开始时间，格式 yyyy-MM-dd HH:mm:ss（必填）
    pub start_time: String,
    /// 结束时间，格式 yyyy-MM-dd HH:mm:ss（必填）
    pub end_time: String,
    /// 出差理由（必填）
    pub reason: String,
    /// 审批通过时间，格式 yyyy-MM-dd HH:mm:ss（必填）
    pub approve_pass_time: String,
    /// 审批申请时间，格式 yyyy-MM-dd HH:mm:ss（必填）
    pub approve_apply_time: String,
    /// 唯一幂等键，用于避免记录重复创建
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotent_id: Option<String>,
    /// 出发地（只有一个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure: Option<RegionPlace>,
    /// 目的地（可写多个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<RegionPlace>>,
    /// 交通工具（1 飞机，2 火车，3 汽车，4 高铁/动车，5 船，6 其他）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transportation: Option<Vec<i32>>,
    /// 出差类型：1 单程、2 往返
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trip_type: Option<i32>,
    /// 出差备注
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
    /// 更正流程实例 ID（系统自动生成，写入时无需传入）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correct_process_id: Option<Vec<String>>,
    /// 撤销流程实例 ID（系统自动生成，写入时无需传入）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_process_id: Option<Vec<String>>,
    /// 发起流程实例 ID（系统自动生成，写入时无需传入）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_id: Option<Vec<String>>,
}

/// 多语言名称展示，key 为 ch（中文）、en（英语）、ja（日语）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct I18nNames {
    /// 中文描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ch: Option<String>,
    /// 英语描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub en: Option<String>,
    /// 日语描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ja: Option<String>,
}

/// 地点信息（region_place）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct RegionPlace {
    /// 地理等级：l1 国家级、l2 省级、l3 市级、l4 区/县级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_level: Option<String>,
    /// 地理 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_id: Option<String>,
}

/// 时间范围（time_range）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct TimeRange {
    /// 开始时间（Unix 时间戳，秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time_stamp: Option<i64>,
    /// 结束时间（Unix 时间戳，秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time_stamp: Option<i64>,
}

/// 请假具体时间（time_range_list）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct TimeRangeList {
    /// 日期 day，如 20220501
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day: Option<i32>,
    /// 时间范围列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_ranges: Option<Vec<TimeRange>>,
}

/// 写入审批结果请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequestBody {
    /// 审批信息
    pub user_approval: UserApproval,
}

/// 写入审批结果响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponse {
    /// 审批信息（各记录含服务端生成的 approval_id）
    pub user_approval: UserApproval,
}

impl ApiResponseTrait for CreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use openlark_core::config::Config;
    use openlark_core::testing::prelude::TestConfigBuilder;
    use serde_json::json;

    #[test]
    fn test_create_request_body_nested_shape() {
        let body = CreateRequestBody {
            user_approval: UserApproval {
                user_id: "abd754f7".to_string(),
                date: "20210104".to_string(),
                leaves: Some(vec![UserLeave {
                    uniq_id: Some("6852582717813440527".to_string()),
                    unit: 1,
                    interval: 3600,
                    start_time: "2021-01-04 09:00:00".to_string(),
                    end_time: "2021-01-04 19:00:00".to_string(),
                    i18n_names: I18nNames {
                        ch: Some("中文描述".to_string()),
                        ..Default::default()
                    },
                    default_locale: "ch".to_string(),
                    reason: "家里有事".to_string(),
                    ..Default::default()
                }]),
                ..Default::default()
            },
        };
        let value = serde_json::to_value(&body).unwrap();
        assert_eq!(
            value,
            json!({
                "user_approval": {
                    "user_id": "abd754f7",
                    "date": "20210104",
                    "leaves": [{
                        "uniq_id": "6852582717813440527",
                        "unit": 1,
                        "interval": 3600,
                        "start_time": "2021-01-04 09:00:00",
                        "end_time": "2021-01-04 19:00:00",
                        "i18n_names": { "ch": "中文描述" },
                        "default_locale": "ch",
                        "reason": "家里有事"
                    }]
                }
            })
        );
        // 臆造的旧字段不应出现在请求体中
        let text = value.to_string();
        for legacy in ["approval_type", "approval_time", "\"result\"", "\"remark\""] {
            assert!(
                !text.contains(legacy),
                "请求体不应含官方不存在的字段 {legacy}: {text}"
            );
        }
    }

    #[test]
    fn test_create_overtime_work_serializes_type_field() {
        let overtime = UserOvertimeWork {
            duration: 1.5,
            unit: 1,
            category: 2,
            r#type: 1,
            start_time: "2021-01-09 09:00:00".to_string(),
            end_time: "2021-01-10 13:00:00".to_string(),
            ..Default::default()
        };
        let value = serde_json::to_value(&overtime).unwrap();
        assert_eq!(value["type"], json!(1));
        assert_eq!(value["duration"], json!(1.5));
    }

    #[test]
    fn test_create_request_builder_new() {
        let request = CreateRequest::new(
            TestConfigBuilder::new().build(),
            "employee_id".to_string(),
            UserApproval {
                user_id: "abd754f7".to_string(),
                date: "20210104".to_string(),
                ..Default::default()
            },
        );
        let _ = request;
    }
    /// 端到端：Builder→execute→Transport→mock→assert 嵌套请求体 + query 参数 + 响应解析。
    #[tokio::test]
    async fn test_attendance_v1_user_approval_create_returns_data_on_success() {
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/attendance/v1/user_approvals"))
            .and(query_param("employee_type", "employee_id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "user_approval": {
                        "user_id": "abd754f7",
                        "date": "20210104",
                        "leaves": [{
                            "approval_id": "6737202939523236113",
                            "uniq_id": "6852582717813440527",
                            "unit": 1,
                            "interval": 3600,
                            "start_time": "2021-01-04 09:00:00",
                            "end_time": "2021-01-04 19:00:00",
                            "i18n_names": { "ch": "中文描述" },
                            "default_locale": "ch",
                            "reason": "家里有事"
                        }]
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

        let data = CreateRequest::new(
            config,
            "employee_id".to_string(),
            UserApproval {
                user_id: "abd754f7".to_string(),
                date: "20210104".to_string(),
                leaves: Some(vec![UserLeave {
                    uniq_id: Some("6852582717813440527".to_string()),
                    unit: 1,
                    interval: 3600,
                    start_time: "2021-01-04 09:00:00".to_string(),
                    end_time: "2021-01-04 19:00:00".to_string(),
                    i18n_names: I18nNames {
                        ch: Some("中文描述".to_string()),
                        ..Default::default()
                    },
                    default_locale: "ch".to_string(),
                    reason: "家里有事".to_string(),
                    ..Default::default()
                }]),
                ..Default::default()
            },
        )
        .execute()
        .await
        .expect("attendance_v1_user_approval_create 应成功");

        assert_eq!(data.user_approval.user_id, "abd754f7");
        assert_eq!(data.user_approval.date, "20210104");
        let leaves = data.user_approval.leaves.expect("响应缺 leaves");
        assert_eq!(
            leaves[0].approval_id.as_deref(),
            Some("6737202939523236113")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/attendance/v1/user_approvals"
        );
        assert!(
            received[0]
                .url
                .query()
                .unwrap_or("")
                .contains("employee_type=employee_id"),
            "缺必填 query 参数 employee_type: {:?}",
            received[0].url.query()
        );
        let body = String::from_utf8(received[0].body.clone()).unwrap();
        assert!(
            body.contains("\"user_approval\""),
            "请求体缺 user_approval 嵌套包装: {body}"
        );
        assert!(body.contains("\"date\""), "请求体缺 date: {body}");
    }
}
