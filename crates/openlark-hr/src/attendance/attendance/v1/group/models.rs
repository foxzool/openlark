//! 考勤组相关模型
//!
//! 包含创建、删除、查询、搜索考勤组等 API 的请求和响应结构体

use serde::{Deserialize, Serialize};

// 创建考勤组相关模型

/// 创建考勤组请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGroupRequestBody {
    /// 考勤组 ID（修改考勤组时必填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// 考勤组名称（必填）
    pub group_name: String,
    /// 考勤组类型（必填）
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
    /// 考勤组绑定的班次列表（固定班制必填）
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
    /// 工作日设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_day_config: Option<Vec<WorkDayConfig>>,
    /// 加班规则配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overtime_info: Option<OvertimeInfo>,
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
