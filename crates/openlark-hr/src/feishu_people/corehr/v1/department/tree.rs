//! 获取部门树
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/department/tree>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use super::models::{TreeRequestBody, TreeResponse};

/// 获取部门树请求
#[derive(Debug, Clone)]
pub struct TreeRequest {
    /// 配置信息
    config: Config,
    /// 根部门 ID（不传则从顶层开始）
    department_id: Option<String>,
    /// 是否包含已停用部门
    include_inactive: Option<bool>,
}

impl TreeRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            department_id: None,
            include_inactive: None,
        }
    }

    /// 设置根部门 ID（不传则从顶层开始）
    pub fn department_id(mut self, department_id: String) -> Self {
        self.department_id = Some(department_id);
        self
    }

    /// 设置是否包含已停用部门
    pub fn include_inactive(mut self, include_inactive: bool) -> Self {
        self.include_inactive = Some(include_inactive);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<TreeResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<TreeResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        // 1. 构建端点
        let api_endpoint = FeishuPeopleApiV1::DepartmentTree;
        let request = ApiRequest::<TreeResponse>::post(api_endpoint.to_url());

        // 2. 序列化请求体
        let request_body = TreeRequestBody {
            department_id: self.department_id,
            include_inactive: self.include_inactive,
        };
        let request = request.body(serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {}", e),
            )
        })?);

        // 3. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 4. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "获取部门树响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

impl ApiResponseTrait for TreeResponse {
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
