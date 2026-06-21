//! 查询设备信息列表
//!
//! docPath: https://open.feishu.cn/document/server-docs/security_and_compliance-v2/device_record-list

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType,
    error::validation_error, http::Transport, req_option::RequestOption,
};

/// 查询设备信息列表请求
///
/// 支持分页及多种过滤。
#[derive(Debug)]
pub struct ListDeviceRecordsRequest {
    /// 配置信息。
    config: Config,
    /// 页面大小（可选）。
    page_size: Option<i32>,
    /// 分页标记（可选）。
    page_token: Option<String>,
    /// 用户 ID 过滤（可选）。
    user_id: Option<String>,
    /// 设备类型过滤（可选）。
    device_type: Option<String>,
    /// 状态过滤（可选）。
    status: Option<String>,
    /// 是否个人设备过滤（可选）。
    personal_device: Option<bool>,
    /// 合规状态过滤（可选）。
    compliance_status: Option<String>,
}

impl ListDeviceRecordsRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            user_id: None,
            device_type: None,
            status: None,
            personal_device: None,
            compliance_status: None,
        }
    }

    /// 设置页面大小。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置用户 ID 过滤。
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }

    /// 设置设备类型过滤。
    pub fn device_type(mut self, device_type: impl Into<String>) -> Self {
        self.device_type = Some(device_type.into());
        self
    }

    /// 设置状态过滤。
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    /// 设置是否个人设备过滤。
    pub fn personal_device(mut self, personal_device: bool) -> Self {
        self.personal_device = Some(personal_device);
        self
    }

    /// 设置合规状态过滤。
    pub fn compliance_status(mut self, compliance_status: impl Into<String>) -> Self {
        self.compliance_status = Some(compliance_status.into());
        self
    }

    /// 执行请求，返回响应 `data` 字段内容。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::get("/open-apis/security_and_compliance/v2/device_records")
                .query_opt("page_size", self.page_size.map(|v| v.to_string()))
                .query_opt("page_token", self.page_token.as_ref())
                .query_opt("user_id", self.user_id.as_ref())
                .query_opt("device_type", self.device_type.as_ref())
                .query_opt("status", self.status.as_ref())
                .query_opt(
                    "personal_device",
                    self.personal_device.map(|v| v.to_string()),
                )
                .query_opt("compliance_status", self.compliance_status.as_ref())
                .with_supported_access_token_types(vec![AccessTokenType::App]);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| validation_error("查询设备信息列表", "响应数据为空"))
    }
}
