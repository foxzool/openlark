//! okr/v2 跨叶共享的 domain entity struct。
//!
//! 这些 struct 代表同一飞书实体，跨多个 API 叶子重复出现（已确认 byte-identical）。
//! 为避免逐字重复（#336），各只在此处定义一次，叶子通过 `use` 引用。

use serde::Deserialize;

/// OKR 目标。
#[derive(Debug, Clone, Deserialize)]
pub struct Objective {
    /// 目标的 ID。
    pub id: String,
    /// 目标的创建时间，毫秒级时间戳。
    pub create_time: String,
    /// 目标的更新时间，毫秒级时间戳。
    pub update_time: String,
    /// 所有者。
    pub owner: ObjectiveOwner,
    /// 目标的用户周期 ID。
    pub cycle_id: String,
    /// 目标的序号：从 1 开始计数。
    pub position: i32,
    /// 目标的内容（文档 block 结构，见 [`ContentBlock`]）。
    #[serde(default)]
    pub content: Option<ContentBlock>,
    /// 目标的分数：\[0,1\]，支持一位小数。
    #[serde(default)]
    pub score: Option<f64>,
    /// 目标的备注（文档 block 结构，见 [`ContentBlock`]）。
    #[serde(default)]
    pub notes: Option<ContentBlock>,
    /// 目标的权重：\[0,1\]，支持三位小数。
    #[serde(default)]
    pub weight: Option<f64>,
    /// 目标的截止时间，毫秒级时间戳。
    #[serde(default)]
    pub deadline: Option<String>,
    /// 目标的分类 ID。
    #[serde(default)]
    pub category_id: Option<String>,
}

/// 目标所有者。
#[derive(Debug, Clone, Deserialize)]
pub struct ObjectiveOwner {
    /// 所有者类型（如 "user"）。
    pub owner_type: String,
    /// 员工 ID。
    #[serde(default)]
    pub user_id: Option<String>,
}

/// 量化指标。
#[derive(Debug, Clone, Deserialize)]
pub struct Indicator {
    /// 指标的 ID。
    pub id: String,
    /// 指标的创建时间，毫秒级时间戳。
    pub create_time: String,
    /// 指标的更新时间，毫秒级时间戳。
    pub update_time: String,
    /// 所有者。
    pub owner: IndicatorOwner,
    /// 指标所属的实体类型。
    pub entity_type: i32,
    /// 指标所属的实体 ID。
    pub entity_id: String,
    /// 指标的状态。
    pub indicator_status: i32,
    /// 指标的状态的计算方式。
    pub status_calculate_type: i32,
    /// 指标的起始值。
    #[serde(default)]
    pub start_value: Option<f64>,
    /// 指标的目标值。
    #[serde(default)]
    pub target_value: Option<f64>,
    /// 指标的当前值。
    #[serde(default)]
    pub current_value: Option<f64>,
    /// 指标的当前值的计算方式。
    #[serde(default)]
    pub current_value_calculate_type: Option<i32>,
    /// 指标的单位。
    #[serde(default)]
    pub unit: Option<IndicatorUnit>,
}

/// 指标所有者。
#[derive(Debug, Clone, Deserialize)]
pub struct IndicatorOwner {
    /// 所有者类型（如 "user"）。
    pub owner_type: String,
    /// 员工 ID。
    #[serde(default)]
    pub user_id: Option<String>,
}

/// 指标单位。
#[derive(Debug, Clone, Deserialize)]
pub struct IndicatorUnit {
    /// 指标的单位类型。
    pub unit_type: i32,
    /// 指标单位的值。
    pub unit_value: String,
}

/// 关键结果。
#[derive(Debug, Clone, Deserialize)]
pub struct KeyResult {
    /// 关键结果的 ID。
    pub id: String,
    /// 关键结果的创建时间，毫秒级时间戳。
    pub create_time: String,
    /// 关键结果的修改时间，毫秒级时间戳。
    pub update_time: String,
    /// 所有者。
    pub owner: KeyResultOwner,
    /// 关键结果的目标 ID。
    pub objective_id: String,
    /// 关键结果的序号：从 1 开始计数。
    pub position: i32,
    /// 关键结果的内容（文档 block 结构，见 [`ContentBlock`]）。
    #[serde(default)]
    pub content: Option<ContentBlock>,
    /// 关键结果的分数：\[0,1\]，支持一位小数。
    #[serde(default)]
    pub score: Option<f64>,
    /// 目标的权重：\[0,1\]，支持三位小数。
    #[serde(default)]
    pub weight: Option<f64>,
    /// 关键结果的截止时间，毫秒级时间戳。
    #[serde(default)]
    pub deadline: Option<String>,
}

/// 关键结果所有者。
#[derive(Debug, Clone, Deserialize)]
pub struct KeyResultOwner {
    /// 所有者类型（如 "user"）。
    pub owner_type: String,
    /// 员工 ID。
    #[serde(default)]
    pub user_id: Option<String>,
}

