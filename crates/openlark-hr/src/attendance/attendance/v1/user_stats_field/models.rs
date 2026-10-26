//! 查询统计字段定义模型
//!
//! docPath: <https://open.feishu.cn/document/attendance-v1/user_stats_field/query>

use serde::{Deserialize, Serialize};

/// 查询统计字段定义请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequestBody {
    /// 语言类型（必填）：en-英语，ja-日语，zh-中文
    pub locale: String,
    /// 统计类型（必填）：daily-日度统计，month-月度统计
    pub stats_type: String,
    /// 开始时间，格式 yyyyMMdd（官方标注「无效字段」，仍必填）
    pub start_date: i32,
    /// 结束时间，格式 yyyyMMdd（时间间隔不超过 40 天；官方标注「无效字段」，仍必填）
    pub end_date: i32,
}

/// 查询统计字段定义响应
///
/// 官方响应 `data.user_stats_field` 为统计数据表头对象。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponse {
    /// 统计数据表头
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_stats_field: Option<UserStatsField>,
}

/// 统计数据表头（user_stats_field）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserStatsField {
    /// 统计类型：daily-日度统计，month-月度统计
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stats_type: Option<String>,
    /// 用户 ID，字段已废弃
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 字段列表
    #[serde(default)]
    pub fields: Vec<StatsFieldGroup>,
}

/// 统计字段分组（field）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StatsFieldGroup {
    /// 分组编号
    pub code: String,
    /// 分组名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 字段列表
    #[serde(default)]
    pub child_fields: Vec<StatsChildField>,
}

/// 统计子字段（child_field）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StatsChildField {
    /// 字段编号
    pub code: String,
    /// 字段名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 时间单位（该字段已停止使用）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_unit: Option<String>,
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_query_response_deserialize_official_example() {
        let response: QueryResponse = serde_json::from_value(serde_json::json!({
            "user_stats_field": {
                "stats_type": "month",
                "user_id": "",
                "fields": [
                    {
                        "code": "501",
                        "title": "Basic info",
                        "child_fields": [
                            {"code": "50103", "title": "Employee ID"}
                        ]
                    },
                    {
                        "code": "524",
                        "title": "Daily statistics",
                        "child_fields": [
                            {"code": "2021-03-16", "title": "2021-03-16 Tue", "time_unit": "Minute"}
                        ]
                    }
                ]
            }
        }))
        .expect("反序列化失败");

        let stats_field = response.user_stats_field.expect("user_stats_field 应存在");
        assert_eq!(stats_field.stats_type.as_deref(), Some("month"));
        assert_eq!(stats_field.fields.len(), 2);
        assert_eq!(stats_field.fields[0].code, "501");
        assert_eq!(
            stats_field.fields[0].child_fields[0].title.as_deref(),
            Some("Employee ID")
        );
        assert_eq!(
            stats_field.fields[1].child_fields[0].time_unit.as_deref(),
            Some("Minute")
        );
    }
}
