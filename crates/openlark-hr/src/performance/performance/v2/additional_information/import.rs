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
    /// 评估周期 ID（必填，1~100 字符，可通过获取周期列表接口获取）
    semester_id: String,
    /// 幂等标记（必填 query 参数 client_token，同一请求多次调用将被拦截，0~64 字符）
    client_token: String,
    /// 补充信息列表（可选，1~1000 条）
    additional_informations: Vec<AdditionalInformation>,
    /// 补充信息导入记录名称（可选，默认 "API导入"，0~200 字符）
    import_record_name: Option<String>,
    /// 用户 ID 类型（可选 query 参数，默认 open_id）
    user_id_type: Option<String>,
    /// 配置信息
    config: Config,
}

impl ImportRequest {
    /// 创建请求
    pub fn new(config: Config, semester_id: String, client_token: String) -> Self {
        Self {
            semester_id,
            client_token,
            additional_informations: Vec::new(),
            import_record_name: None,
            user_id_type: None,
            config,
        }
    }

    /// 追加一条补充信息
    pub fn add_information(mut self, information: AdditionalInformation) -> Self {
        self.additional_informations.push(information);
        self
    }

    /// 设置补充信息导入记录名称（可选）
    pub fn import_record_name(mut self, import_record_name: String) -> Self {
        self.import_record_name = Some(import_record_name);
        self
    }

    /// 设置用户 ID 类型（可选 query 参数，open_id/union_id/user_id）
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
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
        validate_required!(self.semester_id.trim(), "semester_id");
        validate_required!(self.client_token.trim(), "client_token");

        // 2. 构建端点
        let api_endpoint = PerformanceApiV1::AdditionalInformationImport;
        let request = ApiRequest::<ImportResponse>::post(api_endpoint.to_url())
            .query("client_token", self.client_token)
            .query_opt("user_id_type", self.user_id_type);

        // 3. 构建请求体
        let request_body = ImportRequestBody {
            semester_id: self.semester_id,
            additional_informations: self.additional_informations,
            import_record_name: self.import_record_name,
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
            "批量导入补充信息响应数据为空",
        )
        .await
    }
}

/// 导入请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportRequestBody {
    /// 评估周期 ID
    pub semester_id: String,
    /// 补充信息列表
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub additional_informations: Vec<AdditionalInformation>,
    /// 补充信息导入记录名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_record_name: Option<String>,
}

/// 补充信息（additional_information）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdditionalInformation {
    /// 补充信息 ID（可选，传入已有 ID 时更新对应数据）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    /// 外部系统补充信息 ID（可选，用于系统间数据映射）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// 被评估人 ID（必填，与入参 user_id_type 类型一致）
    pub reviewee_user_id: String,
    /// 事项（必填）
    pub item: String,
    /// 时间（必填，文本内容无格式校验）
    pub time: String,
    /// 具体描述（必填）
    pub detailed_description: String,
}

/// 批量导入补充信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImportResponse {
    /// 导入记录 ID
    pub import_record_id: String,
    /// 成功导入后的补充信息列表
    #[serde(default)]
    pub additional_informations: Vec<AdditionalInformation>,
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
        use wiremock::matchers::{body_json, method, path, query_param};
        use wiremock::{Mock, ResponseTemplate};

        let server = MockServer::start().await;
        let data_body: serde_json::Value = serde_json::from_str(
            r#"{"import_record_id": "7350194523185610771", "additional_informations": [{"item_id": "7350195758357807123", "external_id": "6789523104723558912", "reviewee_user_id": "ou_3245842393d09e9428ad4655da6e30b3", "item": "业绩补充说明", "time": "2024-03-12", "detailed_description": "销售额增长目标超额完成"}]}"#,
        )
        .unwrap();
        Mock::given(method("POST"))
            .and(path(
                "/open-apis/performance/v2/additional_informations/import",
            ))
            .and(query_param("client_token", "12454646"))
            .and(body_json(json!({
                "semester_id": "7348736302176534547",
                "additional_informations": [{
                    "item_id": "7350194523185610771",
                    "external_id": "6789523104723558912",
                    "reviewee_user_id": "ou_3245842393d09e9428ad4655da6e30b3",
                    "item": "业绩补充说明",
                    "time": "2024-03-12",
                    "detailed_description": "销售额增长目标超额完成"
                }],
                "import_record_name": "人工导入"
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

        let data = ImportRequest::new(
            config,
            "7348736302176534547".to_string(),
            "12454646".to_string(),
        )
        .add_information(AdditionalInformation {
            item_id: Some("7350194523185610771".to_string()),
            external_id: Some("6789523104723558912".to_string()),
            reviewee_user_id: "ou_3245842393d09e9428ad4655da6e30b3".to_string(),
            item: "业绩补充说明".to_string(),
            time: "2024-03-12".to_string(),
            detailed_description: "销售额增长目标超额完成".to_string(),
        })
        .import_record_name("人工导入".to_string())
        .execute()
        .await
        .expect("performance_v2_additional_information_import 应成功");

        assert_eq!(data.import_record_id, "7350194523185610771");
        assert_eq!(data.additional_informations.len(), 1);

        let received = server.received_requests().await.unwrap_or_default();
        assert_eq!(received.len(), 1);
        assert_eq!(
            received[0].url.path(),
            "/open-apis/performance/v2/additional_informations/import"
        );
    }
}
