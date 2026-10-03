//! 获取应用运营数据
//!
//! docPath: <https://open.feishu.cn/document/apaas-v1/tenant_app_metrics/query>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};

use crate::common::api_utils::serialize_params;

/// 应用运营数据查询端点。
pub const TENANT_APP_METRICS_QUERY: &str = "/open-apis/apaas/v1/tenant_app_metrics/query";

/// 获取应用运营数据请求。
#[derive(Debug, Clone)]
pub struct TenantAppMetricsQueryRequest {
    config: Config,
}

impl TenantAppMetricsQueryRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求。
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<serde_json::Value> {
        if body.is_null() {
            return Err(openlark_core::error::validation_error(
                "body",
                "请求体不能为空",
            ));
        }

        let req: ApiRequest<serde_json::Value> = ApiRequest::post(TENANT_APP_METRICS_QUERY)
            .body(serialize_params(&body, "获取应用运营数据")?);

        Transport::request_typed(req, &self.config, Some(option), "获取应用运营数据").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tenant_app_metrics_query_endpoint() {
        assert_eq!(
            TENANT_APP_METRICS_QUERY,
            "/open-apis/apaas/v1/tenant_app_metrics/query"
        );
    }
}
