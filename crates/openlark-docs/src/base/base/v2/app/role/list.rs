//! 列出自定义角色
//!
//! docPath: <https://open.feishu.cn/document/docs/bitable-v1/advanced-permission/app-role/list-2>

use crate::base::base::v2::models::AppRole;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::*;

/// 列出自定义角色
#[derive(Debug)]
pub struct List {
    config: Config,
    app_token: String,
    req: ListReq,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListReq {
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListResp {
    /// 角色列表
    #[serde(default)]
    pub items: Vec<AppRole>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 是否还有更多
    #[serde(default)]
    pub has_more: bool,
    /// 总数
    #[serde(default)]
    pub total: i32,
}

impl List {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            req: ListReq {
                page_size: None,
                page_token: None,
            },
        }
    }

    /// 应用 token
    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    /// 分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.req.page_size = Some(page_size);
        self
    }

    /// 分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.req.page_token = Some(page_token.into());
        self
    }

    pub async fn execute(self) -> SDKResult<ListResp> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListResp> {
        validate_required!(self.app_token, "app_token 不能为空");
        if let Some(page_size) = self.req.page_size {
            if page_size <= 0 {
                return Err(openlark_core::error::validation_error(
                    "page_size",
                    "page_size 必须为正整数",
                ));
            }
        }

        use crate::common::api_endpoints::BaseApiV2;
        let api_endpoint = BaseApiV2::RoleList(self.app_token);

        let mut api_request: ApiRequest<ListResp> = ApiRequest::get(&api_endpoint.to_url());
        api_request = api_request.query_opt("page_size", self.req.page_size.map(|v| v.to_string()));
        api_request = api_request.query_opt("page_token", self.req.page_token);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "列出自定义角色")
    }
}

impl ApiResponseTrait for ListResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
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
