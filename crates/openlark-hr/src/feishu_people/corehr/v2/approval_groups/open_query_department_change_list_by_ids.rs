//! 批量查询部门调整内容
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v2/approval_groups/open_query_department_change_list_by_ids>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 批量查询部门调整内容请求
#[derive(Debug, Clone)]
pub struct OpenQueryDepartmentChangeListByIdsRequest {
    /// 配置信息
    config: Config,
    body: Option<Value>,
}

impl OpenQueryDepartmentChangeListByIdsRequest {
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
    pub async fn execute(self) -> SDKResult<OpenQueryDepartmentChangeListByIdsResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<OpenQueryDepartmentChangeListByIdsResponse> {
        let mut request = ApiRequest::<OpenQueryDepartmentChangeListByIdsResponse>::post(
            "/open-apis/corehr/v2/approval_groups/open_query_department_change_list_by_ids",
        );

        if let Some(body) = self.body {
            request = request.body(body);
        }

        Transport::request_typed(request, &self.config, Some(option), "接口响应数据为空").await
    }
}

/// 批量查询部门调整内容响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OpenQueryDepartmentChangeListByIdsResponse {
    /// 部门调整记录信息列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_changes: Option<Vec<DepartmentChange>>,
}

/// 部门调整记录
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DepartmentChange {
    /// 关联的部门调整记录 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_change_id: Option<String>,
    /// 部门 ID，对于在本次调整中新建的部门，在调整未生效时将返回为空
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 调整过程部门 ID（固定 people_corehr_department_id，新建部门未生效时为 td_xxx 临时 ID）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft_department_id: Option<String>,
    /// 调整类型：Unknown（未知）/ Create（新建）/ Modify（编辑）/ Inactive（停用）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_change_type: Option<String>,
    /// 调整状态：0 发起审批 / 1 审批通过 / 2 审批被拒绝 / 3 审批被撤销 / 4 执行成功 / 5 执行失败 / 6 待执行
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub department_change_status: Option<i32>,
    /// 调整详细信息
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reorganization_info: Option<ReorganizationInfo>,
}

/// 部门调整详细信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReorganizationInfo {
    /// 原部门名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_department_names: Option<Vec<I18nText>>,
    /// 调整后部门名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_department_names: Option<Vec<I18nText>>,
    /// 原部门编码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_department_code: Option<String>,
    /// 调整后部门编码（新建部门且租户开通部门自动编码时为空值）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_department_code: Option<String>,
    /// 原部门类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_sub_type: Option<EnumValue>,
    /// 调整后部门类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_sub_type: Option<EnumValue>,
    /// 原部门负责人
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_department_manager: Option<String>,
    /// 调整后部门负责人
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_department_manager: Option<String>,
    /// 原描述
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_descriptions: Option<Vec<I18nText>>,
    /// 调整后描述
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_descriptions: Option<Vec<I18nText>>,
    /// 原默认成本中心
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_cost_center: Option<CostCenter>,
    /// 调整后默认成本中心
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_cost_center: Option<CostCenter>,
    /// 原是否保密
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_is_confidential: Option<bool>,
    /// 调整后是否保密
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_is_confidential: Option<bool>,
    /// 原岗职模式
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_staffing_mode_option: Option<EnumValue>,
    /// 调整后岗职模式
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_staffing_mode_option: Option<EnumValue>,
    /// 原上级部门 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_parent_department_id: Option<String>,
    /// 调整后上级部门 ID（新建部门未生效时为空）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_parent_department_id: Option<String>,
    /// 调整后上级部门过程 ID（新建部门未生效时为 td_xxx 临时 ID）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub draft_target_parent_department_id: Option<String>,
    /// 原部门全路径，从根部门开始自上而下返回部门 ID 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub original_department_id_path: Option<Vec<OrgDraftDepartmentId>>,
    /// 调整后部门全路径，从根部门开始自上而下返回部门 ID 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_department_id_path: Option<Vec<OrgDraftDepartmentId>>,
    /// 自定义字段
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<ChangeFieldPair>>,
    /// 调整前后组织角色信息（需 corehr:orgrole_info:read 权限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orgrole_infos: Option<Vec<OrgroleInfo>>,
}

/// 成本中心
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CostCenter {
    /// 成本中心 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center_id: Option<String>,
    /// 成本中心版本 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cost_center_version_id: Option<String>,
    /// 成本中心名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<Vec<I18nText>>,
    /// 成本中心编码
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 上级成本中心 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent_cost_center_id: Option<String>,
    /// 成本中心负责人 ID 列表
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managers: Option<Vec<String>>,
    /// 成本中心描述
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<Vec<I18nText>>,
    /// 生效时间（YYYY-MM-DD）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 过期时间（YYYY-MM-DD）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    /// 当前实体是否启用
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    /// 操作原因
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operation_reason: Option<String>,
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

/// 自定义字段调整前后值
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChangeFieldPair {
    /// 调整前
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_value: Option<CustomFieldData>,
    /// 调整后
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_value: Option<CustomFieldData>,
}

/// 自定义字段数据
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

/// 调整前后组织角色信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrgroleInfo {
    /// 角色 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    /// 原组织角色
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_orgroles: Option<Vec<OrgdraftOrgroleAssignment>>,
    /// 新组织角色
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_orgroles: Option<Vec<OrgdraftOrgroleAssignment>>,
}

