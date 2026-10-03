//! 平台服务常量

/// API 端点常量
pub mod endpoints {
    /// 应用引擎 API 基础路径
    pub const APP_ENGINE_BASE: &str = "/open-apis/app-engine";

    /// 目录服务 API 基础路径
    pub const DIRECTORY_BASE: &str = "/open-apis/directory";

    /// 系统管理 API 基础路径
    pub const ADMIN_BASE: &str = "/open-apis/admin";

    /// 妙搭平台 API 基础路径
    pub const SPARK_BASE: &str = "/open-apis/spark";

    /// 妙搭和飞书用户 ID 转换
    pub const SPARK_V1_DIRECTORY_USER_ID_CONVERT: &str =
        "/open-apis/spark/v1/directory/user/id_convert";

    /// 获取妙搭应用运营数据总览
    pub const SPARK_V1_APPS_ANALYTICS_OVERVIEW: &str =
        "/open-apis/spark/v1/apps/{app_id}/analytics/overview";

    /// 获取妙搭应用消耗 AI 额度
    pub const SPARK_V1_APPS_CREDIT_USAGE: &str = "/open-apis/spark/v1/apps/{app_id}/credit_usage";

    /// 获取妙搭应用运营数据趋势
    pub const SPARK_V1_APPS_QUERY_ANALYTICS_DATA: &str =
        "/open-apis/spark/v1/apps/{app_id}/query_analytics_data";

    /// 妙搭产品使用权限
    pub const SPARK_V1_AVAILABLE_SCOPE: &str = "/open-apis/spark/v1/available_scope";
}

/// 应用状态常量
pub mod app_status {
    /// 应用启用
    pub const ENABLED: &str = "enabled";
    /// 应用禁用
    pub const DISABLED: &str = "disabled";
    /// 应用归档
    pub const ARCHIVED: &str = "archived";
}
