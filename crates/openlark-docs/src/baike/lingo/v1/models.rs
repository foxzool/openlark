//! Lingo（飞书词典）v1 数据模型
//!
//! 说明：
//! - 本文件仅存放模型（不计入 API 数量）。
//! - 字段严格对齐飞书开放平台文档的 `lingo-v1` schema。

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// 用户 ID 类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum UserIdType {
    /// 开放平台用户 ID
    OpenId,
    /// 飞书统一用户 ID（跨应用同一用户）
    UnionId,
    /// 用户 ID（应用内用户标识）
    UserId,
}

impl UserIdType {
    /// 返回对应的字符串表示。
    pub fn as_str(&self) -> &'static str {
        match self {
            UserIdType::OpenId => "open_id",
            UserIdType::UnionId => "union_id",
            UserIdType::UserId => "user_id",
        }
    }
}

/// 语言类型（schema: Language）
#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr, PartialEq, Eq)]
#[repr(i32)]
pub enum Language {
    /// 简体中文（zh_cn）
    ZhCn = 1,
    /// 美式英语（en_us）
    EnUs = 2,
    /// 日语（ja_jp）
    JaJp = 3,
}

/// 名称展示范围（display_status）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DisplayStatus {
    /// 是否允许在 IM 和 Doc 等场景进行高亮提示
    pub allow_highlight: bool,
    /// 是否允许在飞书中被搜索到 / 是否在搜索结果中展示
    pub allow_search: bool,
}

/// 词条名/别名（term）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Term {
    /// 名称
    pub key: String,
    /// 展示状态
    pub display_status: DisplayStatus,
}

/// 外部系统关联数据（outer_info）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OuterInfo {
    /// 数据提供方（不能包含 '-'）
    pub provider: String,
    /// 外部系统唯一 ID（不能包含 '-'）
    pub outer_id: String,
}

/// 相关联系人（referer）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelatedUser {
    /// 格式根据 user_id_type 不同需要符合 open_id、user_id、union_id 格式的有效 id
    pub id: String,
    /// 备注
    pub title: Option<String>,
}

/// 相关公开群（referer）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelatedChat {
    /// 公开群 id
    pub id: String,
}

/// 相关飞书文档/wiki（referer）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelatedDoc {
    /// 文档标题
    pub title: Option<String>,
    /// 文档 url
    pub url: Option<String>,
}

/// 相关值班号（referer）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelatedOncall {
    /// 值班号 id
    pub id: String,
}

/// 其他网页链接（referer）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelatedLink {
    /// 标题
    pub title: Option<String>,
    /// 网页链接
    pub url: Option<String>,
}

/// 相关词条（abbreviation）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Abbreviation {
    /// 其他相关词条 id
    pub id: Option<String>,
}

/// 分类引用（classification）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ClassificationRef {
    /// 二级分类 ID
    pub id: String,
    /// 对应一级分类 ID
    pub father_id: Option<String>,
}

/// 上传图片 token（baike_image）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BaikeImage {
    /// 通过文件接口上传后的图片 token
    pub token: String,
}

/// 关联信息（related_meta）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RelatedMeta {
    /// 相关联系人
    pub users: Option<Vec<RelatedUser>>,
    /// 有关的公开群
    pub chats: Option<Vec<RelatedChat>>,
    /// 飞书文档或飞书 wiki
    pub docs: Option<Vec<RelatedDoc>>,
    /// 飞书值班号
    pub oncalls: Option<Vec<RelatedOncall>>,
    /// 其他网页链接
    pub links: Option<Vec<RelatedLink>>,
    /// 相关词条
    pub abbreviations: Option<Vec<Abbreviation>>,
    /// 当前词条所属分类（仅二级分类）
    pub classifications: Option<Vec<ClassificationRef>>,
    /// 上传的相关图片
    pub images: Option<Vec<BaikeImage>>,
}

/// 国际化词条释义（i18n_entry_desc）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nEntryDesc {
    /// 语言类型
    pub language: Language,
    /// 纯文本释义
    pub description: Option<String>,
    /// 富文本描述
    pub rich_text: Option<String>,
}

/// 反馈统计数据（statistics）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Statistics {
    /// 点赞数量
    pub like_count: i32,
    /// 点踩数量
    pub dislike_count: i32,
}

/// 创建/更新免审词条请求体（entity）
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct EntityInput {
    /// 词条名（最大 1 个）
    #[serde(default)]
    pub main_keys: Vec<Term>,
    /// 别名
    #[serde(default)]
    pub aliases: Option<Vec<Term>>,
    /// 纯文本释义（当填写 rich_text 时会失效）
    #[serde(default)]
    pub description: Option<String>,
    /// 关联信息
    #[serde(default)]
    pub related_meta: Option<RelatedMeta>,
    /// 外部系统关联数据
    #[serde(default)]
    pub outer_info: Option<OuterInfo>,
    /// 富文本释义
    #[serde(default)]
    pub rich_text: Option<String>,
    /// 国际化的词条释义
    #[serde(default)]
    pub i18n_descs: Option<Vec<I18nEntryDesc>>,
}

