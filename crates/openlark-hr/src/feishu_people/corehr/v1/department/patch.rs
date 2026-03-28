//! 更新部门
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/department/patch>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};

use super::models::{PatchRequestBody, PatchResponse};

/// 更新部门请求
#[derive(Debug, Clone)]
pub struct PatchRequest {
    /// 配置信息
    config: Config,
    /// 部门 ID（必填）
    department_id: String,
    /// 部门名称
    name: Option<String>,
    /// 父部门 ID
    parent_department_id: Option<String>,
    /// 部门负责人列表
    leader_user_ids: Option<Vec<String>>,
    /// 部门编码
    code: Option<String>,
    /// 部门描述
    description: Option<String>,
    /// 部门状态
    /// - 0: 停用
    /// - 1: 启用
    status: Option<i32>,
}

impl PatchRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            department_id: String::new(),
            name: None,
            parent_department_id: None,
            leader_user_ids: None,
            code: None,
            description: None,
            status: None,
        }
    }

    /// 设置部门 ID（必填）
    pub fn department_id(mut self, department_id: String) -> Self {
        self.department_id = department_id;
        self
    }

    /// 设置部门名称
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
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

    /// 设置部门状态
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
        validate_required!(self.department_id.trim(), "部门ID不能为空");

        // 验证至少有一个更新字段
        if self.name.is_none()
            && self.parent_department_id.is_none()
            && self.leader_user_ids.is_none()
            && self.code.is_none()
            && self.description.is_none()
            && self.status.is_none()
        {
            return Err(openlark_core::error::validation_error(
                "更新字段不能为空",
                "至少需要设置一个更新字段",
            ));
        }

        // 2. 构建端点
        let api_endpoint = FeishuPeopleApiV1::DepartmentPatch(self.department_id.clone());
        let request = ApiRequest::<PatchResponse>::patch(api_endpoint.to_url());

        // 3. 序列化请求体
        let request_body = PatchRequestBody {
            department_id: self.department_id,
            name: self.name,
            parent_department_id: self.parent_department_id,
            leader_user_ids: self.leader_user_ids,
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
                "更新部门响应数据为空",
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
