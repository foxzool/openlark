//! 批量查询补充信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/performance-v2/additional_information/query>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 批量查询补充信息请求
#[derive(Debug, Clone)]
pub struct QueryRequest {
    /// 评估周期 ID（必填，1~100 字符，可通过获取周期列表接口获取）
    semester_id: String,
    /// 补充信息 ID 列表（可选，最多 50 个）
    item_ids: Option<Vec<String>>,
    /// 外部系统补充信息 ID 列表（可选，最多 50 个）
    external_ids: Option<Vec<String>>,
    /// 被评估人 ID 列表（可选，最多 50 个，与入参 user_id_type 类型一致）
    reviewee_user_ids: Option<Vec<String>>,
    /// 用户 ID 类型（可选 query 参数，默认 open_id）
    user_id_type: Option<String>,
    /// 分页标记（可选 query 参数）
    page_token: Option<String>,
    /// 分页大小（可选 query 参数，0~50，默认 20）
    page_size: Option<i32>,
    /// 配置信息
    config: Config,
}

impl QueryRequest {
    /// 创建请求
    pub fn new(config: Config, semester_id: String) -> Self {
        Self {
            semester_id,
            item_ids: None,
            external_ids: None,
            reviewee_user_ids: None,
            user_id_type: None,
            page_token: None,
            page_size: None,
            config,
        }
    }

    /// 设置补充信息 ID 列表（可选，筛选优先级 item_ids > external_ids > reviewee_user_ids）
    pub fn item_ids(mut self, item_ids: Vec<String>) -> Self {
        self.item_ids = Some(item_ids);
        self
    }

    /// 设置外部系统补充信息 ID 列表（可选）
    pub fn external_ids(mut self, external_ids: Vec<String>) -> Self {
        self.external_ids = Some(external_ids);
        self
    }

    /// 追加一个被评估人 ID（可选）
    pub fn add_reviewee_user_id(mut self, reviewee_user_id: String) -> Self {
        self.reviewee_user_ids
            .get_or_insert_with(Vec::new)
            .push(reviewee_user_id);
        self
    }

    /// 设置用户 ID 类型（可选 query 参数，open_id/union_id/user_id）
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 设置分页标记（可选 query 参数）
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 设置分页大小（可选 query 参数，0~50）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<QueryResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<QueryResponse> {
        use crate::common::api_endpoints::PerformanceApiV1;

        validate_required!(self.semester_id.trim(), "semester_id");
        // 官方允许三个筛选列表为空（0~50），此处只校验上限
        for (field, ids) in [
            ("item_ids", &self.item_ids),
            ("external_ids", &self.external_ids),
            ("reviewee_user_ids", &self.reviewee_user_ids),
        ] {
            if let Some(ids) = ids
                && ids.len() > 50
            {
                return Err(openlark_core::error::validation_error(
                    field,
                    format!("{field} 不能超过 50 个"),
                ));
            }
        }

        // 1. 构建端点
        let api_endpoint = PerformanceApiV1::AdditionalInformationQuery;
        let request = ApiRequest::<QueryResponse>::post(api_endpoint.to_url())
            .query_opt("user_id_type", self.user_id_type)
            .query_opt("page_token", self.page_token)
            .query_opt("page_size", self.page_size.map(|v| v.to_string()));

        // 2. 构建请求体
        let request_body = QueryRequestBody {
            semester_id: self.semester_id,
            item_ids: self.item_ids,
            external_ids: self.external_ids,
            reviewee_user_ids: self.reviewee_user_ids,
        };
        let request_body_json = serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?;
        let request = request.body(request_body_json);

        // 3. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "批量查询补充信息响应数据为空",
        )
        .await
    }
}

/// 查询请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequestBody {
    /// 评估周期 ID
    pub semester_id: String,
    /// 补充信息 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_ids: Option<Vec<String>>,
    /// 外部系统补充信息 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ids: Option<Vec<String>>,
    /// 被评估人 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reviewee_user_ids: Option<Vec<String>>,
}

/// 批量查询补充信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponse {
    /// 补充信息列表
    #[serde(default)]
    pub additional_informations: Vec<AdditionalInformation>,
    /// 是否还有更多数据
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 补充信息（additional_information）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdditionalInformation {
    /// 补充信息 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    /// 外部系统补充信息 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// 被评估人 ID
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reviewee_user_id: Option<String>,
    /// 事项
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub item: Option<String>,
    /// 时间
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<String>,
    /// 具体描述
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detailed_description: Option<String>,
}

impl ApiResponseTrait for QueryResponse {
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
    async fn test_performance_v2_additional_information_query_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{body_json, method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{"additional_informations": [{"item_id": "7350195758357807123", "external_id": "6789523104723558912", "reviewee_user_id": "ou_3245842393d09e9428ad4655da6e30b3", "item": "业绩补充说明", "time": "2024-03-12", "detailed_description": "销售额增长目标超额完成"}], "has_more": true, "page_token": "eVQr"}"#,
        )
        .unwrap();
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/performance/v2/additional_informations/query",
            ))
            .and(body_json(json!({
                "semester_id": "7348736302176534547",
                "reviewee_user_ids": ["ou_3245842393d09e9428ad4655da6e30b3"]
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

        let data = QueryRequest::new(config, "7348736302176534547".to_string())
            .add_reviewee_user_id("ou_3245842393d09e9428ad4655da6e30b3".to_string())
            .execute()
            .await
            .expect("performance_v2_additional_information_query 应成功");

        assert_eq!(data.additional_informations.len(), 1);
        assert_eq!(
            data.additional_informations[0].item_id.as_deref(),
            Some("7350195758357807123")
        );
        assert_eq!(data.has_more, Some(true));

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/performance/v2/additional_informations/query"
        );
    }
}
