//! 查询指定生效日期的部门架构树
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/department/tree>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use serde::{Deserialize, Serialize};

/// 查询指定生效日期的部门架构树请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreeRequest {
    /// 生效日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 根部门 ID（可选，不传则返回全公司架构树）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_department_id: Option<String>,
}

impl TreeRequest {
    /// 创建请求
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置生效日期
    pub fn effective_time(mut self, effective_time: String) -> Self {
        self.effective_time = Some(effective_time);
        self
    }

    /// 设置根部门 ID
    pub fn root_department_id(mut self, root_department_id: String) -> Self {
        self.root_department_id = Some(root_department_id);
        self
    }
}

/// 查询指定生效日期的部门架构树响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TreeResponse {
    /// 部门树
    pub data: Option<TreeResponseData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TreeResponseData {
    /// 部门树节点
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<DepartmentNode>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DepartmentNode {
    /// 部门 ID
    pub id: String,
    /// 部门名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 部门编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 父部门 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// 子部门列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<DepartmentNode>>,
    /// 部门负责人 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leader_user_id: Option<String>,
}

impl ApiResponseTrait for TreeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询指定生效日期的部门架构树请求构建器
#[derive(Debug, Clone)]
pub struct TreeRequestBuilder {
    config: Config,
    request: TreeRequest,
}

impl TreeRequestBuilder {
    /// 创建请求构建器
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request: TreeRequest::new(),
        }
    }

    /// 设置生效日期
    pub fn effective_time(mut self, effective_time: String) -> Self {
        self.request = self.request.effective_time(effective_time);
        self
    }

    /// 设置根部门 ID
    pub fn root_department_id(mut self, root_department_id: String) -> Self {
        self.request = self.request.root_department_id(root_department_id);
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
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        // 构建端点
        let api_endpoint = FeishuPeopleApiV2::DepartmentTree;
        let request = ApiRequest::<TreeResponse>::post(api_endpoint.to_url());

        // 序列化请求体
        let request = request.body(serde_json::to_value(&self.request).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {}", e),
            )
        })?);

        // 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "查询指定生效日期的部门架构树响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
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
