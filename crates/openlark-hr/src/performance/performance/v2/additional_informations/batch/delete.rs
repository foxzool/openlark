//! 批量删除补充信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/performance-v2/additional_informations.batch/delete>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, validate_required_list,
};
use serde::{Deserialize, Serialize};

/// 批量删除补充信息请求
#[derive(Debug, Clone)]
pub struct DeleteRequest {
    /// 评估周期 ID（必填，1~100 字符，可通过获取周期列表接口获取）
    semester_id: String,
    /// 补充信息 ID 列表（必填，1~100 条，可通过批量查询补充信息接口获取）
    additional_informations: Vec<String>,
    /// 用户 ID 类型（可选 query 参数，默认 open_id）
    user_id_type: Option<String>,
    /// 配置信息
    config: Config,
}

impl DeleteRequest {
    /// 创建请求
    pub fn new(config: Config, semester_id: String) -> Self {
        Self {
            semester_id,
            additional_informations: Vec::new(),
            user_id_type: None,
            config,
        }
    }

    /// 追加一条补充信息 ID
    pub fn add_additional_information(mut self, additional_information: String) -> Self {
        self.additional_informations.push(additional_information);
        self
    }

    /// 设置用户 ID 类型（可选 query 参数，open_id/union_id/user_id）
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DeleteResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteResponse> {
        use crate::common::api_endpoints::PerformanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.semester_id.trim(), "semester_id");
        validate_required_list!(
            self.additional_informations,
            100,
            "additional_informations 不能为空且不能超过 100 条"
        );

        // 2. 构建端点
        let api_endpoint = PerformanceApiV1::AdditionalInformationsBatchDelete;
        let request = ApiRequest::<DeleteResponse>::delete(api_endpoint.to_url())
            .query_opt("user_id_type", self.user_id_type);

        // 3. 构建请求体
        let request_body = DeleteRequestBody {
            semester_id: self.semester_id,
            additional_informations: self.additional_informations,
        };
        let request_body_json = serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {e}"),
            )
        })?;
        let request = request.body(request_body_json);

        // 4. 发送请求
        Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "批量删除补充信息响应数据为空",
        )
        .await
    }
}

/// 删除请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRequestBody {
    /// 评估周期 ID
    pub semester_id: String,
    /// 补充信息 ID 列表
    pub additional_informations: Vec<String>,
}

/// 批量删除补充信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteResponse {
    /// 被删除的补充信息 ID 列表
    #[serde(default)]
    pub additional_informations: Vec<String>,
}

impl ApiResponseTrait for DeleteResponse {
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
    async fn test_performance_v2_additional_informations_batch_delete_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{body_json, method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value =
            serde_json::from_str(r#"{"additional_informations": ["7350195758357807123"]}"#)
                .unwrap();
        Mock::given(method("DELETE"))
            .and(path(
                "/open-apis/performance/v2/additional_informations/batch",
            ))
            .and(body_json(json!({
                "semester_id": "7348736302176534547",
                "additional_informations": ["7350195758357807123"]
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

        let data = DeleteRequest::new(config, "7348736302176534547".to_string())
            .add_additional_information("7350195758357807123".to_string())
            .execute()
            .await
            .expect("performance_v2_additional_informations_batch_delete 应成功");

        assert_eq!(data.additional_informations, vec!["7350195758357807123"]);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/performance/v2/additional_informations/batch"
        );
    }

    #[tokio::test]
    async fn test_performance_v2_additional_informations_batch_delete_rejects_empty_ids() {
        let config = Config::builder()
            .app_id("ci_app_id")
            .app_secret("ci_app_secret")
            .base_url("https://127.0.0.1:9")
            .enable_token_cache(false)
            .build();

        let err = DeleteRequest::new(config, "7348736302176534547".to_string())
            .execute()
            .await
            .expect_err("空 additional_informations 应在发请求前校验失败");

        assert!(err.to_string().contains("additional_informations"));
    }
}