/// 创建/更新草稿请求体（entity）
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DraftEntityInput {
    /// 词条 ID（可选）
    #[serde(default)]
    pub id: Option<String>,
    /// 词条名（最大 1 个）
    #[serde(default)]
    pub main_keys: Vec<Term>,
    /// 别名
    #[serde(default)]
    pub aliases: Option<Vec<Term>>,
    /// 纯文本释义（当填写 rich_text 时会失效）
    #[serde(default)]
    pub description: Option<String>,
    /// 关联信息
    #[serde(default)]
    pub related_meta: Option<RelatedMeta>,
    /// 外部系统关联数据（仅创建草稿时支持）
    #[serde(default)]
    pub outer_info: Option<OuterInfo>,
    /// 富文本释义
    #[serde(default)]
    pub rich_text: Option<String>,
    /// 国际化的词条释义
    #[serde(default)]
    pub i18n_descs: Option<Vec<I18nEntryDesc>>,
}

/// 更新草稿请求体（entity）
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DraftUpdateEntityInput {
    /// 词条 ID（可选）
    #[serde(default)]
    pub id: Option<String>,
    /// 词条名（最大 1 个）
    #[serde(default)]
    pub main_keys: Vec<Term>,
    /// 别名
    #[serde(default)]
    pub aliases: Option<Vec<Term>>,
    /// 纯文本释义（当填写 rich_text 时会失效）
    #[serde(default)]
    pub description: Option<String>,
    /// 关联信息
    #[serde(default)]
    pub related_meta: Option<RelatedMeta>,
    /// 富文本释义
    #[serde(default)]
    pub rich_text: Option<String>,
    /// 国际化的词条释义
    #[serde(default)]
    pub i18n_descs: Option<Vec<I18nEntryDesc>>,
}

/// 免审词条实体（entity）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Entity {
    /// 词条 ID
    pub id: Option<String>,
    /// 词条名（最大 1 个）
    pub main_keys: Vec<Term>,
    /// 别名
    pub aliases: Option<Vec<Term>>,
    /// 纯文本释义
    pub description: Option<String>,
    /// 创建者
    pub creator: Option<String>,
    /// 词条创建时间（秒级时间戳）
    pub create_time: Option<String>,
    /// 更新者
    pub updater: Option<String>,
    /// 最近一次更新词条时间（秒级时间戳）
    pub update_time: Option<String>,
    /// 关联信息
    pub related_meta: Option<RelatedMeta>,
    /// 反馈统计数据
    pub statistics: Option<Statistics>,
    /// 外部系统关联数据
    pub outer_info: Option<OuterInfo>,
    /// 富文本释义
    pub rich_text: Option<String>,
    /// 词条的创建来源
    pub source: Option<i32>,
    /// 国际化释义
    pub i18n_descs: Option<Vec<I18nEntryDesc>>,
}

/// 草稿信息（draft）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Draft {
    /// 草稿 ID
    pub draft_id: String,
    /// 词条信息
    pub entity: Entity,
}

/// 分类国际化名称（i18n_cls_name）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nClsName {
    /// 语言类型
    pub language: Language,
    /// 对应语言的分类名称
    pub name: String,
}

/// 分类（classification）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Classification {
    /// 二级分类 ID
    pub id: String,
    /// 二级分类名称
    pub name: Option<String>,
    /// 对应一级分类 ID
    pub father_id: Option<String>,
    /// 国际化分类名
    pub i18n_names: Option<Vec<I18nClsName>>,
}

/// 词库（repo）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Repo {
    /// 词库 id
    pub id: String,
    /// 词库名
    pub name: String,
}

/// 兼容导出：保留历史类型名（不建议在新代码中使用）
pub type LingoEntity = Entity;
/// 兼容导出：保留历史类型名（不建议在新代码中使用）
pub type LingoDraft = Draft;

#[cfg(test)]
mod tests {
    use super::*;

    fn test_roundtrip<T: Serialize + for<'de> Deserialize<'de> + PartialEq + std::fmt::Debug>(
        original: &T,
    ) {
        let json = serde_json::to_string(original).expect("序列化失败");
        let deserialized: T = serde_json::from_str(&json).expect("反序列化失败");
        assert_eq!(original, &deserialized, "roundtrip 后数据不一致");
    }

    #[test]
    fn test_user_id_type_serialization() {
        test_roundtrip(&UserIdType::OpenId);
        test_roundtrip(&UserIdType::UnionId);
        test_roundtrip(&UserIdType::UserId);
    }

    #[test]
    fn test_language_serialization() {
        test_roundtrip(&Language::ZhCn);
        test_roundtrip(&Language::EnUs);
        test_roundtrip(&Language::JaJp);
    }