/// 组织角色授权
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrgdraftOrgroleAssignment {
    /// 授权用户 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grantee_id: Option<String>,
    /// 管理范围，组织角色为交叉角色时有值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub management_scopes: Option<Vec<OrgroleAssignmentOrg>>,
}

/// 组织角色管理范围
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrgroleAssignmentOrg {
    /// 组织 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub org_id: Option<String>,
    /// 组织类型：department / location / custom_org_01 ~ custom_org_05
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub org_type: Option<String>,
    /// 组织名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub org_name: Option<String>,
}

/// 官方 enum 类型字段结构：{enum_name, display: i18n[]}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EnumValue {
    /// 枚举值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enum_name: Option<String>,
    /// 枚举多语展示
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub display: Option<Vec<I18nText>>,
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

impl ApiResponseTrait for OpenQueryDepartmentChangeListByIdsResponse {
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

    /// 端到端：POST /open-apis/corehr/v2/approval_groups/open_query_department_change_list_by_ids
    #[tokio::test]
    async fn test_open_query_department_change_list_by_ids_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/corehr/v2/approval_groups/open_query_department_change_list_by_ids",
            ))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "department_changes": [{
                        "department_change_id": "6991776076699549697",
                        "department_id": "6966236934218579208",
                        "draft_department_id": "3346236933198579208",
                        "department_change_type": "Modify",
                        "reorganization_info": {
                            "original_department_names": [{"lang": "zh-CN", "value": "技术研发部"}],
                            "target_department_names": [{"lang": "zh-CN", "value": "产品技术中心"}],
                            "original_department_code": "D00000456",
                            "target_department_code": "D00000456",
                            "original_sub_type": {
                                "enum_name": "division",
                                "display": [{"lang": "zh-CN", "value": "系"}]
                            },
                            "target_sub_type": {
                                "enum_name": "department",
                                "display": [{"lang": "zh-CN", "value": "部门"}]
                            },
                            "original_department_manager": "6974648866876573198",
                            "target_department_manager": "7013328578351842852",
                            "original_cost_center": {
                                "cost_center_id": "6969828847121885087",
                                "name": [{"lang": "zh-CN", "value": "研发成本中心 - 001"}]
                            },
                            "original_is_confidential": true,
                            "target_is_confidential": true,
                            "original_parent_department_id": "6974659700705068581",
                            "target_parent_department_id": "69746597007987678581",
                            "draft_target_parent_department_id": "6966236933198579208",
                            "original_department_id_path": [{
                                "department_id": "6974612300705068581",
                                "draft_department_id": "6974655643205068581"
                            }],
                            "target_department_id_path": [{
                                "department_id": "6974634700705068581",
                                "draft_department_id": "6974659700705068581"
                            }],
                            "custom_fields": [{
                                "origin_value": {
                                    "custom_api_name": "name",
                                    "name": {"zh_cn": "自定义姓名", "en_us": "Custom Name"},
                                    "type": 1,
                                    "value": "\"231\""
                                },
                                "target_value": {
                                    "custom_api_name": "name",
                                    "name": {"zh_cn": "自定义姓名", "en_us": "Custom Name"},
                                    "type": 1,
                                    "value": "\"231\""
                                }
                            }]
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

        let resp = OpenQueryDepartmentChangeListByIdsRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        let changes = resp
            .department_changes
            .as_ref()
            .expect("department_changes 应被解码");
        assert_eq!(changes.len(), 1);
        assert_eq!(
            changes[0].department_change_id.as_deref(),
            Some("6991776076699549697")
        );
        assert_eq!(changes[0].department_change_type.as_deref(), Some("Modify"));
        let info = changes[0]
            .reorganization_info
            .as_ref()
            .expect("reorganization_info 应被解码");
        assert_eq!(
            info.original_department_names
                .as_ref()
                .and_then(|n| n.first())
                .and_then(|n| n.value.as_deref()),
            Some("技术研发部")
        );
        assert_eq!(info.original_department_code.as_deref(), Some("D00000456"));
        assert_eq!(
            info.original_department_id_path
                .as_ref()
                .and_then(|p| p.first())
                .and_then(|p| p.draft_department_id.as_deref()),
            Some("6974655643205068581")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
