//! 导出会议室预定数据
//!
//! docPath: <https://open.feishu.cn/document/server-docs/vc-v1/export/resource_reservation_list>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, req_option::RequestOption, SDKResult,
};

use crate::common::api_endpoints::VcApiV1;
use crate::common::api_utils::{extract_response_data, serialize_params};

/// 导出会议室预定数据请求
pub struct ExportResourceReservationListRequest {
    config: Config,
}

impl ExportResourceReservationListRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/vc-v1/export/resource_reservation_list>
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
        // url: POST:/open-apis/vc/v1/exports/resource_reservation_list
        let api_endpoint = VcApiV1::ExportResourceReservationList;
        let req: ApiRequest<serde_json::Value> = ApiRequest::post(api_endpoint.to_url())
            .body(serialize_params(&body, "导出会议室预定数据")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "导出会议室预定数据")
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
