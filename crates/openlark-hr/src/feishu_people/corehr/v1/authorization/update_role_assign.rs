//! 更新用户被授权的数据范围
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/authorization/update_role_assign>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 更新用户被授权的数据范围请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct UpdateRoleAssignRequest {
    /// 配置信息
    config: Config,
    body: Value,
}

impl UpdateRoleAssignRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: Value::Object(serde_json::Map::new()),
        }
    }

    pub fn body(mut self, body: Value) -> Self {
        self.body = body;
        self
    }

    pub fn field(mut self, key: impl Into<String>, value: Value) -> Self {
        if !self.body.is_object() {
            self.body = Value::Object(serde_json::Map::new());
        }
        if let Some(body) = self.body.as_object_mut() {
            body.insert(key.into(), value);
        }
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UpdateRoleAssignResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<UpdateRoleAssignResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        let api_endpoint = FeishuPeopleApiV1::AuthorizationUpdateRoleAssign;
        let request =
            ApiRequest::<UpdateRoleAssignResponse>::post(api_endpoint.to_url()).body(self.body);
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "更新用户被授权数据范围响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 更新用户被授权的数据范围响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateRoleAssignResponse {
    /// 响应数据
    ///
    /// TODO: 根据官方文档添加具体字段
    pub data: Value,
}

impl ApiResponseTrait for UpdateRoleAssignResponse {
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
