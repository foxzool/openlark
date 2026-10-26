//! 考勤组相关模型
//!
//! 包含创建、删除、查询、搜索考勤组等 API 的请求和响应结构体

use serde::{Deserialize, Serialize};

// 创建考勤组相关模型
//
// 官方请求体为嵌套结构 `{"group": {...}, "operator_id": "..."}`，
// 必填 query 参数 `employee_type` 与 `dept_type`（见 create.rs）。

/// 创建考勤组请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGroupRequestBody {
    /// 考勤组信息（必填）
    pub group: AttendanceGroup,
    /// 操作人 uid，对应 employee_type；未操作考勤管理后台「API 接入」流程时为必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<String>,
}

/// 考勤组信息（创建/修改考勤组的 `group` 对象，字段对齐官方文档）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttendanceGroup {
    /// 考勤组 ID（仅修改时提供；不填则创建新考勤组）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// 考勤组名称（必填）
    pub group_name: String,
    /// 时区（必填）
    pub time_zone: String,
    /// 绑定的部门 ID，对应 dept_type（与「need_punch_members」同时使用时，以当前字段为准）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_dept_ids: Option<Vec<String>>,
    /// 排除的部门 ID（该字段已下线）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub except_dept_ids: Option<Vec<String>>,
    /// 绑定的用户 ID，对应 employee_type（与「need_punch_members」同时使用时，以当前字段为准）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_user_ids: Option<Vec<String>>,
    /// 排除的用户 ID（该字段已下线）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub except_user_ids: Option<Vec<String>>,
    /// 考勤主负责人 ID 列表，对应 employee_type（必填，需至少拥有考勤组管理员权限）
    pub group_leader_ids: Vec<String>,
    /// 考勤子负责人 ID 列表，对应 employee_type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_group_leader_ids: Option<Vec<String>>,
    /// 是否允许外勤打卡
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_out_punch: Option<bool>,
    /// 外勤打卡需审批（需要允许外勤打卡才能设置生效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_punch_need_approval: Option<bool>,
    /// 外勤打卡需审批，先打卡后审批（需要允许外勤打卡才能设置生效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_punch_need_post_approval: Option<bool>,
    /// 外勤打卡需填写备注（需要允许外勤打卡才能设置生效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_punch_need_remark: Option<bool>,
    /// 外勤打卡需拍照（需要允许外勤打卡才能设置生效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_punch_need_photo: Option<bool>,
    /// 外勤打卡允许员工隐藏详细地址（需要允许外勤打卡才能设置生效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_punch_allowed_hide_addr: Option<bool>,
    /// 外勤打卡允许微调地址（需要允许外勤打卡才能设置生效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_punch_allowed_adjust_addr: Option<bool>,
    /// 微调范围（米），默认 50
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjust_range: Option<i32>,
    /// 是否允许 PC 端打卡
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_pc_punch: Option<bool>,
    /// 是否限制补卡
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_remedy: Option<bool>,
    /// 是否限制补卡次数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remedy_limit: Option<bool>,
    /// 补卡次数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remedy_limit_count: Option<i32>,
    /// 是否限制补卡时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remedy_date_limit: Option<bool>,
    /// 几天内补卡
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remedy_date_num: Option<i32>,
    /// 允许缺卡补卡（需要允许补卡才能设置生效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_remedy_type_lack: Option<bool>,
    /// 允许迟到补卡（需要允许补卡才能设置生效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_remedy_type_late: Option<bool>,
    /// 允许早退补卡（需要允许补卡才能设置生效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_remedy_type_early: Option<bool>,
    /// 允许正常补卡（需要允许补卡才能设置生效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_remedy_type_normal: Option<bool>,
    /// 是否展示累计时长
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_cumulative_time: Option<bool>,
    /// 是否展示加班时长
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_over_time: Option<bool>,
    /// 是否隐藏员工打卡详情
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_staff_punch_time: Option<bool>,
    /// 是否隐藏打卡规则（仅灰度租户有效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hide_clock_in_rule: Option<bool>,
    /// 是否开启人脸识别打卡
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_punch: Option<bool>,
    /// 人脸识别打卡规则：1-每次打卡均需人脸识别，2-疑似作弊打卡时需要人脸识别
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_punch_cfg: Option<i32>,
    /// 人脸打卡规则：false-开启活体验证，true-动作验证（仅在 face_punch_cfg = 1 时有效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_live_need_action: Option<bool>,
    /// 人脸识别失败时是否允许普通拍照打卡
    #[serde(skip_serializing_if = "Option::is_none")]
    pub face_downgrade: Option<bool>,
    /// 人脸识别失败时是否允许替换基准图片
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replace_basic_pic: Option<bool>,
    /// 防作弊打卡配置（不传入时默认关闭/不更新，仅灰度租户有效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anti_cheat_punch_config: Option<AntiCheatPunchConfig>,
    /// 考勤机列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub machines: Option<Vec<AttendanceMachine>>,
    /// GPS 打卡的有效范围
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gps_range: Option<i32>,
    /// 地址列表（仅追加，不会覆盖之前的列表）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locations: Option<Vec<AttendanceLocation>>,
    /// 考勤类型（必填）：0-固定班制，1/2-排班制，3-自由班制
    pub group_type: i32,
    /// 班次 ID 列表（必填）。固定班制必须填够 7 个（周一至周日，休息日填 "0"）；
    /// 排班制或自由班制请传入空数组
    pub punch_day_shift_ids: Vec<String>,
    /// 自由班制配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub free_punch_cfg: Option<FreePunchCfg>,
    /// 国家日历 ID（必填）：0-不根据国家日历排休，1-中国大陆，2-美国，3-日本，4-印度，5-新加坡
    pub calendar_id: i32,
    /// 必须打卡的特殊日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_punch_special_days: Option<Vec<PunchSpecialDateShift>>,
    /// 无需打卡的特殊日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_need_punch_special_days: Option<Vec<PunchSpecialDateShift>>,
    /// 工作日不打卡是否记为缺卡
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_day_no_punch_as_lack: Option<bool>,
    /// 是否立即生效，默认 false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect_now: Option<bool>,
    /// 补卡周期类型：1-按月，2-自定义
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remedy_period_type: Option<i32>,
    /// 补卡自定义周期起始日期，范围 0-28 号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remedy_period_custom_date: Option<i32>,
    /// 打卡类型（位运算）：1-GPS，2-Wi-Fi，4-考勤机，8-IP；如设置多种传加和值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub punch_type: Option<i32>,
    /// 休息日打卡需审批。设为 true 时，休息日一天开始时间会被重置为 4:00
    #[serde(
        rename = "rest_clockIn_need_approval",
        skip_serializing_if = "Option::is_none"
    )]
    pub rest_clock_in_need_approval: Option<bool>,
    /// 每次打卡均需拍照
    #[serde(rename = "clockIn_need_photo", skip_serializing_if = "Option::is_none")]
    pub clock_in_need_photo: Option<bool>,
    /// 人员异动打卡设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_status_change: Option<MemberStatusChange>,
    /// 请假离岗或返岗是否需打卡
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leave_need_punch: Option<bool>,
    /// 请假离岗或返岗打卡规则
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leave_need_punch_cfg: Option<LeaveNeedPunchCfg>,
    /// 外出期间是否需打卡：0-无需打卡，1-需在上下班时间打卡，2-需在离岗前或返岗后打卡
    #[serde(skip_serializing_if = "Option::is_none")]
    pub go_out_need_punch: Option<i32>,
    /// 外出期间打卡规则
    #[serde(skip_serializing_if = "Option::is_none")]
    pub go_out_need_punch_cfg: Option<LeaveNeedPunchCfg>,
    /// 出差期间是否需打卡：0-无需打卡，1-需在上下班时间打卡，2-需在离岗前或返岗后打卡
    #[serde(skip_serializing_if = "Option::is_none")]
    pub travel_need_punch: Option<i32>,
    /// 出差期间打卡规则
    #[serde(skip_serializing_if = "Option::is_none")]
    pub travel_need_punch_cfg: Option<LeaveNeedPunchCfg>,
    /// 需要打卡的人员集合（仅当不传「bind_dept_ids」和「bind_user_ids」时，才会使用该字段）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_punch_members: Option<Vec<PunchMember>>,
    /// 是否允许保存有冲突人员的考勤组，默认 false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_auto_changes: Option<bool>,
    /// 新员工入职或人员异动时，符合条件的人员是否自动加入考勤组
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_change_auto_adjust: Option<bool>,
    /// 参与无需打卡的部门 ID 列表，对应 dept_type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_default_dept_ids: Option<Vec<String>>,
    /// 参与无需打卡的人员 ID 列表，对应 employee_type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bind_default_user_ids: Option<Vec<String>>,
    /// 加班打卡规则
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overtime_clock_cfg: Option<OvertimeClockCfg>,
    /// 节假日 ID（考勤组使用自定义节假日时传入）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub new_calendar_id: Option<String>,
    /// 定位不准时是否允许申请打卡
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_apply_punch: Option<bool>,
    /// 异常卡豁免配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clock_in_abnormal_settings: Option<ClockInAbnormalSettings>,
}

