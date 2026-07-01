//! 升级旧版表单
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-form/upgrade>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::SDKResult,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::BitableApiV1;

/// 表单布局模式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FormDisplayMode {
    /// 传统单页表单。
    Traditional,
    /// 每页一道题。
    OneQuestionPerPage,
}

/// 升级表单请求体（内部使用）。
#[derive(Debug, Clone, Serialize)]
struct UpgradeFormRequestBody {
    form_name: String,
    display_mode: FormDisplayMode,
}

/// 升级后的表单
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpgradedForm {
    /// 升级后的表单 ID。
    pub id: String,
}

/// 升级表单响应（data）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpgradeFormResponse {
    /// 升级后的表单信息。
    pub form: UpgradedForm,
}

impl ApiResponseTrait for UpgradeFormResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 升级表单请求
///
/// 用于把旧版表单升级为新版表单。
#[derive(Debug, Clone)]
pub struct UpgradeFormRequest {
    config: Config,
    app_token: String,
    table_id: String,
    form_id: String,
    form_name: String,
    display_mode: Option<FormDisplayMode>,
}

impl UpgradeFormRequest {
    /// 创建新的表单升级请求。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            table_id: String::new(),
            form_id: String::new(),
            form_name: String::new(),
            display_mode: None,
        }
    }

    /// 设置多维表格 token。
    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    /// 设置数据表 ID。
    pub fn table_id(mut self, table_id: impl Into<String>) -> Self {
        self.table_id = table_id.into();
        self
    }

    /// 设置表单 ID。
    pub fn form_id(mut self, form_id: impl Into<String>) -> Self {
        self.form_id = form_id.into();
        self
    }

    /// 设置升级后的表单名称。
    pub fn form_name(mut self, form_name: impl Into<String>) -> Self {
        self.form_name = form_name.into();
        self
    }

    /// 设置表单展示模式。
    pub fn display_mode(mut self, display_mode: FormDisplayMode) -> Self {
        self.display_mode = Some(display_mode);
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<UpgradeFormResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<UpgradeFormResponse> {
        validate_required!(self.app_token.trim(), "app_token");
        validate_required!(self.table_id.trim(), "table_id");
        validate_required!(self.form_id.trim(), "form_id");
        validate_required!(self.form_name.trim(), "form_name");

        let display_mode = self.display_mode.ok_or_else(|| {
            openlark_core::error::validation_error("display_mode", "display_mode 不能为空")
        })?;

        let api_endpoint = BitableApiV1::FormUpgrade(self.app_token, self.table_id, self.form_id);
        let body = UpgradeFormRequestBody {
            form_name: self.form_name,
            display_mode,
        };
        let api_request: ApiRequest<UpgradeFormResponse> =
            ApiRequest::post(&api_endpoint.to_url()).body(serde_json::to_vec(&body)?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::testing::prelude::test_runtime;

    #[test]
    fn test_missing_display_mode() {
        let config = Config::default();
        let request = UpgradeFormRequest::new(config)
            .app_token("app_token")
            .table_id("table_id")
            .form_id("form_id")
            .form_name("表单");
        let rt = test_runtime();
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(UpgradeFormResponse::data_format(), ResponseFormat::Data);
    }
}
