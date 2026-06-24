//! 获取长连接在线数量
//!
//! 查询应用的长连接在线数量。应用由请求头中的 tenant_access_token 确定。
//!
//! docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/event-v1/connection/get

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};

use crate::{common::api_utils::extract_response_data, endpoints::EVENT_V1_CONNECTION};

/// 获取长连接在线数量响应（data 业务载荷）
///
/// 官方 apiSchema 响应体：code (int, envelope) / msg (string, envelope) /
/// data.online_instance_cnt (int)。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetConnectionOnlineCountResponse {
    /// 在线连接实例数量
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub online_instance_cnt: Option<i64>,
}

impl ApiResponseTrait for GetConnectionOnlineCountResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取长连接在线数量请求
pub struct GetConnectionOnlineCountRequest {
    config: Config,
}

impl GetConnectionOnlineCountRequest {
    /// 创建获取长连接在线数量请求
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/event-v1/connection/get
    pub async fn execute(self) -> SDKResult<GetConnectionOnlineCountResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetConnectionOnlineCountResponse> {
        // url: GET:/open-apis/event/v1/connection
        let req: ApiRequest<GetConnectionOnlineCountResponse> =
            ApiRequest::get(EVENT_V1_CONNECTION);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取长连接在线数量")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化往返
        let resp = GetConnectionOnlineCountResponse {
            online_instance_cnt: Some(42),
        };
        let json = serde_json::to_string(&resp).expect("序列化失败");
        let back: GetConnectionOnlineCountResponse =
            serde_json::from_str(&json).expect("反序列化失败");
        assert_eq!(back.online_instance_cnt, Some(42));
    }

    #[test]
    fn test_deserialization_from_official_payload() {
        // 官方 data 载荷形状：{"online_instance_cnt": <int>}
        // code/msg 属于 envelope，不在此载荷内。
        let json = r#"{"online_instance_cnt": 128}"#;
        let resp: GetConnectionOnlineCountResponse =
            serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(resp.online_instance_cnt, Some(128));
    }

    #[test]
    fn test_deserialization_missing_field_uses_default() {
        // 缺字段时 serde(default) 应回落为 None，不破坏调用方
        let json = r#"{}"#;
        let resp: GetConnectionOnlineCountResponse =
            serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(resp.online_instance_cnt, None);
    }
}
