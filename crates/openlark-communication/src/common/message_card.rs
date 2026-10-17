//! 类型化消息卡片。
//!
//! 只负责把标题和组件序列化成 interactive JSON。发送仍走 `ImClient::send_card`。

use openlark_core::{SDKResult, error::validation_error, validate_required};
use serde_json::{Map, Value};

/// 卡片按钮样式。序列化键是 `type`。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonKind {
    /// 默认样式。
    Default,
    /// 主按钮。
    Primary,
    /// 危险操作。
    Danger,
}

impl ButtonKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::Primary => "primary",
            Self::Danger => "danger",
        }
    }
}

/// 表单按钮动作。JSON 1.0 的键是 `action_type`。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormAction {
    /// 提交表单。
    Submit,
}

impl FormAction {
    fn as_str(self) -> &'static str {
        match self {
            Self::Submit => "form_submit",
        }
    }
}

/// 表格列。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableColumn {
    name: String,
    display_name: String,
    data_type: String,
}

impl TableColumn {
    /// 文本列。`data_type` 固定为 `text`。
    pub fn text(name: impl Into<String>, display_name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            display_name: display_name.into(),
            data_type: "text".to_string(),
        }
    }
}

/// 按钮。链式设置之后用 `element` 放进卡片。
#[derive(Debug, Clone, PartialEq)]
pub struct Button {
    text: String,
    url: Option<String>,
    value: Option<Value>,
    kind: Option<ButtonKind>,
    name: Option<String>,
    action_type: Option<FormAction>,
}

impl Button {
    /// 跳转地址。
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.url = Some(url.into());
        self
    }

    /// 回传给卡片回调的 JSON 对象。
    pub fn value(mut self, value: Value) -> Self {
        self.value = Some(value);
        self
    }

    /// 按钮样式。
    pub fn kind(mut self, kind: ButtonKind) -> Self {
        self.kind = Some(kind);
        self
    }

    /// 按钮名。表单提交按钮必填。
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 表单动作。提交按钮使用 `FormAction::Submit`。
    pub fn action_type(mut self, action_type: FormAction) -> Self {
        self.action_type = Some(action_type);
        self
    }

    /// 放进 `MessageCard::elements` 或容器的元素列表。
    pub fn element(self) -> CardElement {
        CardElement {
            kind: ElementKind::Button(self),
        }
    }
}

/// 分栏里的一列。只能交给 `column_set`。
#[derive(Debug, Clone, PartialEq)]
pub struct Column {
    elements: Vec<CardElement>,
}

/// 消息卡片上的一个组件。
///
/// 调用方通过 `div_plain`、`button`、`column_set`、`form`、`chart`、`table` 构造。
#[derive(Debug, Clone, PartialEq)]
pub struct CardElement {
    kind: ElementKind,
}

