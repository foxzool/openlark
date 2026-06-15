//! 获取用户的签名列表
//!
//! docPath: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/mail-v1/user_mailbox-setting/get_signatures

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::Deserialize;
use std::sync::Arc;

/// 获取签名列表请求。
#[derive(Debug, Clone)]
pub struct GetSignaturesRequest {
    config: Arc<Config>,
    mailbox_id: String,
}

/// 获取签名列表响应。
#[derive(Debug, Clone, Deserialize)]
pub struct GetSignaturesResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<SignaturesData>,
}

impl ApiResponseTrait for GetSignaturesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 签名列表数据。
#[derive(Debug, Clone, Deserialize)]
pub struct SignaturesData {
    /// 用户邮箱签名列表。
    pub signatures: Option<Vec<UserMailboxSignature>>,
    /// 用户邮箱签名使用情况列表。
    pub usages: Option<Vec<SignatureUsage>>,
}

/// 用户邮箱签名。
#[derive(Debug, Clone, Deserialize)]
pub struct UserMailboxSignature {
    /// 签名 ID。
    pub id: Option<String>,
    /// 签名名称。
    pub name: Option<String>,
    /// 签名内容（HTML 格式）。
    pub content: Option<String>,
    /// 签名类型：USER（用户签名）/ TENANT（租户签名）。
    pub signature_type: Option<String>,
    /// 签名适用设备类型：PC / MOBILE。
    pub signature_device: Option<String>,
    /// 企业签名模板变量渲染。
    pub template_json_keys: Option<Vec<String>>,
    /// 签名图片列表。
    pub images: Option<Vec<SignatureImage>>,
    /// 企业签名模版变量值（值结构不明确，用 Value 兜底）。
    pub user_fields: Option<serde_json::Value>,
}

/// 签名图片。
#[derive(Debug, Clone, Deserialize)]
pub struct SignatureImage {
    /// 签名图片名称。
    pub image_name: Option<String>,
    /// 签名图片的文件 key。
    pub file_key: Option<String>,
    /// 签名图片的 Content-ID。
    pub cid: Option<String>,
    /// 签名图片文件大小（字节）。
    pub file_size: Option<String>,
    /// 签名图片宽度（像素）。
    pub image_width: Option<i32>,
    /// 签名图片高度（像素）。
    pub image_height: Option<i32>,
    /// 图片下载 url。
    pub download_url: Option<String>,
}

/// 签名使用情况。
#[derive(Debug, Clone, Deserialize)]
pub struct SignatureUsage {
    /// 邮箱地址。
    pub email_address: Option<String>,
    /// 发送邮件时使用的签名 ID。
    pub send_mail_signature_id: Option<String>,
    /// 回复邮件时使用的签名 ID。
    pub reply_signature_id: Option<String>,
}

impl GetSignaturesRequest {
    /// 创建新的实例。
    pub fn new(config: Arc<Config>, mailbox_id: impl Into<String>) -> Self {
        Self {
            config,
            mailbox_id: mailbox_id.into(),
        }
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<GetSignaturesResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetSignaturesResponse> {
        validate_required!(self.mailbox_id, "user_mailbox_id 不能为空");

        let path = format!(
            "/open-apis/mail/v1/user_mailboxes/{}/settings/signatures",
            self.mailbox_id
        );
        let req: ApiRequest<GetSignaturesResponse> = ApiRequest::get(&path);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("获取签名列表", "响应数据为空"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn response_deserializes() {
        let json = r#"{
            "code": 0, "msg": "success",
            "data": {
                "signatures": [{
                    "id": "sig_xxx", "name": "我的签名", "content": "<div>Best regards</div>",
                    "signature_type": "USER", "signature_device": "PC"
                }],
                "usages": [{ "email_address": "u@e.com", "send_mail_signature_id": "sig_xxx" }]
            }
        }"#;
        let resp: GetSignaturesResponse = serde_json::from_str(json).unwrap();
        let sig = resp.data.unwrap().signatures.unwrap().pop().unwrap();
        assert_eq!(sig.id, Some("sig_xxx".to_string()));
    }
}
