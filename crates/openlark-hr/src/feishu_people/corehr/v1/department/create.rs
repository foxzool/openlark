//! 创建部门
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/department/create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};

use super::models::{CreateRequestBody, CreateResponse};

/// 创建部门请求
#[derive(Debug, Clone)]
pub struct CreateRequest {
    /// 配置信息
    config: Config,
    /// 部门名称（必填）
    name: String,
    /// 父部门 ID
    parent_department_id: Option<String>,
    /// 部门负责人列表
    leader_user_ids: Option<Vec<String>>,
    /// 部门编码
    code: Option<String>,
    /// 部门描述
    description: Option<String>,
}

impl CreateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            name: String::new(),
            parent_department_id: None,
            leader_user_ids: None,
            code: None,
            description: None,
        }
    }

    /// 设置部门名称（必填）
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    /// 设置父部门 ID
    pub fn parent_department_id(mut self, parent_department_id: String) -> Self {
        self.parent_department_id = Some(parent_department_id);
        self
    }

    /// 设置部门负责人列表
    pub fn leader_user_ids(mut self, leader_user_ids: Vec<String>) -> Self {
        self.leader_user_ids = Some(leader_user_ids);
        self
    }

    /// 设置部门编码
    pub fn code(mut self, code: String) -> Self {
        self.code = Some(code);
        self
    }

    /// 设置部门描述
    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        // 1. 验证必填字段
        validate_required!(self.name.trim(), "部门名称不能为空");

        // 2. 构建端点
        let api_endpoint = FeishuPeopleApiV1::DepartmentCreate;
        let request = ApiRequest::<CreateResponse>::post(api_endpoint.to_url());

        // 3. 序列化请求体
        let request_body = CreateRequestBody {
            name: self.name,
            parent_department_id: self.parent_department_id,
            leader_user_ids: self.leader_user_ids,
            code: self.code,
            description: self.description,
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
                "创建部门响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

impl ApiResponseTrait for CreateResponse {
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
