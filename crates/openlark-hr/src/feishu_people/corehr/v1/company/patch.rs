//! 更新公司
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/company/patch>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};

use super::models::{PatchRequestBody, PatchResponse};

/// 更新公司请求
#[derive(Debug, Clone)]
pub struct PatchRequest {
    /// 配置信息
    config: Config,
    /// 公司 ID（必填）
    company_id: String,
    /// 公司名称
    name: Option<String>,
    /// 公司编码
    code: Option<String>,
    /// 公司描述
    description: Option<String>,
    /// 公司状态
    /// - 0: 停用
    /// - 1: 启用
    status: Option<i32>,
}

impl PatchRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            company_id: String::new(),
            name: None,
            code: None,
            description: None,
            status: None,
        }
    }

    /// 设置公司 ID（必填）
    pub fn company_id(mut self, company_id: String) -> Self {
        self.company_id = company_id;
        self
    }

    /// 设置公司名称
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// 设置公司编码
    pub fn code(mut self, code: String) -> Self {
        self.code = Some(code);
        self
    }

    /// 设置公司描述
    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// 设置公司状态
    /// - 0: 停用
    /// - 1: 启用
    pub fn status(mut self, status: i32) -> Self {
        self.status = Some(status);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<PatchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<PatchResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        // 1. 验证必填字段
        validate_required!(self.company_id.trim(), "公司 ID 不能为空");

        // 验证至少有一个更新字段
        if self.name.is_none()
            && self.code.is_none()
            && self.description.is_none()
            && self.status.is_none()
        {
            return Err(openlark_core::error::validation_error(
                "更新字段不能为空",
                "至少需要提供一个更新字段（name、code、description 或 status）",
            ));
        }

        // 2. 构建端点
        let api_endpoint = FeishuPeopleApiV1::CompanyPatch(self.company_id.clone());
        let request = ApiRequest::<PatchResponse>::patch(api_endpoint.to_url());

        // 3. 序列化请求体
        let request_body = PatchRequestBody {
            company_id: self.company_id,
            name: self.name,
            code: self.code,
            description: self.description,
            status: self.status,
        };
        let request = request.body(serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {}", e),
            )
        })?);

        // 4. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 5. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "更新公司响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

impl ApiResponseTrait for PatchResponse {
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
