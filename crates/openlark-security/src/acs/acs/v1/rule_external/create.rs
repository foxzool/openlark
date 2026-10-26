//! 创建或更新权限组
//!
//! docPath: <https://open.feishu.cn/document/acs-v1/rule_external/create>
//!
//! 文档核对：`POST /open-apis/acs/v1/rule_external?rule_id={rule_id}&user_id_type={...}`，
//! body 为 `{"rule": {...}}` 包装（`rule` 必填）。查询参数 `rule_id` 可选
//! （为空创建、不为空则更新）、`user_id_type` 可选（默认 `open_id`）。

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, constants::AccessTokenType,
    error::validation_error, http::Transport, req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 创建或更新权限组请求 body。
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRuleExternalBody {
    /// 权限组信息。
    pub rule: RuleExternal,
}

/// 权限组信息（官方 rule）。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RuleExternal {
    /// 权限组 ID（更新时传入）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 权限组名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 权限组包含的设备（0~5000 个）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<RuleExternalDevice>>,
    /// 权限组包含的员工个数。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_count: Option<String>,
    /// 权限组包含的员工列表（0~10000 个）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<RuleExternalUser>>,
    /// 权限组包含的访客个数。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visitor_count: Option<String>,
    /// 权限组包含的访客列表（0~10000 个）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visitors: Option<Vec<RuleExternalUser>>,
    /// 是否通知人员录入。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remind_face: Option<bool>,
    /// 开门时间段。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opening_time: Option<OpeningTimeExternal>,
    /// 是否为临时权限组。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_temp: Option<bool>,
}

/// 权限组包含的设备（官方 device_external）。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RuleExternalDevice {
    /// 设备 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 设备名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// 权限组包含的用户（官方 user_external，员工/访客共用）。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RuleExternalUser {
    /// 用户类型（必填；`1` 员工/`2` 部门/`10` 全体员工/`11` 访客）。
    pub user_type: i32,
    /// 用户 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户名称。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// 电话号码。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_num: Option<String>,
    /// 部门 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
}

/// 开门时间段（官方 opening_time_external）。
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OpeningTimeExternal {
    /// 有效日期。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_day: Option<OpeningTimeValidDayExternal>,
    /// 有效星期（0~7 个）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekdays: Option<Vec<i32>>,
    /// 有效时间（0~1 个）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day_times: Option<Vec<OpeningTimePeriodExternal>>,
}

/// 有效日期（官方 opening_time_valid_day_external）。
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct OpeningTimeValidDayExternal {
    /// 权限开始时间（1000000000~2147483647）。
    pub start_day: i32,
    /// 权限结束时间（1000000000~2147483647）。
    pub end_day: i32,
}

/// 有效时间（官方 opening_time_period_external）。
#[derive(Debug, Clone, Copy, Serialize, Deserialize, Default)]
pub struct OpeningTimePeriodExternal {
    /// 起始时间（0~2400，如 1200）。
    pub start_hhmm: i32,
    /// 结束时间（0~2400，如 1400）。
    pub end_hhmm: i32,
}

/// 创建或更新权限组请求
///
/// 通过 [`Self::rule`] 传入权限组内容，内部序列化为 `{"rule": ...}` 包装。
#[derive(Debug)]
pub struct CreateRuleExternalRequest {
    /// 配置信息。
    config: Config,
    /// 权限组 ID（查询参数，可选；为空创建、不为空则更新）。
    rule_id: Option<String>,
    /// 用户 ID 类型（查询参数，可选；如 `open_id`/`user_id`/`union_id`，默认 `open_id`）。
    user_id_type: Option<String>,
    /// 请求 body。
    body: CreateRuleExternalBody,
}

