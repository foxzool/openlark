//! 为指定数据项创建索引
//! docPath: <https://open.feishu.cn/document/server-docs/search-v2/open-search/data_source-item/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 数据项访问权限控制（acl）。
///
/// 空数组表示数据不可见；全员可见需 `access="allow"`、`type="user"`、`value="everyone"`。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ItemAcl {
    /// 权限类型，优先级：Deny > Allow（可选值：allow / deny）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<String>,
    /// 权限值，例如 user_id，依赖 type 描述。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// 权限值类型（可选值：user / open_id；group 已下线）。
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

/// 数据项元信息（item_metadata）。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ItemMetadata {
    /// 该条数据记录对应的标题（必填）。
    pub title: String,
    /// 该条数据记录对应的跳转 URL（必填）。
    pub source_url: String,
    /// 数据项的创建时间，Unix 时间，单位为秒。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 数据项的更新时间，Unix 时间，单位为秒。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    /// 移动端搜索命中的跳转地址。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url_mobile: Option<String>,
}

/// 非结构化数据内容（item_content），飞书搜索会用它做召回。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ItemContent {
    /// 内容的格式（可选值：html / plaintext）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// 全文数据。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_data: Option<String>,
}

/// 为指定数据项创建索引请求体。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateDataSourceItemBody {
    /// item 在 datasource 中的唯一标识，只允许英文字母、数字和下划线，最大长度 128 字符（必填）。
    pub id: String,
    /// item 的访问权限控制（必填；空数组=数据不可见）。
    pub acl: Vec<ItemAcl>,
    /// item 的元信息（必填）。
    pub metadata: ItemMetadata,
    /// 结构化数据，以 json 字符串传递（必填）。
    pub structured_data: String,
    /// 非结构化数据，如文档文本。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<ItemContent>,
}

/// 为指定数据项创建索引请求。
#[derive(Debug, Clone)]
pub struct CreateDataSourceItemRequest {
    config: Arc<Config>,
    data_source_id: String,
    body: CreateDataSourceItemBody,
}

/// 为指定数据项创建索引响应。
///
/// 官方成功响应 `data` 为空对象（或缺省），显式给出空成功值。
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CreateDataSourceItemResponse {}

impl ApiResponseTrait for CreateDataSourceItemResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }

    /// 官方成功响应 `data` 为空对象（或缺省），显式给出空成功值
    fn empty_success() -> Option<Self> {
        Some(Self::default())
    }
}

impl CreateDataSourceItemRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, data_source_id: impl Into<String>) -> Self {
        Self {
            config,
            data_source_id: data_source_id.into(),
            body: CreateDataSourceItemBody::default(),
        }
    }

    /// 设置 item 在 datasource 中的唯一标识（必填，≤128 字符）。
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.body.id = id.into();
        self
    }

    /// 设置访问权限控制列表（必填；空数组=数据不可见）。
    pub fn acl(mut self, acl: Vec<ItemAcl>) -> Self {
        self.body.acl = acl;
        self
    }

    /// 追加一条访问权限控制。
    pub fn add_acl(mut self, acl: ItemAcl) -> Self {
        self.body.acl.push(acl);
        self
    }

    /// 设置 item 元信息（必填，title/source_url 必填）。
    pub fn metadata(mut self, metadata: ItemMetadata) -> Self {
        self.body.metadata = metadata;
        self
    }

    /// 设置结构化数据，以 json 字符串传递（必填）。
    pub fn structured_data(mut self, structured_data: impl Into<String>) -> Self {
        self.body.structured_data = structured_data.into();
        self
    }

    /// 设置非结构化数据内容。
    pub fn content(mut self, content: ItemContent) -> Self {
        self.body.content = Some(content);
        self
    }

    /// 执行为指定数据项创建索引请求。
    pub async fn execute(self) -> SDKResult<CreateDataSourceItemResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行为指定数据项创建索引请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CreateDataSourceItemResponse> {
        validate_required!(self.data_source_id, "data_source_id 不能为空");
        validate_required!(self.body.id, "id 不能为空");
        // acl 允许空数组（官方语义：空数组=数据不可见），不做非空校验
        validate_required!(self.body.metadata.title, "metadata.title 不能为空");
        validate_required!(
            self.body.metadata.source_url,
            "metadata.source_url 不能为空"
        );
        validate_required!(self.body.structured_data, "structured_data 不能为空");
        let path = format!(
            "/open-apis/search/v2/data_sources/{}/items",
            self.data_source_id
        );
        let req: ApiRequest<CreateDataSourceItemResponse> =
            ApiRequest::post(&path).body(serde_json::to_value(&self.body)?);

        Transport::request_typed(req, &self.config, Some(option), "为指定数据项创建索引").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST .../data_sources/{data_source_id}/items + 官方形状请求体 → 响应解析。
    #[tokio::test]
    async fn test_create_data_source_item_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/search/v2/data_sources/ds_001/items"))
            .and(wiremock::matchers::body_partial_json(json!({
                "id": "my_item_01010111",
                "acl": [
                    {
                        "access": "allow",
                        "value": "everyone",
                        "type": "user"
                    }
                ],
                "metadata": {
                    "title": "工单：无法创建文章",
                    "source_url": "http://www.abc.com.cn"
                },
                "structured_data": "{\"description\":\"问题出现的环境和复现方法描述\"}"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {}
            })))
            .mount(&server)
            .await;

        let config = std::sync::Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        CreateDataSourceItemRequest::new(config, "ds_001")
            .id("my_item_01010111")
            .acl(vec![ItemAcl {
                access: Some("allow".to_string()),
                value: Some("everyone".to_string()),
                type_: Some("user".to_string()),
            }])
            .metadata(ItemMetadata {
                title: "工单：无法创建文章".to_string(),
                source_url: "http://www.abc.com.cn".to_string(),
                create_time: Some(1618831236),
                update_time: None,
                source_url_mobile: None,
            })
            .structured_data("{\"description\":\"问题出现的环境和复现方法描述\"}")
            .execute()
            .await
            .expect("为指定数据项创建索引应成功");
        // 官方成功响应 data 为空对象，execute 成功即通过

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/search/v2/data_sources/ds_001/items"
        );
        // 下发的请求体与官方文档字段表对齐（rename 的 type / 必填项均到位）
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["id"], "my_item_01010111");
        assert_eq!(sent["acl"][0]["type"], "user");
        assert_eq!(sent["metadata"]["title"], "工单：无法创建文章");
        assert_eq!(sent["metadata"]["create_time"], 1618831236);
    }

    /// 未设置可选 content 时 body 不携带该键（不发虚构字段）。
    #[test]
    fn test_create_data_source_item_body_omits_absent_fields() {
        let body = CreateDataSourceItemBody::default();
        let value = serde_json::to_value(&body).unwrap();
        assert!(value.get("content").is_none());
        // acl 为必填字段：空数组也必须在 wire 上出现（官方语义：空数组=数据不可见）
        assert_eq!(value["acl"], json!([]));
    }
}