#[derive(Debug, Clone, PartialEq)]
enum ElementKind {
    Div(Div),
    Button(Button),
    ColumnSet(ColumnSet),
    Form(Form),
    Chart(Chart),
    Table(Table),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TextKind {
    Plain,
    LarkMd,
}

impl TextKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::Plain => "plain_text",
            Self::LarkMd => "lark_md",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Div {
    kind: TextKind,
    content: String,
}

#[derive(Debug, Clone, PartialEq)]
struct ColumnSet {
    columns: Vec<Column>,
}

#[derive(Debug, Clone, PartialEq)]
struct Form {
    name: String,
    elements: Vec<CardElement>,
}

#[derive(Debug, Clone, PartialEq)]
struct Chart {
    spec: Value,
}

#[derive(Debug, Clone, PartialEq)]
struct Table {
    columns: Vec<TableColumn>,
    rows: Vec<Value>,
}

/// 纯文本内容块。
pub fn div_plain(text: impl Into<String>) -> CardElement {
    CardElement {
        kind: ElementKind::Div(Div {
            kind: TextKind::Plain,
            content: text.into(),
        }),
    }
}

/// Markdown 内容块。
pub fn div_md(text: impl Into<String>) -> CardElement {
    CardElement {
        kind: ElementKind::Div(Div {
            kind: TextKind::LarkMd,
            content: text.into(),
        }),
    }
}

/// 按钮。文本不能为空白。
pub fn button(text: impl Into<String>) -> Button {
    Button {
        text: text.into(),
        url: None,
        value: None,
        kind: None,
        name: None,
        action_type: None,
    }
}

/// 分栏中的一列。
pub fn column(elements: impl IntoIterator<Item = CardElement>) -> Column {
    Column {
        elements: elements.into_iter().collect(),
    }
}

/// 多列布局。
pub fn column_set(columns: impl IntoIterator<Item = Column>) -> CardElement {
    CardElement {
        kind: ElementKind::ColumnSet(ColumnSet {
            columns: columns.into_iter().collect(),
        }),
    }
}

/// 表单。直接子节点不能是内容块，提交按钮要带名称和 `FormAction::Submit`。
pub fn form(
    name: impl Into<String>,
    elements: impl IntoIterator<Item = CardElement>,
) -> CardElement {
    CardElement {
        kind: ElementKind::Form(Form {
            name: name.into(),
            elements: elements.into_iter().collect(),
        }),
    }
}

/// 图表。`spec` 是 VChart 规格对象。
pub fn chart(spec: Value) -> CardElement {
    CardElement {
        kind: ElementKind::Chart(Chart { spec }),
    }
}

/// 表格。只允许放在卡片根上。
pub fn table(
    columns: impl IntoIterator<Item = TableColumn>,
    rows: impl IntoIterator<Item = Value>,
) -> CardElement {
    CardElement {
        kind: ElementKind::Table(Table {
            columns: columns.into_iter().collect(),
            rows: rows.into_iter().collect(),
        }),
    }
}

/// 可序列化成 interactive 卡片 JSON 的消息卡片。
#[derive(Debug, Clone, PartialEq)]
pub struct MessageCard {
    title: Option<String>,
    elements: Vec<CardElement>,
}

impl MessageCard {
    /// 空卡片。标题和元素在 `to_value` 时校验。
    pub fn new() -> Self {
        Self {
            title: None,
            elements: Vec::new(),
        }
    }

    /// 设置标题。空白标题会在 `to_value` 失败。
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// 设置组件列表，重复调用会覆盖上一次。
    pub fn elements(mut self, elements: impl IntoIterator<Item = CardElement>) -> Self {
        self.elements = elements.into_iter().collect();
        self
    }

