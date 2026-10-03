//! 查询统计数据模型
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/user_stats_data/query>

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 查询统计数据请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequestBody {
    /// 查询的起始日期，格式为 yyyy-MM-dd
    pub start_date: String,
    /// 查询的结束日期，格式为 yyyy-MM-dd
    pub end_date: String,
    /// 查询的用户 ID 列表，最多支持 50 个用户
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    /// 查询的考勤组 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    /// 用户 ID 类型，可选值：open_id、union_id、user_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 统计类型，可选值：day（日度统计）、month（月度统计）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stats_type: Option<String>,
    /// 分页大小，默认值为 100，最大值为 200
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记，用于获取下一页数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
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
