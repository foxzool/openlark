//! 获取行为审计日志数据 API
//!
//! API文档: <https://open.feishu.cn/document/ukTMukTMukTM/uQjM5YjL0ITO24CNykjN/audit_log/audit_data_get>
//! docPath: <https://open.feishu.cn/document/ukTMukTMukTM/uQjM5YjL0ITO24CNykjN/audit_log/audit_data_get>
//!
//! Authorization: `tenant_access_token`。查询参数全部可选（`oldest`/`latest` 默认 30 日前此刻 / 此刻），
//! 起止时间相差不能超过 30 天；`page_size` 取值范围 1~200（服务端校验）。

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 获取审计日志列表请求 Builder。
pub struct ListAuditInfoRequestBuilder {
    /// 配置信息。
    config: Config,
    /// 用户 ID 类型（open_id/union_id/user_id，默认 user_id）。
    user_id_type: Option<String>,
    /// 日志时间范围起始时间（秒级时间戳，默认 30 日前此刻）。
    oldest: Option<i64>,
    /// 日志时间范围结束时间（秒级时间戳，默认此刻）。
    latest: Option<i64>,
    /// 行为审计的事件名称。
    event_name: Option<String>,
    /// 过滤操作者类型（user/bot，与 operator_value 配合使用）。
    operator_type: Option<String>,
    /// 操作者值（填写时 operator_type 必填）。
    operator_value: Option<String>,
    /// 行为审计的事件模块。
    event_module: Option<i32>,
    /// 用户类型（0/1/2，填写时 operator_type 必须为 user）。
    user_type: Option<i32>,
    /// 过滤操作对象类型（与 object_value 配合使用）。
    object_type: Option<i32>,
    /// 过滤操作对象 ID（与 object_type 配合使用）。
    object_value: Option<String>,
    /// 分页大小（1~200，默认 20）。
    page_size: Option<u32>,
    /// 分页标记。
    page_token: Option<String>,
}

impl ListAuditInfoRequestBuilder {
    /// 创建新的实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_id_type: None,
            oldest: None,
            latest: None,
            event_name: None,
            operator_type: None,
            operator_value: None,
            event_module: None,
            user_type: None,
            object_type: None,
            object_value: None,
            page_size: None,
            page_token: None,
        }
    }

    /// 设置用户 ID 类型（open_id/union_id/user_id）。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置日志时间范围起始时间（秒级时间戳）。
    pub fn oldest(mut self, oldest: i64) -> Self {
        self.oldest = Some(oldest);
        self
    }

    /// 设置日志时间范围结束时间（秒级时间戳）。
    pub fn latest(mut self, latest: i64) -> Self {
        self.latest = Some(latest);
        self
    }

    /// 设置行为审计的事件名称。
    pub fn event_name(mut self, event_name: impl Into<String>) -> Self {
        self.event_name = Some(event_name.into());
        self
    }

    /// 设置过滤操作者类型（user/bot）。
    pub fn operator_type(mut self, operator_type: impl Into<String>) -> Self {
        self.operator_type = Some(operator_type.into());
        self
    }

    /// 设置操作者值。
    pub fn operator_value(mut self, operator_value: impl Into<String>) -> Self {
        self.operator_value = Some(operator_value.into());
        self
    }

    /// 设置行为审计的事件模块。
    pub fn event_module(mut self, event_module: i32) -> Self {
        self.event_module = Some(event_module);
        self
    }

    /// 设置用户类型（0/1/2）。
    pub fn user_type(mut self, user_type: i32) -> Self {
        self.user_type = Some(user_type);
        self
    }

    /// 设置过滤操作对象类型。
    pub fn object_type(mut self, object_type: i32) -> Self {
        self.object_type = Some(object_type);
        self
    }

    /// 设置过滤操作对象 ID。
    pub fn object_value(mut self, object_value: impl Into<String>) -> Self {
        self.object_value = Some(object_value.into());
        self
    }

    /// 设置分页大小。
    pub fn page_size(mut self, page_size: u32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<ListAuditInfoResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListAuditInfoResponse> {
        let req: ApiRequest<ListAuditInfoResponse> =
            ApiRequest::get("/open-apis/admin/v1/audit_infos")
                .query_opt("user_id_type", self.user_id_type)
                .query_opt("oldest", self.oldest.map(|v| v.to_string()))
                .query_opt("latest", self.latest.map(|v| v.to_string()))
                .query_opt("event_name", self.event_name)
                .query_opt("operator_type", self.operator_type)
                .query_opt("operator_value", self.operator_value)
                .query_opt("event_module", self.event_module.map(|v| v.to_string()))
                .query_opt("user_type", self.user_type.map(|v| v.to_string()))
                .query_opt("object_type", self.object_type.map(|v| v.to_string()))
                .query_opt("object_value", self.object_value)
                .query_opt("page_size", self.page_size.map(|v| v.to_string()))
                .query_opt("page_token", self.page_token);

        Transport::request_typed(req, &self.config, Some(option), "获取审计日志列表").await
    }
}

