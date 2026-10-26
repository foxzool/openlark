//! 查询统计数据模型
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/user_stats_data/query>

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 查询统计数据请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequestBody {
    /// 语言类型，可选值：en、ja、zh
    pub locale: String,
    /// 统计类型，可选值：daily（日度统计）、month（月度统计）
    pub stats_type: String,
    /// 开始时间，格式 yyyyMMdd
    pub start_date: i32,
    /// 结束时间，格式 yyyyMMdd（时间间隔不超过 31 天）
    pub end_date: i32,
    /// 查询的用户 ID 列表，与 employee_type 对应（不超过 200 个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    /// 是否包含离职人员和转出人员，默认为 false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_history: Option<bool>,
    /// 是否只展示员工当前所属考勤组数据，默认为 false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_group_only: Option<bool>,
    /// 操作者的用户 ID，与 employee_type 对应
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// 用户统计数据项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserStatsDataItem {
    /// 用户 ID
    pub user_id: String,
    /// 用户姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// 日期，格式为 yyyy-MM-dd
    pub date: String,
    /// 考勤组 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// 考勤组名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_name: Option<String>,
    /// 基本信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub basic_info: Option<Value>,
    /// 出勤统计
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attendance_stats: Option<Value>,
    /// 异常统计
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abnormal_stats: Option<Value>,
    /// 请假统计
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leave_stats: Option<Value>,
    /// 加班统计
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overtime_stats: Option<Value>,
    /// 打卡时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub punch_time: Option<Value>,
    /// 考勤结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_result: Option<Value>,
    /// 自定义字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Value>,
}

/// 查询统计数据响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponse {
    /// 统计数据列表
    pub items: Vec<UserStatsDataItem>,
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
    fn test_user_stats_data_serialization_roundtrip() {
        let item = UserStatsDataItem {
            user_id: "u_1".to_string(),
            user_name: Some("张三".to_string()),
            date: "2024-01-01".to_string(),
            group_id: Some("g_1".to_string()),
            group_name: Some("研发组".to_string()),
            basic_info: Some(json!({"dept": "RD"})),
            attendance_stats: Some(json!({"normal": 1})),
            abnormal_stats: Some(json!({"late": 0})),
            leave_stats: Some(json!({"annual": 1})),
            overtime_stats: Some(json!({"hours": 2})),
            punch_time: Some(json!({"start": "09:00"})),
            check_result: Some(json!({"status": "normal"})),
            custom_fields: Some(json!({"level": "P7"})),
        };

        let text = serde_json::to_string(&item).expect("序列化失败");
        let parsed: UserStatsDataItem = serde_json::from_str(&text).expect("反序列化失败");
        assert_eq!(parsed.user_id, "u_1");
        assert_eq!(parsed.group_name.as_deref(), Some("研发组"));
    }
}
