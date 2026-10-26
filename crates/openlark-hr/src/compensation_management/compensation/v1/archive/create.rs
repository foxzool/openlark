//! 创建薪资档案
//!
//! docPath: <https://open.feishu.cn/document/server-docs/compensation-v1/archive/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 创建薪资档案请求
#[derive(Debug, Clone)]
pub struct CreateRequest {
    /// 外部幂等 id（必填，标准 UUID V4 格式，避免重复发起）
    unique_id: String,
    /// 操作人 ID（可选，具体类型由 user_id_type 指定；应用身份鉴权时不能为空）
    operator_id: Option<String>,
    /// 员工 id（必填，具体类型由 user_id_type 指定）
    user_id: String,
    /// 生效时间（必填，格式 YYYY-MM-DD，长度 10 字符）
    effective_time: String,
    /// 币种 ID（必填）
    currency_id: String,
    /// 薪资方案 ID（必填）
    plan_id: String,
    /// 薪资方案 TID（必填）
    plan_tid: String,
    /// 调薪原因 ID（必填）
    change_reason_id: String,
    /// 薪资项值集合（必填，仅填写方案中可编辑的薪资项）
    item_value_lists: Vec<ArchiveItemValue>,
    /// 调薪说明（可选，不超过 1000 字符）
    description: Option<String>,
    /// 更正说明（可选，不超过 1000 字符，更正档案时填写）
    edit_remark: Option<String>,
    /// 用户 ID 类型（查询参数：open_id/union_id/user_id/people_corehr_id，默认 open_id）
    user_id_type: Option<String>,
    /// 配置信息
    config: Config,
}

impl CreateRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            unique_id: String::new(),
            operator_id: None,
            user_id: String::new(),
            effective_time: String::new(),
            currency_id: String::new(),
            plan_id: String::new(),
            plan_tid: String::new(),
            change_reason_id: String::new(),
            item_value_lists: Vec::new(),
            description: None,
            edit_remark: None,
            user_id_type: None,
            config,
        }
    }

    /// 设置外部幂等 id（必填，标准 UUID V4 格式）
    pub fn unique_id(mut self, unique_id: String) -> Self {
        self.unique_id = unique_id;
        self
    }

    /// 设置操作人 ID（可选；应用身份鉴权时不能为空）
    pub fn operator_id(mut self, operator_id: String) -> Self {
        self.operator_id = Some(operator_id);
        self
    }

    /// 设置员工 id（必填，具体类型由 user_id_type 指定）
    pub fn user_id(mut self, user_id: String) -> Self {
        self.user_id = user_id;
        self
    }

    /// 设置生效时间（必填，格式 YYYY-MM-DD）
    pub fn effective_time(mut self, effective_time: String) -> Self {
        self.effective_time = effective_time;
        self
    }

    /// 设置币种 ID（必填）
    pub fn currency_id(mut self, currency_id: String) -> Self {
        self.currency_id = currency_id;
        self
    }

    /// 设置薪资方案 ID（必填）
    pub fn plan_id(mut self, plan_id: String) -> Self {
        self.plan_id = plan_id;
        self
    }

    /// 设置薪资方案 TID（必填）
    pub fn plan_tid(mut self, plan_tid: String) -> Self {
        self.plan_tid = plan_tid;
        self
    }

    /// 设置调薪原因 ID（必填）
    pub fn change_reason_id(mut self, change_reason_id: String) -> Self {
        self.change_reason_id = change_reason_id;
        self
    }

    /// 设置薪资项值集合（必填，仅填写方案中可编辑的薪资项）
    pub fn item_value_lists(mut self, item_value_lists: Vec<ArchiveItemValue>) -> Self {
        self.item_value_lists = item_value_lists;
        self
    }

    /// 设置调薪说明（可选，不超过 1000 字符）
    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// 设置更正说明（可选，不超过 1000 字符）
    pub fn edit_remark(mut self, edit_remark: String) -> Self {
        self.edit_remark = Some(edit_remark);
        self
    }

    /// 设置用户 ID 类型（open_id/union_id/user_id/people_corehr_id，默认 open_id）
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateResponse> {
        use crate::common::api_endpoints::CompensationApiV1;

        // 1. 验证必填字段
        validate_required!(self.unique_id, "unique_id 不能为空");
        validate_required!(self.user_id, "user_id 不能为空");
        validate_required!(self.effective_time, "effective_time 不能为空");
        validate_required!(self.currency_id, "currency_id 不能为空");
        validate_required!(self.plan_id, "plan_id 不能为空");
        validate_required!(self.plan_tid, "plan_tid 不能为空");
        validate_required!(self.change_reason_id, "change_reason_id 不能为空");
        validate_required!(self.item_value_lists, "item_value_lists 不能为空");

        // 2. 构建端点与查询参数
        let api_endpoint = CompensationApiV1::ArchiveCreate;
        let mut request = ApiRequest::<CreateResponse>::post(api_endpoint.to_url());
        if let Some(ref user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        // 3. 构建请求体
        let request_body = CreateRequestBody {
            unique_id: self.unique_id,
            operator_id: self.operator_id,
            user_id: self.user_id,
            effective_time: self.effective_time,
            currency_id: self.currency_id,
            plan_id: self.plan_id,
            plan_tid: self.plan_tid,
            change_reason_id: self.change_reason_id,
            item_value_lists: self.item_value_lists,
            description: self.description,
            edit_remark: self.edit_remark,
        };
        let request_body_json = serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "构建请求体失败",
                format!("序列化请求体失败: {e}"),
            )
        })?;
        let request = request.body(request_body_json);

        // 4. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "创建薪资档案响应数据为空",
        )
        .await
    }
}

