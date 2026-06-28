/// 多维表格字段类型定义
///
/// 提供更精确的字段值类型定义，替代通用的 `serde_json::Value`。
use serde::{Deserialize, Serialize};
use serde_json::Value;
#[cfg(test)]
use serde_json::json;

/// 记录字段值枚举
///
/// 支持多维表格中的各种字段类型，包括文本、数字、布尔值、日期、用户/部门/群组、附件、选择器等。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RecordFieldValue {
    /// 文本
    Text(String),

    /// 数字
    Number(i64),

    /// 布尔值
    Bool(bool),

    /// 日期时间戳（毫秒）
    Date(i64),

    /// 日期时间（ISO 8601 格式字符串）
    DateTime(String),

    /// 用户/部门/群组 ID
    ///
    /// 可以是单个 ID 或多个 ID 的数组
    SingleId(String),
    /// 公开项说明。
    MultipleIds(Vec<String>),

    /// 附件信息
    Attachment(Vec<AttachmentInfo>),

    /// 单选
    Select(String),

    /// 多选
    MultipleSelect(Vec<String>),

    /// 进度条
    Progress(ProgressInfo),

    /// 评分
    Rating(f64),

    /// 邮箱
    Email(String),

    /// 电话
    Phone(String),

    /// URL
    Url(String),

    /// 富文本内容
    RichText(Vec<TextSegment>),

    /// 查找引用（@提及）
    Mention(Vec<MentionInfo>),

    /// 级联记录
    Link(Vec<LinkInfo>),

    /// 公式
    Formula(String),

    /// 自动编号
    AutoNumber(AutoNumberInfo),

    /// 复选框
    Checkbox(bool),

    /// 人员
    People(Vec<PeopleInfo>),

    /// 位置
    Location(LocationInfo),

    /// 看板
    Kanban(Vec<KanbanItem>),

    /// 自定义对象
    Object(Value),
}

/// 附件信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AttachmentInfo {
    /// 文件 token
    pub file_token: String,
    /// 文件名称
    pub file_name: String,
    /// 文件大小（字节）
    pub file_size: i64,
    /// 文件类型（MIME）
    pub file_type: String,
    /// 文件 URL
    pub file_url: String,
    /// 上传时间（毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_time: Option<i64>,
    /// 上传者 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uploader_id: Option<String>,
}

/// 文本片段
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextSegment {
    /// 文本内容
    pub text: String,
    /// 文本样式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<TextStyle>,
}

/// 文本样式
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextStyle {
    /// 字体颜色（十六进制）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_color: Option<String>,
    /// 背景颜色
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
    /// 字体大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<String>,
    /// 是否粗体
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    /// 是否斜体
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    /// 是否删除线
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strikethrough: Option<bool>,
    /// 是否下划线
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underline: Option<bool>,
}

/// 提及信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MentionInfo {
    /// 提及的用户 ID
    pub user_id: String,
    /// 用户名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 提及的时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

/// 级联记录
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LinkInfo {
    /// 级联的记录 ID
    pub record_id: String,
    /// 链接的表格名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_name: Option<String>,
    /// 链接的字段名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
}

/// 进度条信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProgressInfo {
    /// 进度值（0-100）
    pub progress: i32,
    /// 进度状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 进度文本（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_text: Option<String>,
}

/// 自动编号信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutoNumberInfo {
    /// 当前编号
    pub number: i32,
    /// 编号规则
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<String>,
}

/// 人员信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PeopleInfo {
    /// 用户 ID
    pub user_id: String,
    /// 用户名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 头像 URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// 部门名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    /// 职位名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<String>,
}

/// 位置信息
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocationInfo {
    /// 地址文本
    pub address: String,
    /// 纬度
    pub latitude: f64,
    /// 纬度
    pub longitude: f64,
    /// 位置名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location_name: Option<String>,
}

/// 看板项目
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KanbanItem {
    /// 项目 ID
    pub item_id: String,
    /// 项目标题
    pub title: String,
    /// 状态
    pub status: String,
    /// 分组名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
}

/// 记录数据类型别名
///
/// 用于简化字段类型定义，提高代码可读性。
pub type RecordFields = std::collections::HashMap<String, RecordFieldValue>;

/// 记录数据序列化辅助函数
///
/// 由于 HashMap 需要特殊处理才能与 serde 兼容
/// reserved：RecordFields serde 辅助，待接入结构体（见 #267 清理）
#[expect(dead_code)]
pub fn serialize_record_fields<S>(fields: &RecordFields, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::ser::SerializeMap;
    let mut map = serializer.serialize_map(Some(fields.len()))?;
    for (k, v) in fields {
        map.serialize_entry(k, v)?;
    }
    map.end()
}

