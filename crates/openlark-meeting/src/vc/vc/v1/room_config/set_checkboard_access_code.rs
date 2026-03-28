//! 创建签到板部署码
//!
//! docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/room_config/set_checkboard_access_code>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
};

use crate::common::api_utils::{extract_response_data, serialize_params};

/// 创建签到板部署码请求
pub struct SetCheckboardAccessCodeRequest {
    config: Config,
}

impl SetCheckboardAccessCodeRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/historic-version/meeting_room-v1/room_config/set_checkboard_access_code>
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        body: serde_json::Value,
        option: RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/vc/v1/room_configs/set_checkboard_access_code
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post("/open-apis/vc/v1/room_configs/set_checkboard_access_code")
                .body(serialize_params(&body, "创建签到板部署码")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "创建签到板部署码")
    }
}

#[cfg(test)]
mod tests {

    use serde_json;

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
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
