//! 创建白板节点（v1）
//! docPath: <https://open.feishu.cn/document/ukTMukTMukTM/uUDN04SN0QjL1QDN/board-v1/whiteboard-node/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, validate_required_list,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// 创建白板节点请求体。
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateWhiteboardNodeBodyV1 {
    /// 子节点数据，不允许传入空数组，长度范围 1 ～ 3000。
    pub nodes: Vec<WhiteboardNodeV1>,
}

/// 白板节点数据（whiteboard.node）。
///
/// 除 `node_type` 外均为可选字段；各图形属性字段（`text`/`style`/`image` 等）
/// 的子结构见官方文档对应 object 定义（whiteboard.node.text 等）。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WhiteboardNodeV1 {
    /// 节点 id，用于唯一标识此节点，不能重复，在创建父子节点、关联其它节点做标记使用。长度范围 0 ～ 100 字符。
    #[serde(skip_serializing_if = "String::is_empty")]
    pub id: String,
    /// 节点图形类型，可选值：image、text_shape、group、composite_shape、svg、connector、table、life_line、activation、section、table_uml、table_er、sticky_note、mind_map、paint、combined_fragment。
    #[serde(rename = "type")]
    pub node_type: String,
    /// 父节点 id，必须是已存在的节点或者在本次创建的节点列表内。长度范围 0 ～ 100 字符。
    #[serde(skip_serializing_if = "String::is_empty")]
    pub parent_id: String,
    /// 图形相对画布的 x 轴位置信息（存在父容器时为相对父容器的坐标，父容器为组合图形 group 时，坐标是穿透的），单位为 px。取值范围 -132070 ～ 132070，默认 0。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    /// 图形相对画布的 y 轴位置信息（存在父容器时为相对父容器的坐标，父容器为组合图形 group 时，坐标是穿透的），单位为 px。取值范围 -132070 ～ 132070，默认 0。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    /// 图形旋转角度，单位度。取值范围 -180 ～ 180，默认 0。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub angle: Option<f64>,
    /// 图形高度，单位为 px。取值范围 0 ～ 132070。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    /// 图形宽度，单位为 px。取值范围 0 ～ 132070。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    /// 图形内文字（whiteboard.node.text）。节点类型为 text_shape 时该字段不能为空；节点类型为 group、section、connector、paint、svg、activation 时该字段必须为空。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<serde_json::Value>,
    /// 图形样式（whiteboard.node.style）。节点类型为 group、paint 时该字段必须为空。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<serde_json::Value>,
    /// 图片属性（whiteboard.node.image）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image: Option<serde_json::Value>,
    /// 基础图形属性（whiteboard.node.composite_shape）。节点的 type 为 composite_shape 时不可为空。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composite_shape: Option<serde_json::Value>,
    /// 连线属性（whiteboard.node.connector）。节点类型为 connector 时该字段不可为空。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector: Option<serde_json::Value>,
    /// 分区属性（whiteboard.node.section）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub section: Option<serde_json::Value>,
    /// 表格属性（whiteboard.node.table）。节点类型为 table、table_er、table_uml、combined_fragment 时该字段不可为空。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table: Option<serde_json::Value>,
    /// 图形是否锁定，锁定后无法编辑图形，默认 false。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    /// 图形在兄弟节点中的层级，层级大的会覆盖层级小的。取值范围 0 ～ 10000。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub z_index: Option<i64>,
    /// 生命对象属性（whiteboard.node.lifeline）。节点类型为 life_line 时该字段不可为空。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lifeline: Option<serde_json::Value>,
    /// 画笔属性（whiteboard.node.paint）。节点类型为 paint 时该字段不可为空。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paint: Option<serde_json::Value>,
    /// svg 图形属性（whiteboard.node.svg）。节点类型为 svg 时该字段不可为空。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub svg: Option<serde_json::Value>,
    /// 便签图形属性（whiteboard.node.sticky_note）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sticky_note: Option<serde_json::Value>,
    /// 思维导图节点属性（whiteboard.node.mind_map_node）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mind_map_node: Option<serde_json::Value>,
    /// 思维导图根节点属性（whiteboard.node.mind_map_root）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mind_map_root: Option<serde_json::Value>,
}

/// 创建白板节点响应。
#[derive(Debug, Clone, Deserialize)]
pub struct CreateWhiteboardNodeResponseV1 {
    /// 所创建的节点 id 列表。
    pub ids: Vec<String>,
    /// 操作的唯一标识，更新请求中使用此值表示幂等地进行此次更新。
    #[serde(default)]
    pub client_token: Option<String>,
}

/// 创建白板节点请求构建器。
#[derive(Debug, Clone)]
pub struct CreateWhiteboardNodeRequestV1 {
    config: Arc<Config>,
    board_id: String,
    client_token: Option<String>,
    user_id_type: Option<String>,
    body: CreateWhiteboardNodeBodyV1,
}