/// OKR 对齐。
#[derive(Debug, Clone, Deserialize)]
pub struct Alignment {
    /// 对齐的 ID。
    pub id: String,
    /// 对齐的创建时间，毫秒级时间戳。
    pub create_time: String,
    /// 对齐的更新时间，毫秒级时间戳。
    pub update_time: String,
    /// 发起对齐的所有者。
    pub from_owner: AlignmentOwner,
    /// 被对齐的所有者。
    pub to_owner: AlignmentOwner,
    /// 发起对齐的实体类型。
    pub from_entity_type: i32,
    /// 发起对齐的实体 ID。
    pub from_entity_id: String,
    /// 被对齐的实体类型。
    pub to_entity_type: i32,
    /// 被对齐的实体 ID。
    pub to_entity_id: String,
}

/// 对齐所有者。
#[derive(Debug, Clone, Deserialize)]
pub struct AlignmentOwner {
    /// 所有者类型（如 "user"）。
    pub owner_type: String,
    /// 员工 ID。
    #[serde(default)]
    pub user_id: Option<String>,
}

// === OKR 文档内容块（content / notes 字段共享）===
//
// 源：飞书 apiSchema objectName=content_block（objective/get api_id=7644764969658567628 的
// data.objective.content / .notes；key_result/get、key_result/progress/list、
// objective/progress/list 等的同名字段共用同一 objectName）。
// 复现：`python3 tools/schema_cache/cache.py` 或 dump_samples.py 取该 api_id 的
// responses.200.content.application/json.schema.properties.data.objective.content。
// 判别联合：block_element_type（paragraph|gallery）+ paragraph_element_type（textRun|docsLink|mention）。
// tag 用 String（非 enum）以容忍飞书未来新增 block/element 类型，未知类型反序列化不报错
// （见测试 content_block_tolerance_unknown_type_and_empty）。

/// OKR 文档内容块（objective/key_result 的 content/notes、progress 的 content）。
#[derive(Debug, Clone, Deserialize)]
pub struct ContentBlock {
    /// 文档结构按行排列，每行内容是一个 Block。
    #[serde(default)]
    pub blocks: Vec<ContentBlockElement>,
}

/// 文档块元素（判别联合，由 `block_element_type` 区分）。
#[derive(Debug, Clone, Deserialize)]
pub struct ContentBlockElement {
    /// 文档元素类型（`"paragraph"` | `"gallery"`）。
    #[serde(default)]
    pub block_element_type: Option<String>,
    /// 文本段落（`block_element_type = paragraph` 时存在）。
    #[serde(default)]
    pub paragraph: Option<ContentParagraph>,
    /// 图片画廊（`block_element_type = gallery` 时存在）。
    #[serde(default)]
    pub gallery: Option<ContentGallery>,
}

/// 文本段落。
#[derive(Debug, Clone, Deserialize)]
pub struct ContentParagraph {
    /// 段落样式。
    #[serde(default)]
    pub style: Option<ContentParagraphStyle>,
    /// 段落元素列表。
    #[serde(default)]
    pub elements: Vec<ContentParagraphElement>,
}

/// 段落样式。
#[derive(Debug, Clone, Deserialize)]
pub struct ContentParagraphStyle {
    /// 列表样式（有序 / 无序 / 任务列表）。
    #[serde(default)]
    pub list: Option<ContentList>,
}

/// 列表样式。
#[derive(Debug, Clone, Deserialize)]
pub struct ContentList {
    /// 列表类型（`"number"` | `"bullet"` | `"checkBox"` | `"checkedBox"` | `"indent"`）。
    #[serde(default)]
    pub list_type: Option<String>,
    /// 缩进层级。
    #[serde(default)]
    pub indent_level: Option<i32>,
    /// 有序列表序号。
    #[serde(default)]
    pub number: Option<i32>,
}

/// 段落元素（判别联合，由 `paragraph_element_type` 区分）。
#[derive(Debug, Clone, Deserialize)]
pub struct ContentParagraphElement {
    /// 段落元素类型（`"textRun"` | `"docsLink"` | `"mention"`）。
    #[serde(default)]
    pub paragraph_element_type: Option<String>,
    /// 文本串（`type = textRun`）。
    #[serde(default)]
    pub text_run: Option<ContentTextRun>,
    /// 文档链接（`type = docsLink`）。
    #[serde(default)]
    pub docs_link: Option<ContentDocsLink>,
    /// @提及（`type = mention`）。
    #[serde(default)]
    pub mention: Option<ContentMention>,
}

/// 文本串。
#[derive(Debug, Clone, Deserialize)]
pub struct ContentTextRun {
    /// 文本内容。
    #[serde(default)]
    pub text: Option<String>,
    /// 文本样式。
    #[serde(default)]
    pub style: Option<ContentTextStyle>,
}

/// 文本样式。
#[derive(Debug, Clone, Deserialize)]
pub struct ContentTextStyle {
    /// 加粗。
    #[serde(default)]
    pub bold: Option<bool>,
    /// 删除线。
    #[serde(default)]
    pub strike_through: Option<bool>,
    /// 背景色。
    #[serde(default)]
    pub back_color: Option<ContentColor>,
    /// 文本色。
    #[serde(default)]
    pub text_color: Option<ContentColor>,
    /// 超链接。
    #[serde(default)]
    pub link: Option<ContentLink>,
}

