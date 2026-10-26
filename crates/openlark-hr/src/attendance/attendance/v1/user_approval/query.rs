//! 获取审批数据
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/user_approval/query>

use super::create::{I18nNames, RegionPlace};
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, validate_required_list,
};
use serde::{Deserialize, Serialize};

/// 获取审批数据请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 员工 ID 类型（必填，query 参数）：employee_id / employee_no / open_id
    employee_type: String,
    /// 用户 ID 列表（必填）
    user_ids: Vec<String>,
    /// 查询起始日期 `yyyyMMdd`（必填，不能超过当天 +1）
    check_date_from: i32,
    /// 查询结束日期 `yyyyMMdd`（必填，与 `check_date_from` 间隔不超过 30 天）
    check_date_to: i32,
    /// 查询依据的时间类型（可选，默认 `PeriodTime`）
    check_date_type: Option<String>,
    /// 查询状态（可选）
    status: Option<i32>,
    /// 查询起始时间（可选）
    check_time_from: Option<String>,
    /// 查询结束时间（可选）
    check_time_to: Option<String>,
    /// 配置信息
    config: Config,
}

impl QueryRequest {
    /// 创建请求
    ///
    /// - `employee_type`: 员工 ID 类型（query 参数，必填），user_ids 需与其取值一致
    pub fn new(
        config: Config,
        employee_type: String,
        user_ids: Vec<String>,
        check_date_from: i32,
        check_date_to: i32,
    ) -> Self {
        Self {
            employee_type,
            user_ids,
            check_date_from,
            check_date_to,
            check_date_type: None,
            status: None,
            check_time_from: None,
            check_time_to: None,
            config,
        }
    }

    /// 设置查询依据的时间类型（可选）
    pub fn check_date_type(mut self, check_date_type: String) -> Self {
        self.check_date_type = Some(check_date_type);
        self
    }

    /// 设置查询状态（可选）
    pub fn status(mut self, status: i32) -> Self {
        self.status = Some(status);
        self
    }

    /// 设置查询起始时间（可选）
    pub fn check_time_from(mut self, check_time_from: String) -> Self {
        self.check_time_from = Some(check_time_from);
        self
    }

    /// 设置查询结束时间（可选）
    pub fn check_time_to(mut self, check_time_to: String) -> Self {
        self.check_time_to = Some(check_time_to);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<QueryResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.employee_type, "employee_type 不能为空");
        validate_required_list!(self.user_ids, 50, "user_ids 不能为空且不能超过 50 个");
        if self.check_date_to < self.check_date_from {
            return Err(openlark_core::error::validation_error(
                "日期范围无效",
                "check_date_to 不能早于 check_date_from",
            ));
        }

        // 2. 构建端点并附加必填 query 参数 employee_type
        let api_endpoint = AttendanceApiV1::UserApprovalQuery;
        let request = ApiRequest::<QueryResponse>::post(api_endpoint.to_url())
            .query("employee_type", &self.employee_type);

        // 3. 构建请求体
        let request_body = QueryRequestBody {
            user_ids: self.user_ids,
            check_date_from: self.check_date_from,
            check_date_to: self.check_date_to,
            check_date_type: self.check_date_type,
            status: self.status,
            check_time_from: self.check_time_from,
            check_time_to: self.check_time_to,
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
            "获取审批数据响应数据为空",
        )
        .await
    }
}

/// 获取审批数据请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequestBody {
    /// 用户 ID 列表
    pub user_ids: Vec<String>,
    /// 查询起始日期 `yyyyMMdd`
    pub check_date_from: i32,
    /// 查询结束日期 `yyyyMMdd`
    pub check_date_to: i32,
    /// 查询依据的时间类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_date_type: Option<String>,
    /// 查询状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 查询起始时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_time_from: Option<String>,
    /// 查询结束时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_time_to: Option<String>,
}

/// 获取审批数据响应
///
/// 官方响应体表格完整给出 `data.user_approvals[]` 的全部字段，
/// 与 create（写入审批结果）共享 schema 但方向不同，此处为响应专用建模：
/// 除 `user_id` 外全部字段 `Option` + `serde(default)`，服务端缺字段时不会硬解析失败。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponse {
    /// 审批结果列表（user_approval[]）
    #[serde(default)]
    pub user_approvals: Vec<UserApproval>,
}

