//! 新增设备记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/security_and_compliance-v2/device_record-create>
//!
//! 文档核对：`POST /open-apis/security_and_compliance/v2/device_records`。
//! body 用 `serde_json::Value` 透传（字段细化不在本次范围）。

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType,
    error::validation_error, http::Transport, req_option::RequestOption,
};

/// 新增设备记录请求
///
/// 通过 body 传入设备信息。本计划范围只做 Transport 迁移，body 用 `serde_json::Value`
/// 透传（字段细化不在本次范围）。
#[derive(Debug)]
pub struct CreateDeviceRecordRequest {
    /// 配置信息。
    config: Config,
    /// 请求体（必填，调用方自行构造 JSON）。
    body: Option<serde_json::Value>,
}

impl CreateDeviceRecordRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self { config, body: None }
    }

    /// 设置请求体。
    pub fn body(mut self, body: serde_json::Value) -> Self {
        self.body = Some(body);
        self
    }

    /// 执行请求，返回响应 `data` 字段内容。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        let body = self
            .body
            .ok_or_else(|| validation_error("新增设备记录", "body 不能为空"))?;

        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post("/open-apis/security_and_compliance/v2/device_records")
                .body(body)
                .with_supported_access_token_types(vec![AccessTokenType::App]);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| validation_error("新增设备记录", "响应数据为空"))
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

    #[tokio::test]
    async fn test_create_device_record_rejects_empty_body() {
        let req = CreateDeviceRecordRequest::new(test_config());
        let result = req.execute().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("body"));
    }
}
