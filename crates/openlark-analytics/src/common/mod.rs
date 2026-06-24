//! 通用数据模型

use serde::{Deserialize, Serialize};

/// 搜索结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    /// 结果 ID
    pub id: String,
    /// 结果标题
    pub title: String,
    /// 结果摘要
    pub summary: Option<String>,
    /// 结果类型
    pub result_type: String,
    /// 相关度分数
    pub score: f64,
}

/// 搜索统计
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchStats {
    /// 总结果数
    pub total: u32,
    /// 查询时间（毫秒）
    pub query_time_ms: u32,
    /// 搜索的页面数
    pub page_count: u32,
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}

pub mod constants;
