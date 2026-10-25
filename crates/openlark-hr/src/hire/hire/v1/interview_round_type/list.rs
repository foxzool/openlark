//! 获取面试轮次类型列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/interview_round_type/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::common::shared_models::I18nText;

/// `ListRequestBody`。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    page_token: Option<String>,
}

/// 获取面试轮次类型列表请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 配置信息
    config: Config,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置分页大小。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记。
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListResponse> {
        use crate::common::api_endpoints::HireApiV1;

        if let Some(size) = self.page_size
            && !(1..=100).contains(&size)
        {
            return Err(openlark_core::error::validation_error(
                "分页大小超出范围",
                "page_size 必须在 1-100 之间",
            ));
        }

        let api_endpoint = HireApiV1::InterviewRoundTypeList;
        let request = ApiRequest::<ListResponse>::get(api_endpoint.to_url());
        let request_body = ListRequestBody {
            page_size: self.page_size,
            page_token: self.page_token,
        };
        let request = request.body(serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?);
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取面试轮次类型列表响应数据为空",
        )
        .await
    }
}

/// 面试评价表（官方 `interview_round_type_assessment_template`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewRoundTypeAssessmentTemplate {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试评价表 ID。在面试评价表更新时，评价表 ID 也会更新。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试评价表对应的业务 ID。在面试评价表更新时，业务 ID 保持不变。
    pub biz_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试评价表名称。
    pub name: Option<I18nText>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 面试轮次类型（官方 `interview_round_type`）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InterviewRoundType {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试轮次类型 ID。在面试轮次类型更新时，类型 ID 也会更新。
    pub id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试轮次类型对应的业务 ID。在面试轮次类型更新时，业务 ID 保持不变。
    pub biz_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试轮次类型名称。
    pub name: Option<I18nText>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 职位流程类型：1 社招流程 / 2 校招流程。
    pub process_type: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 启用状态：1 启用 / 2 未启用。
    pub active_status: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试评价表。
    pub interview_assessment_template_info: Option<InterviewRoundTypeAssessmentTemplate>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// 获取面试轮次类型列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListResponse {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    /// 面试轮次类型启用状态：1 已启用 / 2 未启用。
    pub active_status: Option<i32>,
    #[serde(default)]
    /// 面试轮次类型列表。
    pub items: Vec<InterviewRoundType>,
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

impl ApiResponseTrait for ListResponse {
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

    /// 端到端：GET /open-apis/hire/v1/interview_round_types
    #[tokio::test]
    async fn test_list_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/hire/v1/interview_round_types"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "ok", "data": {
                    "active_status": 1,
                    "items": [
                        {
                            "id": "7012129842917869868",
                            "biz_id": "7012129842917869868",
                            "name": { "zh_cn": "默认面试轮次类型", "en_us": "default interview round type" },
                            "process_type": 1,
                            "active_status": 1,
                            "interview_assessment_template_info": {
                                "id": "7012129842917869868",
                                "biz_id": "7012129842917869868",
                                "name": { "zh_cn": "默认面试评价表", "en_us": "default interview assessment template" }
                            }
                        }
                    ]
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

        let resp = ListRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");
        assert_eq!(resp.active_status, Some(1));
        assert_eq!(resp.items.len(), 1);
        let item = &resp.items[0];
        assert_eq!(item.id.as_deref(), Some("7012129842917869868"));
        assert_eq!(item.biz_id.as_deref(), Some("7012129842917869868"));
        assert_eq!(item.process_type, Some(1));
        assert_eq!(
            item.name.as_ref().and_then(|n| n.zh_cn.as_deref()),
            Some("默认面试轮次类型")
        );
        let template = item
            .interview_assessment_template_info
            .as_ref()
            .expect("interview_assessment_template_info 应存在");
        assert_eq!(template.biz_id.as_deref(), Some("7012129842917869868"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