impl AttendanceGroup {
    /// 创建考勤组信息（必填字段构造器，可选字段为 pub 字段直接赋值）
    ///
    /// - `group_name`: 考勤组名称
    /// - `time_zone`: 时区（如 "Asia/Shanghai"）
    /// - `group_leader_ids`: 考勤主负责人 ID 列表，对应 employee_type
    /// - `group_type`: 考勤类型（0-固定班制，1/2-排班制，3-自由班制）
    /// - `punch_day_shift_ids`: 班次 ID 列表（固定班制填够 7 个，排班制/自由班制传空数组）
    /// - `calendar_id`: 国家日历 ID
    pub fn new(
        group_name: String,
        time_zone: String,
        group_leader_ids: Vec<String>,
        group_type: i32,
        punch_day_shift_ids: Vec<String>,
        calendar_id: i32,
    ) -> Self {
        Self {
            group_id: None,
            group_name,
            time_zone,
            bind_dept_ids: None,
            except_dept_ids: None,
            bind_user_ids: None,
            except_user_ids: None,
            group_leader_ids,
            sub_group_leader_ids: None,
            allow_out_punch: None,
            out_punch_need_approval: None,
            out_punch_need_post_approval: None,
            out_punch_need_remark: None,
            out_punch_need_photo: None,
            out_punch_allowed_hide_addr: None,
            out_punch_allowed_adjust_addr: None,
            adjust_range: None,
            allow_pc_punch: None,
            allow_remedy: None,
            remedy_limit: None,
            remedy_limit_count: None,
            remedy_date_limit: None,
            remedy_date_num: None,
            allow_remedy_type_lack: None,
            allow_remedy_type_late: None,
            allow_remedy_type_early: None,
            allow_remedy_type_normal: None,
            show_cumulative_time: None,
            show_over_time: None,
            hide_staff_punch_time: None,
            hide_clock_in_rule: None,
            face_punch: None,
            face_punch_cfg: None,
            face_live_need_action: None,
            face_downgrade: None,
            replace_basic_pic: None,
            anti_cheat_punch_config: None,
            machines: None,
            gps_range: None,
            locations: None,
            group_type,
            punch_day_shift_ids,
            free_punch_cfg: None,
            calendar_id,
            need_punch_special_days: None,
            no_need_punch_special_days: None,
            work_day_no_punch_as_lack: None,
            effect_now: None,
            remedy_period_type: None,
            remedy_period_custom_date: None,
            punch_type: None,
            rest_clock_in_need_approval: None,
            clock_in_need_photo: None,
            member_status_change: None,
            leave_need_punch: None,
            leave_need_punch_cfg: None,
            go_out_need_punch: None,
            go_out_need_punch_cfg: None,
            travel_need_punch: None,
            travel_need_punch_cfg: None,
            need_punch_members: None,
            save_auto_changes: None,
            org_change_auto_adjust: None,
            bind_default_dept_ids: None,
            bind_default_user_ids: None,
            overtime_clock_cfg: None,
            new_calendar_id: None,
            allow_apply_punch: None,
            clock_in_abnormal_settings: None,
        }
    }
}