/// 创建薪资档案请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRequestBody {
    /// 外部幂等 id，标准 UUID V4 格式
    pub unique_id: String,
    /// 操作人 ID，具体类型由入参中的 user_id_type 指定
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<String>,
    /// 员工 id，具体类型由入参中的 user_id_type 指定
    pub user_id: String,
    /// 生效时间，格式 YYYY-MM-DD
    pub effective_time: String,
    /// 币种 ID
    pub currency_id: String,
    /// 薪资方案 ID
    pub plan_id: String,
    /// 薪资方案 TID
    pub plan_tid: String,
    /// 调薪原因 ID
    pub change_reason_id: String,
    /// 薪资项值集合（仅填写方案中可编辑的薪资项）
    pub item_value_lists: Vec<ArchiveItemValue>,
    /// 调薪说明（不超过 1000 字符）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 更正说明（不超过 1000 字符）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edit_remark: Option<String>,
}

/// 薪资项值（官方 archive_item_value）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveItemValue {
    /// 薪资项 ID
    pub item_id: String,
    /// 薪资项的值，字符串表达的数字（1～18 字符，不支持负数）
    pub item_value: String,
    /// 员工转正后薪资项的值（可选，方案开启试用期且员工处于试用期时填写）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_value_regular: Option<String>,
}

/// 创建薪资档案响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateResponse {
    /// 定调薪任务创建的唯一 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
    /// 薪资档案的 TID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub archive_tid: Option<String>,
}

impl ApiResponseTrait for CreateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }

    /// 官方成功响应可能无 `data`；全 Option 字段时声明空成功（非 `{}` 探测）。
    fn empty_success() -> Option<Self> {
        Some(Self {
            unique_id: None,
            archive_tid: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;

    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_create_archive_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{body_json, method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        // 官方响应示例形状：data 含 unique_id + archive_tid
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{"unique_id": "123e4567-e89b-42d3-a456-426614174000", "archive_tid": "7434007780111336970"}"#,
        )
        .unwrap();
        Mock::given(method("POST"))
            .and(path("/open-apis/compensation/v1/archives"))
            .and(query_param("user_id_type", "open_id"))
            .and(body_json(json!({
                "unique_id": "123e4567-e89b-42d3-a456-426614174000",
                "operator_id": "7337149697626801708",
                "user_id": "7337149697626801708",
                "effective_time": "2024-11-12",
                "currency_id": "6863329932261459464",
                "plan_id": "7431430313074247212",
                "plan_tid": "7431430313074279980",
                "change_reason_id": "7125907336899888684",
                "item_value_lists": [{
                    "item_id": "7244131355509917228",
                    "item_value": "200.00",
                    "item_value_regular": "600.00"
                }],
                "description": "因2024年Q2绩效优秀，对该同学调薪10%",
                "edit_remark": "更正2024年Q2绩效调薪金额"
            })))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": data_body
            })))
            .mount(&server)
            .await;

        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url(server.uri())
            .enable_token_cache(false)
            .build();

        let data = CreateRequest::new(config)
            .unique_id("123e4567-e89b-42d3-a456-426614174000".to_string())
            .operator_id("7337149697626801708".to_string())
            .user_id("7337149697626801708".to_string())
            .effective_time("2024-11-12".to_string())
            .currency_id("6863329932261459464".to_string())
            .plan_id("7431430313074247212".to_string())
            .plan_tid("7431430313074279980".to_string())
            .change_reason_id("7125907336899888684".to_string())
            .item_value_lists(vec![ArchiveItemValue {
                item_id: "7244131355509917228".to_string(),
                item_value: "200.00".to_string(),
                item_value_regular: Some("600.00".to_string()),
            }])
            .description("因2024年Q2绩效优秀，对该同学调薪10%".to_string())
            .edit_remark("更正2024年Q2绩效调薪金额".to_string())
            .user_id_type("open_id".to_string())
            .execute()
            .await
            .expect("创建薪资档案应成功");

        assert_eq!(
            data.unique_id.as_deref(),
            Some("123e4567-e89b-42d3-a456-426614174000")
        );
        assert_eq!(data.archive_tid.as_deref(), Some("7434007780111336970"));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/compensation/v1/archives"
        );
    }

    /// unique_id / user_id / effective_time 等为必填
    #[tokio::test]
    async fn test_create_archive_validates_required_fields() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url("https://127.0.0.1:9")
            .enable_token_cache(false)
            .build();

        // 未设置任何必填字段
        let result = CreateRequest::new(config.clone()).execute().await;
        assert!(result.is_err());

        // 缺 item_value_lists
        let result = CreateRequest::new(config)
            .unique_id("123e4567-e89b-42d3-a456-426614174000".to_string())
            .user_id("7337149697626801708".to_string())
            .effective_time("2024-11-12".to_string())
            .currency_id("6863329932261459464".to_string())
            .plan_id("7431430313074247212".to_string())
            .plan_tid("7431430313074279980".to_string())
            .change_reason_id("7125907336899888684".to_string())
            .execute()
            .await;
        assert!(result.is_err());
    }
}