impl CreateWhiteboardNodeRequestV1 {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>, board_id: impl Into<String>) -> Self {
        Self {
            config,
            board_id: board_id.into(),
            client_token: None,
            user_id_type: None,
            body: CreateWhiteboardNodeBodyV1::default(),
        }
    }

    /// 设置幂等操作标识（query 参数，长度范围 10 ～ 64 字符；与接口返回值的 client_token 相对应）。
    pub fn client_token(mut self, client_token: impl Into<String>) -> Self {
        self.client_token = Some(client_token.into());
        self
    }

    /// 设置用户 ID 类型（query 参数），可选值：open_id、union_id、user_id，默认 open_id。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置节点列表（整体替换）。
    pub fn nodes(mut self, nodes: Vec<WhiteboardNodeV1>) -> Self {
        self.body.nodes = nodes;
        self
    }

    /// 追加单个节点。
    pub fn add_node(mut self, node: WhiteboardNodeV1) -> Self {
        self.body.nodes.push(node);
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<CreateWhiteboardNodeResponseV1> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateWhiteboardNodeResponseV1> {
        validate_required!(self.board_id.trim(), "白板 ID 不能为空");
        validate_required_list!(self.body.nodes, 3000, "nodes 不能为空且不能超过 3000 个");

        let api_endpoint =
            crate::common::api_endpoints::BoardApiV1::WhiteboardNodeCreate(self.board_id);
        let mut request = ApiRequest::<CreateWhiteboardNodeResponseV1>::post(api_endpoint.to_url());

        if let Some(client_token) = self.client_token {
            request = request.query_param("client_token", client_token);
        }
        if let Some(user_id_type) = self.user_id_type {
            request = request.query_param("user_id_type", user_id_type);
        }

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

impl ApiResponseTrait for CreateWhiteboardNodeResponseV1 {
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
    fn test_whiteboard_node_create_v1_url() {
        let endpoint = crate::common::api_endpoints::BoardApiV1::WhiteboardNodeCreate(
            "test_board_id".to_string(),
        );
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/board/v1/whiteboards/test_board_id/nodes"
        );
    }

    #[test]
    fn test_create_whiteboard_node_builder() {
        let request = CreateWhiteboardNodeRequestV1::new(Arc::new(Config::default()), "wb_123")
            .client_token("fe599b60-450f")
            .nodes(vec![WhiteboardNodeV1 {
                node_type: "text_shape".to_string(),
                ..Default::default()
            }])
            .add_node(WhiteboardNodeV1 {
                id: "o1:3".to_string(),
                node_type: "composite_shape".to_string(),
                parent_id: "n1:1".to_string(),
                x: Some(347.1015625),
                y: Some(200.0),
                ..Default::default()
            });

        assert_eq!(request.board_id, "wb_123");
        assert_eq!(request.body.nodes.len(), 2);
        assert_eq!(request.body.nodes[1].node_type, "composite_shape");
        assert_eq!(request.body.nodes[1].x, Some(347.1015625));
    }

    /// 端到端：POST /open-apis/board/v1/whiteboards/{whiteboard_id}/nodes，
    /// 请求体为官方 { "nodes": [ ... ] } 信封，响应 data 为 { ids, client_token }。
    #[tokio::test]
    async fn test_create_whiteboard_node_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/board/v1/whiteboards/wb_123/nodes"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "ids": ["o1:3", "o2:27"],
                    "client_token": "fe599b60-450f-46ff-b2ef-9f6675625b97"
                }
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

        let resp = CreateWhiteboardNodeRequestV1::new(config, "wb_123")
            .add_node(WhiteboardNodeV1 {
                id: "o1:3".to_string(),
                node_type: "composite_shape".to_string(),
                parent_id: "n1:1".to_string(),
                x: Some(347.1015625),
                y: Some(200.0),
                ..Default::default()
            })
            .execute()
            .await
            .expect("创建节点应成功");

        assert_eq!(resp.ids, vec!["o1:3".to_string(), "o2:27".to_string()]);
        assert_eq!(
            resp.client_token.as_deref(),
            Some("fe599b60-450f-46ff-b2ef-9f6675625b97")
        );

        let received = server.received_requests().await.expect("应收到请求");
        assert_eq!(received.len(), 1);
        let body: serde_json::Value =
            serde_json::from_slice(&received[0].body).expect("请求体应为 JSON");
        assert!(
            body.as_object()
                .expect("请求体为对象")
                .contains_key("nodes"),
            "请求体顶层应仅含 nodes 信封: {body}"
        );
        assert_eq!(body["nodes"][0]["id"], "o1:3");
        assert_eq!(body["nodes"][0]["type"], "composite_shape");
        assert_eq!(body["nodes"][0]["parent_id"], "n1:1");
        assert_eq!(body["nodes"][0]["x"], 347.1015625);
        assert_eq!(body["nodes"][0]["y"], 200.0);
        assert!(
            body["nodes"][0].get("title").is_none(),
            "官方节点无 title 字段: {body}"
        );
        assert!(
            body["nodes"][0].get("position").is_none(),
            "官方节点无 position 嵌套: {body}"
        );
    }
}