/// 审批信息（响应体 `user_approval`）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserApproval {
    /// 审批用户 ID，类型与 employee_type 的取值一致
    pub user_id: String,
    /// 审批作用日期，格式 yyyyMMdd
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
    /// 外出信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outs: Option<Vec<UserOut>>,
    /// 请假信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub leaves: Option<Vec<UserLeave>>,
    /// 加班信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overtime_works: Option<Vec<UserOvertimeWork>>,
    /// 出差信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trips: Option<Vec<UserTrip>>,
}

/// 外出记录（响应体 `user_out`）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserOut {
    /// 审批实例 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_id: Option<String>,
    /// 外出类型唯一 ID，代表一种外出类型，长度小于 14
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uniq_id: Option<String>,
    /// 外出时长单位：1 天、2 小时、3 半天、4 半小时
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<i32>,
    /// 关联审批单外出时长，单位为秒，与 unit 无关
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    /// 开始时间，格式 yyyy-MM-dd HH:mm:ss
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 结束时间，格式 yyyy-MM-dd HH:mm:ss
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 外出多语言展示
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_names: Option<I18nNames>,
    /// 默认语言类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_locale: Option<String>,
    /// 外出理由
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// 审批通过时间，格式 yyyy-MM-dd HH:mm:ss
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approve_pass_time: Option<String>,
    /// 审批申请时间，格式 yyyy-MM-dd HH:mm:ss
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approve_apply_time: Option<String>,
    /// 唯一幂等键
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotent_id: Option<String>,
    /// 更正流程实例 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correct_process_id: Option<Vec<String>>,
    /// 撤销流程实例 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancel_process_id: Option<Vec<String>>,
    /// 发起流程实例 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_id: Option<Vec<String>>,
}

/// 请假记录（响应体 `user_leave`）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserLeave {
    /// 审批实例 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_id: Option<String>,
    /// 假期类型唯一 ID，代表一种假期类型，长度小于 14
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uniq_id: Option<String>,
    /// 假期时长单位：1 天、2 小时、3 半天、4 半小时
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<i32>,
    /// 关联审批单休假时长，单位为秒，与 unit 无关
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<i64>,
    /// 开始时间，格式 yyyy-MM-dd HH:mm:ss
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 结束时间，格式 yyyy-MM-dd HH:mm:ss
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 假期多语言展示
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub i18n_names: Option<I18nNames>,
    /// 默认语言类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_locale: Option<String>,
    /// 请假理由
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// 审批通过时间，格式 yyyy-MM-dd HH:mm:ss
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approve_pass_time: Option<String>,
    /// 审批申请时间，格式 yyyy-MM-dd HH:mm:ss
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approve_apply_time: Option<String>,
    /// 唯一幂等键
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotent_id: Option<String>,
}

/// 加班记录（响应体 `user_overtime_work`）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserOvertimeWork {
    /// 审批实例 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_id: Option<String>,
    /// 加班时长
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
    /// 加班时长单位：1 天、2 小时、3 半天、4 半小时
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unit: Option<i32>,
    /// 加班日期类型：1 工作日、2 休息日、3 节假日
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<i32>,
    /// 加班规则类型：0 不关联加班规则、1 调休、2 加班费、3 关联加班规则
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub overtime_type: Option<i32>,
    /// 开始时间，格式 yyyy-MM-dd HH:mm:ss
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 结束时间，格式 yyyy-MM-dd HH:mm:ss
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 加班事由
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// 唯一幂等键
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotent_id: Option<String>,
    /// 更正流程实例 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correct_process_id: Option<Vec<String>>,
    /// 撤销流程实例 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancel_process_id: Option<Vec<String>>,
    /// 发起流程实例 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_id: Option<Vec<String>>,
}