/// 记录数据反序列化辅助函数
#[expect(dead_code)]
pub fn deserialize_record_fields<'de, D>(deserializer: D) -> Result<RecordFields, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use std::collections::HashMap;
    struct RecordFieldsVisitor;

    impl<'de> serde::de::Visitor<'de> for RecordFieldsVisitor {
        type Value = RecordFields;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a map of field names to field values")
        }

        fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
        where
            M: serde::de::MapAccess<'de>,
        {
            let mut map = HashMap::new();
            while let Some(key) = access.next_key::<String>()? {
                let value = access.next_value()?;
                map.insert(key, value);
            }
            Ok(map)
        }
    }

    deserializer.deserialize_map(RecordFieldsVisitor)
}

/// 记录数据构建器
///
/// 提供便捷的方法来构建记录数据。
#[derive(Debug, Clone)]
pub struct RecordFieldsBuilder {
    inner: RecordFields,
}

impl Default for RecordFieldsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl RecordFieldsBuilder {
    /// 创建空的记录字段构建器
    pub fn new() -> Self {
        Self {
            inner: RecordFields::new(),
        }
    }

    /// 添加文本字段
    pub fn add_text(mut self, field_name: impl Into<String>, value: impl Into<String>) -> Self {
        self.inner
            .insert(field_name.into(), RecordFieldValue::Text(value.into()));
        self
    }

    /// 添加数字字段
    pub fn add_number(mut self, field_name: impl Into<String>, value: i64) -> Self {
        self.inner
            .insert(field_name.into(), RecordFieldValue::Number(value));
        self
    }

    /// 添加布尔字段
    pub fn add_bool(mut self, field_name: impl Into<String>, value: bool) -> Self {
        self.inner
            .insert(field_name.into(), RecordFieldValue::Bool(value));
        self
    }

    /// 添加日期字段
    pub fn add_date(mut self, field_name: impl Into<String>, timestamp: i64) -> Self {
        self.inner
            .insert(field_name.into(), RecordFieldValue::Date(timestamp));
        self
    }

    /// 添加用户/部门/群组 ID（单个）
    pub fn add_user_id(
        mut self,
        field_name: impl Into<String>,
        user_id: impl Into<String>,
    ) -> Self {
        self.inner.insert(
            field_name.into(),
            RecordFieldValue::SingleId(user_id.into()),
        );
        self
    }

    /// 添加用户/部门/群组 ID（多个）
    pub fn add_user_ids(mut self, field_name: impl Into<String>, user_ids: Vec<String>) -> Self {
        self.inner
            .insert(field_name.into(), RecordFieldValue::MultipleIds(user_ids));
        self
    }

    /// 添加附件字段
    pub fn add_attachment(
        mut self,
        field_name: impl Into<String>,
        attachment: AttachmentInfo,
    ) -> Self {
        self.inner.insert(
            field_name.into(),
            RecordFieldValue::Attachment(vec![attachment]),
        );
        self
    }

    /// 添加附件字段（多个）
    pub fn add_attachments(
        mut self,
        field_name: impl Into<String>,
        attachments: Vec<AttachmentInfo>,
    ) -> Self {
        self.inner
            .insert(field_name.into(), RecordFieldValue::Attachment(attachments));
        self
    }

    /// 添加单选字段
    pub fn add_select(mut self, field_name: impl Into<String>, value: impl Into<String>) -> Self {
        self.inner
            .insert(field_name.into(), RecordFieldValue::Select(value.into()));
        self
    }

    /// 添加多选字段
    pub fn add_multi_select(mut self, field_name: impl Into<String>, values: Vec<String>) -> Self {
        self.inner
            .insert(field_name.into(), RecordFieldValue::MultipleSelect(values));
        self
    }

    /// 添加进度条字段
    pub fn add_progress(
        mut self,
        field_name: impl Into<String>,
        progress: i32,
        status: impl Into<String>,
    ) -> Self {
        self.inner.insert(
            field_name.into(),
            RecordFieldValue::Progress(ProgressInfo {
                progress,
                status: Some(status.into()),
                progress_text: None,
            }),
        );
        self
    }

