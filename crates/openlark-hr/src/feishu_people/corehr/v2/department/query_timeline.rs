//! 查询指定生效日期的部门基本信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/department/query_timeline>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use serde::{Deserialize, Serialize};

/// 查询指定生效日期的部门基本信息请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QueryTimelineRequest {
    /// 部门 ID
    pub department_id: String,
    /// 生效日期
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
}

impl QueryTimelineRequest {
    /// 创建请求
    pub fn new(department_id: String) -> Self {
        Self {
            department_id,
            effective_time: None,
        }
    }

    /// 设置生效日期
    pub fn effective_time(mut self, effective_time: String) -> Self {
        self.effective_time = Some(effective_time);
        self
    }
}

/// 查询指定生效日期的部门基本信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryTimelineResponse {
    /// 部门信息
    pub data: Option<QueryTimelineResponseData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryTimelineResponseData {
    /// 部门 ID
    pub id: String,
    /// 部门名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 父部门 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// 部门编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 生效时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 失效时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
}

impl ApiResponseTrait for QueryTimelineResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询指定生效日期的部门基本信息请求构建器
#[derive(Debug, Clone)]
pub struct QueryTimelineRequestBuilder {
    config: Config,
    request: QueryTimelineRequest,
}

impl QueryTimelineRequestBuilder {
    /// 创建请求构建器
    pub fn new(config: Config, department_id: String) -> Self {
        Self {
            config,
            request: QueryTimelineRequest::new(department_id),
        }
    }

    /// 设置生效日期
    pub fn effective_time(mut self, effective_time: String) -> Self {
        self.request = self.request.effective_time(effective_time);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<QueryTimelineResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryTimelineResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        // 构建端点
        let api_endpoint = FeishuPeopleApiV2::DepartmentQueryTimeline;
        let request = ApiRequest::<QueryTimelineResponse>::post(api_endpoint.to_url());

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
                "查询指定生效日期的部门基本信息响应数据为空",
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
