//! 千人千面菜单的菜单树。
//!
//! 官方示例把第三层 `behaviors` 写成单个对象，前两层是数组。
//! 字段说明仍是「动作列表」。反序列化时单个对象收成一项，序列化始终输出数组。

use serde::Deserialize;
use serde::Serialize;
use std::collections::BTreeMap;

/// 机器人菜单。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BotMenu {
    /// 菜单 ID。可忽略。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bot_menu_id: Option<String>,
    /// 一级菜单。最多 5 项。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bot_menu_items: Option<Vec<BotMenuItem>>,
}

/// 菜单项。最多三层，`children` 指向下一层。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BotMenuItem {
    /// 默认名称。2～60 个字符。
    pub name: String,
    /// 多语言名称。键是语种，例如 `en_us`。
    #[serde(default)]
    pub i18n_name: BTreeMap<String, String>,
    /// 前缀图标。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon: Option<BotMenuIcon>,
    /// 点击时上报的标记。0～20 个字符。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// 点击后的动作。与 `children` 二选一。
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "deserialize_behaviors"
    )]
    pub behaviors: Option<Vec<BotMenuBehavior>>,
    /// 子菜单。与 `behaviors` 二选一。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<BotMenuItem>>,
}

/// 菜单图标。图标库与自定义图片可以同时出现。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BotMenuIcon {
    /// 图标库图标。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ud_icon: Option<BotMenuUdIcon>,
    /// 自定义图片的 image_key。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub file_key: Option<String>,
}

/// 图标库中的图标。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BotMenuUdIcon {
    /// 图标 token，例如 `add_outlined`。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
    /// 图标颜色，例如 `blue`。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
}

/// 点击菜单后执行的一个动作。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BotMenuBehavior {
    /// `target`、`event_key` 或 `send_message`。
    #[serde(default, rename = "type", skip_serializing_if = "Option::is_none")]
    pub behavior_type: Option<String>,
    /// `type = target` 时打开的链接。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<BotMenuRedirectLink>,
    /// `type = event_key` 时回调里的 event key。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_key: Option<String>,
    /// 多个动作里的主要动作。客户端处理不了主要动作时，该项会被隐藏。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<bool>,
}

/// 按客户端打开的链接。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BotMenuRedirectLink {
    /// 兜底链接。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub common_url: Option<String>,
    /// iOS 链接。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ios_url: Option<String>,
    /// Android 链接。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub android_url: Option<String>,
    /// PC 链接。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pc_url: Option<String>,
    /// 浏览器链接。
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub web_url: Option<String>,
}

fn deserialize_behaviors<'de, D>(deserializer: D) -> Result<Option<Vec<BotMenuBehavior>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = Option::<serde_json::Value>::deserialize(deserializer)?;
    match value {
        None | Some(serde_json::Value::Null) => Ok(None),
        Some(serde_json::Value::Array(items)) => {
            let parsed = serde_json::from_value(serde_json::Value::Array(items))
                .map_err(serde::de::Error::custom)?;
            Ok(Some(parsed))
        }
        Some(serde_json::Value::Object(item)) => {
            let parsed = serde_json::from_value(serde_json::Value::Object(item))
                .map_err(serde::de::Error::custom)?;
            Ok(Some(vec![parsed]))
        }
        Some(_) => Err(serde::de::Error::custom("behaviors 必须是对象或对象数组")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn third_level_behavior_object_becomes_one_item_and_serializes_as_array() {
        let raw = r#"{
            "name": "测试按钮",
            "i18n_name": {"en_us": "New topic"},
            "behaviors": {"type": "target", "event_key": "xxx", "is_primary": false}
        }"#;
        let item: BotMenuItem = serde_json::from_str(raw).expect("第三层对象应能解析");
        let behaviors = item.behaviors.as_ref().expect("behaviors");
        assert_eq!(behaviors.len(), 1);
        assert_eq!(behaviors[0].behavior_type.as_deref(), Some("target"));
        assert_eq!(behaviors[0].event_key.as_deref(), Some("xxx"));
        assert_eq!(behaviors[0].is_primary, Some(false));

        let encoded = serde_json::to_value(&item).expect("序列化");
        assert!(encoded["behaviors"].is_array());
        assert_eq!(encoded["behaviors"][0]["type"], "target");
    }

    #[test]
    fn behavior_array_stays_array() {
        let raw = r#"{
            "name": "测试按钮",
            "i18n_name": {"en_us": "New topic"},
            "behaviors": [{"type": "send_message", "is_primary": true}]
        }"#;
        let item: BotMenuItem = serde_json::from_str(raw).expect("数组应能解析");
        assert_eq!(item.behaviors.expect("behaviors").len(), 1);
    }
}