/// 防作弊打卡配置（anti_cheat_punch_config）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AntiCheatPunchConfig {
    /// 是否拦截疑似作弊打卡（必填）；关闭时，其余防作弊开关都会关闭
    pub intercept_suspected_cheat_punch: bool,
    /// 是否校验疑似作弊软件打卡
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_cheat_software_punch: Option<bool>,
    /// 是否校验疑似他人代打卡
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_buddy_punch: Option<bool>,
    /// 是否校验疑似模拟 Wi-Fi 打卡（仅灰度租户有效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_simulate_wifi_punch: Option<bool>,
    /// 是否校验更换设备打卡
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_change_device_punch: Option<bool>,
    /// 同一考勤人员最多可绑定打卡设备数量上限（1-3，开启校验更换设备打卡时必填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_change_device_num: Option<i32>,
    /// 疑似作弊打卡时的处理方式（1-使用人脸识别打卡，2-仅记录疑似作弊信息；开启拦截时必填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspected_cheat_handle_method: Option<i32>,
}

/// 考勤机（machine）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AttendanceMachine {
    /// 考勤机序列号（必填）
    pub machine_sn: String,
    /// 考勤机名称（必填）
    pub machine_name: String,
}

/// 打卡地点（location）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AttendanceLocation {
    /// 地址名称（必填）
    pub location_name: String,
    /// 地址类型（必填）：1-GPS，2-Wi-Fi，8-IP
    pub location_type: i32,
    /// 地址纬度（需配合 gps_range 使用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    /// 地址经度（需配合 gps_range 使用）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    /// Wi-Fi 名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssid: Option<String>,
    /// Wi-Fi 的 MAC 地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bssid: Option<String>,
    /// 地图类型：1-高德，2-谷歌
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map_type: Option<i32>,
    /// 地址名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// IP 地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// 额外信息，例如运营商信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub feature: Option<String>,
    /// GPS 打卡的有效范围（历史无效字段）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gps_range: Option<i32>,
}