/// RGBA 颜色。
#[derive(Debug, Clone, Deserialize)]
pub struct ContentColor {
    /// 红色通道。
    #[serde(default)]
    pub red: Option<i32>,
    /// 绿色通道。
    #[serde(default)]
    pub green: Option<i32>,
    /// 蓝色通道。
    #[serde(default)]
    pub blue: Option<i32>,
    /// 透明度。
    #[serde(default)]
    pub alpha: Option<f64>,
}

/// 超链接（文本串 style.link）。
#[derive(Debug, Clone, Deserialize)]
pub struct ContentLink {
    /// 链接 URL。
    #[serde(default)]
    pub url: Option<String>,
}

/// 文档链接（段落元素 docs_link）。
#[derive(Debug, Clone, Deserialize)]
pub struct ContentDocsLink {
    /// 链接 URL。
    #[serde(default)]
    pub url: Option<String>,
    /// 标题。
    #[serde(default)]
    pub title: Option<String>,
}

/// @提及（段落元素 mention）。
#[derive(Debug, Clone, Deserialize)]
pub struct ContentMention {
    /// 用户 ID。
    #[serde(default)]
    pub user_id: Option<String>,
}

/// 图片画廊（block_element_type = gallery）。
#[derive(Debug, Clone, Deserialize)]
pub struct ContentGallery {
    /// 图片元素列表。
    #[serde(default)]
    pub images: Vec<ContentImageItem>,
}

/// 图片项。
#[derive(Debug, Clone, Deserialize)]
pub struct ContentImageItem {
    /// 图片 token（不支持编辑）。
    #[serde(default)]
    pub file_token: Option<String>,
    /// 图片源。
    #[serde(default)]
    pub src: Option<String>,
    /// 宽度。
    #[serde(default)]
    pub width: Option<f64>,
    /// 高度。
    #[serde(default)]
    pub height: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn content_block_deserialize_paragraph_and_gallery() {
        // 覆盖两类 block + 段落元素判别联合（textRun/docsLink/mention）+ 列表样式 + 图片。
        let json = serde_json::json!({
            "blocks": [
                {
                    "block_element_type": "paragraph",
                    "paragraph": {
                        "style": { "list": { "list_type": "bullet", "indent_level": 1 } },
                        "elements": [
                            {
                                "paragraph_element_type": "textRun",
                                "text_run": {
                                    "text": "Q3 目标",
                                    "style": {
                                        "bold": true,
                                        "text_color": { "red": 255, "green": 0, "blue": 0, "alpha": 1.0 }
                                    }
                                }
                            },
                            {
                                "paragraph_element_type": "docsLink",
                                "docs_link": { "url": "https://example.feishu.cn/docs/x", "title": "周报" }
                            },
                            {
                                "paragraph_element_type": "mention",
                                "mention": { "user_id": "ou_abc" }
                            }
                        ]
                    }
                },
                {
                    "block_element_type": "gallery",
                    "gallery": {
                        "images": [
                            { "file_token": "boxcnOj88GDkmWGm2zsTyCBqoLb", "src": "x", "width": 100.0, "height": 50.0 }
                        ]
                    }
                }
            ]
        });
        let block: ContentBlock = serde_json::from_value(json).expect("反序列化失败");
        assert_eq!(block.blocks.len(), 2);

        let para = block.blocks[0].paragraph.as_ref().expect("paragraph");
        assert_eq!(para.elements.len(), 3);
        assert_eq!(
            para.style
                .as_ref()
                .and_then(|s| s.list.as_ref())
                .and_then(|l| l.list_type.as_deref()),
            Some("bullet")
        );
        let run = para.elements[0].text_run.as_ref().expect("text_run");
        assert_eq!(run.text.as_deref(), Some("Q3 目标"));
        assert_eq!(run.style.as_ref().and_then(|s| s.bold), Some(true));

        let gal = block.blocks[1].gallery.as_ref().expect("gallery");
        assert_eq!(gal.images.len(), 1);
        assert_eq!(gal.images[0].width, Some(100.0));
    }

    #[test]
    fn content_block_tolerance_unknown_type_and_empty() {
        // 未知 block_element_type / 缺字段 / 空 blocks 均不报错（向前兼容）。
        let json = serde_json::json!({
            "blocks": [
                { "block_element_type": "futureBlockType", "paragraph": null, "gallery": null }
            ]
        });
        let block: ContentBlock = serde_json::from_value(json).expect("未知类型应容忍");
        assert_eq!(
            block.blocks[0].block_element_type.as_deref(),
            Some("futureBlockType")
        );
        assert!(block.blocks[0].paragraph.is_none());

        let empty: ContentBlock =
            serde_json::from_value(serde_json::json!({})).expect("空对象应容忍");
        assert!(empty.blocks.is_empty());
    }
}
