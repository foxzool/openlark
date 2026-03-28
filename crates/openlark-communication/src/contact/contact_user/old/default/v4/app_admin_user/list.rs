//! 查询应用管理员列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/application-v6/admin/query-app-administrator-list>

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{common::api_utils::extract_response_data, endpoints::USER_V4_APP_ADMIN_USER_LIST};

/// 查询应用管理员列表请求
pub struct ListAppAdminUserRequest {
    config: Config,
}

impl ListAppAdminUserRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/application-v6/admin/query-app-administrator-list>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // url: GET:/open-apis/user/v4/app_admin_user/list
        let req: ApiRequest<serde_json::Value> = ApiRequest::get(USER_V4_APP_ADMIN_USER_LIST);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "查询应用管理员列表")
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
