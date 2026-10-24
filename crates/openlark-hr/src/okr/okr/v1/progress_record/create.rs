//! 创建 OKR 进展记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v1/progress_record/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 创建 OKR 进展记录请求
#[derive(Debug, Clone)]
pub struct CreateRequest {
    /// 进展来源（必填）
    source_title: String,
    /// 进展来源链接（必填，须以 http:// 或 https:// 开头）
    source_url: String,
    /// 目标 ID（必填，与 target_type 对应）
    target_id: String,
    /// 目标类型（必填）
    /// - 2: okr 的 O
    /// - 3: okr 的 KR
    target_type: i32,
    /// 进展详情 富文本格式（必填，content_block）
    content: ContentBlock,
    /// pc 进展来源链接（可选，须以 http:// 或 https:// 开头）
    source_url_pc: Option<String>,
    /// mobile 进展来源链接（可选，须以 http:// 或 https:// 开头）
    source_url_mobile: Option<String>,
    /// 进展，包括百分比和状态（可选，progress_rate_new）
    progress_rate: Option<ProgressRateNew>,
    /// 用户 ID 类型（可选 query 参数，默认 open_id）
    user_id_type: Option<String>,
    /// 配置信息
    config: Config,
}

impl CreateRequest {
    /// 创建请求
    pub fn new(
        config: Config,
        source_title: String,
        source_url: String,
        target_id: String,
        target_type: i32,
        content: ContentBlock,
    ) -> Self {
        Self {
            source_title,
            source_url,
            target_id,
            target_type,
            content,
            source_url_pc: None,
            source_url_mobile: None,
            progress_rate: None,
            user_id_type: None,
            config,
        }
    }

    /// 设置 pc 进展来源链接（可选）
    pub fn source_url_pc(mut self, source_url_pc: String) -> Self {
        self.source_url_pc = Some(source_url_pc);
        self
    }

    /// 设置 mobile 进展来源链接（可选）
    pub fn source_url_mobile(mut self, source_url_mobile: String) -> Self {
        self.source_url_mobile = Some(source_url_mobile);
        self
    }

    /// 设置进展百分比与状态（可选）
    pub fn progress_rate(mut self, progress_rate: ProgressRateNew) -> Self {
        self.progress_rate = Some(progress_rate);
        self
    }

    /// 设置用户 ID 类型（可选 query 参数，open_id/union_id/user_id）
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateResponse> {
        use crate::common::api_endpoints::OkrApiV1;

        // 1. 验证必填字段
        validate_required!(self.source_title.trim(), "进展来源不能为空");
        validate_source_url(&self.source_url, "source_url")?;
        validate_required!(self.target_id.trim(), "目标 ID 不能为空");
        if !(2..=3).contains(&self.target_type) {
            return Err(openlark_core::error::validation_error(
                "target_type 无效",
                "target_type 必须为 2（okr 的 O）或 3（okr 的 KR）",
            ));
        }
        validate_required!(self.content.blocks, "进展内容不能为空");
        if let Some(url) = &self.source_url_pc {
            validate_source_url(url, "source_url_pc")?;
        }
        if let Some(url) = &self.source_url_mobile {
            validate_source_url(url, "source_url_mobile")?;
        }

        // 2. 构建端点
        let api_endpoint = OkrApiV1::ProgressRecordCreate;
        let request = ApiRequest::<CreateResponse>::post(api_endpoint.to_url())
            .query_opt("user_id_type", self.user_id_type);

        // 3. 序列化请求体
        let request_body = CreateRequestBody {
            source_title: self.source_title,
            source_url: self.source_url,
            target_id: self.target_id,
            target_type: self.target_type,
            content: self.content,
            source_url_pc: self.source_url_pc,
            source_url_mobile: self.source_url_mobile,
            progress_rate: self.progress_rate,
        };
        let request = request.body(serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?);

        // 4. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "创建 OKR 进展记录响应数据为空",
        )
        .await
    }
}

/// 校验进展来源链接符合官方正则 `^https?://.*$`
fn validate_source_url(url: &str, field: &str) -> SDKResult<()> {
    let trimmed = url.trim();
    if trimmed.is_empty() || !(trimmed.starts_with("http://") || trimmed.starts_with("https://")) {
        return Err(openlark_core::error::validation_error(
            field,
            format!("{field} 必须是以 http:// 或 https:// 开头的链接"),
        ));
    }
    Ok(())
}

