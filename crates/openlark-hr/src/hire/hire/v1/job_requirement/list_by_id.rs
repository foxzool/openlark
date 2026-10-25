//! 获取招聘需求信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/job_requirement/list_by_id>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::hire::hire::common_models::JobRequirementDto;

/// 获取招聘需求信息请求
#[derive(Debug, Clone)]
pub struct ListByIdRequest {
    request_body: ListByIdRequestBody,
    /// 配置信息
    config: Config,
}

impl ListByIdRequest {
    /// 创建请求
    pub fn new(config: Config, request_body: ListByIdRequestBody) -> Self {
        Self {
            request_body,
            config,
        }
    }

    /// 设置 `request_body`。
    pub fn request_body(mut self, request_body: ListByIdRequestBody) -> Self {
        self.request_body = request_body;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListByIdResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListByIdResponse> {
        use crate::common::api_endpoints::HireApiV1;

        self.request_body.validate()?;

        let api_endpoint = HireApiV1::JobRequirementListById;
        let request = ApiRequest::<ListByIdResponse>::post(api_endpoint.to_url());
        let request = request.body(serde_json::to_value(&self.request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?);
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取招聘需求信息响应数据为空",
        )
        .await
    }
}

/// `ListByIdRequestBody`。
///
/// `id_list` 与 `short_code_list` 不允许同时使用（否则报错 1002725），
/// 两个字段均不传时返回空；单次列表最多 100 条。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListByIdRequestBody {
    /// 招聘需求 ID 列表，详情参考获取招聘需求列表接口，单次最多 100 条。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_list: Option<Vec<String>>,
    /// 招聘需求编号列表，与 `id_list` 互斥，单次最多 100 条。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub short_code_list: Option<Vec<String>>,
}

impl ListByIdRequestBody {
    /// 创建新的请求实例。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置招聘需求 ID 列表（与 `short_code_list` 互斥）。
    pub fn id_list(mut self, value: Vec<String>) -> Self {
        self.id_list = Some(value);
        self
    }

    /// 设置招聘需求编号列表（与 `id_list` 互斥）。
    pub fn short_code_list(mut self, value: Vec<String>) -> Self {
        self.short_code_list = Some(value);
        self
    }

    fn validate(&self) -> SDKResult<()> {
        if let Some(ids) = &self.id_list
            && ids.len() > 100
        {
            return Err(openlark_core::error::validation_error(
                "id_list 超出上限",
                "id_list 单次最多传入 100 条",
            ));
        }
        if let Some(codes) = &self.short_code_list
            && codes.len() > 100
        {
            return Err(openlark_core::error::validation_error(
                "short_code_list 超出上限",
                "short_code_list 单次最多传入 100 条",
            ));
        }
        if self.id_list.is_some() && self.short_code_list.is_some() {
            return Err(openlark_core::error::validation_error(
                "id_list 与 short_code_list 互斥",
                "id_list 与 short_code_list 不允许同时使用（错误码 1002725），请只传其一",
            ));
        }

        Ok(())
    }
}

/// 获取招聘需求信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListByIdResponse {
    #[serde(default, alias = "job_requirements")]
    /// 结果项列表（官方 `job_requirement_dto[]`）。
    pub items: Vec<JobRequirementDto>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 下一页分页标记。
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 是否还有更多结果。
    pub has_more: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for ListByIdResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/hire/v1/job_requirements/search
    #[tokio::test]
    async fn test_list_by_id_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/hire/v1/job_requirements/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": {
                    "job_requirements": [
                        {
                            "id": "6949805467799537964",
                            "short_code": "xx1",
                            "name": "test",
                            "display_progress": 1,
                            "head_count": 11
                        }
                    ],
                    "has_more": false
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = ListByIdRequest::new(
            config,
            ListByIdRequestBody::new().short_code_list(vec!["R00407".to_string()]),
        )
        .execute()
        .await
        .expect("请求应成功");
        assert_eq!(resp.items.len(), 1);
        let item = &resp.items[0];
        assert_eq!(item.id.as_deref(), Some("6949805467799537964"));
        assert_eq!(item.short_code.as_deref(), Some("xx1"));
        assert_eq!(item.name.as_deref(), Some("test"));
        assert_eq!(item.display_progress, Some(1));
        assert_eq!(item.head_count, Some(11));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let wire_body: serde_json::Value =
            serde_json::from_slice(&received[0].body).expect("请求体应为合法 JSON");
        assert_eq!(wire_body, json!({ "short_code_list": ["R00407"] }));
    }

    /// `id_list` 与 `short_code_list` 同时传入时应快速失败。
    #[tokio::test]
    async fn test_list_by_id_rejects_mutually_exclusive_lists() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build();

        let result = ListByIdRequest::new(
            config,
            ListByIdRequestBody::new()
                .id_list(vec!["6952811314255265796".to_string()])
                .short_code_list(vec!["R00407".to_string()]),
        )
        .execute()
        .await;
        assert!(result.is_err());
    }
}
