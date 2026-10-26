//! 获取文件签署节点信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/signature_node/list_by_file_id>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 获取文件签署节点信息请求
#[derive(Debug, Clone)]
pub struct ListByFileIdRequest {
    /// 配置信息
    config: Config,
    /// 电子签文件 ID（必填），可通过批量获取电子签文件接口获取
    file_id: String,
    /// 用户 ID 类型（open_id/union_id/user_id/people_corehr_id）
    user_id_type: Option<String>,
}

impl ListByFileIdRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            file_id: String::new(),
            user_id_type: None,
        }
    }

    /// 设置电子签文件 ID（必填）。
    pub fn file_id(mut self, file_id: impl Into<String>) -> Self {
        self.file_id = file_id.into();
        self
    }

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListByFileIdResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListByFileIdResponse> {
        validate_required!(self.file_id, "file_id 不能为空");

        let request = ApiRequest::<ListByFileIdResponse>::get(
            "/open-apis/corehr/v2/signature_nodes/list_by_file_id",
        )
        .query("file_id", self.file_id)
        .query_opt("user_id_type", self.user_id_type);

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 获取文件签署节点信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListByFileIdResponse {
    /// 电子签文件签署流程各个节点列表信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub signature_nodes: Option<Vec<SignatureNode>>,
}

/// 签署流程节点（官方 signature_node 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignatureNode {
    /// 节点相关的角色信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_infos: Option<Vec<SignatureUserInfo>>,
    /// 节点状态（枚举值见官方 signature_file_state 枚举定义）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// 节点完成时间
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finish_time: Option<String>,
    /// 节点最近更新时间
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_time: Option<String>,
    /// 当前节点是否为正在处理的节点
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_ongoing: Option<bool>,
    /// 当前节点操作名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_label: Option<SignatureRoleLabel>,
    /// 操作人类型（枚举值见官方 signature_node_sign_role 枚举定义）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sign_role: Option<String>,
}

/// 节点角色信息（官方 signature_user_info 结构）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignatureUserInfo {
    /// 员工雇佣 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// 节点操作名称（官方 signature_enum_info_label 结构：{zh, en}）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SignatureRoleLabel {
    /// 当前节点的操作名称中文展示，如：发起
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh: Option<String>,
    /// 当前节点的操作名称英文展示，如：Initiate
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en: Option<String>,
}

impl ApiResponseTrait for ListByFileIdResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path, query_param};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/corehr/v2/signature_nodes/list_by_file_id
    /// （官方查询参数 file_id 必填；data 内层键为 signature_nodes）
    #[tokio::test]
    async fn test_list_by_file_id_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v2/signature_nodes/list_by_file_id"))
            .and(query_param("file_id", "7149088311624091180"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "signature_nodes": [
                        {
                            "user_infos": [
                                { "id": "7300476612163913260" }
                            ],
                            "state": "Initiated",
                            "finish_time": "2021-12-31 12:21:10",
                            "updated_time": "2021-12-31 12:21:10",
                            "is_ongoing": false,
                            "role_label": {
                                "zh": "发起",
                                "en": "Initiate"
                            },
                            "sign_role": "Initiator"
                        }
                    ]
                }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let resp = ListByFileIdRequest::new(config)
            .file_id("7149088311624091180")
            .execute()
            .await
            .expect("请求应成功");

        let nodes = resp.signature_nodes.expect("signature_nodes 应解码");
        assert_eq!(nodes.len(), 1);
        assert_eq!(
            nodes[0]
                .user_infos
                .as_ref()
                .and_then(|u| u.first())
                .and_then(|u| u.id.clone()),
            Some("7300476612163913260".to_string())
        );
        assert_eq!(nodes[0].state.as_deref(), Some("Initiated"));
        assert_eq!(nodes[0].finish_time.as_deref(), Some("2021-12-31 12:21:10"));
        assert_eq!(nodes[0].is_ongoing, Some(false));
        assert_eq!(
            nodes[0].role_label.as_ref().and_then(|r| r.zh.clone()),
            Some("发起".to_string())
        );
        assert_eq!(nodes[0].sign_role.as_deref(), Some("Initiator"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
