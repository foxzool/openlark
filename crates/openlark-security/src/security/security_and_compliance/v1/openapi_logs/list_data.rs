//! 获取 OpenAPI 审计日志数据
//!
//! docPath: https://open.feishu.cn/document/server-docs/security_and_compliance-v1/openapi_log-list
//!
//! 文档核对：`POST /open-apis/security_and_compliance/v1/openapi_logs/list_data`，
//! 过滤条件在 body。本计划范围只做 Transport 迁移，body 用 `serde_json::Value`
//! 透传（字段细化见 spec，不在本次范围）。

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType,
    error::validation_error, http::Transport, req_option::RequestOption,
};

/// 获取 OpenAPI 审计日志数据请求
///
/// 通过 body 传入过滤条件（`start_time`/`end_time`/`user_id_filter`/`page_size` 等），
/// 调用方自行构造 JSON。便捷方法 [`Self::last_days`] / [`Self::time_range`] 可快速
/// 设置时间范围。
#[derive(Debug)]
pub struct ListOpenApiLogsRequest {
    /// 配置信息。
    config: Config,
    /// 请求 body（过滤 + 分页条件）。
    body: serde_json::Value,
}

impl ListOpenApiLogsRequest {
    /// 创建新的请求构建器（空 body）。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: serde_json::json!({}),
        }
    }

    /// 设置请求 body（覆盖已有内容）。
    pub fn body(mut self, body: serde_json::Value) -> Self {
        self.body = body;
        self
    }

    /// 设置时间范围（写入 body 的 `start_time`/`end_time`）。
    pub fn time_range(mut self, start_time: i64, end_time: i64) -> Self {
        if let Some(obj) = self.body.as_object_mut() {
            obj.insert("start_time".into(), start_time.into());
            obj.insert("end_time".into(), end_time.into());
        }
        self
    }

    /// 设置最近 N 天的日志。
    pub fn last_days(self, days: i64) -> Self {
        let now = chrono::Utc::now().timestamp();
        self.time_range(now - days * 24 * 3600, now)
    }

    /// 设置最近 N 小时的日志。
    pub fn last_hours(self, hours: i64) -> Self {
        let now = chrono::Utc::now().timestamp();
        self.time_range(now - hours * 3600, now)
    }

    /// 执行请求，返回响应 `data` 字段内容。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post("/open-apis/security_and_compliance/v1/openapi_logs/list_data")
                .body(self.body)
                .with_supported_access_token_types(vec![AccessTokenType::App]);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| validation_error("获取审计日志数据", "响应数据为空"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> Config {
        Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build()
    }

    #[test]
    fn test_last_days_sets_time_range() {
        // 不发请求，仅验证便捷方法把时间范围写进 body。
        let req = ListOpenApiLogsRequest::new(test_config()).last_days(7);
        let obj = req.body.as_object().expect("body should be object");
        assert!(obj.get("start_time").is_some());
        assert!(obj.get("end_time").is_some());
        let start = obj["start_time"].as_i64().unwrap();
        let end = obj["end_time"].as_i64().unwrap();
        assert!(end > start);
    }
}
