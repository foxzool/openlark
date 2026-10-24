//! 创建数据知识
//!
//! docPath: <https://open.feishu.cn/document/aily-v1/data-knowledge/data-knowledge-management/create>

use crate::endpoints::AILY_V1_DATA_ASSETS;
use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, validate_required,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 知识切片配置（官方 data_asset_knowledge_chunk_setting）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataAssetKnowledgeChunkSetting {
    /// 切片规则：`separator`（按标识符）/ `intelligent`（智能切片）。
    pub rule_type: String,
    /// 切片分割符类型：`paragraph`（段落分隔符）/ `title`（标题分割符）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub separate_type: Option<String>,
    /// 分段最大长度（字符），范围 200~1000，按标识符切片时必须填写。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i32>,
    /// 分段重叠字符数，范围 0~200，按标识符切片时必须填写，不能超过 size 的数值。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overlap: Option<i32>,
}

impl DataAssetKnowledgeChunkSetting {
    /// 创建新的知识切片配置。
    pub fn new(rule_type: impl Into<String>) -> Self {
        Self {
            rule_type: rule_type.into(),
            separate_type: None,
            size: None,
            overlap: None,
        }
    }

    /// 设置切片分割符类型。
    pub fn separate_type(mut self, separate_type: impl Into<String>) -> Self {
        self.separate_type = Some(separate_type.into());
        self
    }

    /// 设置分段最大长度。
    pub fn size(mut self, size: i32) -> Self {
        self.size = Some(size);
        self
    }

    /// 设置分段重叠字符数。
    pub fn overlap(mut self, overlap: i32) -> Self {
        self.overlap = Some(overlap);
        self
    }
}

/// 知识导入-文件（官方 data_asset_import_knowledge_file）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataAssetImportKnowledgeFile {
    /// 文件标题。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 上传文件获取到的 token。和 content 二选一，优先使用 token。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// 文件内容。和 token 二选一，优先使用 token。有长度限制，大文件优先使用 token 方式。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 文件内容对应的 MIME 类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// 文件源的 URL。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl DataAssetImportKnowledgeFile {
    /// 创建新的知识导入文件配置。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置文件标题。
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 设置上传文件获取到的 token。
    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// 设置文件内容。
    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.content = Some(content.into());
        self
    }

    /// 设置 MIME 类型。
    pub fn mime_type(mut self, mime_type: impl Into<String>) -> Self {
        self.mime_type = Some(mime_type.into());
        self
    }

    /// 设置文件源的 URL。
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
}

/// 知识导入-飞书云文档（官方 data_asset_import_knowledge_lark_doc）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataAssetImportKnowledgeLarkDoc {
    /// 云文档类型：`doc`/`file`/`wiki`/`docx`/`folder`。
    pub r#type: String,
    /// 云文档 token，可以通过搜索云文档 API 获取。
    pub token: String,
    /// 是否包含子文档，只有 wiki 类型的云文档支持。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_sub_docs: Option<bool>,
}

impl DataAssetImportKnowledgeLarkDoc {
    /// 创建新的知识导入云文档配置。
    pub fn new(r#type: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            r#type: r#type.into(),
            token: token.into(),
            with_sub_docs: None,
        }
    }

    /// 设置是否包含子文档。
    pub fn with_sub_docs(mut self, with_sub_docs: bool) -> Self {
        self.with_sub_docs = Some(with_sub_docs);
        self
    }
}

/// 知识导入-飞书知识库-子文档（官方 data_asset_import_knowledge_wiki_sub_doc）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataAssetImportKnowledgeWikiSubDoc {
    /// 云文档类型，只支持 wiki 中的云文档。
    pub r#type: String,
    /// 云文档 token，可以通过搜索云文档 API 获取。
    pub token: String,
    /// 云文档链接。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl DataAssetImportKnowledgeWikiSubDoc {
    /// 创建新的知识库子文档配置。
    pub fn new(r#type: impl Into<String>, token: impl Into<String>) -> Self {
        Self {
            r#type: r#type.into(),
            token: token.into(),
            url: None,
        }
    }

    /// 设置云文档链接。
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
}

/// 知识导入-飞书知识空间（官方 data_asset_import_knowledge_wiki）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataAssetImportKnowledgeWiki {
    /// 飞书知识空间 ID，可以通过搜索 Wiki API 获取。
    pub space_id: String,
    /// 指定知识空间子节点时使用。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_docs: Option<Vec<DataAssetImportKnowledgeWikiSubDoc>>,
    /// 知识空间 URL。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl DataAssetImportKnowledgeWiki {
    /// 创建新的知识导入知识空间配置。
    pub fn new(space_id: impl Into<String>) -> Self {
        Self {
            space_id: space_id.into(),
            sub_docs: None,
            url: None,
        }
    }

    /// 设置知识空间子节点。
    pub fn sub_docs(mut self, sub_docs: Vec<DataAssetImportKnowledgeWikiSubDoc>) -> Self {
        self.sub_docs = Some(sub_docs);
        self
    }

    /// 设置知识空间 URL。
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }
}

