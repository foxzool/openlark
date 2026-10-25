//! 批量查询人员调整内容
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/approval_groups/open_query_job_change_list_by_ids>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 批量查询人员调整内容请求
#[derive(Debug, Clone)]
pub struct OpenQueryJobChangeListByIdsRequest {
    /// 配置信息
    config: Config,
    body: Option<Value>,
}

impl OpenQueryJobChangeListByIdsRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self { config, body: None }
    }

    /// 设置请求体。
    pub fn body(mut self, body: Value) -> Self {
        self.body = Some(body);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<OpenQueryJobChangeListByIdsResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<OpenQueryJobChangeListByIdsResponse> {
        let mut request = ApiRequest::<OpenQueryJobChangeListByIdsResponse>::post(
            "/open-apis/corehr/v2/approval_groups/open_query_job_change_list_by_ids",
        );

        if let Some(body) = self.body {
            request = request.body(body);
        }

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 批量查询人员调整内容响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OpenQueryJobChangeListByIdsResponse {
    /// 人员异动记录信息列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_changes: Option<Vec<JobChange>>,
}

/// 人员异动记录
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JobChange {
    /// 异动记录 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub job_change_id: Option<String>,
    /// 雇员 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id: Option<String>,
    /// 异动详细信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub transfer_info: Option<TransferInfo>,
}

/// 人员异动详细信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransferInfo {
    /// 原部门（people_corehr_department_id 类型）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_department: Option<String>,
    /// 新部门（people_corehr_id 类型）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_department: Option<String>,
    /// 新部门过程 ID（组织架构调整中新建部门的临时 ID，格式 td_xxx）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_draft_department: Option<String>,
    /// 原部门全路径，从根部门开始自上而下返回部门 ID 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_department_id_path: Option<Vec<OrgDraftDepartmentId>>,
    /// 新部门全路径，从根部门开始自上而下返回部门 ID 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_department_id_path: Option<Vec<OrgDraftDepartmentId>>,
    /// 原直属上级
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_direct_manager: Option<String>,
    /// 新直属上级
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_direct_manager: Option<String>,
    /// 原职务
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_job: Option<String>,
    /// 新职务
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_job: Option<String>,
    /// 原序列
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_job_family: Option<String>,
    /// 新序列
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_job_family: Option<String>,
    /// 原职级
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_job_level: Option<String>,
    /// 新职级
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_job_level: Option<String>,
    /// 原成本中心分摊方式
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_cost_center_rate: Option<Vec<JobDataCostCenter>>,
    /// 新成本中心分摊方式
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_cost_center_rate: Option<Vec<JobDataCostCenter>>,
    /// 新分摊失效时间（YYYY-MM-DD HH:MM:SS）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_allocation_expiration_time: Option<String>,
    /// 原分摊失效时间（YYYY-MM-DD HH:MM:SS）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_allocation_expiration_time: Option<String>,
    /// 新分摊生效时间（YYYY-MM-DD HH:MM:SS）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_allocation_effective_time: Option<String>,
    /// 原分摊生效时间（YYYY-MM-DD HH:MM:SS）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_allocation_effective_time: Option<String>,
    /// 原默认成本中心 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_default_cost_center: Option<String>,
    /// 新默认成本中心 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_default_cost_center: Option<String>,
    /// 原默认成本中心是否继承
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_is_default_cost_center_inherited: Option<bool>,
    /// 新默认成本中心是否继承
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_is_default_cost_center_inherited: Option<bool>,
    /// 原职等
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_job_grade: Option<String>,
    /// 新职等
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_job_grade: Option<String>,
    /// 原岗位
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_position: Option<String>,
    /// 新岗位
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_position: Option<String>,
    /// 新岗位过程 ID（新建岗位未生效前为 td_xxx 临时 ID）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_draft_position: Option<String>,
}

/// 部门全路径元素：{department_id, draft_department_id}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrgDraftDepartmentId {
    /// 部门 ID（新建部门未生效时为空）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 调整过程部门 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft_department_id: Option<String>,
}

/// 成本中心分摊方式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JobDataCostCenter {
    /// 成本中心 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
    /// 分摊比例（整数部分，实际值为小数时向下取整）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rate: Option<i64>,
    /// 分摊比例（可能包含小数位）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_rate: Option<f64>,
}

impl ApiResponseTrait for OpenQueryJobChangeListByIdsResponse {
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

    /// 端到端：POST /open-apis/corehr/v2/approval_groups/open_query_job_change_list_by_ids
    #[tokio::test]
    async fn test_open_query_job_change_list_by_ids_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/corehr/v2/approval_groups/open_query_job_change_list_by_ids",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "job_changes": [{
                        "job_change_id": "6991776076699549697",
                        "employment_id": "ou_a294793e8fa21529f2a60e3e9de45520",
                        "transfer_info": {
                            "original_department": "6966236933198579208",
                            "target_department": "6966236933198579208",
                            "target_draft_department": "td_704442734715974602312",
                            "original_department_id_path": [{
                                "department_id": "6974659700705068581",
                                "draft_department_id": "6974659700705068581"
                            }],
                            "target_department_id_path": [{
                                "department_id": "6974659700705068581",
                                "draft_department_id": "6974659700705068581"
                            }],
                            "original_direct_manager": "6974641477444060708",
                            "target_direct_manager": "7013619729281713671",
                            "original_job": "6969469398088287751",
                            "target_job": "6969469557836760606",
                            "original_cost_center_rate": [{
                                "cost_center_id": "6950635856373745165",
                                "rate": 100,
                                "new_rate": 50.2
                            }],
                            "target_allocation_effective_time": "2022-03-01 00:00:00",
                            "original_default_cost_center": "7380264299728602661",
                            "original_is_default_cost_center_inherited": true,
                            "target_job_grade": "7289005963599693366",
                            "target_draft_position": "td_7289005963599693367"
                        }
                    }]
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

        let resp = OpenQueryJobChangeListByIdsRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let changes = resp.job_changes.as_ref().expect("job_changes 应被解码");
        assert_eq!(changes.len(), 1);
        assert_eq!(
            changes[0].job_change_id.as_deref(),
            Some("6991776076699549697")
        );
        assert_eq!(
            changes[0].employment_id.as_deref(),
            Some("ou_a294793e8fa21529f2a60e3e9de45520")
        );
        let info = changes[0]
            .transfer_info
            .as_ref()
            .expect("transfer_info 应被解码");
        assert_eq!(
            info.original_department.as_deref(),
            Some("6966236933198579208")
        );
        assert_eq!(
            info.target_draft_department.as_deref(),
            Some("td_704442734715974602312")
        );
        assert_eq!(
            info.original_department_id_path
                .as_ref()
                .and_then(|p| p.first())
                .and_then(|p| p.department_id.as_deref()),
            Some("6974659700705068581")
        );
        assert_eq!(
            info.original_cost_center_rate
                .as_ref()
                .and_then(|r| r.first())
                .and_then(|r| r.new_rate),
            Some(50.2)
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
