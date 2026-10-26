//! 更新设备信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/security_and_compliance-v2/device_record-update>
//!
//! 文档核对：`PUT /open-apis/security_and_compliance/v2/device_records/{device_record_id}?version={...}`，
//! 查询参数 `version` 必填（版本号）；body 字段（设备归属/可信状态/公共设备）均为可选。

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType,
    error::validation_error, http::Transport, req_option::RequestOption, validate_required,
};
use serde::{Deserialize, Serialize};

/// 更新设备信息请求 body（字段均可选）。
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct UpdateDeviceRecordBody {
    /// 设备归属（`0` 未知设备/`1` 个人设备/`2` 企业设备）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_ownership: Option<i32>,
    /// 可信状态（`0` 未知状态/`1` 信任设备/`2` 非信任设备）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_status: Option<i32>,
    /// 是否为公共设备。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_public: Option<bool>,
}

/// 更新设备信息请求
#[derive(Debug)]
pub struct UpdateDeviceRecordRequest {
    /// 配置信息。
    config: Config,
    /// 设备记录 ID（路径参数，必填）。
    device_record_id: String,
    /// 版本号（查询参数，必填）。
    version: String,
    /// 请求 body。
    body: UpdateDeviceRecordBody,
}

impl UpdateDeviceRecordRequest {
    /// 创建新的请求构建器。
    pub fn new(
        config: Config,
        device_record_id: impl Into<String>,
        version: impl Into<String>,
    ) -> Self {
        Self {
            config,
            device_record_id: device_record_id.into(),
            version: version.into(),
            body: UpdateDeviceRecordBody::default(),
        }
    }

    /// 设置设备归属（`0` 未知/`1` 个人/`2` 企业）。
    pub fn device_ownership(mut self, device_ownership: i32) -> Self {
        self.body.device_ownership = Some(device_ownership);
        self
    }

    /// 设置可信状态（`0` 未知/`1` 信任/`2` 非信任）。
    pub fn device_status(mut self, device_status: i32) -> Self {
        self.body.device_status = Some(device_status);
        self
    }

    /// 设置是否为公共设备。
    pub fn is_public(mut self, is_public: bool) -> Self {
        self.body.is_public = Some(is_public);
        self
    }

    /// 执行请求，返回响应 `data` 字段内容。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(self.device_record_id, "device_record_id 不能为空");
        validate_required!(self.version, "version 不能为空");

        let path = format!(
            "/open-apis/security_and_compliance/v2/device_records/{}",
            self.device_record_id
        );
        let req: ApiRequest<serde_json::Value> = ApiRequest::put(&path)
            .query("version", &self.version)
            .body(
                serde_json::to_value(self.body)
                    .map_err(|e| validation_error("更新设备信息", format!("序列化失败: {e}")))?,
            )
            .with_supported_access_token_types(vec![AccessTokenType::Tenant]);

        Transport::request_typed(req, &self.config, Some(option), "更新设备信息").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{body_json, method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    fn test_config() -> Config {
        Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build()
    }

    #[tokio::test]
    async fn test_update_device_record_rejects_empty_id() {
        let req = UpdateDeviceRecordRequest::new(test_config(), "", "0");
        let result = req.execute().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("device_record_id"));
    }

    #[tokio::test]
    async fn test_update_device_record_rejects_empty_version() {
        let req = UpdateDeviceRecordRequest::new(test_config(), "dr_001", "");
        let result = req.execute().await;
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("version"));
    }

    /// 端到端：PUT .../device_records/{id}?version= + body 可选字段 + 响应解析。
    #[tokio::test]
    async fn test_update_device_record_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("PUT"))
            .and(path(
                "/open-apis/security_and_compliance/v2/device_records/dr_001",
            ))
            .and(query_param("version", "0"))
            .and(body_json(json!({
                "device_ownership": 1,
                "device_status": 1,
                "is_public": false
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {}
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        UpdateDeviceRecordRequest::new(config, "dr_001", "0")
            .device_ownership(1)
            .device_status(1)
            .is_public(false)
            .execute()
            .await
            .expect("更新设备记录应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/security_and_compliance/v2/device_records/dr_001"
        );
        let query = received[0].url.query().unwrap_or("");
        assert!(query.contains("version=0"));
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["device_ownership"], 1);
        assert_eq!(sent["is_public"], false);
    }
}
