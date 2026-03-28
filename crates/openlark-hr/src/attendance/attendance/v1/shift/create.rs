//! 创建班次
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/shift/create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};

use super::models::{
    CreateShiftRequestBody, CreateShiftResponse, EarlyLeaveRule, LateRule, OvertimeRule, PunchTime,
    RestRule,
};

/// 创建班次请求
#[derive(Debug, Clone)]
pub struct CreateShiftRequest {
    /// 班次 ID（修改班次时必填）
    shift_id: Option<String>,
    /// 班次名称（必填）
    shift_name: String,
    /// 班次类型（必填）
    /// - 0: 固定班次
    /// - 1: 弹性班次
    shift_type: i32,
    /// 弹性时长（分钟），弹性班次必填
    flexible_minutes: Option<i32>,
    /// 弹性打卡时间（分钟），弹性班次必填
    flexible_on_duty_time: Option<i32>,
    /// 弹性下班时间（分钟），弹性班次必填
    flexible_off_duty_time: Option<i32>,
    /// 打卡时段列表（必填）
    punch_times: Vec<PunchTime>,
    /// 迟到规则
    late_rule: Option<LateRule>,
    /// 早退规则
    early_leave_rule: Option<EarlyLeaveRule>,
    /// 午休规则
    rest_rule: Option<RestRule>,
    /// 加班规则
    overtime_rule: Option<OvertimeRule>,
    /// 是否允许外勤打卡
    allow_out_punch: Option<bool>,
    /// 外勤打卡是否需要审批
    out_punch_need_approval: Option<bool>,
    /// 是否允许 PC 端打卡
    allow_pc_punch: Option<bool>,
    /// 是否需要拍照打卡
    need_photo: Option<bool>,
    /// 拍照打卡类型
    photo_punch_type: Option<i32>,
    /// 是否允许补卡
    allow_remedy: Option<bool>,
    /// 补卡限制次数
    remedy_limit: Option<i32>,
    /// 补卡限制周期（单位：天）
    remedy_period: Option<i32>,
    /// 配置信息
    config: Config,
}

