//! 录入被评估人关键指标数据
//!
//! docPath: <https://open.feishu.cn/document/server-docs/performance-v2/metric_detail/import>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 录入被评估人关键指标数据请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ImportRequest {
    /// 绩效周期 ID（必填）
    cycle_id: String,
    /// 指标数据列表（必填）
    metric_details: Vec<MetricDetail>,
    /// 配置信息
    config: Config,
}

impl ImportRequest {
    /// 创建请求
    pub fn new(config: Config, cycle_id: String) -> Self {
        Self {
            cycle_id,
            metric_details: Vec::new(),
            config,
        }
    }

    /// 添加指标数据
    pub fn add_metric_detail(mut self, user_id: String, metric_id: String, value: f64) -> Self {
        self.metric_details.push(MetricDetail {
            user_id,
            metric_id,
            value,
        });
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
        let api_endpoint = PerformanceApiV1::MetricDetailImport;
        let request = ApiRequest::<ImportResponse>::post(api_endpoint.to_url());

        // 3. 构建请求体
        let request_body = ImportRequestBody {
            cycle_id: self.cycle_id,
            metric_details: self.metric_details,
        };
        let request_body_json = serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {}", e),
            )
        })?;
        let request = request.body(request_body_json);

        // 4. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 5. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "录入关键指标数据响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 录入请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportRequestBody {
    /// 绩效周期 ID
    pub cycle_id: String,
    /// 指标数据列表
    pub metric_details: Vec<MetricDetail>,
}

/// 指标数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricDetail {
    /// 用户 ID
    pub user_id: String,
    /// 指标 ID
    pub metric_id: String,
    /// 指标值
    pub value: f64,
}

/// 录入被评估人关键指标数据响应
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
    use serde_json::json;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build()
    }

    #[test]
    fn test_metric_detail_import_request_builder() {
        let request = ImportRequest::new(create_test_config(), "cycle_1".to_string())
            .add_metric_detail("u_1".to_string(), "m_1".to_string(), 98.5);

        assert_eq!(request.cycle_id, "cycle_1");
        assert_eq!(request.metric_details.len(), 1);
        assert_eq!(request.metric_details[0].value, 98.5);
    }

    #[test]
    fn test_metric_detail_import_request_body_serialize() {
        let body = ImportRequestBody {
            cycle_id: "cycle_2".to_string(),
            metric_details: vec![MetricDetail {
                user_id: "u_2".to_string(),
                metric_id: "m_2".to_string(),
                value: 88.0,
            }],
        };

        let value = serde_json::to_value(body).expect("序列化请求体失败");
        assert_eq!(value["cycle_id"], json!("cycle_2"));
        assert_eq!(value["metric_details"][0]["metric_id"], json!("m_2"));
        assert_eq!(value["metric_details"][0]["value"], json!(88.0));
    }

    #[test]
    fn test_metric_detail_import_response_deserialize() {
        let value = json!({"success_count": 12, "failed_count": 1});
        let response: ImportResponse = serde_json::from_value(value).expect("反序列化响应失败");
        assert_eq!(response.success_count, 12);
        assert_eq!(response.failed_count, 1);
    }

    #[test]
    fn test_metric_detail_import_validation() {
        let request = ImportRequest::new(create_test_config(), "   ".to_string());
        let result: SDKResult<()> = (|| {
            validate_required!(request.cycle_id.trim(), "cycle_id");
            Ok(())
        })();
        assert!(result.is_err());
    }
}