/// 自由班制配置（free_punch_cfg）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FreePunchCfg {
    /// 自由班制打卡开始时间，格式 x点x分（小时小于 10 不需补零，如 "7:00"）（必填）
    pub free_start_time: String,
    /// 自由班制打卡结束时间，格式 x点x分（必填）
    pub free_end_time: String,
    /// 打卡时间，7 位数字依次代表周一到周日，0 为不上班，1 为上班（必填）
    pub punch_day: i32,
    /// 工作日不打卡是否记为缺卡
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_day_no_punch_as_lack: Option<bool>,
    /// 工作日出勤是否需满足时长要求
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_hours_demand: Option<bool>,
    /// 每日工作时长（分钟），范围 \[0,1440\]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_hours: Option<i32>,
}

/// 必须打卡的特殊日期（punch_special_date_shift）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PunchSpecialDateShift {
    /// 打卡日期，格式 yyyyMMdd（必填）
    pub punch_day: i32,
    /// 班次 ID（必填）
    pub shift_id: String,
}

/// 人员异动打卡设置（member_status_change）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MemberStatusChange {
    /// 是否入职日上班无需打卡
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onboarding_on_no_need_punch: Option<bool>,
    /// 是否入职日下班无需打卡
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onboarding_off_no_need_punch: Option<bool>,
    /// 是否离职日上班无需打卡
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offboarding_on_no_need_punch: Option<bool>,
    /// 是否离职日下班无需打卡
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offboarding_off_no_need_punch: Option<bool>,
}

