//! 批量导入补充信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/performance-v2/additional_information/import>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 批量导入补充信息请求
#[derive(Debug, Clone)]
pub struct ImportRequest {
    /// 绩效周期 ID（必填）
    cycle_id: String,
    /// 补充信息列表（必填）
    additional_informations: Vec<AdditionalInformation>,
    /// 配置信息
    config: Config,
}

impl ImportRequest {
    /// 创建请求
    pub fn new(config: Config, cycle_id: String) -> Self {
        Self {
            cycle_id,
            additional_informations: Vec::new(),
            config,
        }
    }

    /// 添加补充信息
    pub fn add_information(mut self, user_id: String, content: String) -> Self {
        self.additional_informations
            .push(AdditionalInformation { user_id, content });
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ImportResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ImportResponse> {
        use crate::common::api_endpoints::PerformanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.cycle_id.trim(), "cycle_id");

        // 2. 构建端点
        let api_endpoint = PerformanceApiV1::AdditionalInformationImport;
        let request = ApiRequest::<ImportResponse>::post(api_endpoint.to_url());

        // 3. 构建请求体
        let request_body = ImportRequestBody {
            cycle_id: self.cycle_id,
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
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 5. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "批量导入补充信息响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 导入请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportRequestBody {
    /// 绩效周期 ID
    pub cycle_id: String,
    /// 补充信息列表
    pub additional_informations: Vec<AdditionalInformation>,
}

/// 补充信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AdditionalInformation {
    /// 用户 ID
    pub user_id: String,
    /// 补充内容
    pub content: String,
}

/// 批量导入补充信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImportResponse {
    /// 成功导入的条数
    pub success_count: i32,
    /// 失败的条数
    pub failed_count: i32,
}

impl ApiResponseTrait for ImportResponse {
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
    async fn test_performance_v2_additional_information_import_returns_data_on_success() {
        use serde_json::json;
        use wiremock::MockServer;
        use wiremock::matchers::{method, path};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value =
            serde_json::from_str(r#"{"success_count": 0, "failed_count": 0}"#).unwrap();
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/performance/v2/additional_informations/import",
            ))
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

        let data = ImportRequest::new(config, "cycle_001".to_string())
            .execute()
            .await
            .expect("performance_v2_additional_information_import 应成功");

        let _ = data.success_count;
        let _ = data.failed_count;

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/performance/v2/additional_informations/import"
        );
    }
}
