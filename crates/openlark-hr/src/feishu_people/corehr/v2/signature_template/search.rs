//! 获取电子签模板内容
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/signature_template/search>

use super::models::SignatureTemplate;
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 获取电子签模板内容请求
#[derive(Debug, Clone)]
pub struct SearchRequest {
    /// 配置信息
    config: Config,
    body: Option<Value>,
}

impl SearchRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self { config, body: None }
    }

    /// 设置请求体。
    pub fn body(mut self, body: Value) -> Self {
        self.body = Some(body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<SearchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<SearchResponse> {
        let mut request =
            ApiRequest::<SearchResponse>::get("/open-apis/corehr/v2/signature_templates/search");

        if let Some(body) = self.body {
            request = request.body(body);
        }

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 获取电子签模板内容响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SearchResponse {
    /// 返回的电子签模板列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature_templates: Option<Vec<SignatureTemplate>>,
}

impl ApiResponseTrait for SearchResponse {
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

    /// 端到端：GET /open-apis/corehr/v2/signature_templates/search
    /// （官方 data 内层键为 signature_templates，含 brief_info / content_info）
    #[tokio::test]
    async fn test_search_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v2/signature_templates/search"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "signature_templates": [
                        {
                            "id": "7246307700768902956",
                            "brief_info": {
                                "id": "1",
                                "label": [
                                    { "lang": "zh-CN", "value": "合同续约模板" }
                                ],
                                "category": {
                                    "enum_name": "contract_agreement",
                                    "display": [ { "lang": "zh-CN", "value": "人事合同 / 协议" } ]
                                },
                                "usage": {
                                    "enum_name": "offboarding",
                                    "display": [ { "lang": "zh-CN", "value": "离职" } ]
                                },
                                "signatory_labels": [
                                    {
                                        "template_signatory_type": {
                                            "enum_name": "Person",
                                            "display": [ { "lang": "zh-CN", "value": "中文示例" } ]
                                        },
                                        "label": [ { "lang": "zh-CN", "value": "中文示例" } ],
                                        "apiname": "status"
                                    }
                                ],
                                "active": true,
                                "create_by": "123123123123123123",
                                "modify_by": "7287500478366090796",
                                "applicability": {
                                    "enum_name": "signature",
                                    "display": [ { "lang": "zh-CN", "value": "电子签" } ]
                                },
                                "creation_method": "online_edit",
                                "version": "v1",
                                "update_time": "31231232123",
                                "create_time": "3123123211",
                                "template_setting": {
                                    "page_seal_types": [ "page_seal" ]
                                },
                                "template_region_info": {
                                    "is_global_scope": "global",
                                    "meta_infos": [
                                        {
                                            "api_name": "7246307700768902956",
                                            "wk_id": "7246307700768902956",
                                            "label": [ { "lang": "zh-CN", "value": "中文示例" } ]
                                        }
                                    ]
                                },
                                "template_code": "7246307700768902956",
                                "template_desc": [ { "lang": "zh-CN", "value": "中文示例" } ]
                            },
                            "content_info": {
                                "custom_fields": [
                                    {
                                        "label": [ { "lang": "zh-CN", "value": "中文示例" } ],
                                        "apiname": "status",
                                        "custom_field_type": {
                                            "enum_name": "phone_type",
                                            "display": [ { "lang": "zh-CN", "value": "中文示例" } ]
                                        },
                                        "used": true,
                                        "is_required": true,
                                        "custom_desc": "desc",
                                        "common_info": {
                                            "source": {
                                                "enum_name": "phone_type",
                                                "display": [ { "lang": "zh-CN", "value": "中文示例" } ]
                                            },
                                            "apiname": "status"
                                        }
                                    }
                                ],
                                "system_setting_fields": [
                                    {
                                        "field_type": {
                                            "enum_name": "phone_type",
                                            "display": [ { "lang": "zh-CN", "value": "中文示例" } ]
                                        },
                                        "label": [ { "lang": "zh-CN", "value": "中文示例" } ],
                                        "common_info": {
                                            "source": {
                                                "enum_name": "System",
                                                "display": [ { "lang": "zh-CN", "value": "中文示例" } ]
                                            },
                                            "apiname": "status"
                                        }
                                    }
                                ]
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

        let resp = SearchRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let templates = resp
            .signature_templates
            .expect("signature_templates 应解码");
        assert_eq!(templates.len(), 1);
        assert_eq!(templates[0].id.as_deref(), Some("7246307700768902956"));
        let brief = templates[0].brief_info.as_ref().expect("brief_info 应解码");
        assert_eq!(
            brief
                .label
                .as_ref()
                .and_then(|l| l.first())
                .and_then(|t| t.value.clone()),
            Some("合同续约模板".to_string())
        );
        assert_eq!(
            brief.usage.as_ref().and_then(|u| u.enum_name.clone()),
            Some("offboarding".to_string())
        );
        let signatory = brief
            .signatory_labels
            .as_ref()
            .and_then(|s| s.first())
            .expect("signatory_labels 应解码");
        assert_eq!(
            signatory
                .template_signatory_type
                .as_ref()
                .and_then(|t| t.enum_name.clone()),
            Some("Person".to_string())
        );
        let custom_fields = templates[0]
            .content_info
            .as_ref()
            .and_then(|c| c.custom_fields.as_ref())
            .expect("custom_fields 应解码");
        assert_eq!(custom_fields.len(), 1);
        assert_eq!(custom_fields[0].apiname.as_deref(), Some("status"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
