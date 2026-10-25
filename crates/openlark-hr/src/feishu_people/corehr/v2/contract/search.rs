//! 搜索合同
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/contract/search>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};

use serde::{Deserialize, Serialize};

use crate::feishu_people::corehr::v2::probation::search::EnumValue;

/// 搜索合同请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SearchRequest {
    /// 分页大小（1-100，默认 20）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 员工 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employee_id: Option<String>,
    /// 合同名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl SearchRequest {
    /// 创建请求
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 设置员工 ID
    pub fn employee_id(mut self, employee_id: String) -> Self {
        self.employee_id = Some(employee_id);
        self
    }

    /// 设置合同名称
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
}

/// 搜索合同响应
///
/// 响应 `data` 子树形状：`{items, page_token, has_more}`（由 decode 层解包）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResponse {
    /// 查询的合同信息列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<ContractItem>>,
    /// 分页标记，当 has_more 为 true 时返回
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 是否还有更多项
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// `ContractItem`（官方 `contract` 结构，多为字段级权限门控，未授权不返回）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContractItem {
    /// 合同 ID
    pub id: String,
    /// 合同开始日期（需 corehr:contract.period:read/write 权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 合同结束日期（需 corehr:contract.period:read/write 权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_end_date: Option<String>,
    /// 实际结束日期（需 corehr:contract.period:read/write 权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    /// 雇佣 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    /// 合同类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_type: Option<EnumValue>,
    /// 合同主体（需 corehr:contract.company:read/write 权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_party_company_id: Option<String>,
    /// 个人信息 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person_id: Option<String>,
    /// 期限类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration_type: Option<EnumValue>,
    /// 合同编号
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_number: Option<String>,
    /// 签订类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signing_type: Option<EnumValue>,
    /// 合同协议状态
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contract_status: Option<EnumValue>,
    /// 续签状态
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renewal_status: Option<EnumValue>,
    /// 第几次签署
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signing_times: Option<i64>,
}

impl ApiResponseTrait for SearchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 搜索合同请求构建器
#[derive(Debug, Clone)]
pub struct SearchRequestBuilder {
    config: Config,
    request: SearchRequest,
}

impl SearchRequestBuilder {
    /// 创建请求构建器
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request: SearchRequest::new(),
        }
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request = self.request.page_size(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: String) -> Self {
        self.request = self.request.page_token(page_token);
        self
    }

    /// 设置员工 ID
    pub fn employee_id(mut self, employee_id: String) -> Self {
        self.request = self.request.employee_id(employee_id);
        self
    }

    /// 设置合同名称
    pub fn name(mut self, name: String) -> Self {
        self.request = self.request.name(name);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<SearchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<SearchResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV2;

        // 构建端点
        let api_endpoint = FeishuPeopleApiV2::ContractSearch;
        let request = ApiRequest::<SearchResponse>::post(api_endpoint.to_url());

        // 序列化请求体
        let request = request.body(serde_json::to_value(&self.request).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?);

        // 发送请求
        Transport::request_typed(request, &self.config, Some(option), "搜索合同响应数据为空").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v2/contracts/search
    #[tokio::test]
    async fn test_search_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v2/contracts/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "id": "7147527056140813828",
                            "effective_time": "2023-01-01 00:00:00",
                            "contract_end_date": "2024-01-01",
                            "expiration_time": "2023-11-01 00:00:00",
                            "employment_id": "6893014062142064135",
                            "contract_type": {
                                "enum_name": "labor_contract",
                                "display": [
                                    { "lang": "zh-CN", "value": "劳动合同" }
                                ]
                            },
                            "first_party_company_id": "7091599096804394540",
                            "person_id": "7088589447189022252",
                            "duration_type": {
                                "enum_name": "fix_term",
                                "display": [
                                    { "lang": "zh-CN", "value": "固定期限" }
                                ]
                            },
                            "contract_number": "HT-2023-001",
                            "signing_type": {
                                "enum_name": "new_sign",
                                "display": [
                                    { "lang": "zh-CN", "value": "新签" }
                                ]
                            },
                            "contract_status": {
                                "enum_name": "effective",
                                "display": [
                                    { "lang": "zh-CN", "value": "生效中" }
                                ]
                            },
                            "renewal_status": {
                                "enum_name": "not_renewal",
                                "display": [
                                    { "lang": "zh-CN", "value": "不续签" }
                                ]
                            },
                            "signing_times": 1
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

        let resp = SearchRequestBuilder::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let item = resp
            .items
            .as_ref()
            .and_then(|items| items.first())
            .expect("items 应包含合同");
        assert_eq!(item.id, "7147527056140813828");
        assert_eq!(item.effective_time.as_deref(), Some("2023-01-01 00:00:00"));
        assert_eq!(item.contract_end_date.as_deref(), Some("2024-01-01"));
        assert_eq!(item.expiration_time.as_deref(), Some("2023-11-01 00:00:00"));
        assert_eq!(item.employment_id.as_deref(), Some("6893014062142064135"));
        assert_eq!(
            item.contract_type
                .as_ref()
                .and_then(|t| t.enum_name.as_deref()),
            Some("labor_contract")
        );
        assert_eq!(item.signing_times, Some(1));
        assert_eq!(resp.has_more, Some(false));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
