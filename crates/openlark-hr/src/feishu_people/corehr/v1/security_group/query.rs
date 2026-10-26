//! 查询部门 / 地点的 HRBP / 属地 BP
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/security_group/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, validate_required_list,
};
use serde::{Deserialize, Serialize};

/// 查询部门 / 地点的 HRBP / 属地 BP请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 配置信息
    config: Config,
    /// 部门 ID 类型（查询参数，可选）
    department_id_type: Option<String>,
    /// 请求体
    body: QueryBody,
}

impl QueryRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            department_id_type: None,
            body: QueryBody::default(),
        }
    }

    /// 设置部门 ID 类型（可选：open_department_id / department_id / people_corehr_department_id）。
    pub fn department_id_type(mut self, department_id_type: String) -> Self {
        self.department_id_type = Some(department_id_type);
        self
    }

    /// 设置角色列表 `item_list`（必填，一次最多 50 个）。
    pub fn item_list(mut self, item_list: Vec<BpRoleOrganization>) -> Self {
        self.body.item_list = item_list;
        self
    }

    /// 追加单个角色项到 `item_list`。
    pub fn item(mut self, item: BpRoleOrganization) -> Self {
        self.body.item_list.push(item);
        self
    }

    /// 设置授权时间下界 `updated_at_gte`（可选，秒级时间戳字符串）。
    pub fn updated_at_gte(mut self, updated_at_gte: String) -> Self {
        self.body.updated_at_gte = Some(updated_at_gte);
        self
    }

    /// 设置授权时间上界 `updated_at_lte`（可选，秒级时间戳字符串）。
    pub fn updated_at_lte(mut self, updated_at_lte: String) -> Self {
        self.body.updated_at_lte = Some(updated_at_lte);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<QueryResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        validate_required_list!(
            self.body.item_list,
            50,
            "item_list 不能为空且一次最多查询 50 个"
        );
        for item in &self.body.item_list {
            validate_required!(item.role_key, "item_list.role_key 不能为空");
            validate_required!(item.department_id, "item_list.department_id 不能为空");
        }

        let api_endpoint = FeishuPeopleApiV1::SecurityGroupQuery;
        let mut request = ApiRequest::<QueryResponse>::post(api_endpoint.to_url())
            .body(serde_json::to_value(&self.body)?);
        if let Some(department_id_type) = self.department_id_type {
            request = request.query("department_id_type", department_id_type);
        }
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "查询部门或地点 HRBP/属地 BP 响应数据为空",
        )
        .await
    }
}

/// 查询部门 / 地点 HRBP 请求体（官方字段：`item_list` 必填）
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct QueryBody {
    /// 角色列表，一次最多支持查询 50 个
    pub item_list: Vec<BpRoleOrganization>,
    /// 授权时间大于（秒级时间戳字符串）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_gte: Option<String>,
    /// 授权时间小于（秒级时间戳字符串）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at_lte: Option<String>,
}

/// 角色条目（bp_role_organization：`role_key`/`department_id` 必填）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BpRoleOrganization {
    /// 角色类型的唯一标识：HRBP 固定 `hrbp`；属地 BP 固定 `location_bp`
    pub role_key: String,
    /// 部门 ID（查询 HRBP 必填，ID 类型与 department_id_type 取值一致）
    pub department_id: String,
    /// 工作地点 ID（查询属地 BP 需输入部门 ID 与工作地点 ID）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub work_location_id: Option<String>,
}

/// 查询部门 / 地点的 HRBP / 属地 BP响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponse {
    /// HRBP/属地 BP 信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hrbp_list: Option<Vec<Hrbp>>,
}

/// HRBP/属地 BP 信息
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct Hrbp {
    /// HRBP/属地 BP 的雇员 ID 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub employment_id_list: Option<Vec<String>>,
    /// 部门 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 工作地点 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub work_location_id: Option<String>,
}

impl ApiResponseTrait for QueryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value, json};
    use wiremock::MockServer;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端：POST /open-apis/corehr/v1/security_groups/query
    #[tokio::test]
    async fn test_query_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/corehr/v1/security_groups/query"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "hrbp_list": [
                        {
                            "employment_id_list": ["7063072995761456670"],
                            "department_id": "7063072995761456670",
                            "work_location_id": "6892687221355185677"
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

        let resp = QueryRequest::new(config)
            .item(BpRoleOrganization {
                role_key: "location_bp".to_string(),
                department_id: "7063072995761456670".to_string(),
                work_location_id: Some("6892687221355185677".to_string()),
            })
            .updated_at_gte("1729773628".to_string())
            .updated_at_lte("1729773629".to_string())
            .execute()
            .await
            .expect("请求应成功");

        let hrbp_list = resp.hrbp_list.expect("hrbp_list 不能为空");
        assert_eq!(hrbp_list.len(), 1);
        assert_eq!(
            hrbp_list[0].employment_id_list,
            Some(vec!["7063072995761456670".to_string()])
        );
        assert_eq!(
            hrbp_list[0].department_id,
            Some("7063072995761456670".to_string())
        );
        assert_eq!(
            hrbp_list[0].work_location_id,
            Some("6892687221355185677".to_string())
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        // 请求体须与官方文档一致：item_list 数组 + updated_at_gte/lte
        let sent: Value = received[0].body_json().expect("请求体应为 JSON");
        assert_eq!(
            sent,
            json!({
                "item_list": [
                    {
                        "role_key": "location_bp",
                        "department_id": "7063072995761456670",
                        "work_location_id": "6892687221355185677"
                    }
                ],
                "updated_at_gte": "1729773628",
                "updated_at_lte": "1729773629"
            })
        );
    }

    /// 必填校验：空 item_list 快速失败
    #[tokio::test]
    async fn test_query_missing_item_list_rejected() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .build();

        let err = QueryRequest::new(config)
            .execute()
            .await
            .expect_err("空 item_list 应被拒绝");
        assert!(err.to_string().contains("item_list"), "实际: {err}");
    }
}
