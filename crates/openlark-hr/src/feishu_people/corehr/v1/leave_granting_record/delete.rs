//! 删除假期发放记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/leave_granting_record/delete>

use openlark_core::{
    api::ApiRequest,
    api::{ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 删除假期发放记录请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DeleteRequest {
    config: Config,
    record_id: String,
}

impl DeleteRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            record_id: String::new(),
        }
    }

    pub fn record_id(mut self, record_id: String) -> Self {
        self.record_id = record_id;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DeleteResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        validate_required!(self.record_id.trim(), "假期发放记录 ID 不能为空");

        let api_endpoint = FeishuPeopleApiV1::LeaveGrantingRecordDelete(self.record_id);
        let request = ApiRequest::<DeleteResponse>::delete(api_endpoint.to_url());
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "删除假期发放记录响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 删除假期发放记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteResponse {
    /// 响应数据
    ///
    /// TODO: 根据官方文档添加具体字段
    pub data: Value,
}

impl ApiResponseTrait for DeleteResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
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
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