/// 知识导入-飞书服务台（官方 data_asset_import_knowledge_helpdesk）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataAssetImportKnowledgeHelpdesk {
    /// 飞书服务台 ID。
    pub helpdesk_id: String,
}

impl DataAssetImportKnowledgeHelpdesk {
    /// 创建新的知识导入服务台配置。
    pub fn new(helpdesk_id: impl Into<String>) -> Self {
        Self {
            helpdesk_id: helpdesk_id.into(),
        }
    }
}

/// 知识导入配置（官方 data_asset_import_knowledge_setting）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DataAssetImportKnowledgeSetting {
    /// 知识切片配置。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chunk_setting: Option<DataAssetKnowledgeChunkSetting>,
    /// 知识导入-文件。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<DataAssetImportKnowledgeFile>,
    /// 知识导入-飞书云文档。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lark_doc: Option<DataAssetImportKnowledgeLarkDoc>,
    /// 知识导入-飞书知识空间。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lark_wiki_space: Option<DataAssetImportKnowledgeWiki>,
    /// 知识导入-飞书服务台。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lark_helpdesk: Option<DataAssetImportKnowledgeHelpdesk>,
}

impl DataAssetImportKnowledgeSetting {
    /// 创建新的知识导入配置。
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置知识切片配置。
    pub fn chunk_setting(mut self, chunk_setting: DataAssetKnowledgeChunkSetting) -> Self {
        self.chunk_setting = Some(chunk_setting);
        self
    }

    /// 设置知识导入-文件。
    pub fn file(mut self, file: DataAssetImportKnowledgeFile) -> Self {
        self.file = Some(file);
        self
    }

    /// 设置知识导入-飞书云文档。
    pub fn lark_doc(mut self, lark_doc: DataAssetImportKnowledgeLarkDoc) -> Self {
        self.lark_doc = Some(lark_doc);
        self
    }

    /// 设置知识导入-飞书知识空间。
    pub fn lark_wiki_space(mut self, lark_wiki_space: DataAssetImportKnowledgeWiki) -> Self {
        self.lark_wiki_space = Some(lark_wiki_space);
        self
    }

    /// 设置知识导入-飞书服务台。
    pub fn lark_helpdesk(mut self, lark_helpdesk: DataAssetImportKnowledgeHelpdesk) -> Self {
        self.lark_helpdesk = Some(lark_helpdesk);
        self
    }
}

/// 创建数据知识请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDataAssetBody {
    /// 连接类型：`import`（导入模式）/ `direct`（直连模式）。
    pub connect_type: String,
    /// 数据源类型：`file`/`lark_wiki_space`/`lark_doc`/`lark_helpdesk`。
    pub source_type: String,
    /// 知识导入配置。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_knowledge_setting: Option<DataAssetImportKnowledgeSetting>,
    /// 数据知识描述信息，key 为语言编码（如 `zh_cn`）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<HashMap<String, String>>,
}

impl CreateDataAssetBody {
    /// 创建新的数据知识请求体。
    pub fn new(connect_type: impl Into<String>, source_type: impl Into<String>) -> Self {
        Self {
            connect_type: connect_type.into(),
            source_type: source_type.into(),
            import_knowledge_setting: None,
            description: None,
        }
    }

    /// 设置知识导入配置。
    pub fn import_knowledge_setting(
        mut self,
        import_knowledge_setting: DataAssetImportKnowledgeSetting,
    ) -> Self {
        self.import_knowledge_setting = Some(import_knowledge_setting);
        self
    }

    /// 设置数据知识描述信息。
    pub fn description(mut self, description: HashMap<String, String>) -> Self {
        self.description = Some(description);
        self
    }
}

/// 创建数据知识请求
///
/// 用于创建新的数据知识。
///
/// # 字段说明
///
/// - `config`: 配置信息
/// - `app_id`: 应用 ID，必填
///
/// # 请求体字段
///
/// - `connect_type`: 连接类型，必填（`import`/`direct`）
/// - `source_type`: 数据源类型，必填（`file`/`lark_wiki_space`/`lark_doc`/`lark_helpdesk`）
/// - `import_knowledge_setting`: 知识导入配置（可选）
/// - `description`: 数据知识描述信息（可选，map，key 为语言编码）
///
/// # 示例
///
/// ```rust,ignore
/// let body = CreateDataAssetBody::new("direct", "file")
///     .description(HashMap::from([("zh_cn".to_string(), "描述".to_string())]));
/// let request = CreateDataAssetRequest::new(config)
///     .app_id("app_xxx")
///     .execute(body).await?;
/// ```
pub struct CreateDataAssetRequest {
    config: Config,
    app_id: String,
}

