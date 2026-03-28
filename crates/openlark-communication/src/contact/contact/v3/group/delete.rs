//! 删除用户组
//!
//! docPath: <https://open.feishu.cn/document/server-docs/contact-v3/group/delete>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::{api_utils::extract_response_data, models::EmptyData},
    endpoints::CONTACT_V3_GROUP,
};

/// 删除用户组请求
///
/// 用于删除通讯录中的用户组。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `group_id`: 用户组 ID，必填
///
/// # 示例
///
/// ```rust,ignore
/// let request = DeleteGroupRequest::new(config)
///     .group_id("group_xxx");
/// ```
pub struct DeleteGroupRequest {
    config: Config,
    group_id: String,
}

impl DeleteGroupRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            group_id: String::new(),
        }
    }

    /// 用户组 ID（路径参数）
    pub fn group_id(mut self, group_id: impl Into<String>) -> Self {
        self.group_id = group_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/server-docs/contact-v3/group/delete>
    pub async fn execute(self) -> SDKResult<EmptyData> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EmptyData> {
        // === 必填字段验证 ===
        validate_required!(self.group_id, "group_id 不能为空");

        // url: DELETE:/open-apis/contact/v3/group/:group_id
        let req: ApiRequest<EmptyData> =
            ApiRequest::delete(format!("{}/{}", CONTACT_V3_GROUP, self.group_id));
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "删除用户组")
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_group_request_builder() {
        let config = Config::default();
        let request = DeleteGroupRequest::new(config).group_id("group_xxx");
        assert_eq!(request.group_id, "group_xxx");
    }

    #[test]
    fn test_delete_group_request_default_values() {
        let config = Config::default();
        let request = DeleteGroupRequest::new(config);
        assert_eq!(request.group_id, "");
    }

    #[test]
    fn test_delete_group_request_with_multiple_ids() {
        let config = Config::default();
        let request1 = DeleteGroupRequest::new(config.clone()).group_id("group_111");
        let request2 = DeleteGroupRequest::new(config).group_id("group_222");
        assert_eq!(request1.group_id, "group_111");
        assert_eq!(request2.group_id, "group_222");
    }
}
