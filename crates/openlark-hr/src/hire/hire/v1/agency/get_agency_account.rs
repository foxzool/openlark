//! 查询猎头供应商下猎头列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/agency/get_agency_account>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::hire::hire::common_models::AgencyAccountSummary;

/// 查询猎头供应商下猎头列表请求
#[derive(Debug, Clone)]
pub struct GetAgencyAccountRequest {
    /// 配置信息
    config: Config,
    agency_id: String,
    request_body: Option<Value>,
}

impl GetAgencyAccountRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            agency_id: String::new(),
            request_body: None,
        }
    }

    /// 设置 `agency_id`。
    pub fn agency_id(mut self, agency_id: String) -> Self {
        self.agency_id = agency_id;
        self
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: Value) -> Self {
        self.request_body = Some(request_body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetAgencyAccountResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetAgencyAccountResponse> {
        let mut request = ApiRequest::<GetAgencyAccountResponse>::post(
            "/open-apis/hire/v1/agencies/get_agency_account",
        );
        if let Some(request_body) = self.request_body {
            request = request.body(request_body);
        }

        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "查询猎头供应商下猎头列表响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 查询猎头供应商下猎头列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetAgencyAccountResponse {
    #[serde(default, alias = "accounts")]
    /// 结果项列表。
    pub items: Vec<AgencyAccountSummary>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 下一页分页标记。
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 是否还有更多结果。
    pub has_more: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for GetAgencyAccountResponse {
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
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
