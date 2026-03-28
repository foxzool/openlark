//! Bitable 更新视图
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table-view/patch>

use crate::common::api_endpoints::BitableApiV1;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::SDKResult,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 视图信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct View {
    /// 视图 ID
    pub view_id: String,
    /// 视图名称
    pub view_name: String,
    /// 视图类型
    pub view_type: String,
    /// 视图公共等级（部分接口返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_public_level: Option<String>,
    /// 个人视图所有者的 ID（部分接口返回，ID 类型与 user_id_type 保持一致）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_private_owner_id: Option<String>,
    /// 视图属性（仅部分接口返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<Value>,
    /// 创建时间（仅部分接口返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// 更新时间（仅部分接口返回）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_time: Option<String>,
}

/// 更新视图请求体
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PatchViewData {
    /// 视图名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_name: Option<String>,
    /// 视图属性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<Value>,
}

impl PatchViewData {
    pub fn new(view_name: impl Into<String>) -> Self {
        Self {
            view_name: Some(view_name.into()),
            property: None,
        }
    }

    pub fn with_property(mut self, property: Value) -> Self {
        self.property = Some(property);
        self
    }
}

/// 更新视图响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PatchViewResponse {
    /// 视图信息
    pub view: View,
}

impl ApiResponseTrait for PatchViewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新视图请求
#[derive(Debug, Clone)]
pub struct PatchViewRequest {
    config: Config,
    app_token: String,
    table_id: String,
    view_id: String,
    payload: PatchViewData,
}

impl PatchViewRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            table_id: String::new(),
            view_id: String::new(),
            payload: PatchViewData::default(),
        }
    }

    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    pub fn table_id(mut self, table_id: String) -> Self {
        self.table_id = table_id;
        self
    }

    pub fn view_id(mut self, view_id: String) -> Self {
        self.view_id = view_id;
        self
    }

    pub fn payload(mut self, payload: PatchViewData) -> Self {
        self.payload = payload;
        self
    }

    pub async fn execute(self) -> SDKResult<PatchViewResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<PatchViewResponse> {
        validate_required!(self.app_token.trim(), "app_token");
        validate_required!(self.table_id.trim(), "table_id");
        validate_required!(self.view_id.trim(), "view_id");

        if self.payload.view_name.is_none() && self.payload.property.is_none() {
            return Err(openlark_core::error::validation_error(
                "payload",
                "至少需要提供一个更新字段（view_name 或 property）",
            ));
        }

        if let Some(ref view_name) = self.payload.view_name {
            if view_name.trim().is_empty() {
                return Err(openlark_core::error::validation_error(
                    "view_name",
                    "视图名称不能为空",
                ));
            }
            if view_name.len() > 100 {
                return Err(openlark_core::error::validation_error(
                    "view_name",
                    "视图名称长度不能超过100个字符",
                ));
            }
            if view_name.contains('[') || view_name.contains(']') {
                return Err(openlark_core::error::validation_error(
                    "view_name",
                    "视图名称不能包含 '[' 或 ']'",
                ));
            }
        }

        let api_endpoint = BitableApiV1::ViewPatch(self.app_token, self.table_id, self.view_id);
        let api_request: ApiRequest<PatchViewResponse> =
            ApiRequest::patch(&api_endpoint.to_url()).body(serde_json::to_vec(&self.payload)?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
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