/// 创建 OKR 进展记录请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequestBody {
    /// 进展来源（必填）
    pub source_title: String,
    /// 进展来源链接（必填）
    pub source_url: String,
    /// 目标 ID（必填）
    pub target_id: String,
    /// 目标类型（必填，2=O/3=KR）
    pub target_type: i32,
    /// 进展详情 富文本格式（必填）
    pub content: ContentBlock,
    /// pc 进展来源链接（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url_pc: Option<String>,
    /// mobile 进展来源链接（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_url_mobile: Option<String>,
    /// 进展，包括百分比和状态（可选）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress_rate: Option<ProgressRateNew>,
}

/// 进展记录详情 富文本格式（content_block，v1 进展记录接口专用）
///
/// 官方 JSON 键为 camelCase（type/textRun/imageList/fileToken 等），与 OKR v2 的
/// content_block 不同，不可混用。
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ContentBlock {
    /// 文档结构按行排列，每行内容是一个 Block
    #[serde(default)]
    pub blocks: Vec<ContentBlockElement>,
}

/// 文档块元素（判别联合，`type` 为 paragraph / gallery）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ContentBlockElement {
    /// 文档元素类型（"paragraph" | "gallery"）
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub block_element_type: Option<String>,
    /// 文本段落（type = paragraph 时存在）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub paragraph: Option<ContentParagraph>,
    /// 图片画廊（type = gallery 时存在）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gallery: Option<ContentGallery>,
}

/// 文本段落（content_paragraph）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ContentParagraph {
    /// 段落样式
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<ContentParagraphStyle>,
    /// 段落元素列表
    #[serde(default)]
    pub elements: Vec<ContentParagraphElement>,
}

/// 段落样式（content_paragraph_style）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ContentParagraphStyle {
    /// 有序列表/无序列表/任务列表样式
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub list: Option<ContentList>,
}

/// 列表样式（content_list）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ContentList {
    /// 列表类型（"number" | "bullet" | "checkBox" | "checkedBox" | "indent"）
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub list_type: Option<String>,
    /// 列表的缩进级别（1-16）
    #[serde(
        rename = "indentLevel",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub indent_level: Option<i32>,
    /// 有序列表行号
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub number: Option<i32>,
}

/// 段落元素（判别联合，`type` 为 textRun / docsLink / person）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ContentParagraphElement {
    /// 元素类型（"textRun" | "docsLink" | "person"）
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub paragraph_element_type: Option<String>,
    /// 文本（type = textRun 时存在）
    #[serde(rename = "textRun", default, skip_serializing_if = "Option::is_none")]
    pub text_run: Option<ContentTextRun>,
    /// 飞书云文档链接（type = docsLink 时存在）
    #[serde(rename = "docsLink", default, skip_serializing_if = "Option::is_none")]
    pub docs_link: Option<ContentDocsLink>,
    /// 艾特用户（type = person 时存在）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person: Option<ContentPerson>,
}

/// 文本串（content_text_run）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ContentTextRun {
    /// 具体的文本内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// 文本内容的样式
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<ContentTextStyle>,
}

/// 文本样式（content_text_style）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ContentTextStyle {
    /// 是否加粗
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    /// 是否删除线
    #[serde(
        rename = "strikeThrough",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub strike_through: Option<bool>,
    /// 背景颜色
    #[serde(rename = "backColor", default, skip_serializing_if = "Option::is_none")]
    pub back_color: Option<ContentColor>,
    /// 字体颜色
    #[serde(rename = "textColor", default, skip_serializing_if = "Option::is_none")]
    pub text_color: Option<ContentColor>,
    /// 链接地址
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link: Option<ContentLink>,
}

/// RGBA 颜色（content_color）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ContentColor {
    /// 红 \[0,255\]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub red: Option<i32>,
    /// 绿 \[0,255\]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub green: Option<i32>,
    /// 蓝 \[0,255\]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blue: Option<i32>,
    /// 透明度 \[0,1\]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub alpha: Option<f64>,
}

/// 链接地址（content_link）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ContentLink {
    /// 链接地址
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// 飞书云文档链接（content_docs_link）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ContentDocsLink {
    /// 飞书云文档链接地址
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 飞书云文档标题
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// 艾特用户（content_person）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ContentPerson {
    /// 员工的 OpenID
    #[serde(rename = "openId", default, skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    /// 员工的 UserID（响应中可能返回）
    #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// 图片画廊（content_gallery）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ContentGallery {
    /// 图片元素列表
    #[serde(rename = "imageList", default)]
    pub image_list: Vec<ContentImageItem>,
}

