//! 批量查询岗位调整内容
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/approval_groups/open_query_position_change_list_by_ids>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 批量查询岗位调整内容请求
#[derive(Debug, Clone)]
pub struct OpenQueryPositionChangeListByIdsRequest {
    /// 配置信息
    config: Config,
    body: Option<Value>,
}

impl OpenQueryPositionChangeListByIdsRequest {
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
    pub async fn execute(self) -> SDKResult<OpenQueryPositionChangeListByIdsResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<OpenQueryPositionChangeListByIdsResponse> {
        let mut request = ApiRequest::<OpenQueryPositionChangeListByIdsResponse>::post(
            "/open-apis/corehr/v2/approval_groups/open_query_position_change_list_by_ids",
        );

        if let Some(body) = self.body {
            request = request.body(body);
        }

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 批量查询岗位调整内容响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OpenQueryPositionChangeListByIdsResponse {
    /// 岗位调整记录信息列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position_changes: Option<Vec<PositionChangeItem>>,
}

/// 岗位调整记录
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PositionChangeItem {
    /// 关联的岗位调整记录 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position_change_id: Option<String>,
    /// 岗位 ID（本次调整中新建的岗位在调整未生效时为空）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position_id: Option<String>,
    /// 调整过程岗位 ID（新建岗位未生效前为 td_xxx 格式）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft_position_id: Option<String>,
    /// 调整类型：Unknown 未知 / Create 新建 / Modify 编辑 / Inactive 停用
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position_change_type: Option<String>,
    /// 调整详细信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position_adjustment_info: Option<PositionAdjustmentInfo>,
    /// 兼容保留字段
    #[serde(flatten)]
    pub extra: Value,
}

/// 岗位调整详细信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PositionAdjustmentInfo {
    /// 原序列 ID 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_job_families: Option<Vec<String>>,
    /// 调整后序列 ID 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_job_families: Option<Vec<String>>,
    /// 原所属部门 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_department: Option<String>,
    /// 调整后所属部门 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_department: Option<String>,
    /// 调整后所属部门临时 ID（td_xxx 格式）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_draft_department: Option<String>,
    /// 原岗位默认成本中心 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_cost_center: Option<String>,
    /// 调整后岗位默认成本中心 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_cost_center: Option<String>,
    /// 原工时制度 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_working_hours_type: Option<String>,
    /// 调整后工时制度 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_working_hours_type: Option<String>,
    /// 原职务 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_job: Option<String>,
    /// 调整后职务 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_job: Option<String>,
    /// 原是否关键岗位
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_is_key_position: Option<bool>,
    /// 调整后是否关键岗位
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_is_key_position: Option<bool>,
    /// 原人员类型 ID 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_employee_types: Option<Vec<String>>,
    /// 调整后人员类型 ID 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_employee_types: Option<Vec<String>>,
    /// 原名称（多语）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_names: Option<Vec<I18nText>>,
    /// 调整后名称（多语）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_names: Option<Vec<I18nText>>,
    /// 原职等 ID 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_job_grades: Option<Vec<String>>,
    /// 调整后职等 ID 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_job_grades: Option<Vec<String>>,
    /// 原编码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_code: Option<String>,
    /// 调整后编码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_code: Option<String>,
    /// 原职级 ID 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_job_levels: Option<Vec<String>>,
    /// 调整后职级 ID 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_job_levels: Option<Vec<String>>,
    /// 原状态
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_active: Option<bool>,
    /// 调整后状态
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_active: Option<bool>,
    /// 原直线上级（岗位）ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_direct_leader: Option<String>,
    /// 调整后直线上级（岗位）ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_direct_leader: Option<String>,
    /// 调整后直线上级（岗位）临时 ID（td_xxx 格式）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_draft_direct_leader: Option<String>,
    /// 原工作地点 ID 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_work_locations: Option<Vec<String>>,
    /// 调整后工作地点 ID 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_work_locations: Option<Vec<String>>,
    /// 原描述（多语）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_descriptions: Option<Vec<I18nText>>,
    /// 调整后描述（多语）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_descriptions: Option<Vec<I18nText>>,
    /// 原部门全路径
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_department_id_paths: Option<Vec<OrgDraftDepartmentId>>,
    /// 调整后部门全路径
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_department_id_paths: Option<Vec<OrgDraftDepartmentId>>,
    /// 自定义字段
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<ChangeFieldPair>>,
}

/// 官方 i18n 多语文本结构：{lang, value}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nText {
    /// 语言编码（IETF BCP 47）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lang: Option<String>,
    /// 文本内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 官方 orgdraft_department_id 结构：{department_id, draft_department_id}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrgDraftDepartmentId {
    /// 部门 ID（新建部门未生效时为空）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 调整过程部门 ID（td_xxx 格式）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft_department_id: Option<String>,
}

