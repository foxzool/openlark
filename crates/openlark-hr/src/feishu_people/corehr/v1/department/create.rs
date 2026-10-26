//! 创建部门
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/department/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};

use super::models::{CreateRequestBody, CreateResponse, HiberarchyCommon};

/// 创建部门请求
#[derive(Debug, Clone)]
pub struct CreateRequest {
    /// 配置信息
    config: Config,
    /// 部门名称（必填）
    name: String,
    /// 组织实体公共字段（必填：名称、启停用状态等基础信息）
    hiberarchy_common: Option<HiberarchyCommon>,
    /// 版本生效日期（格式：YYYY-MM-DD 00:00:00，必填）
    effective_time: String,
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
            hiberarchy_common: None,
            effective_time: String::new(),
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

    /// 设置组织实体公共字段（必填，含多语名称、启停用状态、编码、描述等）
    pub fn hiberarchy_common(mut self, hiberarchy_common: HiberarchyCommon) -> Self {
        self.hiberarchy_common = Some(hiberarchy_common);
        self
    }

    /// 设置版本生效日期（必填，格式：YYYY-MM-DD 00:00:00）
    pub fn effective_time(mut self, effective_time: String) -> Self {
        self.effective_time = effective_time;
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
        validate_required!(self.effective_time.trim(), "版本生效日期不能为空");
        let hiberarchy_common = self.hiberarchy_common.ok_or_else(|| {
            openlark_core::error::validation_error(
                "hiberarchy_common 不能为空",
                "创建部门必须提供组织实体公共字段（名称、启停用状态等）",
            )
        })?;

        // 2. 构建端点
        let api_endpoint = FeishuPeopleApiV1::DepartmentCreate;
        let request = ApiRequest::<CreateResponse>::post(api_endpoint.to_url());

        // 3. 序列化请求体
        let request_body = CreateRequestBody {
            name: self.name,
            hiberarchy_common,
            effective_time: self.effective_time,
            parent_department_id: self.parent_department_id,
            leader_user_ids: self.leader_user_ids,
            code: self.code,
            description: self.description,
        };
        let request = request.body(serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?);

        // 4. 发送请求
        Transport::request_typed(request, &self.config, Some(option), "创建部门响应数据为空").await
    }
}

impl ApiResponseTrait for CreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{body_partial_json, method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v1/departments
    #[tokio::test]
    async fn test_create_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v1/departments"))
            .and(body_partial_json(json!({
                "hiberarchy_common": {
                    "name": [{ "lang": "zh-CN", "value": "研发部" }],
                    "active": true
                },
                "effective_time": "2020-05-01 00:00:00"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success", "data": { "department_id": "" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        CreateRequest::new(config)
            .name("test001".to_string())
            .hiberarchy_common(HiberarchyCommon {
                name: Some(vec![super::super::models::I18n {
                    lang: Some("zh-CN".to_string()),
                    value: Some("研发部".to_string()),
                }]),
                active: Some(true),
                ..Default::default()
            })
            .effective_time("2020-05-01 00:00:00".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
