//! 获取单个流程详情
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/process/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::common::shared_models::I18nText;

/// 获取单个流程详情请求
#[derive(Debug, Clone)]
pub struct GetRequest {
    /// 配置信息
    config: Config,
    /// 流程 ID
    process_id: Option<String>,
    body: Option<Value>,
}

impl GetRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            process_id: None,
            body: None,
        }
    }

    /// 设置 `process_id`。
    pub fn process_id(mut self, process_id: impl Into<String>) -> Self {
        self.process_id = Some(process_id.into());
        self
    }

    /// 设置请求体。
    pub fn body(mut self, body: Value) -> Self {
        self.body = Some(body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetResponse> {
        let process_id = self.process_id.unwrap_or_default();
        validate_required!(process_id.trim(), "process_id 不能为空");

        let mut request =
            ApiRequest::<GetResponse>::get(format!("/open-apis/corehr/v2/processes/{process_id}"));

        if let Some(body) = self.body {
            request = request.body(body);
        }

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 获取单个流程详情响应
///
/// 官方响应字段直接平铺在 `data` 顶层（无 `process` 包装键）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetResponse {
    /// 响应数据
    /// 流程实例ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
    /// 流程状态（1 进行中 / 2 拒绝 / 4 撤回 / 8 撤销 / 9 已完成 / 15 撤销中）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 业务类型ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_template_id: Option<String>,
    /// 业务类型名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_template_name: Option<I18nText>,
    /// 流程定义ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_definition_id: Option<String>,
    /// 流程定义名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_definition_name: Option<I18nText>,
    /// 流程发起人ID（发起人为系统时为空）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator_id: Option<String>,
    /// 流程发起人姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator_name: Option<I18nText>,
    /// 流程发起时间（Unix 毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 流程结束时间（Unix 毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete_time: Option<String>,
    /// 发起单据地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_links: Option<ProcessLink>,
    /// 流程摘要（会随流程流转变化）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub abstracts: Option<Vec<ProcessAbstractItem>>,
    /// 待办列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub todos: Option<Vec<ProcessTodoItem>>,
    /// 抄送列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_list: Option<Vec<ProcessCcItem>>,
    /// 已办列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub done_list: Option<Vec<ProcessDoneItem>>,
    /// 普通流程或撤销流程等（1 普通 / 2 撤销 / 3 更正）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<i32>,
    /// 系统待办列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_todos: Option<Vec<ProcessSystemTodoItem>>,
    /// 系统已办列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_done_list: Option<Vec<ProcessSystemDoneItem>>,
    /// 评论列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_infos: Option<Vec<ProcessCommentInfo>>,
    /// 更正或撤销流程的原流程ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_process_id: Option<String>,
    /// 是否最新的「已完成」的更正流程
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_last_completed_correct_process: Option<bool>,
    /// 流程实例名称（取流程发起时的流程定义名称）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_name: Option<I18nText>,
    /// 兼容保留字段
    #[serde(flatten)]
    pub extra: Value,
}

/// 单据地址
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProcessLink {
    /// web 端单据详情页地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_link: Option<String>,
    /// 飞书 pc 端单据详情页地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pc_link: Option<String>,
    /// 飞书移动端单据详情页地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_link: Option<String>,
}

/// 流程摘要条目
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProcessAbstractItem {
    /// 摘要标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<I18nText>,
    /// 摘要值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<I18nText>,
}

/// 待办条目
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProcessTodoItem {
    /// 单据ID（注意单据 id 和 operator_id 并不是一对一的）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approver_id: Option<String>,
    /// 单据类型（1 审批单 / 5 表单）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    /// 单据地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<ProcessLink>,
    /// 待办人ID（每个节点下待办人标识唯一）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<String>,
    /// 操作人姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_name: Option<I18nText>,
    /// 节点名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: Option<I18nText>,
    /// 创建时间（Unix 毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 节点定义ID（回退场景下同一节点会对应多个节点实例）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_definition_id: Option<String>,
}

/// 抄送条目
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProcessCcItem {
    /// 单据ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approver_id: Option<String>,
    /// 单据地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<ProcessLink>,
    /// 抄送人ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<String>,
    /// 抄送人姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_name: Option<I18nText>,
    /// 节点名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: Option<I18nText>,
    /// 抄送时间（Unix 毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 节点定义ID（回退场景下同一节点会对应多个节点实例）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_definition_id: Option<String>,
}

/// 已办条目
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProcessDoneItem {
    /// 单据ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approver_id: Option<String>,
    /// 单据类型（1 审批单 / 5 表单）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    /// 单据状态（3 已完成 / 2 拒绝 / 4 取消）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 单据地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<ProcessLink>,
    /// 操作人ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<String>,
    /// 操作人姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_name: Option<I18nText>,
    /// 节点名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: Option<I18nText>,
    /// 创建时间（Unix 毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 完成时间（Unix 毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete_time: Option<String>,
    /// 节点定义ID（回退场景下同一节点会对应多个节点实例）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_definition_id: Option<String>,
    /// 审批意见
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_opinion: Option<String>,
}

/// 系统待办条目
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProcessSystemTodoItem {
    /// 单据ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approver_id: Option<String>,
    /// 单据类型（1 审批单 / 5 表单）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    /// 单据地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<ProcessLink>,
    /// 操作人姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_name: Option<I18nText>,
    /// 节点名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: Option<I18nText>,
    /// 创建时间（Unix 毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 节点定义ID（回退场景下同一节点会对应多个节点实例）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_definition_id: Option<String>,
}

/// 系统已办条目
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProcessSystemDoneItem {
    /// 单据ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approver_id: Option<String>,
    /// 单据类型（1 审批单 / 5 表单）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i32>,
    /// 单据状态（3 已完成 / 2 拒绝 / 4 取消）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i32>,
    /// 单据地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<ProcessLink>,
    /// 操作人姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_name: Option<I18nText>,
    /// 节点名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: Option<I18nText>,
    /// 创建时间（Unix 毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 完成时间（Unix 毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete_time: Option<String>,
    /// 节点定义ID（回退场景下同一节点会对应多个节点实例）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_definition_id: Option<String>,
    /// 审批意见
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_opinion: Option<String>,
}

/// 评论条目
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProcessCommentInfo {
    /// 评论人ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commentor_id: Option<String>,
    /// 评论人姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub commentor_name: Option<I18nText>,
    /// 评论时间（Unix 毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_time: Option<String>,
    /// 评论内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_msg: Option<String>,
    /// 在评论中被提及（@）到的人的 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at_user_ids: Option<Vec<String>>,
    /// 评论中附件数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_count: Option<i32>,
    /// 评论中图片数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_count: Option<i32>,
    /// 一条评论的唯一 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_id: Option<String>,
}

impl ApiResponseTrait for GetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：GET /open-apis/corehr/v2/processes/test001
    #[tokio::test]
    async fn test_get_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v2/processes/test001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": {
                    "process_id": "7278949005675988535",
                    "status": 9,
                    "flow_template_id": "leave",
                    "flow_template_name": { "zh_cn": "中文", "en_us": "English" },
                    "flow_definition_id": "people_6961286846093788680_7081951411982077732",
                    "flow_definition_name": { "zh_cn": "中文", "en_us": "English" },
                    "initiator_id": "7124991993901827628",
                    "initiator_name": { "zh_cn": "张三", "en_us": "Zhang San" },
                    "create_time": "1694769814036",
                    "complete_time": "1694769814036",
                    "start_links": {
                        "web_link": "http://xxxx.com/xxx/xxx?a=b",
                        "pc_link": "https://applink.feishu.cn/client/mini_program/open?appId=xxx",
                        "mobile_link": "https://applink.feishu.cn/client/mini_program/open?appId=xxx"
                    },
                    "abstracts": [
                        {
                            "name": { "zh_cn": "中文", "en_us": "English" },
                            "value": { "zh_cn": "中文", "en_us": "English" }
                        }
                    ],
                    "todos": [
                        {
                            "approver_id": "7278949005675988535",
                            "type": 1,
                            "links": {
                                "web_link": "http://xxxx.com/xxx/xxx?a=b",
                                "pc_link": "https://applink.feishu.cn/client/mini_program/open?appId=xxx",
                                "mobile_link": "https://applink.feishu.cn/client/mini_program/open?appId=xxx"
                            },
                            "operator_id": "7124991993901827628",
                            "operator_name": { "zh_cn": "中文", "en_us": "English" },
                            "node_name": { "zh_cn": "中文", "en_us": "English" },
                            "create_time": "1694769814036",
                            "node_definition_id": "approval_d25b5eddfef"
                        }
                    ],
                    "comment_infos": [
                        {
                            "commentor_id": "7355397217231831060",
                            "commentor_name": { "zh_cn": "中文", "en_us": "English" },
                            "comment_time": "1694769814036",
                            "comment_msg": "评论内容",
                            "at_user_ids": ["7355397217231831060"],
                            "file_count": 10,
                            "image_count": 10,
                            "comment_id": "7355397217231831060"
                        }
                    ],
                    "process_name": { "zh_cn": "中文", "en_us": "English" }
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

        let resp = GetRequest::new(config)
            .process_id("test001".to_string())
            .execute()
            .await
            .expect("请求应成功");

        // 官方响应字段平铺在 data 顶层，直接从 GetResponse 解码
        assert_eq!(resp.process_id.as_deref(), Some("7278949005675988535"));
        assert_eq!(resp.status, Some(9));
        assert_eq!(resp.flow_template_id.as_deref(), Some("leave"));
        assert_eq!(
            resp.flow_template_name
                .as_ref()
                .and_then(|n| n.zh_cn.as_deref()),
            Some("中文")
        );
        assert_eq!(
            resp.initiator_name
                .as_ref()
                .and_then(|n| n.en_us.as_deref()),
            Some("Zhang San")
        );
        assert_eq!(
            resp.start_links
                .as_ref()
                .and_then(|l| l.web_link.as_deref()),
            Some("http://xxxx.com/xxx/xxx?a=b")
        );
        let todo = resp
            .todos
            .as_ref()
            .and_then(|todos| todos.first())
            .expect("应返回待办条目");
        assert_eq!(todo.r#type, Some(1));
        assert_eq!(
            todo.links.as_ref().and_then(|l| l.pc_link.as_deref()),
            Some("https://applink.feishu.cn/client/mini_program/open?appId=xxx")
        );
        let comment = resp
            .comment_infos
            .as_ref()
            .and_then(|infos| infos.first())
            .expect("应返回评论条目");
        assert_eq!(comment.file_count, Some(10));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