/// 获取审计日志列表响应。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ListAuditInfoResponse {
    /// 审计日志条目列表。
    pub items: Vec<AuditInfoItem>,
    /// 分页标记（has_more 为 true 时返回）。
    pub page_token: Option<String>,
    /// 是否还有更多数据。
    pub has_more: bool,
}

/// 审计日志条目（audit_info）。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuditInfoItem {
    /// 事件 ID，不唯一，可用作聚合。
    pub event_id: String,
    /// 事件唯一 ID，可以用于去重，倾向使用该字段识别用户的行为。
    pub unique_id: String,
    /// 事件名称。
    pub event_name: String,
    /// 用户所属部门的 ID 列表（需开启 contact:user.employee_id:readonly 权限才返回）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    /// 事件模块。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_module: Option<i32>,
    /// 操作人类型（1：组织内成员；12：机器人；1001：组织外成员）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_type: Option<i32>,
    /// 操作人 ID（operator_type 为 1001 时为脱敏后的值）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_value: Option<String>,
    /// 操作对象列表。
    #[serde(default)]
    pub objects: Vec<AuditObjectEntity>,
    /// 接收者对象列表。
    #[serde(default)]
    pub recipients: Vec<AuditRecipientEntity>,
    /// 事件时间（秒级时间戳）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<i64>,
    /// IP 信息。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    /// 第三方 ISV ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_app: Option<String>,
    /// 环境信息。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_context: Option<AuditContext>,
    /// 事件扩展字段。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extend: Option<AuditEventExtend>,
    /// 第三方 ISV 名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_app_name: Option<String>,
    /// 事件扩展字段（抽屉信息）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub common_drawers: Option<ApiAuditCommonDrawers>,
    /// 设备信息。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audit_detail: Option<AuditDetail>,
    /// 操作人所在企业编号。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_tenant: Option<String>,
}

/// 操作对象实体（audit_object_entity）。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuditObjectEntity {
    /// 操作对象类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_type: Option<String>,
    /// 操作对象值。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_value: Option<String>,
    /// 操作对象名称（当前针对文档、会话、应用类型开放）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_name: Option<String>,
    /// 操作对象所有者（当前针对文档类型开放）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_owner: Option<String>,
    /// 操作对象扩展字段。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_detail: Option<AuditObjectDetail>,
}

/// 操作对象扩展字段（audit_object_detail）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuditObjectDetail {
    /// 复制来源。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clone_source: Option<String>,
    /// 第三方 APPID。
    #[serde(rename = "third_party_appID", skip_serializing_if = "Option::is_none")]
    pub third_party_app_id: Option<String>,
    /// 文件或文件夹数量。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contain_file_num: Option<i32>,
    /// 链接分享设置。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_setting_type: Option<String>,
    /// 是否开启外部访问设置。
    #[serde(
        rename = "permission_external_access_Type",
        skip_serializing_if = "Option::is_none"
    )]
    pub permission_external_access_type: Option<bool>,
    /// 权限分享设置。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_share_type: Option<String>,
    /// 上传文件的来源。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_service_source: Option<String>,
    /// 下载 OKR 的内容。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub okr_download_content: Option<String>,
    /// 容器类型，标识是否 wiki。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_type: Option<String>,
    /// 容器 ID，wiki 标识字段。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_id: Option<String>,
}

/// 接收者对象实体（audit_recipient_entity）。
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AuditRecipientEntity {
    /// 接收者对象类型（1 代表用户，2 代表部门，4 代表会话）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_type: Option<String>,
    /// 接收者对象值。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_value: Option<String>,
    /// 接收者对象扩展字段。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_detail: Option<AuditRecipientDetail>,
}

/// 接收者对象扩展字段（audit_recipient_detail）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuditRecipientDetail {
    /// 权限操作类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_action_type: Option<String>,
}

/// 环境信息（audit_context）。
///
/// 注：审计环境信息有可能由于客户端版本不同、用户使用终端不同等原因导致缺失。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuditContext {
    /// 终端类型（0：iOS；1：安卓；2：PC 端；3：Web 端）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal_type: Option<i32>,
    /// iOS 环境信息。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ios_context: Option<AuditIosContext>,
    /// PC 环境信息。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pc_context: Option<AuditPcContext>,
    /// Web 环境信息。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_context: Option<AuditWebContext>,
    /// Android 环境信息。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub android_context: Option<AuditAndroidContext>,
}