/// 出差记录（响应体 `user_trip`）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserTrip {
    /// 审批实例 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_id: Option<String>,
    /// 开始时间，格式 yyyy-MM-dd HH:mm:ss
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 结束时间，格式 yyyy-MM-dd HH:mm:ss
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 出差理由
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// 审批通过时间，格式 yyyy-MM-dd HH:mm:ss
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approve_pass_time: Option<String>,
    /// 审批申请时间，格式 yyyy-MM-dd HH:mm:ss
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approve_apply_time: Option<String>,
    /// 唯一幂等键
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub idempotent_id: Option<String>,
    /// 更正流程实例 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correct_process_id: Option<Vec<String>>,
    /// 撤销流程实例 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancel_process_id: Option<Vec<String>>,
    /// 发起流程实例 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub process_id: Option<Vec<String>>,
    /// 出发地（只有一个）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub departure: Option<RegionPlace>,
    /// 目的地（可写多个）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destinations: Option<Vec<RegionPlace>>,
    /// 交通工具（1 飞机，2 火车，3 汽车，4 高铁/动车，5 船，6 其他）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transportation: Option<Vec<i32>>,
    /// 出差类型：1 单程、2 往返
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trip_type: Option<i32>,
    /// 出差备注
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remarks: Option<String>,
}

impl ApiResponseTrait for QueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;
    use openlark_core::testing::prelude::TestConfigBuilder;

    #[test]
    fn test_query_request_builder_new() {
        let request = QueryRequest::new(
            TestConfigBuilder::new().build(),
            "employee_id".to_string(),
            vec!["abd754f7".to_string()],
            20_190_817,
            20_190_820,
        );
        let _ = request;
    }

    /// 端到端：Builder→execute→Transport→mock→assert 请求体字段对齐飞书官网 schema。
    #[tokio::test]
    async fn test_attendance_v1_user_approval_query_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 官方响应示例形状：data.user_approvals[]（typed 元素，此处取 outs 分支）
        Mock::given(method("POST"))
            .and(path("/open-apis/attendance/v1/user_approvals/query"))
            .and(query_param("employee_type", "employee_id"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "user_approvals": [
                    {
                        "user_id": "abd754f7",
                        "date": "20210104",
                        "outs": [{
                            "approval_id": "6737202939523236113",
                            "uniq_id": "9496E43696967658A512969523E89870",
                            "unit": 1,
                            "interval": 8,
                            "start_time": "2021-01-04 09:00:00",
                            "end_time": "2021-01-04 19:00:00",
                            "i18n_names": { "ch": "中文描述", "en": "English description", "ja": "日本語の説明" },
                            "default_locale": "ch",
                            "reason": "外出办事"
                        }]
                    }
                ] }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = QueryRequest::new(
            config,
            "employee_id".to_string(),
            vec!["abd754f7".to_string()],
            20_190_817,
            20_190_820,
        )
        .execute()
        .await
        .expect("attendance_v1_user_approval_query 应成功");

        assert_eq!(data.user_approvals.len(), 1);
        let approval = &data.user_approvals[0];
        assert_eq!(approval.user_id, "abd754f7");
        assert_eq!(approval.date.as_deref(), Some("20210104"));
        let outs = approval.outs.as_ref().expect("响应缺 outs");
        assert_eq!(outs.len(), 1);
        assert_eq!(outs[0].approval_id.as_deref(), Some("6737202939523236113"));
        assert_eq!(
            outs[0].uniq_id.as_deref(),
            Some("9496E43696967658A512969523E89870")
        );
        assert_eq!(outs[0].unit, Some(1));
        assert_eq!(outs[0].interval, Some(8));
        assert_eq!(
            outs[0].i18n_names.as_ref().and_then(|n| n.ch.as_deref()),
            Some("中文描述")
        );
        assert_eq!(outs[0].reason.as_deref(), Some("外出办事"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/attendance/v1/user_approvals/query"
        );
        let query = received[0].url.query().unwrap_or_default();
        assert!(
            query.contains("employee_type=employee_id"),
            "缺必填 query 参数 employee_type: {query}"
        );
        let body = String::from_utf8(received[0].body.clone()).unwrap();
        assert!(
            body.contains("\"check_date_from\""),
            "请求体缺 check_date_from: {body}"
        );
        assert!(body.contains("\"user_ids\""), "请求体缺 user_ids: {body}");
        assert!(
            !body.contains("\"start_time\""),
            "请求体不应含旧字段 start_time: {body}"
        );
    }
}