    #[test]
    fn test_display_status_serialization() {
        let status = DisplayStatus {
            allow_highlight: true,
            allow_search: false,
        };
        test_roundtrip(&status);
    }

    #[test]
    fn test_term_serialization() {
        let term = Term {
            key: "测试词条".to_string(),
            display_status: DisplayStatus {
                allow_highlight: true,
                allow_search: true,
            },
        };
        test_roundtrip(&term);
    }

    #[test]
    fn test_outer_info_serialization() {
        let info = OuterInfo {
            provider: "provider1".to_string(),
            outer_id: "outer123".to_string(),
        };
        test_roundtrip(&info);
    }

    #[test]
    fn test_related_user_serialization() {
        let user = RelatedUser {
            id: "user123".to_string(),
            title: Some("负责人".to_string()),
        };
        test_roundtrip(&user);
    }

    #[test]
    fn test_related_chat_serialization() {
        let chat = RelatedChat {
            id: "chat456".to_string(),
        };
        test_roundtrip(&chat);
    }

    #[test]
    fn test_related_doc_serialization() {
        let doc = RelatedDoc {
            title: Some("文档标题".to_string()),
            url: Some("https://doc.example.com".to_string()),
        };
        test_roundtrip(&doc);
    }

    #[test]
    fn test_related_oncall_serialization() {
        let oncall = RelatedOncall {
            id: "oncall789".to_string(),
        };
        test_roundtrip(&oncall);
    }

    #[test]
    fn test_related_link_serialization() {
        let link = RelatedLink {
            title: Some("链接标题".to_string()),
            url: Some("https://example.com".to_string()),
        };
        test_roundtrip(&link);
    }

    #[test]
    fn test_abbreviation_serialization() {
        let abbr = Abbreviation {
            id: Some("abbr123".to_string()),
        };
        test_roundtrip(&abbr);
    }

    #[test]
    fn test_classification_ref_serialization() {
        let class = ClassificationRef {
            id: "class456".to_string(),
            father_id: Some("father123".to_string()),
        };
        test_roundtrip(&class);
    }

    #[test]
    fn test_baike_image_serialization() {
        let img = BaikeImage {
            token: "img_token_123".to_string(),
        };
        test_roundtrip(&img);
    }

    #[test]
    fn test_related_meta_serialization() {
        let meta = RelatedMeta {
            users: Some(vec![RelatedUser {
                id: "user1".to_string(),
                title: None,
            }]),
            chats: None,
            docs: None,
            oncalls: None,
            links: None,
            abbreviations: None,
            classifications: None,
            images: None,
        };
        test_roundtrip(&meta);
    }

    #[test]
    fn test_i18n_entry_desc_serialization() {
        let desc = I18nEntryDesc {
            language: Language::ZhCn,
            description: Some("描述".to_string()),
            rich_text: Some("<p>富文本</p>".to_string()),
        };
        test_roundtrip(&desc);
    }

    #[test]
    fn test_statistics_serialization() {
        let stats = Statistics {
            like_count: 100,
            dislike_count: 5,
        };
        test_roundtrip(&stats);
    }

    #[test]
    fn test_entity_input_serialization() {
        let input = EntityInput {
            main_keys: vec![Term {
                key: "主词条".to_string(),
                display_status: DisplayStatus {
                    allow_highlight: true,
                    allow_search: true,
                },
            }],
            aliases: None,
            description: Some("词条描述".to_string()),
            related_meta: None,
            outer_info: None,
            rich_text: None,
            i18n_descs: None,
        };
        test_roundtrip(&input);
    }

    #[test]
    fn test_entity_serialization() {
        let entity = Entity {
            id: Some("entity123".to_string()),
            main_keys: vec![Term {
                key: "词条名".to_string(),
                display_status: DisplayStatus {
                    allow_highlight: true,
                    allow_search: true,
                },
            }],
            aliases: None,
            description: Some("描述".to_string()),
            creator: Some("creator1".to_string()),
            create_time: Some("1234567890".to_string()),
            updater: None,
            update_time: None,
            related_meta: None,
            statistics: None,
            outer_info: None,
            rich_text: None,
            source: Some(1),
            i18n_descs: None,
        };
        test_roundtrip(&entity);
    }

    #[test]
    fn test_classification_serialization() {
        let class = Classification {
            id: "class123".to_string(),
            name: Some("分类名称".to_string()),
            father_id: Some("father123".to_string()),
            i18n_names: None,
        };
        test_roundtrip(&class);
    }

    #[test]
    fn test_repo_serialization() {
        let repo = Repo {
            id: "repo123".to_string(),
            name: "词库名称".to_string(),
        };
        test_roundtrip(&repo);
    }

    #[test]
    fn test_i18n_cls_name_serialization() {
        let name = I18nClsName {
            language: Language::EnUs,
            name: "English Name".to_string(),
        };
        test_roundtrip(&name);
    }
}
