//! 班次相关模型
//!
//! 包含创建、删除、查询、搜索班次等 API 的请求和响应结构体

use serde::{Deserialize, Serialize};

// 创建班次相关模型

/// 创建班次请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateShiftRequestBody {
    /// 班次 ID（修改班次时必填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shift_id: Option<String>,
    /// 班次名称（必填）
    pub shift_name: String,
    /// 班次类型（必填）
    /// - 0: 固定班次
    /// - 1: 弹性班次
    pub shift_type: i32,
    /// 弹性时长（分钟），弹性班次必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flexible_minutes: Option<i32>,
    /// 弹性打卡时间（分钟），弹性班次必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flexible_on_duty_time: Option<i32>,
    /// 弹性下班时间（分钟），弹性班次必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flexible_off_duty_time: Option<i32>,
    /// 打卡时段列表（必填）
    pub punch_times: Vec<PunchTime>,
    /// 迟到规则
    #[serde(skip_serializing_if = "Option::is_none")]
    pub late_rule: Option<LateRule>,
    /// 早退规则
    #[serde(skip_serializing_if = "Option::is_none")]
    pub early_leave_rule: Option<EarlyLeaveRule>,
    /// 午休规则
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rest_rule: Option<RestRule>,
    /// 加班规则
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overtime_rule: Option<OvertimeRule>,
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
    /// - 0: 不需要拍照
    /// - 1: 仅异常时拍照
    /// - 2: 每次打卡都拍照
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photo_punch_type: Option<i32>,
    /// 是否允许补卡
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_remedy: Option<bool>,
    /// 补卡限制次数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remedy_limit: Option<i32>,
    /// 补卡限制周期（单位：天）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remedy_period: Option<i32>,
}

/// 打卡时段
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PunchTime {
    /// 上班打卡时间（分钟，从 0 点开始计算）
    pub on_duty_time: i32,
    /// 下班打卡时间（分钟，从 0 点开始计算）
    pub off_duty_time: i32,
    /// 上班最早可打卡时间（分钟）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub earliest_on_duty_time: Option<i32>,
    /// 下班最晚可打卡时间（分钟）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_off_duty_time: Option<i32>,
    /// 上班打卡地点列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_duty_places: Option<Vec<PunchPlace>>,
    /// 下班打卡地点列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_duty_places: Option<Vec<PunchPlace>>,
    /// 上班打卡 Wi-Fi 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_duty_wifis: Option<Vec<PunchWifi>>,
    /// 下班打卡 Wi-Fi 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_duty_wifis: Option<Vec<PunchWifi>>,
}

/// 打卡地点
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PunchPlace {
    /// 地点名称
    pub place_name: String,
    /// 地点 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub place_id: Option<String>,
    /// 经度
    pub longitude: f64,
    /// 纬度
    pub latitude: f64,
    /// 打卡范围（米）
    pub punch_range: i32,
    /// 地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}

/// 打卡 Wi-Fi
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PunchWifi {
    /// Wi-Fi 名称
    pub wifi_name: String,
    /// Wi-Fi MAC 地址
    pub wifi_mac: String,
}

/// 迟到规则
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LateRule {
    /// 迟到类型
    /// - 0: 按次计算
    /// - 1: 按分钟计算
    pub late_type: i32,
    /// 允许迟到分钟数
    pub allow_minutes: i32,
    /// 严重迟到分钟数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serious_late_minutes: Option<i32>,
}

/// 早退规则
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EarlyLeaveRule {
    /// 早退类型
    /// - 0: 按次计算
    /// - 1: 按分钟计算
    pub early_leave_type: i32,
    /// 允许早退分钟数
    pub allow_minutes: i32,
    /// 严重早退分钟数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serious_early_leave_minutes: Option<i32>,
}

/// 午休规则
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RestRule {
    /// 午休开始时间（分钟）
    pub rest_begin_time: i32,
    /// 午休结束时间（分钟）
    pub rest_end_time: i32,
    /// 是否允许午休期间打卡
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_punch_during_rest: Option<bool>,
}

/// 加班规则
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OvertimeRule {
    /// 加班类型
    /// - 0: 以加班申请为准
    /// - 1: 以审批为准
    /// - 2: 以打卡时间为准
    pub overtime_type: i32,
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

/// 创建班次响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateShiftResponse {
    /// 班次 ID
    pub shift_id: String,
}

// 删除班次相关模型

/// 删除班次响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteShiftResponse {
    /// 删除结果
    pub result: bool,
}

// 查询班次相关模型

/// 获取班次响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetShiftResponse {
    /// 班次信息
    pub shift: ShiftInfo,
}

