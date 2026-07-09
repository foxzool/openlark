use serde::Deserialize;

/// 获取应用详情响应。
#[derive(Debug, Clone, Deserialize)]
pub struct GetAppResponse {
    /// 应用 ID。
    pub app_id: String,
    /// 应用名称。
    pub app_name: String,
    /// 应用类型。
    pub app_type: String,
    /// 应用描述。
    pub description: Option<String>,
}
