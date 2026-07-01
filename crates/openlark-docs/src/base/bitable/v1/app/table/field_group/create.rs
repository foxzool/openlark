//! 创建字段编组
//!
//! docPath: <https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/app-table-field_group/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::common::api_endpoints::BitableApiV1;

/// 字段编组成员
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FieldGroupChild {
    /// 编组成员类型，当前仅支持 field
    #[serde(rename = "type")]
    pub child_type: String,
    /// 编组成员 ID
    pub id: String,
}

/// 创建字段编组项
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateFieldGroupItem {
    /// 字段编组 ID，不传时由系统生成
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 字段编组名称
    pub name: String,
    /// 字段编组成员
    pub children: Vec<FieldGroupChild>,
    /// 字段编组描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct CreateFieldGroupRequestBody {
    field_groups: Vec<CreateFieldGroupItem>,
}

/// 创建字段编组响应（data）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateFieldGroupResponse {
    /// 字段编组内容
    pub field_groups: Value,
}

impl ApiResponseTrait for CreateFieldGroupResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建字段编组请求
#[derive(Debug, Clone)]
pub struct CreateFieldGroupRequest {
    config: Config,
    app_token: String,
    table_id: String,
    field_groups: Vec<CreateFieldGroupItem>,
}

impl CreateFieldGroupRequest {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            table_id: String::new(),
            field_groups: Vec::new(),
        }
    }

    /// 设置 `app_token`。
    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    /// 设置 `table_id`。
    pub fn table_id(mut self, table_id: impl Into<String>) -> Self {
        self.table_id = table_id.into();
        self
    }

    /// 设置 `field_groups`。
    pub fn field_groups(mut self, field_groups: Vec<CreateFieldGroupItem>) -> Self {
        self.field_groups = field_groups;
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<CreateFieldGroupResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CreateFieldGroupResponse> {
        validate_required!(self.app_token.trim(), "app_token");
        validate_required!(self.table_id.trim(), "table_id");

        if self.field_groups.is_empty() {
            return Err(openlark_core::error::validation_error(
                "field_groups",
                "field_groups 不能为空",
            ));
        }

        let api_endpoint = BitableApiV1::FieldGroupCreate(self.app_token, self.table_id);
        let body = CreateFieldGroupRequestBody {
            field_groups: self.field_groups,
        };
        let api_request: ApiRequest<CreateFieldGroupResponse> =
            ApiRequest::post(&api_endpoint.to_url()).body(serde_json::to_vec(&body)?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use openlark_core::testing::prelude::test_runtime;

    #[test]
    fn test_empty_field_groups() {
        let config = Config::default();
        let request = CreateFieldGroupRequest::new(config)
            .app_token("app_token")
            .table_id("table_id")
            .field_groups(Vec::new());
        let rt = test_runtime();
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            CreateFieldGroupResponse::data_format(),
            ResponseFormat::Data
        );
    }
}
