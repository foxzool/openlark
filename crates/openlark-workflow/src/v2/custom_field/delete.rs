//! 删除自定义字段
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/task-v2/custom_field/delete>

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::custom_field::models::DeleteCustomFieldResponse;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use std::sync::Arc;

/// 删除自定义字段请求
#[derive(Debug, Clone)]
pub struct DeleteCustomFieldRequest {
    /// 配置信息
    config: Arc<Config>,
    /// 字段 GUID
    field_guid: String,
}

impl DeleteCustomFieldRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, field_guid: String) -> Self {
        Self { config, field_guid }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DeleteCustomFieldResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteCustomFieldResponse> {
        // 验证必填字段
        validate_required!(self.field_guid.trim(), "字段GUID不能为空");

        let api_endpoint = TaskApiV2::CustomFieldDelete(self.field_guid.clone());
        let request = ApiRequest::<DeleteCustomFieldResponse>::delete(api_endpoint.to_url());

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "删除自定义字段")
    }
}

impl ApiResponseTrait for DeleteCustomFieldResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use std::sync::Arc;

    use super::*;

    #[test]
    fn test_delete_custom_field_request() {
        let config = openlark_core::config::Config::builder()
            .app_id("test")
            .app_secret("test")
            .build();

        let request = DeleteCustomFieldRequest::new(Arc::new(config), "field_456".to_string());

        assert_eq!(request.field_guid, "field_456");
    }
}