/// Web 环境信息（audit_web_context）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuditWebContext {
    /// UA 信息。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
    /// 公网 IP。
    #[serde(rename = "IP", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

/// iOS 环境信息（audit_ios_context）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuditIosContext {
    /// UDID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udid: Option<String>,
    /// 设备 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub did: Option<String>,
    /// App 的版本。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_ver: Option<String>,
    /// SecSDK 版本。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ver: Option<String>,
    /// 系统类型及版本。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// 系统时区。
    #[serde(rename = "STZone", skip_serializing_if = "Option::is_none")]
    pub st_zone: Option<String>,
    /// 当前语言。
    #[serde(rename = "ML", skip_serializing_if = "Option::is_none")]
    pub ml: Option<String>,
    /// 是否越狱。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sjd: Option<String>,
    /// 代理 IP。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxyip: Option<String>,
    /// WiFi IP。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wifip: Option<String>,
    /// 公网 IP。
    #[serde(rename = "IP", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

/// Android 环境信息（audit_android_context）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuditAndroidContext {
    /// UDID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udid: Option<String>,
    /// 设备 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub did: Option<String>,
    /// App 的版本。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_ver: Option<String>,
    /// SecSDK 版本。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ver: Option<String>,
    /// 设备语言。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    /// 安卓版本号。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_i: Option<String>,
    /// 安卓版本。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_r: Option<String>,
    /// Brand。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hw_brand: Option<String>,
    /// 制造商。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hw_manuf: Option<String>,
    /// WiFi IP。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wifip: Option<String>,
    /// 路由 IP。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_iip: Option<String>,
    /// 路由网关 IP。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_gip: Option<String>,
    /// 表示当前是否 root。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_su: Option<String>,
    /// 手机系统时区。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_tz: Option<String>,
    /// 手机系统语言。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub env_ml: Option<String>,
    /// 公网 IP。
    #[serde(rename = "IP", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

/// PC 环境信息（audit_pc_context）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuditPcContext {
    /// UDID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub udid: Option<String>,
    /// 设备 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub did: Option<String>,
    /// App 的版本。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_ver: Option<String>,
    /// SecSDK 版本。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ver: Option<String>,
    /// 客户端类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
    /// WiFi IP。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wifip: Option<String>,
    /// 公网 IP。
    #[serde(rename = "IP", skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
}

/// 事件扩展字段（audit_event_extend）。
///
/// 官方文档未定义字段表（示例为空对象，内容参考 `common_drawers` 中的信息），
/// 以透传 JSON 对象承载。
pub type AuditEventExtend = serde_json::Value;

/// 事件扩展字段（api_audit_common_drawers，抽屉信息）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ApiAuditCommonDrawers {
    /// 抽屉信息列表。
    #[serde(default)]
    pub common_draw_info_list: Vec<AuditCommonDrawInfo>,
}

/// 单条抽屉信息。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuditCommonDrawInfo {
    /// 信息键。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info_key: Option<String>,
    /// 信息值。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info_val: Option<String>,
    /// 键的国际化键。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_i18n_key: Option<String>,
    /// 值的国际化键。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub val_i18n_key: Option<String>,
    /// 值类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub val_type: Option<String>,
}

/// 设备信息（audit_detail）。
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct AuditDetail {
    /// IP 位置，城市名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// 设备型号。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_model: Option<String>,
    /// Mac 地址。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mc: Option<String>,
    /// 操作系统。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub os: Option<String>,
}

