//! 审批设备申报
//!
//! docPath: <https://open.feishu.cn/document/server-docs/security_and_compliance-v2/device_apply_record-approve>
//!
//! 文档核对：`PUT /open-apis/security_and_compliance/v2/device_apply_records/{device_apply_record_id}`，
//! body `{approved, comment, remark}`。

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType,
    error::validation_error, http::Transport, req_option::RequestOption, validate_required,
};
use serde::{Deserialize, Serialize};

/// 审批设备申报请求 body
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApproveDeviceApplyRecordBody {
    /// 是否批准。
    pub approved: bool,
    /// 审批意见（可选）。
    pub comment: Option<String>,
    /// 审批备注（可选）。
    pub remark: Option<String>,
}

/// 审批设备申报请求
#[derive(Debug)]
pub struct ApproveDeviceApplyRecordRequest {
    /// 配置信息。
    config: Config,
    /// 申报记录 ID（路径参数，必填）。
    device_apply_record_id: String,
    /// 请求 body。
    body: ApproveDeviceApplyRecordBody,
}

impl ApproveDeviceApplyRecordRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config, device_apply_record_id: impl Into<String>) -> Self {
        Self {
            config,
            device_apply_record_id: device_apply_record_id.into(),
            body: ApproveDeviceApplyRecordBody {
                approved: false,
                comment: None,
                remark: None,
            },
        }
    }

    /// 设置是否批准。
    pub fn approved(mut self, approved: bool) -> Self {
        self.body.approved = approved;
        self
    }

    /// 快速批准。
    pub fn approve(self) -> Self {
        self.approved(true)
    }

    /// 快速拒绝。
    pub fn reject(self) -> Self {
        self.approved(false)
    }

    /// 设置审批意见。
    pub fn comment(mut self, comment: impl Into<String>) -> Self {
        self.body.comment = Some(comment.into());
        self
    }

    /// 设置审批备注。
    pub fn remark(mut self, remark: impl Into<String>) -> Self {
        self.body.remark = Some(remark.into());
        self
    }

    /// 执行请求，返回响应 `data` 字段内容。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        validate_required!(
            self.device_apply_record_id,
            "device_apply_record_id 不能为空"
        );

        let path = format!(
            "/open-apis/security_and_compliance/v2/device_apply_records/{}",
            self.device_apply_record_id
        );
        let req: ApiRequest<serde_json::Value> = ApiRequest::put(&path)
            .body(
                serde_json::to_value(&self.body)
                    .map_err(|e| validation_error("审批设备申报", format!("序列化失败: {e}")))?,
            )
            .with_supported_access_token_types(vec![AccessTokenType::App]);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| validation_error("审批设备申报", "响应数据为空"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_config() -> Config {
        Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build()
    }

    #[tokio::test]
    async fn test_approve_rejects_empty_id() {
        let req = ApproveDeviceApplyRecordRequest::new(test_config(), "").approve();
        let result = req.execute().await;
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("device_apply_record_id")
        );
    }
}
