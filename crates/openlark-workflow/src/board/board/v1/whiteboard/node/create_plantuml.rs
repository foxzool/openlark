//! 创建 PlantUML 节点（v1）
//! docPath: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard-node/create_plantuml>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 创建 PlantUML 节点请求体。
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreatePlantumlNodeBodyV1 {
    /// PlantUML 源码，长度范围 1 ～ 1000000 字符。
    pub plant_uml_code: String,
    /// 画板样式：1 Artboard style，2 Classic style。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style_type: Option<i32>,
    /// 语法类型：0 unknown，1 PlantUML 解析，2 Mermaid 解析。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub syntax_type: Option<i32>,
    /// PlantUML 语法类型，传 0 会自动识别语法类型：0 unknown，1 mind map，2 sequence diagram，3 Activity diagram，4 class diagram。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagram_type: Option<i32>,
}

/// 创建 PlantUML 节点响应。
#[derive(Debug, Clone, Deserialize)]
pub struct CreatePlantumlNodeResponseV1 {
    /// 节点 ID。
    pub node_id: String,
}

/// 创建 PlantUML 节点请求构建器。
#[derive(Debug, Clone)]
pub struct CreatePlantumlNodeRequestV1 {
    config: Arc<Config>,
    board_id: String,
    body: CreatePlantumlNodeBodyV1,
}

impl CreatePlantumlNodeRequestV1 {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, board_id: impl Into<String>) -> Self {
        Self {
            config,
            board_id: board_id.into(),
            body: CreatePlantumlNodeBodyV1::default(),
        }
    }

    /// 设置 PlantUML 源码（必填）。
    pub fn plant_uml_code(mut self, plant_uml_code: impl Into<String>) -> Self {
        self.body.plant_uml_code = plant_uml_code.into();
        self
    }

    /// 设置画板样式：1 Artboard style，2 Classic style。
    pub fn style_type(mut self, style_type: i32) -> Self {
        self.body.style_type = Some(style_type);
        self
    }

    /// 设置语法类型：0 unknown，1 PlantUML 解析，2 Mermaid 解析。
    pub fn syntax_type(mut self, syntax_type: i32) -> Self {
        self.body.syntax_type = Some(syntax_type);
        self
    }

    /// 设置 PlantUML 语法类型，传 0 自动识别：0 unknown，1 mind map，2 sequence diagram，3 Activity diagram，4 class diagram。
    pub fn diagram_type(mut self, diagram_type: i32) -> Self {
        self.body.diagram_type = Some(diagram_type);
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<CreatePlantumlNodeResponseV1> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreatePlantumlNodeResponseV1> {
        validate_required!(self.board_id.trim(), "白板 ID 不能为空");
        validate_required!(self.body.plant_uml_code.trim(), "PlantUML 代码不能为空");

        let api_endpoint =
            crate::common::api_endpoints::BoardApiV1::WhiteboardNodeCreatePlantuml(self.board_id);
        let mut request = ApiRequest::<CreatePlantumlNodeResponseV1>::post(api_endpoint.to_url());

        let body_json = serde_json::to_value(&self.body).map_err(|e| {
            openlark_core::error::validation_error("序列化请求体失败", e.to_string().as_str())
        })?;

        request = request.body(body_json);

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "响应数据为空",
        )
        .await
    }
}

impl ApiResponseTrait for CreatePlantumlNodeResponseV1 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[test]
    fn test_whiteboard_node_create_plantuml_v1_url() {
        let endpoint = crate::common::api_endpoints::BoardApiV1::WhiteboardNodeCreatePlantuml(
            "test_board_id".to_string(),
        );
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/board/v1/whiteboards/test_board_id/nodes/plantuml"
        );
    }

    #[test]
    fn test_create_plantuml_node_body_shape() {
        let body = CreatePlantumlNodeBodyV1 {
            plant_uml_code: "@startuml\nAlice -> Bob: hi\n@enduml".to_string(),
            diagram_type: Some(0),
            ..Default::default()
        };
        let value = serde_json::to_value(&body).expect("序列化应成功");
        assert_eq!(
            value["plant_uml_code"],
            "@startuml\nAlice -> Bob: hi\n@enduml"
        );
        assert_eq!(value["diagram_type"], 0);
        assert!(
            value.get("style_type").is_none(),
            "未设置的选填字段不应序列化"
        );
        assert!(
            value.get("syntax_type").is_none(),
            "未设置的选填字段不应序列化"
        );
        assert!(value.get("title").is_none(), "官方无 title 字段");
        assert!(value.get("position").is_none(), "官方无 position 字段");
    }

    /// 端到端：POST /open-apis/board/v1/whiteboards/{whiteboard_id}/nodes/plantuml，
    /// 官方请求体为扁平 4 字段，响应 data 仅 node_id。
    #[tokio::test]
    async fn test_create_plantuml_node_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/board/v1/whiteboards/wb_123/nodes/plantuml",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "node_id": "t1:1" }
            })))
            .mount(&server)
            .await;

        let config = Arc::new(
            Config::builder()
                .app_id("ci_app_id")
                .app_secret("ci_app_secret")
                .base_url(server.uri())
                .enable_token_cache(false)
                .build(),
        );

        let resp = CreatePlantumlNodeRequestV1::new(config, "wb_123")
            .plant_uml_code("@startuml\nAlice -> Bob: hi\n@enduml")
            .diagram_type(0)
            .execute()
            .await
            .expect("创建 PlantUML 节点应成功");

        assert_eq!(resp.node_id, "t1:1");

        let received = server.received_requests().await.expect("应收到请求");
        assert_eq!(received.len(), 1);
        let body: serde_json::Value =
            serde_json::from_slice(&received[0].body).expect("请求体应为 JSON");
        assert_eq!(
            body["plant_uml_code"],
            "@startuml\nAlice -> Bob: hi\n@enduml"
        );
        assert_eq!(body["diagram_type"], 0);
        assert!(
            body.get("title").is_none()
                && body.get("parent_id").is_none()
                && body.get("position").is_none(),
            "请求体不应包含官方不存在的字段: {body}"
        );
    }
}