/// 班次信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShiftInfo {
    /// 班次 ID
    pub shift_id: String,
    /// 班次名称
    pub shift_name: String,
    /// 班次类型
    /// - 0: 固定班次
    /// - 1: 弹性班次
    pub shift_type: i32,
    /// 弹性时长（分钟）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flexible_minutes: Option<i32>,
    /// 弹性打卡时间（分钟）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flexible_on_duty_time: Option<i32>,
    /// 弹性下班时间（分钟）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flexible_off_duty_time: Option<i32>,
    /// 打卡时段列表
    pub punch_times: Vec<PunchTime>,
    /// 迟到规则
    #[serde(skip_serializing_if = "Option::is_none")]
    pub late_rule: Option<LateRule>,
    /// 早退规则
    #[serde(skip_serializing_if = "Option::is_none")]
    pub early_leave_rule: Option<EarlyLeaveRule>,
    /// 午休规则
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rest_rule: Option<RestRule>,
    /// 加班规则
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overtime_rule: Option<OvertimeRule>,
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
    /// 补卡限制周期（单位：天）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remedy_period: Option<i32>,
}

// 列出班次相关模型

/// 列出班次响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListShiftResponse {
    /// 班次列表
    pub shift_list: Vec<ShiftListItem>,
    /// 是否有更多数据
    pub has_more: bool,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 班次列表项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ShiftListItem {
    /// 班次 ID
    pub shift_id: String,
    /// 班次名称
    pub shift_name: String,
    /// 班次类型
    /// - 0: 固定班次
    /// - 1: 弹性班次
    pub shift_type: i32,
}

// 搜索班次相关模型

/// 搜索班次响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryShiftResponse {
    /// 班次列表
    pub shift_list: Vec<ShiftListItem>,
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
    fn test_shift_models_serialization_roundtrip() {
        let shift = ShiftInfo {
            shift_id: "shift_1".to_string(),
            shift_name: "白班".to_string(),
            shift_type: 0,
            flexible_minutes: None,
            flexible_on_duty_time: None,
            flexible_off_duty_time: None,
            punch_times: vec![PunchTime {
                on_duty_time: 540,
                off_duty_time: 1080,
                earliest_on_duty_time: Some(510),
                latest_off_duty_time: Some(1110),
                on_duty_places: Some(vec![PunchPlace {
                    place_name: "总部".to_string(),
                    place_id: Some("p_1".to_string()),
                    longitude: 121.4737,
                    latitude: 31.2304,
                    punch_range: 200,
                    address: Some("上海黄浦区".to_string()),
                }]),
                off_duty_places: None,
                on_duty_wifis: Some(vec![PunchWifi {
                    wifi_name: "Office-Wifi".to_string(),
                    wifi_mac: "AA:BB:CC:DD:EE:FF".to_string(),
                }]),
                off_duty_wifis: None,
            }],
            late_rule: Some(LateRule {
                late_type: 1,
                allow_minutes: 5,
                serious_late_minutes: Some(30),
            }),
            early_leave_rule: Some(EarlyLeaveRule {
                early_leave_type: 1,
                allow_minutes: 5,
                serious_early_leave_minutes: Some(30),
            }),
            rest_rule: Some(RestRule {
                rest_begin_time: 720,
                rest_end_time: 780,
                allow_punch_during_rest: Some(false),
            }),
            overtime_rule: Some(OvertimeRule {
                overtime_type: 2,
                overtime_start_time: Some(30),
                overtime_min_unit: Some(15),
                deduct_rest_time: Some(true),
            }),
            allow_out_punch: Some(true),
            out_punch_need_approval: Some(false),
            allow_pc_punch: Some(true),
            need_photo: Some(false),
            photo_punch_type: Some(1),
            allow_remedy: Some(true),
            remedy_limit: Some(2),
            remedy_period: Some(30),
        };

        let json = serde_json::to_string(&shift).expect("序列化失败");
        let parsed: ShiftInfo = serde_json::from_str(&json).expect("反序列化失败");
        assert_eq!(parsed.shift_id, "shift_1");
        assert_eq!(parsed.punch_times.len(), 1);
    }

    #[test]
    fn test_shift_response_deserialization() {
        let response: ListShiftResponse = serde_json::from_value(json!({
            "shift_list": [{"shift_id": "shift_2", "shift_name": "夜班", "shift_type": 1}],
            "has_more": true,
            "page_token": "next"
        }))
        .expect("反序列化失败");

        assert!(response.has_more);
        assert_eq!(response.page_token.as_deref(), Some("next"));
    }
}
