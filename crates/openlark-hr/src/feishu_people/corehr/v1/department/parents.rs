//! 获取父部门信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/department/parents>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};

use super::models::ParentsResponse;

/// 获取父部门信息请求
#[derive(Debug, Clone)]
pub struct ParentsRequest {
    /// 配置信息
    config: Config,
    /// 部门 ID（必填）
    department_id: String,
}

impl ParentsRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            department_id: String::new(),
        }
    }

    /// 设置部门 ID（必填）
    pub fn department_id(mut self, department_id: String) -> Self {
        self.department_id = department_id;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ParentsResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ParentsResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        // 1. 验证必填字段
        validate_required!(self.department_id.trim(), "部门ID不能为空");

        // 2. 构建端点
        let api_endpoint = FeishuPeopleApiV1::DepartmentParents(self.department_id);
        let request = ApiRequest::<ParentsResponse>::get(api_endpoint.to_url());

        // 3. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 4. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "获取父部门信息响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

impl ApiResponseTrait for ParentsResponse {
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
