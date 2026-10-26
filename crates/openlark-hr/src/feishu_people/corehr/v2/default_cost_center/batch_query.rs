//! 查询默认成本中心
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/default_cost_center/batch_query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required_list,
};
use serde::{Deserialize, Serialize};

/// 查询默认成本中心请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchQueryBody {
    /// 员工雇佣 ID 列表（必填，1～100 个）
    pub employment_ids: Vec<String>,
}

/// 查询默认成本中心请求
#[derive(Debug, Clone)]
pub struct BatchQueryRequest {
    /// 配置信息
    config: Config,
    /// 请求体
    body: BatchQueryBody,
    /// 用户 ID 类型（open_id/union_id/user_id/people_corehr_id）
    user_id_type: Option<String>,
}

impl BatchQueryRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: BatchQueryBody::default(),
            user_id_type: None,
        }
    }

    /// 设置请求体。
    pub fn body(mut self, body: BatchQueryBody) -> Self {
        self.body = body;
        self
    }

    /// 设置员工雇佣 ID 列表（必填，1～100 个）。
    pub fn employment_ids(mut self, employment_ids: Vec<String>) -> Self {
        self.body.employment_ids = employment_ids;
        self
    }

    /// 设置用户 ID 类型（open_id/union_id/user_id/people_corehr_id）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchQueryResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchQueryResponse> {
        validate_required_list!(
            self.body.employment_ids,
            100,
            "employment_ids 不能为空且单次最多 100 个"
        );

        let request = ApiRequest::<BatchQueryResponse>::post(
            "/open-apis/corehr/v2/default_cost_centers/batch_query",
        )
        .query_opt("user_id_type", self.user_id_type)
        .body(serde_json::to_value(&self.body)?);

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 查询默认成本中心响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchQueryResponse {
    /// 员工默认成本中心列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<EmployeeDefaultCostCenter>>,
}

/// 员工默认成本中心（employee_default_cost_center）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmployeeDefaultCostCenter {
    /// 用户ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    /// 默认成本中心信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_cost_centers: Option<Vec<EmploymentDefaultCostCenter>>,
}

/// 默认成本中心信息（employment_default_cost_center）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmploymentDefaultCostCenter {
    /// 默认成本中心ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wk_id: Option<String>,
    /// 默认成本中心版本ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wk_tid: Option<String>,
    /// 生效日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 成本中心ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
    /// 任职ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_data_id: Option<String>,
    /// 是否继承自岗位/部门的默认成本中心
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_inherit: Option<bool>,
    /// 继承来源
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inherit_source: Option<EnumValue>,
    /// 变更原因
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// 官方 enum 类型字段结构：{enum_name, display: i18n[]}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnumValue {
    /// 枚举值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_name: Option<String>,
    /// 枚举多语展示
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<Vec<I18nText>>,
}

/// 官方 i18n 多语文本结构：{lang, value}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nText {
    /// 语言编码（IETF BCP 47）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 文本内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl ApiResponseTrait for BatchQueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{body_json, method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v2/default_cost_centers/batch_query
    #[tokio::test]
    async fn test_batch_query_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/corehr/v2/default_cost_centers/batch_query",
            ))
            .and(body_json(json!({
                "employment_ids": ["7475686493613920050"]
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": {
                    "items": [
                        {
                            "employment_id": "7475686493613920050",
                            "default_cost_centers": [
                                {
                                    "wk_id": "703912325303191204",
                                    "wk_tid": "703912321231239801",
                                    "effective_time": "2024-12-01",
                                    "cost_center_id": "7039123253031711012",
                                    "job_data_id": "7039123253031711012",
                                    "is_inherit": true,
                                    "inherit_source": {
                                        "enum_name": "phone_type",
                                        "display": [ { "lang": "zh-CN", "value": "中文示例" } ]
                                    },
                                    "reason": "异动"
                                }
                            ]
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

        let resp = BatchQueryRequest::new(config)
            .employment_ids(vec!["7475686493613920050".to_string()])
            .execute()
            .await
            .expect("请求应成功");

        let items = resp.items.expect("items 应存在");
        assert_eq!(items.len(), 1);
        assert_eq!(
            items[0].employment_id,
            Some("7475686493613920050".to_string())
        );
        let centers = items[0]
            .default_cost_centers
            .as_ref()
            .expect("default_cost_centers 应存在");
        assert_eq!(centers.len(), 1);
        assert_eq!(centers[0].wk_tid, Some("703912321231239801".to_string()));
        let inherit_source = centers[0]
            .inherit_source
            .as_ref()
            .expect("inherit_source 应存在");
        assert_eq!(inherit_source.enum_name, Some("phone_type".to_string()));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
