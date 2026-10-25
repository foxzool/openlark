//! 获取员工薪资标准
//!
//! docPath: <https://open.feishu.cn/document/server-docs/corehr-v1/compensation_standard/match>

use openlark_core::{
    SDKResult,
    api::ApiRequest,
    api::{ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 获取员工薪资标准请求
#[derive(Debug, Clone)]
pub struct MatchRequest {
    config: Config,
    body: Option<Value>,
    query_params: Vec<(String, String)>,
}

impl MatchRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            body: None,
            query_params: Vec::new(),
        }
    }

    /// 设置请求体。
    pub fn body(mut self, body: Value) -> Self {
        self.body = Some(body);
        self
    }

    /// 追加查询参数。
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<MatchResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<MatchResponse> {
        use crate::common::api_endpoints::FeishuPeopleApiV1;

        let api_endpoint = FeishuPeopleApiV1::CompensationStandardMatch;
        let mut request = ApiRequest::<MatchResponse>::get(api_endpoint.to_url());
        for (key, value) in self.query_params {
            request = request.query(&key, value);
        }
        if let Some(body) = self.body {
            request = request.body(body);
        }

        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "获取员工薪资标准响应数据为空",
        )
        .await
    }
}

/// 获取员工薪资标准响应
///
/// 对应官方响应体 `data` 节点（`cpst_match_item`，`ResponseFormat::Data`
/// 下直接反序列化）。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MatchResponse {
    /// 薪资标准表 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub standard_id: Option<String>,
    /// 薪资等级
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grade: Option<CpstGrade>,
    /// 生效时间（毫秒级时间戳）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub effective_time: Option<String>,
    /// 透传的扩展字段
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, Value>,
}

/// 薪资等级
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CpstGrade {
    /// 薪资等级 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grade_id: Option<String>,
    /// 薪资等级时间轴 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grade_tid: Option<String>,
    /// 带宽上下限和标准值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grade_standard_value: Option<CpstGradeStandardValue>,
    /// 币种
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency: Option<CpstCurrency>,
    /// 薪资标准描述
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<crate::common::shared_models::I18nText>,
}

/// 带宽上下限和标准值
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CpstGradeStandardValue {
    /// 薪资标准的关联对象，项目或者指标
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_object: Option<CpstReferenceObject>,
    /// 薪资标准类型
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub standard_type: Option<CpstStandardType>,
    /// 上下限
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub band_width: Option<CpstBandWidth>,
    /// 标准值
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub standard_value: Option<String>,
}

/// 薪资标准的关联对象（项目或指标）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CpstReferenceObject {
    /// cpst_item（项目）/ cpst_indicator（指标）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_name: Option<String>,
    /// 值列表，例如薪资项和薪资统计指标 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// 薪资标准类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CpstStandardType {
    /// standard_value（标准值）/ bandwidth_and_standard_value（上下限和标准值）/ bandwidth_upper_and_lower_limit（上下限）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_name: Option<String>,
}

/// 上下限
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CpstBandWidth {
    /// 上限
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upper_limit: Option<String>,
    /// 下限
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lower_limit: Option<String>,
}

/// 币种
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CpstCurrency {
    /// 币种 ID，可通过【查询货币信息】接口获取
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub currency_id: Option<String>,
    /// 币种 code
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// 币种名称
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<crate::common::shared_models::I18nText>,
}

impl ApiResponseTrait for MatchResponse {
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

    /// 端到端：GET /open-apis/corehr/v1/compensation_standards/match
    #[tokio::test]
    async fn test_match_returns_data_on_success() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("/open-apis/corehr/v1/compensation_standards/match"))
            .respond_with(ResponseTemplate::new(200).set_body_json(json!({
                "code": 0,
                "msg": "success",
                "data": {
                    "effective_time": "1660924800000",
                    "grade": {
                        "currency": {
                            "code": "CNY",
                            "currency_id": "6863329932261459464",
                            "name": {"en_us": "Renminbi (Chinese) yuan", "zh_cn": "人民币元"}
                        },
                        "grade_id": "7174758360888215084",
                        "grade_standard_value": {
                            "band_width": {"lower_limit": "5000", "upper_limit": "10000"},
                            "reference_object": {"api_name": "cpst_item", "id": "7156853394442044972"},
                            "standard_type": {"api_name": "bandwidth_and_standard_value"},
                            "standard_value": "8888"
                        },
                        "grade_tid": "7174758360888247852"
                    },
                    "standard_id": "7174758593538295340"
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

        let resp = MatchRequest::new(config)
            .execute()
            .await
            .expect("请求应成功");

        assert_eq!(resp.standard_id.as_deref(), Some("7174758593538295340"));
        assert_eq!(resp.effective_time.as_deref(), Some("1660924800000"));
        let grade = resp.grade.as_ref().expect("grade 应解析");
        assert_eq!(grade.grade_id.as_deref(), Some("7174758360888215084"));
        assert_eq!(
            grade
                .currency
                .as_ref()
                .and_then(|c| c.currency_id.as_deref()),
            Some("6863329932261459464")
        );
        let standard_value = grade
            .grade_standard_value
            .as_ref()
            .expect("grade_standard_value 应解析");
        assert_eq!(standard_value.standard_value.as_deref(), Some("8888"));
        assert_eq!(
            standard_value
                .band_width
                .as_ref()
                .and_then(|b| b.upper_limit.as_deref()),
            Some("10000")
        );
        assert_eq!(
            standard_value
                .reference_object
                .as_ref()
                .and_then(|r| r.api_name.as_deref()),
            Some("cpst_item")
        );

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
    }
}
