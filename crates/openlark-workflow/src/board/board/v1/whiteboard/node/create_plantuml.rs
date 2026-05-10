//! 创建 PlantUML 节点（v1）
//! docPath:

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Default)]
/// 创建 PlantUML 节点请求体。
pub struct CreatePlantumlNodeBodyV1 {
    /// 节点标题。
    pub title: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    /// 父节点 ID。
    pub parent_id: String,
    /// PlantUML 源码。
    pub plantuml_code: String,
    #[serde(default)]
    /// 节点位置。
    pub position: NodePosition,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// 节点位置信息。
pub struct NodePosition {
    #[serde(default)]
    /// 横坐标。
    pub x: f64,
    #[serde(default)]
    /// 纵坐标。
    pub y: f64,
    #[serde(default)]
    /// 宽度。
    pub width: f64,
    #[serde(default)]
    /// 高度。
    pub height: f64,
}

#[derive(Debug, Clone, Deserialize)]
/// 创建 PlantUML 节点响应。
pub struct CreatePlantumlNodeResponseV1 {
    /// 节点 ID。
    pub node_id: String,
    /// 节点标题。
    pub title: String,
    /// 生成图片地址。
    pub image_url: String,
    /// 创建时间。
    pub create_time: i64,
}

#[derive(Debug, Clone)]
/// 创建 PlantUML 节点请求构建器。
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

    /// 设置节点标题。
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.body.title = title.into();
        self
    }

    /// 设置父节点 ID。
    pub fn parent_id(mut self, parent_id: impl Into<String>) -> Self {
        self.body.parent_id = parent_id.into();
        self
    }

    /// 设置 PlantUML 源码。
    pub fn plantuml_code(mut self, plantuml_code: impl Into<String>) -> Self {
        self.body.plantuml_code = plantuml_code.into();
        self
    }

    /// 设置节点位置和尺寸。
    pub fn position(mut self, x: f64, y: f64, width: f64, height: f64) -> Self {
        self.body.position = NodePosition {
            x,
            y,
            width,
            height,
        };
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
        validate_required!(self.body.title.trim(), "节点标题不能为空");
        validate_required!(self.body.plantuml_code.trim(), "PlantUML 代码不能为空");

        let api_endpoint =
            crate::common::api_endpoints::BoardApiV1::WhiteboardNodeCreatePlantuml(self.board_id);
        let mut request = ApiRequest::<CreatePlantumlNodeResponseV1>::post(api_endpoint.to_url());

        let body_json = serde_json::to_value(&self.body).map_err(|e| {
            openlark_core::error::validation_error("序列化请求体失败", e.to_string().as_str())
        })?;

        request = request.body(body_json);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

impl ApiResponseTrait for CreatePlantumlNodeResponseV1 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

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
}