    /// 序列化成 `send_card` 接受的卡片 JSON。
    pub fn to_value(self) -> SDKResult<Value> {
        let title = self.title.unwrap_or_default();
        validate_required!(title, "title 不能为空");
        for element in &self.elements {
            validate_element(element, Place::Root)?;
        }
        let mut elements = Vec::with_capacity(self.elements.len());
        for element in &self.elements {
            elements.push(write_element(element, Place::Root)?);
        }
        Ok(serde_json::json!({
            "header": {
                "title": {
                    "tag": "plain_text",
                    "content": title.trim(),
                }
            },
            "elements": elements,
        }))
    }
}

impl Default for MessageCard {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Place {
    Root,
    Column,
    /// 表单的直接子节点。内容块不能放在这里。
    Form,
    /// 表单内部的列。内容块可以放，图表、表单、表格仍然不行。
    FormColumn,
}

fn column_child_place(place: Place) -> Place {
    match place {
        Place::Form | Place::FormColumn => Place::FormColumn,
        Place::Root | Place::Column => Place::Column,
    }
}

fn validate_element(element: &CardElement, place: Place) -> SDKResult<()> {
    match &element.kind {
        ElementKind::Div(div) => {
            if place == Place::Form {
                return Err(validation_error("div", "表单不能直接包含 div"));
            }
            validate_required!(div.content, "text 不能为空");
        }
        ElementKind::Button(button) => validate_button(button)?,
        ElementKind::ColumnSet(set) => {
            let child_place = column_child_place(place);
            for column in &set.columns {
                for child in &column.elements {
                    validate_element(child, child_place)?;
                }
            }
        }
        ElementKind::Form(form) => {
            if place != Place::Root {
                return Err(validation_error("form", "form 只能放在卡片根上"));
            }
            validate_form(form)?;
        }
        ElementKind::Chart(chart) => {
            if matches!(place, Place::Form | Place::FormColumn) {
                return Err(validation_error("chart", "表单不能包含 chart"));
            }
            if !chart.spec.is_object() {
                return Err(validation_error(
                    "chart_spec",
                    "chart_spec 必须是 JSON 对象",
                ));
            }
        }
        ElementKind::Table(table) => {
            if place != Place::Root {
                return Err(validation_error("table", "table 只能放在卡片根上"));
            }
            validate_table(table)?;
        }
    }
    Ok(())
}

fn validate_button(button: &Button) -> SDKResult<()> {
    validate_required!(button.text, "text 不能为空");
    if let Some(url) = &button.url {
        validate_required!(url, "url 不能为空");
    }
    if let Some(name) = &button.name {
        validate_required!(name, "name 不能为空");
    }
    if let Some(value) = &button.value
        && !value.is_object()
    {
        return Err(validation_error("value", "value 必须是 JSON 对象"));
    }
    Ok(())
}

fn validate_form(form: &Form) -> SDKResult<()> {
    validate_required!(form.name, "name 不能为空");
    for element in &form.elements {
        validate_element(element, Place::Form)?;
    }
    let has_submit = form.elements.iter().any(contains_submit);
    if !has_submit {
        return Err(validation_error(
            "action_type",
            "表单必须包含 action_type 为 form_submit 的按钮",
        ));
    }
    Ok(())
}

fn is_submit(button: &Button) -> bool {
    let named = button
        .name
        .as_ref()
        .is_some_and(|name| !name.trim().is_empty());
    named && button.action_type == Some(FormAction::Submit)
}

fn contains_submit(element: &CardElement) -> bool {
    match &element.kind {
        ElementKind::Button(button) => is_submit(button),
        ElementKind::ColumnSet(set) => set
            .columns
            .iter()
            .any(|column| column.elements.iter().any(contains_submit)),
        ElementKind::Div(_)
        | ElementKind::Form(_)
        | ElementKind::Chart(_)
        | ElementKind::Table(_) => false,
    }
}

fn validate_table(table: &Table) -> SDKResult<()> {
    if table.columns.is_empty() {
        return Err(validation_error("columns", "columns 不能为空"));
    }
    for column in &table.columns {
        validate_required!(column.name, "name 不能为空");
        validate_required!(column.display_name, "display_name 不能为空");
        validate_required!(column.data_type, "data_type 不能为空");
    }
    for row in &table.rows {
        if !row.is_object() {
            return Err(validation_error("rows", "rows 中的每一行必须是 JSON 对象"));
        }
    }
    Ok(())
}

fn write_element(element: &CardElement, place: Place) -> SDKResult<Value> {
    match &element.kind {
        ElementKind::Div(div) => Ok(serde_json::json!({
            "tag": "div",
            "text": {
                "tag": div.kind.as_str(),
                "content": div.content.trim(),
            }
        })),
        ElementKind::Button(button) => {
            let written = write_button(button)?;
            if place == Place::Root {
                Ok(serde_json::json!({
                    "tag": "action",
                    "actions": [written],
                }))
            } else {
                Ok(written)
            }
        }
        ElementKind::ColumnSet(set) => {
            let child_place = column_child_place(place);
            let mut columns = Vec::with_capacity(set.columns.len());
            for column in &set.columns {
                let mut elements = Vec::with_capacity(column.elements.len());
                for child in &column.elements {
                    elements.push(write_element(child, child_place)?);
                }
                columns.push(serde_json::json!({
                    "tag": "column",
                    "elements": elements,
                }));
            }
            Ok(serde_json::json!({
                "tag": "column_set",
                "columns": columns,
            }))
        }
        ElementKind::Form(form) => {
            let mut elements = Vec::with_capacity(form.elements.len());
            for child in &form.elements {
                elements.push(write_element(child, Place::Form)?);
            }
            Ok(serde_json::json!({
                "tag": "form",
                "name": form.name.trim(),
                "elements": elements,
            }))
        }
        ElementKind::Chart(chart) => Ok(serde_json::json!({
            "tag": "chart",
            "chart_spec": chart.spec,
        })),
        ElementKind::Table(table) => {
            let columns: Vec<Value> = table
                .columns
                .iter()
                .map(|column| {
                    serde_json::json!({
                        "name": column.name.trim(),
                        "display_name": column.display_name.trim(),
                        "data_type": column.data_type.trim(),
                    })
                })
                .collect();
            Ok(serde_json::json!({
                "tag": "table",
                "columns": columns,
                "rows": table.rows,
            }))
        }
    }
}

fn write_button(button: &Button) -> SDKResult<Value> {
    let mut object = Map::new();
    object.insert("tag".to_string(), Value::String("button".to_string()));
    object.insert(
        "text".to_string(),
        serde_json::json!({
            "tag": "plain_text",
            "content": button.text.trim(),
        }),
    );
    if let Some(kind) = button.kind {
        object.insert("type".to_string(), Value::String(kind.as_str().to_string()));
    }
    if let Some(url) = &button.url {
        object.insert("url".to_string(), Value::String(url.trim().to_string()));
    }
    if let Some(name) = &button.name {
        object.insert("name".to_string(), Value::String(name.trim().to_string()));
    }
    if let Some(action_type) = button.action_type {
        object.insert(
            "action_type".to_string(),
            Value::String(action_type.as_str().to_string()),
        );
    }
    if let Some(value) = &button.value {
        object.insert("value".to_string(), value.clone());
    }
    Ok(Value::Object(object))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::chain::MessageRecipient;
    use crate::common::chain::interactive_message_body;

    fn sample_card() -> MessageCard {
        MessageCard::new().title("发布完成").elements([
            div_plain("今天已上线"),
            button("查看")
                .url("https://example.com")
                .kind(ButtonKind::Primary)
                .element(),
            column_set([column([div_plain("左")]), column([div_plain("右")])]),
            form(
                "approve",
                [
                    column_set([column([div_plain("请填写原因")])]),
                    button("提交")
                        .name("submit")
                        .action_type(FormAction::Submit)
                        .element(),
                ],
            ),
            chart(serde_json::json!({ "type": "bar" })),
            table(
                [TableColumn::text("name", "名称")],
                vec![serde_json::json!({ "name": "openlark" })],
            ),
        ])
    }

    #[test]
    fn title_div_button_json() {
        let card = MessageCard::new().title("发布完成").elements([
            div_plain("今天已上线"),
            button("查看")
                .url("https://example.com")
                .kind(ButtonKind::Primary)
                .element(),
        ]);
        let value = card.to_value().expect("卡片应该能序列化");
        let expected = serde_json::json!({
            "header": {
                "title": { "tag": "plain_text", "content": "发布完成" }
            },
            "elements": [
                {
                    "tag": "div",
                    "text": { "tag": "plain_text", "content": "今天已上线" }
                },
                {
                    "tag": "action",
                    "actions": [
                        {
                            "tag": "button",
                            "text": { "tag": "plain_text", "content": "查看" },
                            "type": "primary",
                            "url": "https://example.com"
                        }
                    ]
                }
            ]
        });
        assert_eq!(value, expected);
        assert!(value.get("schema").is_none());

        let body = interactive_message_body(MessageRecipient::open_id("ou_xxx"), value.clone())
            .expect("消息体应该能组装");
        assert_eq!(body.msg_type, "interactive");
        let parsed: Value = serde_json::from_str(&body.content).expect("content 应该是 JSON");
        assert_eq!(parsed, value);
    }

    #[test]
    fn card_elements_one_list() {
        let value = sample_card().to_value().expect("整张卡片应该能序列化");
        let elements = value["elements"].as_array().expect("正文是数组");
        assert_eq!(
            elements[2],
            serde_json::json!({
                "tag": "column_set",
                "columns": [
                    {
                        "tag": "column",
                        "elements": [
                            {
                                "tag": "div",
                                "text": { "tag": "plain_text", "content": "左" }
                            }
                        ]
                    },
                    {
                        "tag": "column",
                        "elements": [
                            {
                                "tag": "div",
                                "text": { "tag": "plain_text", "content": "右" }
                            }
                        ]
                    }
                ]
            })
        );
        assert_eq!(
            elements[3],
            serde_json::json!({
                "tag": "form",
                "name": "approve",
                "elements": [
                    {
                        "tag": "column_set",
                        "columns": [
                            {
                                "tag": "column",
                                "elements": [
                                    {
                                        "tag": "div",
                                        "text": { "tag": "plain_text", "content": "请填写原因" }
                                    }
                                ]
                            }
                        ]
                    },
                    {
                        "tag": "button",
                        "text": { "tag": "plain_text", "content": "提交" },
                        "name": "submit",
                        "action_type": "form_submit"
                    }
                ]
            })
        );
        assert_eq!(
            elements[4],
            serde_json::json!({
                "tag": "chart",
                "chart_spec": { "type": "bar" }
            })
        );
        assert_eq!(
            elements[5],
            serde_json::json!({
                "tag": "table",
                "columns": [
                    { "name": "name", "display_name": "名称", "data_type": "text" }
                ],
                "rows": [{ "name": "openlark" }]
            })
        );
    }

    #[test]
    fn card_rejects_blank() {
        let title = MessageCard::new()
            .title("  ")
            .elements([div_plain("内容"), button("查看").element()])
            .to_value()
            .expect_err("空白标题");
        assert!(title.to_string().contains("title"));

        let text = MessageCard::new()
            .title("标题")
            .elements([div_plain("  "), button("查看").element()])
            .to_value()
            .expect_err("空白内容");
        assert!(text.to_string().contains("text"));

        let button_text = MessageCard::new()
            .title("标题")
            .elements([div_plain("内容"), button("  ").element()])
            .to_value()
            .expect_err("空白按钮");
        assert!(button_text.to_string().contains("text"));

        let name = MessageCard::new()
            .title("标题")
            .elements([form(
                "  ",
                [button("提交")
                    .name("submit")
                    .action_type(FormAction::Submit)
                    .element()],
            )])
            .to_value()
            .expect_err("空表单名");
        assert!(name.to_string().contains("name"));
    }

    #[test]
    fn card_rejects_placement() {
        let direct_div = MessageCard::new()
            .title("标题")
            .elements([form(
                "approve",
                [
                    div_plain("请填写原因"),
                    button("提交")
                        .name("submit")
                        .action_type(FormAction::Submit)
                        .element(),
                ],
            )])
            .to_value();
        assert!(direct_div.is_err());

        let missing_submit = MessageCard::new()
            .title("标题")
            .elements([form(
                "approve",
                [column_set([column([div_plain("请填写原因")])])],
            )])
            .to_value();
        assert!(missing_submit.is_err());

        let form_in_column = MessageCard::new()
            .title("标题")
            .elements([column_set([column([form(
                "approve",
                [button("提交")
                    .name("submit")
                    .action_type(FormAction::Submit)
                    .element()],
            )])])])
            .to_value();
        assert!(form_in_column.is_err());

        let table_in_column = MessageCard::new()
            .title("标题")
            .elements([column_set([column([table(
                [TableColumn::text("name", "名称")],
                vec![serde_json::json!({ "name": "openlark" })],
            )])])])
            .to_value();
        assert!(table_in_column.is_err());
    }

    #[test]
    fn div_md_uses_lark_md_text() {
        let value = MessageCard::new()
            .title("标题")
            .elements([div_md("**完成**"), button("查看").element()])
            .to_value()
            .expect("markdown 内容块应该能序列化");
        assert_eq!(value["elements"][0]["text"]["tag"], "lark_md");
        assert_eq!(value["elements"][0]["text"]["content"], "**完成**");
    }
}
