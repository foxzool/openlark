//! 获取电子签模板列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/signature_template_info_with_thumbnail/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 获取电子签模板列表请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 配置信息
    config: Config,
    body: Option<Value>,
}

impl ListRequest {
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
    pub async fn execute(self) -> SDKResult<ListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListResponse> {
        let mut request = ApiRequest::<ListResponse>::get(
            "/open-apis/corehr/v2/signature_template_info_with_thumbnails",
        );

        if let Some(body) = self.body {
            request = request.body(body);
        }

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 获取电子签模板列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 电子签模板列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<SignatureTemplateInfoWithThumbnailItem>>,
    /// 分页标记
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<i64>,
    /// 数据总数
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}

/// 电子签模板（官方 signature_template_info_with_thumbnail 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignatureTemplateInfoWithThumbnailItem {
    /// id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 名称，支持多语
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<Vec<I18nText>>,
    /// 模版类别
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<EnumValue>,
    /// 模版用途
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub usage: Option<EnumValue>,
    /// 创建日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 修改日期
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<String>,
    /// 创建人
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_by: Option<SignatureUserInfo>,
    /// 修改人
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_by: Option<SignatureUserInfo>,
    /// 缩略图 url
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thumbnail_url: Option<String>,
    /// 模版签署人标签
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signatory_labels: Option<Vec<SignatureSignatoryLabel>>,
    /// 模板编码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_code: Option<String>,
    /// 模板描述
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_desc: Option<String>,
    /// 模板适用区域
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_region_info: Option<SignatureTemplateRegionInfo>,
}

/// 多语文本（官方 i18n 结构：{lang, value}）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nText {
    /// 语言
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
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

/// 电子签模板用户信息（官方 signature_user_info 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignatureUserInfo {
    /// 雇佣 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// 模版签署人标签（官方 signature_signatory_label 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignatureSignatoryLabel {
    /// 电子签模板签订人类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub template_signatory_type: Option<EnumValue>,
    /// 中英文描述
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<Vec<I18nText>>,
    /// 主数据 apiname（官方字段名无下划线）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub apiname: Option<String>,
}

/// 模板适用区域（官方 signature_template_region_info 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignatureTemplateRegionInfo {
    /// 是否全球适用
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_global_scope: Option<String>,
    /// 适用区域名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub meta_infos: Option<Vec<SignatureMetaInfo>>,
}

/// 适用区域元数据（官方 signature_meta_info 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignatureMetaInfo {
    /// 元数据 api_name（官方字段名带下划线）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_name: Option<String>,
    /// wukong id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub wk_id: Option<String>,
    /// 多语描述
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<Vec<I18nText>>,
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

    /// 端到端：GET /open-apis/corehr/v2/signature_template_info_with_thumbnails
    #[tokio::test]
    async fn test_list_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path(
                "/open-apis/corehr/v2/signature_template_info_with_thumbnails",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "items": [
                        {
                            "id": "1",
                            "label": [
                                { "lang": "zh-CN", "value": "张三" }
                            ],
                            "category": {
                                "enum_name": "offer",
                                "display": [
                                    { "lang": "zh-CN", "value": "张三" }
                                ]
                            },
                            "usage": {
                                "enum_name": "onboarding",
                                "display": [
                                    { "lang": "zh-CN", "value": "张三" }
                                ]
                            },
                            "create_time": "2021-12-21",
                            "modify_time": "2021-12-21",
                            "created_by": { "id": "7300476612163913260" },
                            "updated_by": { "id": "7300476612163913260" },
                            "thumbnail_url": "1",
                            "signatory_labels": [
                                {
                                    "template_signatory_type": {
                                        "enum_name": "employee",
                                        "display": [
                                            { "lang": "zh-CN", "value": "张三" }
                                        ]
                                    },
                                    "label": [
                                        { "lang": "zh-CN", "value": "张三" }
                                    ],
                                    "apiname": "status"
                                }
                            ],
                            "template_code": "1234",
                            "template_desc": "desc",
                            "template_region_info": {
                                "is_global_scope": "global",
                                "meta_infos": [
                                    {
                                        "api_name": "status",
                                        "wk_id": "123124124124123",
                                        "label": [
                                            { "lang": "zh-CN", "value": "张三" }
                                        ]
                                    }
                                ]
                            }
                        }
                    ],
                    "page_token": 1000,
                    "count": 1000
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

        let items = resp.items.expect("items 应解码");
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].id.as_deref(), Some("1"));
        assert_eq!(
            items[0]
                .label
                .as_ref()
                .and_then(|labels| labels.first())
                .and_then(|l| l.value.as_deref()),
            Some("张三")
        );
        assert_eq!(
            items[0]
                .category
                .as_ref()
                .and_then(|c| c.enum_name.as_deref()),
            Some("offer")
        );
        assert_eq!(
            items[0].created_by.as_ref().and_then(|u| u.id.as_deref()),
            Some("7300476612163913260")
        );
        let signatory_labels = items[0]
            .signatory_labels
            .as_ref()
            .expect("signatory_labels 应解码");
        assert_eq!(signatory_labels[0].apiname.as_deref(), Some("status"));
        assert_eq!(resp.page_token, Some(1000));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