impl ApiResponseTrait for ListAuditInfoResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 旧名兼容别名（将在 v1.0 移除）
#[deprecated(note = "renamed to ListAuditInfoRequestBuilder, will be removed in v1.0 (#271)")]
pub type ListAuditInfoBuilder = ListAuditInfoRequestBuilder;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::{
        Mock, MockServer, ResponseTemplate,
        matchers::{method, path, query_param},
    };

    /// 端到端：GET /open-apis/admin/v1/audit_infos?oldest=..&latest=.. → 官方响应示例结构。
    #[tokio::test]
    async fn test_list_audit_info_parses_official_response_shape() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/admin/v1/audit_infos"))
            .and(query_param("oldest", "1668528000"))
            .and(query_param("latest", "1668700799"))
            .and(query_param("event_name", "space_edit_doc"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "has_more": false,
                    "page_token": "LC39/f1%2B/Sz9Uv39Gf39",
                    "items": [
                        {
                            "event_id": "7254062411181719572",
                            "unique_id": "7254062413199179796",
                            "event_module": 1,
                            "event_name": "space_edit_doc",
                            "operator_type": 1,
                            "operator_value": "4a3b8541",
                            "operator_tenant": "F686619755",
                            "event_time": 1688968015,
                            "audit_context": {
                                "terminal_type": 3,
                                "web_context": {
                                    "IP": "fdbd:dc02:ff:1:1:174:246:126",
                                    "user_agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36"
                                }
                            },
                            "audit_detail": { "city": "", "device_model": "", "mc": "", "os": "" },
                            "common_drawers": {
                                "common_draw_info_list": [
                                    {
                                        "info_key": "CCM_op_status",
                                        "info_val": "success",
                                        "key_i18n_key": "SuiteAdmin_DrawerKeyspaceeditdoc_Kccmopstatus",
                                        "val_i18n_key": "SuiteAdmin_DrawerValspaceeditdoc_Kccmopstatusvsuccess",
                                        "val_type": "1"
                                    },
                                    { "info_key": "ccm_type", "info_val": "-", "key_i18n_key": "文件类型" }
                                ]
                            },
                            "department_ids": ["0", "od-ab89476fbcf901c3e5ccd78f788f85bf"],
                            "extend": {},
                            "ip": "fdbd:dc02:ff:1:1:174:246:126",
                            "objects": [
                                {
                                    "object_detail": {},
                                    "object_name": "",
                                    "object_owner": "",
                                    "object_type": "106",
                                    "object_value": "Lwd1smp3nl01AndDEMzbsfqacBb"
                                }
                            ],
                            "operator_app": "",
                            "operator_app_name": "",
                            "recipients": [
                                {
                                    "recipient_type": "",
                                    "recipient_value": "",
                                    "recipient_detail": { "permission_action_type": "" }
                                }
                            ]
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

        let resp = ListAuditInfoRequestBuilder::new(config)
            .oldest(1668528000)
            .latest(1668700799)
            .event_name("space_edit_doc")
            .execute()
            .await
            .expect("获取行为审计日志数据应成功");

        assert!(!resp.has_more);
        assert_eq!(resp.page_token.as_deref(), Some("LC39/f1%2B/Sz9Uv39Gf39"));
        assert_eq!(resp.items.len(), 1);
        let item = &resp.items[0];
        assert_eq!(item.event_id, "7254062411181719572");
        assert_eq!(item.unique_id, "7254062413199179796");
        assert_eq!(item.event_name, "space_edit_doc");
        assert_eq!(item.event_module, Some(1));
        assert_eq!(item.operator_type, Some(1));
        assert_eq!(item.operator_value.as_deref(), Some("4a3b8541"));
        assert_eq!(item.operator_tenant.as_deref(), Some("F686619755"));
        assert_eq!(item.event_time, Some(1688968015));
        assert_eq!(item.department_ids.as_deref().map(|v| v.len()), Some(2));
        assert_eq!(item.ip.as_deref(), Some("fdbd:dc02:ff:1:1:174:246:126"));
        assert_eq!(item.operator_app.as_deref(), Some(""));
        assert_eq!(item.operator_app_name.as_deref(), Some(""));
        assert_eq!(item.objects.len(), 1);
        assert_eq!(
            item.objects[0].object_value.as_deref(),
            Some("Lwd1smp3nl01AndDEMzbsfqacBb")
        );
        assert_eq!(item.objects[0].object_type.as_deref(), Some("106"));
        assert!(item.objects[0].object_detail.is_some());
        assert_eq!(item.recipients.len(), 1);
        assert_eq!(
            item.recipients[0]
                .recipient_detail
                .as_ref()
                .and_then(|d| d.permission_action_type.as_deref()),
            Some("")
        );
        let context = item.audit_context.as_ref().expect("audit_context 应存在");
        assert_eq!(context.terminal_type, Some(3));
        let web = context.web_context.as_ref().expect("web_context 应存在");
        assert_eq!(web.ip.as_deref(), Some("fdbd:dc02:ff:1:1:174:246:126"));
        assert!(
            web.user_agent
                .as_deref()
                .is_some_and(|ua| ua.starts_with("Mozilla/5.0"))
        );
        let detail = item.audit_detail.as_ref().expect("audit_detail 应存在");
        assert_eq!(detail.city.as_deref(), Some(""));
        let drawers = item.common_drawers.as_ref().expect("common_drawers 应存在");
        assert_eq!(drawers.common_draw_info_list.len(), 2);
        assert_eq!(
            drawers.common_draw_info_list[0].info_key.as_deref(),
            Some("CCM_op_status")
        );
        // 官方示例第二条缺 val_i18n_key / val_type，应为 None 而非反序列化失败
        assert!(drawers.common_draw_info_list[1].val_i18n_key.is_none());
        assert!(drawers.common_draw_info_list[1].val_type.is_none());
        // extend 官方示例为空对象
        assert!(matches!(item.extend, Some(serde_json::Value::Object(_))));
    }
}
