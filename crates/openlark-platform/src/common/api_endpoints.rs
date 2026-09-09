//! OpenLark Platform API 端点定义
//!
//! 提供统一的 API 端点管理和 URL 生成功能

/// Admin V1 API 端点枚举
#[derive(Debug, Clone)]
pub enum AdminApiV1 {
    /// 创建勋章
    CreateBadge,
    /// 获取勋章列表
    ListBadge,
}

impl AdminApiV1 {
    /// 获取对应的 API 路径
    pub fn path(&self) -> &'static str {
        match self {
            AdminApiV1::CreateBadge => "/open-apis/admin/v1/badges",
            AdminApiV1::ListBadge => "/open-apis/admin/v1/badges",
        }
    }
}

/// Spark v1 端点。
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SparkApiV1 {
    /// 获取妙搭应用运营数据总览。
    /// GET /open-apis/spark/v1/apps/:app_id/analytics/overview
    AppsAnalyticsOverview(String),
    /// 获取妙搭应用消耗 AI 额度。
    /// GET /open-apis/spark/v1/apps/:app_id/credit_usage
    AppsCreditUsage(String),
    /// 获取妙搭应用运营数据趋势。
    /// POST /open-apis/spark/v1/apps/:app_id/query_analytics_data
    AppsQueryAnalyticsData(String),
    /// 妙搭产品使用权限。
    /// GET/PUT /open-apis/spark/v1/available_scope
    AvailableScope,
}

impl SparkApiV1 {
    /// 生成对应 URL。
    pub fn to_url(&self) -> String {
        match self {
            Self::AppsAnalyticsOverview(app_id) => {
                format!("/open-apis/spark/v1/apps/{app_id}/analytics/overview")
            }
            Self::AppsCreditUsage(app_id) => {
                format!("/open-apis/spark/v1/apps/{app_id}/credit_usage")
            }
            Self::AppsQueryAnalyticsData(app_id) => {
                format!("/open-apis/spark/v1/apps/{app_id}/query_analytics_data")
            }
            Self::AvailableScope => "/open-apis/spark/v1/available_scope".to_string(),
        }
    }
}

/// 模块导出
pub mod prelude {
    pub use super::{AdminApiV1, SparkApiV1};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adminapiv1_path_coverage() {
        let paths = [AdminApiV1::CreateBadge.path(), AdminApiV1::ListBadge.path()];
        assert!(paths.iter().all(|path| path.starts_with("/open-apis/")));
    }

    #[test]
    fn test_spark_api_v1_urls() {
        assert_eq!(
            SparkApiV1::AppsAnalyticsOverview("app_1".into()).to_url(),
            "/open-apis/spark/v1/apps/app_1/analytics/overview"
        );
        assert_eq!(
            SparkApiV1::AppsCreditUsage("app_1".into()).to_url(),
            "/open-apis/spark/v1/apps/app_1/credit_usage"
        );
        assert_eq!(
            SparkApiV1::AppsQueryAnalyticsData("app_1".into()).to_url(),
            "/open-apis/spark/v1/apps/app_1/query_analytics_data"
        );
        assert_eq!(
            SparkApiV1::AvailableScope.to_url(),
            "/open-apis/spark/v1/available_scope"
        );
    }
}
