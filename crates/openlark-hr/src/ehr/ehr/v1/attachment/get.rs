//! 下载人员的附件
//!
//! docPath: <https://open.feishu.cn/document/server-docs/ehr-v1/attachment/get>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 下载人员的附件请求
#[derive(Debug, Clone)]
pub struct GetRequest {
    /// 附件 ID（必填）
    attachment_id: String,
    /// 用户 ID（必填）
    user_id: String,
    /// 配置信息
    config: Config,
}

impl GetRequest {
    /// 创建请求
    pub fn new(config: Config, attachment_id: String, user_id: String) -> Self {
        Self {
            attachment_id,
            user_id,
            config,
        }
    }

    fn validate(&self) -> SDKResult<()> {
        validate_required!(self.attachment_id.trim(), "attachment_id");
        validate_required!(self.user_id.trim(), "user_id");
        Ok(())
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetResponse> {
        use crate::common::api_endpoints::EhrApiV1;

        self.validate()?;

        // 1. 构建端点
        let api_endpoint = EhrApiV1::AttachmentGet(self.attachment_id.clone());
        let mut request = ApiRequest::<GetResponse>::get(api_endpoint.to_url());

        // 2. 添加查询参数
        request = request.query("user_id", &self.user_id);

        // 3. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 4. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "下载人员附件响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 下载人员的附件响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetResponse {
    /// 附件名称
    pub name: String,
    /// 附件类型
    pub file_type: String,
    /// 附件大小（字节）
    pub size: i64,
    /// 附件下载 URL
    pub download_url: String,
    /// 附件 Token（用于下载文件）
    pub token: String,
}

impl ApiResponseTrait for GetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use openlark_core::config::Config;
    use serde_json::json;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build()
    }

    #[test]
    fn test_ehr_attachment_get_request_builder() {
        let request = GetRequest::new(create_test_config(), "att_1".to_string(), "u_1".to_string());
        assert_eq!(request.attachment_id, "att_1");
        assert_eq!(request.user_id, "u_1");
    }

    #[test]
    fn test_ehr_attachment_get_query_build() {
        let request = GetRequest::new(create_test_config(), "att_2".to_string(), "u_2".to_string());
        let api_endpoint =
            crate::common::api_endpoints::EhrApiV1::AttachmentGet(request.attachment_id.clone());
        let api_request = ApiRequest::<GetResponse>::get(api_endpoint.to_url())
            .query("user_id", &request.user_id);

        assert!(api_request.build_url().contains("att_2"));
        assert!(api_request.build_url().contains("user_id=u_2"));
    }

    #[test]
    fn test_ehr_attachment_get_response_deserialize() {
        let value = json!({
            "name": "resume.pdf",
            "file_type": "pdf",
            "size": 1024,
            "download_url": "https://example.com/resume.pdf",
            "token": "file_token"
        });
        let response: GetResponse = serde_json::from_value(value).expect("反序列化响应失败");

        assert_eq!(response.name, "resume.pdf");
        assert_eq!(response.size, 1024);
    }

    #[test]
    fn test_ehr_attachment_get_validation() {
        let invalid_attachment_request =
            GetRequest::new(create_test_config(), "  ".to_string(), "u_1".to_string());
        assert!(invalid_attachment_request.validate().is_err());

        let invalid_user_request =
            GetRequest::new(create_test_config(), "att_1".to_string(), " ".to_string());
        assert!(invalid_user_request.validate().is_err());

        let valid_request =
            GetRequest::new(create_test_config(), "att_1".to_string(), "u_1".to_string());
        assert!(valid_request.validate().is_ok());
    }
}