/// 官方 change_field_pair 结构：{origin_value, target_value}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChangeFieldPair {
    /// 调整前
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_value: Option<CustomFieldData>,
    /// 调整后
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_value: Option<CustomFieldData>,
}

/// 官方 custom_field_data 结构
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomFieldData {
    /// 自定义字段 apiname，即自定义字段的唯一标识
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_api_name: Option<String>,
    /// 自定义字段名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<CustomFieldName>,
    /// 自定义字段类型
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<i64>,
    /// 字段值，是 json 转义后的字符串
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 自定义字段名称：{zh_cn, en_us}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CustomFieldName {
    /// 中文
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zh_cn: Option<String>,
    /// 英文
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub en_us: Option<String>,
}

impl ApiResponseTrait for OpenQueryPositionChangeListByIdsResponse {
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

    /// 端到端：POST /open-apis/corehr/v2/approval_groups/open_query_position_change_list_by_ids
    #[tokio::test]
    async fn test_open_query_position_change_list_by_ids_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/corehr/v2/approval_groups/open_query_position_change_list_by_ids",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "position_changes": [
                        {
                            "position_change_id": "6991776076699549697",
                            "position_id": "6966236933198579208",
                            "draft_position_id": "6966236933198579208",
                            "position_change_type": "Create",
                            "position_adjustment_info": {
                                "original_job_families": ["6974659700705068581"],
                                "target_job_families": ["6974659700705068581"],
                                "original_department": "6974659700705068581",
                                "target_department": "6974659700705068581",
                                "target_draft_department": "td_704442734715974602312",
                                "original_is_key_position": true,
                                "target_is_key_position": true,
                                "original_names": [
                                    { "lang": "zh-CN", "value": "中文示例" }
                                ],
                                "target_names": [
                                    { "lang": "zh-CN", "value": "中文示例" }
                                ],
                                "original_department_id_paths": [
                                    {
                                        "department_id": "6974659700705068581",
                                        "draft_department_id": "6974659700705068581"
                                    }
                                ],
                                "target_department_id_paths": [
                                    {
                                        "department_id": "6974659700705068581",
                                        "draft_department_id": "6974659700705068581"
                                    }
                                ],
                                "custom_fields": [
                                    {
                                        "origin_value": {
                                            "custom_api_name": "name",
                                            "name": { "zh_cn": "自定义姓名", "en_us": "Custom Name" },
                                            "type": 1,
                                            "value": "\"231\""
                                        },
                                        "target_value": {
                                            "custom_api_name": "name",
                                            "name": { "zh_cn": "自定义姓名", "en_us": "Custom Name" },
                                            "type": 1,
                                            "value": "\"232\""
                                        }
                                    }
                                ]
                            }
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

        let resp = OpenQueryPositionChangeListByIdsRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let changes = resp.position_changes.expect("应解析岗位调整列表");
        assert_eq!(changes.len(), 1);
        assert_eq!(
            changes[0].position_change_id,
            Some("6991776076699549697".to_string())
        );
        assert_eq!(changes[0].position_change_type, Some("Create".to_string()));
        let info = changes[0]
            .position_adjustment_info
            .as_ref()
            .expect("应解析调整详细信息");
        assert_eq!(
            info.original_department,
            Some("6974659700705068581".to_string())
        );
        assert_eq!(
            info.target_draft_department,
            Some("td_704442734715974602312".to_string())
        );
        assert_eq!(info.target_is_key_position, Some(true));
        assert_eq!(
            info.original_department_id_paths.as_deref(),
            Some(
                &[OrgDraftDepartmentId {
                    department_id: Some("6974659700705068581".to_string()),
                    draft_department_id: Some("6974659700705068581".to_string()),
                }][..]
            )
        );
        let custom_fields = info.custom_fields.as_deref().expect("应解析自定义字段");
        assert_eq!(custom_fields.len(), 1);
        assert_eq!(
            custom_fields[0]
                .origin_value
                .as_ref()
                .and_then(|v| v.r#type),
            Some(1)
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