impl CreateRuleExternalRequest {
    /// 创建新的请求构建器（不传 `rule_id` 即为创建模式）。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            rule_id: None,
            user_id_type: None,
            body: CreateRuleExternalBody {
                rule: RuleExternal::default(),
            },
        }
    }

    /// 设置权限组 ID（更新模式；为空创建、不为空则更新）。
    pub fn rule_id(mut self, rule_id: impl Into<String>) -> Self {
        self.rule_id = Some(rule_id.into());
        self
    }

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置权限组内容（`rule` 字段整体替换）。
    pub fn rule(mut self, rule: RuleExternal) -> Self {
        self.body.rule = rule;
        self
    }

    /// 设置权限组名称。
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.body.rule.name = Some(name.into());
        self
    }

    /// 设置权限组包含的设备。
    pub fn devices(mut self, devices: Vec<RuleExternalDevice>) -> Self {
        self.body.rule.devices = Some(devices);
        self
    }

    /// 设置权限组包含的员工列表。
    pub fn users(mut self, users: Vec<RuleExternalUser>) -> Self {
        self.body.rule.users = Some(users);
        self
    }

    /// 设置权限组包含的访客列表。
    pub fn visitors(mut self, visitors: Vec<RuleExternalUser>) -> Self {
        self.body.rule.visitors = Some(visitors);
        self
    }

    /// 设置开门时间段。
    pub fn opening_time(mut self, opening_time: OpeningTimeExternal) -> Self {
        self.body.rule.opening_time = Some(opening_time);
        self
    }

    /// 执行请求，返回响应 `data` 字段内容。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post("/open-apis/acs/v1/rule_external")
                .query_opt("rule_id", self.rule_id.as_ref())
                .query_opt("user_id_type", self.user_id_type.as_ref())
                .body(serde_json::to_value(&self.body).map_err(|e| {
                    validation_error("创建或更新权限组", format!("序列化失败: {e}"))
                })?)
                .with_supported_access_token_types(vec![AccessTokenType::User]);

        Transport::request_typed(req, &self.config, Some(option), "创建或更新权限组").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use wiremock::MockServer;
    use wiremock::matchers::{body_json, method, path};
    use wiremock::{Mock, ResponseTemplate};

    /// 端到端（更新模式）：POST .../rule_external?rule_id= + {"rule": {...}} 包装 + 嵌套 opening_time 序列化。
    #[tokio::test]
    async fn test_create_rule_external_update_mode() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/acs/v1/rule_external"))
            .and(body_json(json!({
                "rule": {
                    "id": "34252345234523",
                    "name": "南门",
                    "devices": [{ "id": "534545234523452345", "name": "北门" }],
                    "users": [
                        { "user_type": 1, "user_id": "ou_7dab8a3d3cdcc9da365777c7ad535d62", "user_name": "张三" }
                    ],
                    "remind_face": false,
                    "opening_time": {
                        "valid_day": { "start_day": 1699031483, "end_day": 1699931483 },
                        "weekdays": [1, 2, 3],
                        "day_times": [{ "start_hhmm": 1200, "end_hhmm": 1400 }]
                    },
                    "is_temp": false
                }
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "rule_id": "34252345234523" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = CreateRuleExternalRequest::new(config)
            .rule_id("34252345234523")
            .user_id_type("open_id")
            .rule(RuleExternal {
                id: Some("34252345234523".to_string()),
                name: Some("南门".to_string()),
                devices: Some(vec![RuleExternalDevice {
                    id: Some("534545234523452345".to_string()),
                    name: Some("北门".to_string()),
                }]),
                users: Some(vec![RuleExternalUser {
                    user_type: 1,
                    user_id: Some("ou_7dab8a3d3cdcc9da365777c7ad535d62".to_string()),
                    user_name: Some("张三".to_string()),
                    phone_num: None,
                    department_id: None,
                }]),
                remind_face: Some(false),
                opening_time: Some(OpeningTimeExternal {
                    valid_day: Some(OpeningTimeValidDayExternal {
                        start_day: 1699031483,
                        end_day: 1699931483,
                    }),
                    weekdays: Some(vec![1, 2, 3]),
                    day_times: Some(vec![OpeningTimePeriodExternal {
                        start_hhmm: 1200,
                        end_hhmm: 1400,
                    }]),
                }),
                is_temp: Some(false),
                user_count: None,
                visitor_count: None,
                visitors: None,
            })
            .execute()
            .await
            .expect("创建或更新权限组应成功");
        assert_eq!(data["rule_id"], "34252345234523");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let query = received[0].url.query().unwrap_or("");
        assert!(query.contains("rule_id=34252345234523"));
        assert!(query.contains("user_id_type=open_id"));
        // body 应被包装为 {"rule": {...}}
        let sent: serde_json::Value = serde_json::from_slice(&received[0].body).unwrap();
        assert_eq!(sent["rule"]["name"], "南门");
        assert_eq!(
            sent["rule"]["opening_time"]["valid_day"]["start_day"],
            1699031483
        );
    }

    /// 端到端（创建模式）：不传 rule_id 时查询参数不带 rule_id。
    #[tokio::test]
    async fn test_create_rule_external_create_mode_omits_rule_id() {
        let server = MockServer::start().await;
        Mock::given(method("POST"))
            .and(path("/open-apis/acs/v1/rule_external"))
            .and(body_json(json!({ "rule": { "name": "前台权限组" } })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": { "rule_id": "7298933941867135276" }
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = CreateRuleExternalRequest::new(config)
            .name("前台权限组")
            .execute()
            .await
            .expect("创建权限组应成功");
        assert_eq!(data["rule_id"], "7298933941867135276");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        let query = received[0].url.query().unwrap_or("");
        assert!(!query.contains("rule_id="));
    }
}