/// 图片元素（content_image_item）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ContentImageItem {
    /// 图片 token，通过上传图片接口获取
    #[serde(rename = "fileToken", default, skip_serializing_if = "Option::is_none")]
    pub file_token: Option<String>,
    /// 图片链接
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub src: Option<String>,
    /// 图片宽（px）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    /// 图片高（px）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
}

/// 进展，包括百分比和状态（progress_rate_new）
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ProgressRateNew {
    /// 进展百分比，保留两位小数
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percent: Option<f64>,
    /// 进展状态（-1 暂无/0 正常/1 风险/2 延期）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
}

/// 创建 OKR 进展记录响应（progress_record）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponse {
    /// OKR 进展 ID
    pub progress_id: String,
    /// 进展更新时间（毫秒）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modify_time: Option<String>,
    /// 进展对应的 Content 详细内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<ContentBlock>,
    /// 进展，包括百分比和状态
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_rate: Option<ProgressRateNew>,
}

impl ApiResponseTrait for CreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;

    /// 构造一段官方形状的富文本进展内容
    fn sample_content() -> ContentBlock {
        ContentBlock {
            blocks: vec![ContentBlockElement {
                block_element_type: Some("paragraph".to_string()),
                paragraph: Some(ContentParagraph {
                    style: None,
                    elements: vec![ContentParagraphElement {
                        paragraph_element_type: Some("textRun".to_string()),
                        text_run: Some(ContentTextRun {
                            text: Some("本周进展顺利".to_string()),
                            style: Some(ContentTextStyle {
                                bold: Some(true),
                                ..Default::default()
                            }),
                        }),
                        docs_link: None,
                        person: None,
                    }],
                }),
                gallery: None,
            }],
        }
    }

    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_okr_v1_progress_record_create_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{body_json, method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{"progress_id": "test", "modify_time": "1640677213095", "content": {"blocks": []}}"#,
        )
        .unwrap();
        Mock::given(method("POST"))
            .and(path("/open-apis/okr/v1/progress_records"))
            .and(body_json(json!({
                "source_title": "周报系统",
                "source_url": "https://www.zhoubao.com",
                "target_id": "7041430377642082323",
                "target_type": 2,
                "content": {"blocks": [{"type": "paragraph", "paragraph": {"elements": [{"type": "textRun", "textRun": {"text": "本周进展顺利", "style": {"bold": true}}}]}}, {"type": "gallery", "gallery": {"imageList": []}}]}
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": data_body
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let mut content = sample_content();
        content.blocks.push(ContentBlockElement {
            block_element_type: Some("gallery".to_string()),
            paragraph: None,
            gallery: Some(ContentGallery {
                image_list: Vec::new(),
            }),
        });

        let data = CreateRequest::new(
            config,
            "周报系统".to_string(),
            "https://www.zhoubao.com".to_string(),
            "7041430377642082323".to_string(),
            2,
            content,
        )
        .execute()
        .await
        .expect("okr_v1_progress_record_create 应成功");

        assert_eq!(data.progress_id, "test");
        assert_eq!(data.modify_time.as_deref(), Some("1640677213095"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/okr/v1/progress_records");
    }

    /// 官方请求体形状：无 okr_id/description/attachments，progress_rate 为嵌套对象
    #[test]
    fn test_create_request_body_shape() {
        let body = CreateRequestBody {
            source_title: "周报系统".to_string(),
            source_url: "https://www.zhoubao.com".to_string(),
            target_id: "7041430377642082323".to_string(),
            target_type: 2,
            content: sample_content(),
            source_url_pc: None,
            source_url_mobile: None,
            progress_rate: Some(ProgressRateNew {
                percent: Some(50.21),
                status: Some(0),
            }),
        };

        let value = serde_json::to_value(&body).expect("序列化请求体失败");
        assert_eq!(value["source_title"], serde_json::json!("周报系统"));
        assert_eq!(value["target_type"], serde_json::json!(2));
        assert_eq!(value["progress_rate"]["percent"], serde_json::json!(50.21));
        assert!(value.get("okr_id").is_none());
        assert!(value.get("description").is_none());
        assert!(value.get("attachments").is_none());
        assert!(value.get("source_url_pc").is_none());
        // 富文本键为官方 camelCase
        assert_eq!(
            value["content"]["blocks"][0]["type"],
            serde_json::json!("paragraph")
        );
        assert_eq!(
            value["content"]["blocks"][0]["paragraph"]["elements"][0]["textRun"]["text"],
            serde_json::json!("本周进展顺利")
        );
    }
}