/// 请假/外出/出差期间打卡规则（leave_need_punch_cfg，单位：分钟）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LeaveNeedPunchCfg {
    /// 晚到超过多久记为迟到
    #[serde(skip_serializing_if = "Option::is_none")]
    pub late_minutes_as_late: Option<i32>,
    /// 晚到超过多久记为缺卡
    #[serde(skip_serializing_if = "Option::is_none")]
    pub late_minutes_as_lack: Option<i32>,
    /// 早走超过多久记为早退
    #[serde(skip_serializing_if = "Option::is_none")]
    pub early_minutes_as_early: Option<i32>,
    /// 早走超过多久记为缺卡
    #[serde(skip_serializing_if = "Option::is_none")]
    pub early_minutes_as_lack: Option<i32>,
    /// 中间离岗无需在离岗前或返岗后打卡（仅灰度租户有效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_during_shift: Option<bool>,
}

/// 需要打卡的人员集合（punch_member）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PunchMember {
    /// 圈人方式：0-无，1-全部，2-自定义
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_scope_type: Option<i32>,
    /// 圈人规则列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_group_list: Option<ScopeGroup>,
}

/// 圈人规则（scope_group）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScopeGroup {
    /// 类型：1-部门，2-人员，3-国家地区，4-员工类型，5-工作城市，6-职级，7-序列，8-职务，9-工时制度，100-自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_value_type: Option<i32>,
    /// 范围类型（是否包含）：1-包含，2-不包含，3-相等，4-小于等于，5-大于等于，6-大于，7-小于，8-不相等
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_type: Option<i32>,
    /// 圈人值列表（人员/部门类型不需要使用该字段）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub right: Option<Vec<ScopeValue>>,
    /// 部门/人员 ID 列表（根据 scope_value_type 判断为部门或人员）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_ids: Option<Vec<String>>,
    /// 企业版自定义字段唯一键 ID，需要从飞书人事获取（暂不支持）
    #[serde(rename = "custom_field_ID", skip_serializing_if = "Option::is_none")]
    pub custom_field_id: Option<String>,
    /// 企业版自定义字段对象类型（暂不支持）："Employment"-员工雇佣信息，"Person"-个人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_field_obj_type: Option<String>,
}

/// 圈人值（scope_value）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScopeValue {
    /// 标识 Key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// 名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// 加班打卡规则（overtime_clock_cfg）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OvertimeClockCfg {
    /// 是否允许在非打卡时段申请打卡（仅灰度租户有效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_punch_approval: Option<bool>,
    /// 加班开始和结束需打卡（仅灰度租户有效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_clock_over_time_start_and_end: Option<bool>,
}

/// 异常卡豁免配置（clock_in_abnormal_settings）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClockInAbnormalSettings {
    /// 在最晚下班打卡之前忽略异常卡（仅灰度租户有效）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_until_latest_clockout: Option<bool>,
}

/// 班次信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShiftInfo {
    /// 班次 ID
    pub shift_id: String,
    /// 班次名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift_name: Option<String>,
}

/// 工作日配置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WorkDayConfig {
    /// 星期几（1-7 表示周一到周日）
    pub day: i32,
    /// 是否工作
    pub is_work_day: bool,
    /// 班次 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift_id: Option<String>,
}

/// 加班信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OvertimeInfo {
    /// 加班类型
    /// - 0: 以加班申请为准
    /// - 1: 以审批为准
    /// - 2: 以打卡时间为准
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overtime_type: Option<i32>,
    /// 加班开始时间（分钟，从 0 点开始计算）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overtime_start_time: Option<i32>,
    /// 加班最小单位（分钟）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overtime_min_unit: Option<i32>,
    /// 是否扣除休息时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deduct_rest_time: Option<bool>,
}

/// 创建考勤组响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateGroupResponse {
    /// 考勤组 ID
    pub group_id: String,
}

// 删除考勤组相关模型

/// 删除考勤组响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteGroupResponse {
    /// 删除结果
    pub result: bool,
}

// 查询考勤组相关模型

/// 获取考勤组响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetGroupResponse {
    /// 考勤组信息
    pub group: GroupInfo,
}