impl CreateDataAssetRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_id: String::new(),
        }
    }

    /// 应用 ID（路径参数）
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = app_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/aily-v1/data-knowledge/data-knowledge-management/create>
    pub async fn execute(self, body: CreateDataAssetBody) -> SDKResult<serde_json::Value> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        body: CreateDataAssetBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        // === 必填字段验证 ===
        validate_required!(self.app_id, "app_id 不能为空");
        validate_required!(body.connect_type, "connect_type 不能为空");
        validate_required!(body.source_type, "source_type 不能为空");

        let url = AILY_V1_DATA_ASSETS.replace("{app_id}", &self.app_id);
        let req: ApiRequest<CreateDataAssetBody> = ApiRequest::post(&url).json_body(&body);

        Transport::request_typed(req, &self.config, Some(option), "创建数据知识").await
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_create_data_asset_request_builder() {
        let config = Config::default();
        let request = CreateDataAssetRequest::new(config).app_id("app_xxx");
        assert_eq!(request.app_id, "app_xxx");
    }

    #[test]
    fn test_create_data_asset_body_builder() {
        let body = CreateDataAssetBody::new("direct", "file");
        assert_eq!(body.connect_type, "direct");
        assert_eq!(body.source_type, "file");
        assert_eq!(body.description, None);
        assert!(body.import_knowledge_setting.is_none());
    }

    #[test]
    fn test_create_data_asset_body_with_description() {
        let mut description = HashMap::new();
        description.insert("zh_cn".to_string(), "描述".to_string());
        let body = CreateDataAssetBody::new("direct", "file").description(description);
        assert_eq!(
            body.description.as_ref().map(|m| m.get("zh_cn").cloned()),
            Some(Some("描述".to_string()))
        );
    }

    #[test]
    fn test_create_data_asset_request_default_values() {
        let config = Config::default();
        let request = CreateDataAssetRequest::new(config);
        assert_eq!(request.app_id, "");
    }

    #[test]
    fn test_create_data_asset_body_official_shape() {
        // 官方请求示例形状（截取核心字段）
        let body = CreateDataAssetBody::new("direct", "file").import_knowledge_setting(
            DataAssetImportKnowledgeSetting::new()
                .chunk_setting(
                    DataAssetKnowledgeChunkSetting::new("intelligent")
                        .separate_type("paragraph")
                        .size(600)
                        .overlap(10),
                )
                .file(
                    DataAssetImportKnowledgeFile::new()
                        .title("文件标题")
                        .token("bb690637b49440b08f39459a2fdcd2ca")
                        .mime_type("application/pdf"),
                )
                .lark_doc(DataAssetImportKnowledgeLarkDoc::new(
                    "docx",
                    "T8FAcuilgC1fdaxkt58vcp91xngh",
                ))
                .lark_wiki_space(
                    DataAssetImportKnowledgeWiki::new("798546548961351").sub_docs(vec![
                        DataAssetImportKnowledgeWikiSubDoc::new(
                            "wiki",
                            "T8FAcuilgC1fdaxkt58vcp91xngh",
                        )
                        .url("https://cdas.feishu.cn/wiki/fdisu1"),
                    ]),
                )
                .lark_helpdesk(DataAssetImportKnowledgeHelpdesk::new("123")),
        );
        let json = serde_json::to_value(&body).unwrap();
        assert_eq!(json["connect_type"], "direct");
        assert_eq!(json["source_type"], "file");
        assert_eq!(
            json["import_knowledge_setting"]["chunk_setting"]["rule_type"],
            "intelligent"
        );
        assert_eq!(
            json["import_knowledge_setting"]["chunk_setting"]["size"],
            600
        );
        assert_eq!(
            json["import_knowledge_setting"]["chunk_setting"]["overlap"],
            10
        );
        assert_eq!(
            json["import_knowledge_setting"]["file"]["token"],
            "bb690637b49440b08f39459a2fdcd2ca"
        );
        assert_eq!(json["import_knowledge_setting"]["lark_doc"]["type"], "docx");
        assert_eq!(
            json["import_knowledge_setting"]["lark_wiki_space"]["sub_docs"][0]["url"],
            "https://cdas.feishu.cn/wiki/fdisu1"
        );
        assert_eq!(
            json["import_knowledge_setting"]["lark_helpdesk"]["helpdesk_id"],
            "123"
        );
        // 官方请求体无 name/tags 字段
        assert!(json.get("name").is_none());
        assert!(json.get("tags").is_none());
    }

    #[test]
    fn test_create_data_asset_request_url_construction() {
        use crate::endpoints::aily::AILY_V1_DATA_ASSETS;
        let url = AILY_V1_DATA_ASSETS.replace("{app_id}", "app_1");
        assert_eq!(url, "/open-apis/aily/v1/apps/app_1/data_assets");
        assert!(
            !url.contains("{"),
            "URL should not contain unreplaced placeholders"
        );
    }
}
