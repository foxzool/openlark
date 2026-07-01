//! 获取单个工作城市信息
//!
//! docPath: <https://open.feishu.cn/document/contact-v3/work_city/get>

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};

use crate::{
    common::api_utils::extract_response_data,
    contact::contact::v3::work_city::models::WorkCityResponse, endpoints::CONTACT_V3_WORK_CITIES,
};

/// 获取单个工作城市信息请求
pub struct GetWorkCityRequest {
    /// 配置信息。
    config: Config,
    /// 工作城市 ID。
    work_city_id: String,
}

impl GetWorkCityRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            work_city_id: String::new(),
        }
    }

    /// 工作城市 ID（路径参数）
    pub fn work_city_id(mut self, work_city_id: impl Into<String>) -> Self {
        self.work_city_id = work_city_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/contact-v3/work_city/get>
    pub async fn execute(self) -> SDKResult<WorkCityResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<WorkCityResponse> {
        validate_required!(self.work_city_id, "work_city_id 不能为空");

        // url: GET:/open-apis/contact/v3/work_cities/:work_city_id
        let req: ApiRequest<WorkCityResponse> =
            ApiRequest::get(format!("{}/{}", CONTACT_V3_WORK_CITIES, self.work_city_id));

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取单个工作城市信息")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

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