/// 考勤组信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GroupInfo {
    /// 考勤组 ID
    pub group_id: String,
    /// 考勤组名称
    pub group_name: String,
    /// 考勤组类型
    /// - 0: 固定班制
    /// - 1: 排班制
    /// - 2: 自由班制
    pub group_type: i32,
    /// 参与考勤人员列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_list: Option<Vec<String>>,
    /// 无需考勤人员列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_user_list: Option<Vec<String>>,
    /// 考勤负责人列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager_list: Option<Vec<String>>,
    /// 考勤组绑定的部门列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept_list: Option<Vec<String>>,
    /// 考勤组绑定的班次列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift_list: Option<Vec<ShiftInfo>>,
    /// 是否允许外勤打卡
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_out_punch: Option<bool>,
    /// 外勤打卡是否需要审批
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_punch_need_approval: Option<bool>,
    /// 是否允许 PC 端打卡
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_pc_punch: Option<bool>,
    /// 是否需要拍照打卡
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_photo: Option<bool>,
    /// 拍照打卡类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_punch_type: Option<i32>,
    /// 是否允许补卡
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_remedy: Option<bool>,
    /// 补卡限制次数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remedy_limit: Option<i32>,
    /// 补卡限制周期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remedy_period: Option<i32>,
    /// 工作日设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_day_config: Option<Vec<WorkDayConfig>>,
    /// 加班规则配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overtime_info: Option<OvertimeInfo>,
}

// 列出考勤组相关模型

/// 列出考勤组响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListGroupResponse {
    /// 考勤组列表
    pub group_list: Vec<GroupListItem>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 考勤组列表项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GroupListItem {
    /// 考勤组 ID
    pub group_id: String,
    /// 考勤组名称
    pub group_name: String,
    /// 考勤组类型
    /// - 0: 固定班制
    /// - 1: 排班制
    /// - 2: 自由班制
    pub group_type: i32,
    /// 参与考勤人员数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_count: Option<i32>,
    /// 考勤负责人列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager_list: Option<Vec<String>>,
}

// 搜索考勤组相关模型

/// 搜索考勤组响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchGroupResponse {
    /// 考勤组列表
    pub group_list: Vec<GroupListItem>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_group_models_serialization_roundtrip() {
        let group = GroupInfo {
            group_id: "grp_001".to_string(),
            group_name: "研发组".to_string(),
            group_type: 0,
            user_list: Some(vec!["u_1".to_string()]),
            excluded_user_list: None,
            manager_list: Some(vec!["u_mgr".to_string()]),
            dept_list: Some(vec!["d_1".to_string()]),
            shift_list: Some(vec![ShiftInfo {
                shift_id: "s_1".to_string(),
                shift_name: Some("早班".to_string()),
            }]),
            allow_out_punch: Some(true),
            out_punch_need_approval: Some(false),
            allow_pc_punch: Some(true),
            need_photo: Some(false),
            photo_punch_type: Some(1),
            allow_remedy: Some(true),
            remedy_limit: Some(3),
            remedy_period: Some(30),
            work_day_config: Some(vec![WorkDayConfig {
                day: 1,
                is_work_day: true,
                shift_id: Some("s_1".to_string()),
            }]),
            overtime_info: Some(OvertimeInfo {
                overtime_type: Some(2),
                overtime_start_time: Some(30),
                overtime_min_unit: Some(15),
                deduct_rest_time: Some(true),
            }),
        };

        let json = serde_json::to_string(&group).expect("序列化失败");
        let parsed: GroupInfo = serde_json::from_str(&json).expect("反序列化失败");
        assert_eq!(parsed.group_name, "研发组");
        assert_eq!(parsed.group_type, 0);
    }

    #[test]
    fn test_group_response_deserialization() {
        let response: ListGroupResponse = serde_json::from_value(json!({
            "group_list": [{"group_id": "grp_1", "group_name": "A组", "group_type": 1}],
            "has_more": false
        }))
        .expect("反序列化失败");

        assert_eq!(response.group_list.len(), 1);
        assert_eq!(response.group_list[0].group_id, "grp_1");
    }
}