    /// 添加进度条字段（带文本）
    pub fn add_progress_with_text(
        mut self,
        field_name: impl Into<String>,
        progress: i32,
        status: impl Into<String>,
        progress_text: impl Into<String>,
    ) -> Self {
        self.inner.insert(
            field_name.into(),
            RecordFieldValue::Progress(ProgressInfo {
                progress,
                status: Some(status.into()),
                progress_text: Some(progress_text.into()),
            }),
        );
        self
    }

    /// 添加评分字段
    pub fn add_rating(mut self, field_name: impl Into<String>, rating: f64) -> Self {
        self.inner
            .insert(field_name.into(), RecordFieldValue::Rating(rating));
        self
    }

    /// 添加邮箱字段
    pub fn add_email(mut self, field_name: impl Into<String>, email: impl Into<String>) -> Self {
        self.inner
            .insert(field_name.into(), RecordFieldValue::Email(email.into()));
        self
    }

    /// 添加电话字段
    pub fn add_phone(mut self, field_name: impl Into<String>, phone: impl Into<String>) -> Self {
        self.inner
            .insert(field_name.into(), RecordFieldValue::Phone(phone.into()));
        self
    }

    /// 添加 URL 字段
    pub fn add_url(mut self, field_name: impl Into<String>, url: impl Into<String>) -> Self {
        self.inner
            .insert(field_name.into(), RecordFieldValue::Url(url.into()));
        self
    }

    /// 添加富文本字段
    pub fn add_rich_text(
        mut self,
        field_name: impl Into<String>,
        segments: Vec<TextSegment>,
    ) -> Self {
        self.inner
            .insert(field_name.into(), RecordFieldValue::RichText(segments));
        self
    }

    /// 添加提及字段
    pub fn add_mention(mut self, field_name: impl Into<String>, mention: MentionInfo) -> Self {
        self.inner
            .insert(field_name.into(), RecordFieldValue::Mention(vec![mention]));
        self
    }

    /// 添加提及字段（多个）
    pub fn add_mentions(
        mut self,
        field_name: impl Into<String>,
        mentions: Vec<MentionInfo>,
    ) -> Self {
        self.inner
            .insert(field_name.into(), RecordFieldValue::Mention(mentions));
        self
    }

    /// 添加级联字段（单个）
    pub fn add_link(mut self, field_name: impl Into<String>, link: LinkInfo) -> Self {
        self.inner
            .insert(field_name.into(), RecordFieldValue::Link(vec![link]));
        self
    }

    /// 添加级联字段（多个）
    pub fn add_links(mut self, field_name: impl Into<String>, links: Vec<LinkInfo>) -> Self {
        self.inner
            .insert(field_name.into(), RecordFieldValue::Link(links));
        self
    }

    /// 构建记录字段
    pub fn build(self) -> RecordFields {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_types_text() {
        let field = RecordFieldValue::Text("Hello".to_string());
        let value = json!(field);
        assert_eq!(value, json!("Hello"));
    }

    #[test]
    fn test_field_types_number() {
        let field = RecordFieldValue::Number(42);
        let value = json!(field);
        assert_eq!(value, json!(42));
    }

    #[test]
    fn test_field_types_bool() {
        let field = RecordFieldValue::Bool(true);
        let value = json!(field);
        assert_eq!(value, json!(true));
    }

    #[test]
    fn test_field_types_single_id() {
        let field = RecordFieldValue::SingleId("user_123".to_string());
        let value = json!(field);
        assert_eq!(value, json!("user_123"));
    }

    #[test]
    fn test_field_types_multiple_ids() {
        let field =
            RecordFieldValue::MultipleIds(vec!["user_123".to_string(), "user_456".to_string()]);
        let value = json!(field);
        assert_eq!(value, json!(["user_123", "user_456"]));
    }

    #[test]
    fn test_builder() {
        let fields = RecordFieldsBuilder::new()
            .add_text("姓名", "张三")
            .add_number("年龄", 25)
            .add_bool("在职", true)
            .add_user_id("负责人", "user_123")
            .add_date("入职日期", 1234567890000i64)
            .add_select("状态", "进行中")
            .build();

        assert_eq!(
            fields.get("姓名"),
            Some(&RecordFieldValue::Text("张三".to_string()))
        );
        assert_eq!(fields.get("年龄"), Some(&RecordFieldValue::Number(25)));
        assert_eq!(fields.get("在职"), Some(&RecordFieldValue::Bool(true)));
        assert_eq!(
            fields.get("负责人"),
            Some(&RecordFieldValue::SingleId("user_123".to_string()))
        );
    }
}
