//! 批量获取 OKR
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v1/okr/batch_get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required_list,
};
use serde::{Deserialize, Serialize};

/// 批量获取 OKR 请求
#[derive(Debug, Clone)]
pub struct BatchGetRequest {
    /// OKR ID 列表（必填，最多 10 个，query 参数 okr_ids）
    okr_ids: Vec<String>,
    /// 用户 ID 类型（可选，默认 open_id）
    user_id_type: Option<String>,
    /// 请求 OKR 的语言版本（可选，zh_cn/en_us，默认 zh_cn）
    lang: Option<String>,
    /// 配置信息
    config: Config,
}

impl BatchGetRequest {
    /// 创建请求
    pub fn new(config: Config, okr_ids: Vec<String>) -> Self {
        Self {
            okr_ids,
            user_id_type: None,
            lang: None,
            config,
        }
    }

    /// 设置用户 ID 类型（可选，open_id/union_id/user_id）
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 设置语言版本（可选，zh_cn/en_us）
    pub fn lang(mut self, lang: String) -> Self {
        self.lang = Some(lang);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<BatchGetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchGetResponse> {
        use crate::common::api_endpoints::OkrApiV1;

        // 1. 验证必填字段（官方 Maximum length: 10）
        validate_required_list!(self.okr_ids, 10, "OKR ID 列表不能为空且不能超过 10 个");

        // 2. 构建端点（GET，参数全部走 query string，无请求体）
        let api_endpoint = OkrApiV1::OkrBatchGet;
        let request = ApiRequest::<BatchGetResponse>::get(api_endpoint.to_url())
            // 官方类型为 string[]，与 ehr/employee/list 相同以逗号拼接传递
            .query("okr_ids", self.okr_ids.join(","))
            .query_opt("user_id_type", self.user_id_type)
            .query_opt("lang", self.lang);

        // 3. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "批量获取 OKR 响应数据为空",
        )
        .await
    }
}

/// 批量获取 OKR 响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchGetResponse {
    /// OKR 列表
    #[serde(default)]
    pub okr_list: Vec<OkrBatch>,
}

/// OKR 汇总信息（okr_batch）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OkrBatch {
    /// OKR ID
    pub id: String,
    /// OKR 的访问权限（0：只返回 id；1：返回其他具体字段）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<i32>,
    /// 周期 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period_id: Option<String>,
    /// OKR 名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Objective 列表
    #[serde(default)]
    pub objective_list: Vec<OkrObjective>,
    /// OKR 确认状态（0 初始/1 待提交/2 待确认/3 已拒绝/4 已通过）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirm_status: Option<i32>,
}

/// Objective 信息（okr_objective）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OkrObjective {
    /// Objective ID
    pub id: String,
    /// 权限（0：只返回 id；1：返回其他具体字段）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub permission: Option<i32>,
    /// Objective 内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// Objective 备注内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_report: Option<String>,
    /// Objective 分数（0 - 100）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<i32>,
    /// Objective 的权重（0 - 100）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<f64>,
    /// Objective 进度
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_rate: Option<OkrProgressRate>,
    /// Objective KeyResult 列表
    #[serde(default)]
    pub kr_list: Vec<OkrKeyResult>,
    /// 该 Objective 的进度列表
    #[serde(default)]
    pub progress_record_list: Vec<ProgressRecordSimplify>,
    /// 该 Objective 提到的人员列表
    #[serde(default)]
    pub mentioned_user_list: Vec<OkrUser>,
    /// 对齐到该 Objective 的 Objective 列表
    #[serde(default)]
    pub aligned_objective_list: Vec<AlignedObjective>,
    /// 该 Objective 对齐到的 Objective 列表
    #[serde(default)]
    pub aligning_objective_list: Vec<AlignedObjective>,
    /// 截止时间（毫秒）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deadline: Option<String>,
    /// 最后一次进度百分比更新时间（毫秒）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_rate_percent_last_updated_time: Option<String>,
    /// 最后一次状态更新时间（毫秒）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_rate_status_last_updated_time: Option<String>,
    /// 最后一次新增或编辑进展的时间（毫秒）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_record_last_updated_time: Option<String>,
    /// 最后一次编辑备注的时间（毫秒）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_report_last_updated_time: Option<String>,
    /// 最后一次打分更新时间（毫秒）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score_last_updated_time: Option<String>,
}

/// Key Result 信息（okr_objective_kr）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OkrKeyResult {
    /// Key Result ID
    pub id: String,
    /// KeyResult 内容
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// KeyResult 打分（0 - 100）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score: Option<i32>,
    /// KeyResult 权重（0 - 100，已废弃）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    /// KeyResult 的权重（0 - 100）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kr_weight: Option<f64>,
    /// KR 进度
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_rate: Option<OkrProgressRate>,
    /// 该 KR 的进度列表
    #[serde(default)]
    pub progress_record_list: Vec<ProgressRecordSimplify>,
    /// 该 KR 提到的人员列表
    #[serde(default)]
    pub mentioned_user_list: Vec<OkrUser>,
    /// 截止时间（毫秒）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deadline: Option<String>,
    /// 最后一次进度百分比更新时间（毫秒）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_rate_percent_last_updated_time: Option<String>,
    /// 最后一次状态更新时间（毫秒）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_rate_status_last_updated_time: Option<String>,
    /// 最后一次新增或编辑进展的时间（毫秒）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_record_last_updated_time: Option<String>,
    /// 最后一次编辑备注的时间（毫秒）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_report_last_updated_time: Option<String>,
    /// 最后一次打分更新时间（毫秒）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub score_last_updated_time: Option<String>,
}

/// Objective/KR 进度（okr_objective_progress_rate）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OkrProgressRate {
    /// 进度百分比（>= 0）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percent: Option<i32>,
    /// 进度状态（"-1" 暂无/"0" 正常/"1" 风险/"2" 延期）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// 进展记录简要信息（progress_record_simplify）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProgressRecordSimplify {
    /// OKR 进展记录 ID
    pub id: String,
}

/// 用户信息（okr_objective_aligned_objective_owner）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OkrUser {
    /// 用户的 open_id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    /// 用户的 user_id
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

/// 对齐的 Objective（okr_objective_aligned_objective）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AlignedObjective {
    /// Objective 的 ID
    pub id: String,
    /// OKR 的 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub okr_id: Option<String>,
    /// 该 Objective 的 Owner
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<OkrUser>,
}

impl ApiResponseTrait for BatchGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::config::Config;

    /// 端到端：Builder→execute→Transport→mock→assert 响应解析 + 实际请求形状。
    #[tokio::test]
    async fn test_batch_get_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{"okr_list": [{"id": "okr_1", "name": "目标1", "period_id": "period_1"}]}"#,
        )
        .unwrap();
        Mock::given(method("GET"))
            .and(path("/open-apis/okr/v1/okrs/batch_get"))
            .and(query_param("okr_ids", "id_001,id_002"))
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

        let data = BatchGetRequest::new(config, vec!["id_001".to_string(), "id_002".to_string()])
            .lang("zh_cn".to_string())
            .execute()
            .await
            .expect("batch_get 应成功");

        assert_eq!(data.okr_list.len(), 1);
        assert_eq!(data.okr_list[0].id, "okr_1");

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(received[0].url.path(), "/open-apis/okr/v1/okrs/batch_get");
        // 官方无请求体：okr_ids 必须出现在 query string 中
        assert!(received[0].url.query().unwrap_or("").contains("okr_ids="));
        assert!(received[0].body.is_empty());
    }
}
