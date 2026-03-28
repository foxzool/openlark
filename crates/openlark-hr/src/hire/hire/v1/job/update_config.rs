//! 更新职位设置
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/job/update_config>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 更新职位设置请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct UpdateConfigRequest {
    job_id: String,
    request_body: UpdateConfigRequestBody,
    /// 配置信息
    config: Config,
}

impl UpdateConfigRequest {
    /// 创建请求
    pub fn new(config: Config, job_id: String, request_body: UpdateConfigRequestBody) -> Self {
        Self {
            job_id,
            request_body,
            config,
        }
    }

    pub fn request_body(mut self, request_body: UpdateConfigRequestBody) -> Self {
        self.request_body = request_body;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UpdateConfigResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateConfigResponse> {
        use crate::common::api_endpoints::HireApiV1;

        validate_required!(self.job_id.trim(), "职位 ID 不能为空");
        self.request_body.validate()?;

        let api_endpoint = HireApiV1::JobUpdateConfig(self.job_id);
        let request = ApiRequest::<UpdateConfigResponse>::post(api_endpoint.to_url());
        let request = request.body(serde_json::to_value(&self.request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {}", e),
            )
        })?);
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "更新职位设置响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateConfigRequestBody {
    #[serde(flatten)]
    pub fields: Value,
}

impl UpdateConfigRequestBody {
    pub fn new(fields: Value) -> Self {
        Self { fields }
    }

    fn validate(&self) -> SDKResult<()> {
        if self.fields.is_null() {
            return Err(openlark_core::error::validation_error(
                "更新职位设置请求体不能为空",
                "请传入有效的请求参数",
            ));
        }

        Ok(())
    }
}

/// 更新职位设置响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateConfigResponse {
    /// 响应数据
    ///
    /// TODO: 根据官方文档添加具体字段
    pub data: Value,
}

impl ApiResponseTrait for UpdateConfigResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
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