impl CreateShiftRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            shift_id: None,
            shift_name: String::new(),
            shift_type: 0,
            flexible_minutes: None,
            flexible_on_duty_time: None,
            flexible_off_duty_time: None,
            punch_times: Vec::new(),
            late_rule: None,
            early_leave_rule: None,
            rest_rule: None,
            overtime_rule: None,
            allow_out_punch: None,
            out_punch_need_approval: None,
            allow_pc_punch: None,
            need_photo: None,
            photo_punch_type: None,
            allow_remedy: None,
            remedy_limit: None,
            remedy_period: None,
            config,
        }
    }

    /// 设置班次 ID（修改班次时必填）
    pub fn shift_id(mut self, shift_id: String) -> Self {
        self.shift_id = Some(shift_id);
        self
    }

    /// 设置班次名称（必填）
    pub fn shift_name(mut self, shift_name: String) -> Self {
        self.shift_name = shift_name;
        self
    }

    /// 设置班次类型（必填）
    /// - 0: 固定班次
    /// - 1: 弹性班次
    pub fn shift_type(mut self, shift_type: i32) -> Self {
        self.shift_type = shift_type;
        self
    }

    /// 设置弹性时长（分钟），弹性班次必填
    pub fn flexible_minutes(mut self, flexible_minutes: i32) -> Self {
        self.flexible_minutes = Some(flexible_minutes);
        self
    }

    /// 设置弹性打卡时间（分钟），弹性班次必填
    pub fn flexible_on_duty_time(mut self, flexible_on_duty_time: i32) -> Self {
        self.flexible_on_duty_time = Some(flexible_on_duty_time);
        self
    }

    /// 设置弹性下班时间（分钟），弹性班次必填
    pub fn flexible_off_duty_time(mut self, flexible_off_duty_time: i32) -> Self {
        self.flexible_off_duty_time = Some(flexible_off_duty_time);
        self
    }

    /// 设置打卡时段列表（必填）
    pub fn punch_times(mut self, punch_times: Vec<PunchTime>) -> Self {
        self.punch_times = punch_times;
        self
    }

    /// 添加单个打卡时段
    pub fn add_punch_time(mut self, punch_time: PunchTime) -> Self {
        self.punch_times.push(punch_time);
        self
    }

    /// 设置迟到规则
    pub fn late_rule(mut self, late_rule: LateRule) -> Self {
        self.late_rule = Some(late_rule);
        self
    }

    /// 设置早退规则
    pub fn early_leave_rule(mut self, early_leave_rule: EarlyLeaveRule) -> Self {
        self.early_leave_rule = Some(early_leave_rule);
        self
    }

    /// 设置午休规则
    pub fn rest_rule(mut self, rest_rule: RestRule) -> Self {
        self.rest_rule = Some(rest_rule);
        self
    }

    /// 设置加班规则
    pub fn overtime_rule(mut self, overtime_rule: OvertimeRule) -> Self {
        self.overtime_rule = Some(overtime_rule);
        self
    }

    /// 设置是否允许外勤打卡
    pub fn allow_out_punch(mut self, allow_out_punch: bool) -> Self {
        self.allow_out_punch = Some(allow_out_punch);
        self
    }

    /// 设置外勤打卡是否需要审批
    pub fn out_punch_need_approval(mut self, out_punch_need_approval: bool) -> Self {
        self.out_punch_need_approval = Some(out_punch_need_approval);
        self
    }

    /// 设置是否允许 PC 端打卡
    pub fn allow_pc_punch(mut self, allow_pc_punch: bool) -> Self {
        self.allow_pc_punch = Some(allow_pc_punch);
        self
    }

    /// 设置是否需要拍照打卡
    pub fn need_photo(mut self, need_photo: bool) -> Self {
        self.need_photo = Some(need_photo);
        self
    }

    /// 设置拍照打卡类型
    /// - 0: 不需要拍照
    /// - 1: 仅异常时拍照
    /// - 2: 每次打卡都拍照
    pub fn photo_punch_type(mut self, photo_punch_type: i32) -> Self {
        self.photo_punch_type = Some(photo_punch_type);
        self
    }

    /// 设置是否允许补卡
    pub fn allow_remedy(mut self, allow_remedy: bool) -> Self {
        self.allow_remedy = Some(allow_remedy);
        self
    }

    /// 设置补卡限制次数
    pub fn remedy_limit(mut self, remedy_limit: i32) -> Self {
        self.remedy_limit = Some(remedy_limit);
        self
    }

    /// 设置补卡限制周期（单位：天）
    pub fn remedy_period(mut self, remedy_period: i32) -> Self {
        self.remedy_period = Some(remedy_period);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateShiftResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateShiftResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.shift_name.trim(), "班次名称不能为空");
        if self.punch_times.is_empty() {
            return Err(openlark_core::error::validation_error(
                "打卡时段不能为空",
                "至少需要设置一个打卡时段",
            ));
        }

        // 2. 构建端点
        let api_endpoint = AttendanceApiV1::ShiftCreate;
        let request = ApiRequest::<CreateShiftResponse>::post(api_endpoint.to_url());

        // 3. 序列化请求体
        let request_body = CreateShiftRequestBody {
            shift_id: self.shift_id,
            shift_name: self.shift_name,
            shift_type: self.shift_type,
            flexible_minutes: self.flexible_minutes,
            flexible_on_duty_time: self.flexible_on_duty_time,
            flexible_off_duty_time: self.flexible_off_duty_time,
            punch_times: self.punch_times,
            late_rule: self.late_rule,
            early_leave_rule: self.early_leave_rule,
            rest_rule: self.rest_rule,
            overtime_rule: self.overtime_rule,
            allow_out_punch: self.allow_out_punch,
            out_punch_need_approval: self.out_punch_need_approval,
            allow_pc_punch: self.allow_pc_punch,
            need_photo: self.need_photo,
            photo_punch_type: self.photo_punch_type,
            allow_remedy: self.allow_remedy,
            remedy_limit: self.remedy_limit,
            remedy_period: self.remedy_period,
        };
        let request = request.body(serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {}", e),
            )
        })?);

        // 4. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 5. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "创建班次响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

impl ApiResponseTrait for CreateShiftResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use openlark_core::testing::prelude::TestConfigBuilder;

    #[test]
    fn test_create_shift_request_builder_new() {
        let request =
            CreateShiftRequest::new(TestConfigBuilder::new().build()).shift_id("test".to_string());
        let _ = request;
    }

    #[test]
    fn test_create_shift_request_validation_fails_on_default_request() {
        let request = CreateShiftRequest::new(TestConfigBuilder::new().build());
        let rt = tokio::runtime::Runtime::new().expect("创建 tokio runtime 失败");
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
    }
}
