//! 根据流程 ID 查询组织架构调整记录
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/approval_groups/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 根据流程 ID 查询组织架构调整记录请求
#[derive(Debug, Clone)]
pub struct GetRequest {
    /// 配置信息
    config: Config,
    process_id: Option<String>,
}

impl GetRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            process_id: None,
        }
    }

    /// 设置 `process_id`。
    pub fn process_id(mut self, process_id: String) -> Self {
        self.process_id = Some(process_id);
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

        let request = ApiRequest::<GetResponse>::get(format!(
            "/open-apis/corehr/v2/approval_groups/{process_id}"
        ));

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 根据流程 ID 查询组织架构调整记录响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetResponse {
    /// 组织架构调整流程信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_group: Option<ApprovalGroup>,
}

/// 组织架构调整流程信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApprovalGroup {
    /// 组织架构调整审批组 ID（与 process_id 一一对应）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_group_id: Option<String>,
    /// 组织架构调整对应的流程 ID（与入参 process_id 对应）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_id: Option<String>,
    /// 组织架构调整流程状态（不推荐使用，推荐使用 approval_group_status_v2）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_group_status: Option<String>,
    /// 组织架构调整流程状态（0 待发起 / 1 审批中 / 2 审批通过 / 3 已完成 / 4 已拒绝 / 5 已撤销 / 6 执行失败或等待执行）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_group_status_v2: Option<i64>,
    /// 调整主题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub topic: Option<String>,
    /// 调整原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjust_reason: Option<String>,
    /// 生效日期（YYYY-MM-DD）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,
    /// 发起人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by: Option<String>,
    /// 组织架构调整 ID（发起调整时生成的唯一 ID）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft_id: Option<String>,
    /// 流程对应组织架构调整的状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft_status: Option<String>,
    /// 关联的部门调整记录 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_changes: Option<Vec<String>>,
    /// 关联的人员调整记录 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_changes: Option<Vec<String>>,
    /// 关联的岗位调整记录 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_changes: Option<Vec<String>>,
    /// 兼容保留字段
    #[serde(flatten)]
    pub extra: Value,
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

    /// 端到端：GET /open-apis/corehr/v2/approval_groups/test001
    #[tokio::test]
    async fn test_get_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v2/approval_groups/test001"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0, "msg": "success",
                "data": {
                    "approval_group": {
                        "approval_group_id": "6991776076699549697",
                        "process_id": "6991776076699549697",
                        "approval_group_status": "1",
                        "approval_group_status_v2": 1,
                        "topic": "因组织发展需要， 变更XXX 部门组织架构",
                        "adjust_reason": "例如：因业务扩展需要， 现需增设 XXX 和 XXX 两个区域部门，便于上午拓展。",
                        "effective_date": "2022-03-01",
                        "created_by": "6974641477444060708",
                        "draft_id": "6991776076699549697",
                        "draft_status": "Edit",
                        "department_changes": ["7044427347159741231"],
                        "job_changes": ["7044427347159746085"],
                        "position_changes": ["7044427347159746085"]
                    }
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

        let group = resp.approval_group.as_ref().expect("应返回审批组信息");
        assert_eq!(
            group.approval_group_id.as_deref(),
            Some("6991776076699549697")
        );
        assert_eq!(group.approval_group_status.as_deref(), Some("1"));
        assert_eq!(group.approval_group_status_v2, Some(1));
        assert_eq!(group.draft_status.as_deref(), Some("Edit"));
        assert_eq!(
            group.department_changes,
            Some(vec!["7044427347159741231".to_string()])
        );
        assert_eq!(
            group.job_changes,
            Some(vec!["7044427347159746085".to_string()])
        );
        assert_eq!(
            group.topic.as_deref(),
            Some("因组织发展需要， 变更XXX 部门组织架构")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
