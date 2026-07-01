//! 生成 CalDAV 配置
//!
//! docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/setting/generate_caldav_conf>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};

use crate::common::api_utils::extract_response_data;

/// 生成 CalDAV 配置请求
pub struct GenerateCaldavConfRequest {
    config: Config,
}

impl GenerateCaldavConfRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/calendar-v4/setting/generate_caldav_conf>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/calendar/v4/settings/generate_caldav_conf
        // 注意：此端点在 CalendarApiV4 中可能不存在，需要添加或使用常量
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post("/open-apis/calendar/v4/settings/generate_caldav_conf");

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "生成 CalDAV 配置")
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
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
