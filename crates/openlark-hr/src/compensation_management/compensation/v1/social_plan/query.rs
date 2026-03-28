//! 根据方案ID和生效日期批量查询参保方案
//!
//! docPath: <https://open.feishu.cn/document/server-docs/compensation-v1/social_plan/query>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 根据方案ID和生效日期批量查询参保方案请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct QueryRequest {
    /// 方案 ID 列表（必填）
    plan_ids: Vec<String>,
    /// 生效日期（必填）
    effective_date: i64,
    /// 配置信息
    config: Config,
}

impl QueryRequest {
    /// 创建请求
    pub fn new(config: Config, plan_ids: Vec<String>, effective_date: i64) -> Self {
        Self {
            plan_ids,
            effective_date,
            config,
        }
    }

    fn validate(&self) -> SDKResult<()> {
        if self.plan_ids.is_empty() {
            return Err(openlark_core::error::validation_error(
                "plan_ids 不能为空",
                "至少需要提供一个参保方案 ID",
            ));
        }
        if self.effective_date <= 0 {
            return Err(openlark_core::error::validation_error(
                "effective_date 无效",
                "effective_date 必须为正整数时间戳",
            ));
        }
        Ok(())
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
        use crate::common::api_endpoints::CompensationApiV1;

        self.validate()?;

        // 1. 构建端点
        let api_endpoint = CompensationApiV1::SocialPlanQuery;
        let request = ApiRequest::<QueryResponse>::post(api_endpoint.to_url());

        // 2. 构建请求体
        let request_body = QueryRequestBody {
            plan_ids: self.plan_ids,
            effective_date: self.effective_date,
        };
        let request_body_json = serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {}", e),
            )
        })?;
        let request = request.body(request_body_json);

        // 3. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 4. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "查询参保方案响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 查询请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequestBody {
    /// 方案 ID 列表
    pub plan_ids: Vec<String>,
    /// 生效日期
    pub effective_date: i64,
}

/// 根据方案ID和生效日期批量查询参保方案响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryResponse {
    /// 参保方案列表
    pub items: Vec<SocialPlan>,
}

/// 参保方案
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SocialPlan {
    /// 方案 ID
    pub id: String,
    /// 方案名称
    pub name: String,
    /// 生效日期
    pub effective_date: i64,
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
    use serde_json::json;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build()
    }

    #[test]
    fn test_social_plan_query_request_builder() {
        let request = QueryRequest::new(
            create_test_config(),
            vec!["plan_1".to_string(), "plan_2".to_string()],
            1_706_000_000,
        );

        assert_eq!(request.plan_ids.len(), 2);
        assert_eq!(request.effective_date, 1_706_000_000);
    }

    #[test]
    fn test_social_plan_query_request_body_serialize() {
        let body = QueryRequestBody {
            plan_ids: vec!["plan_a".to_string()],
            effective_date: 1_707_000_000,
        };
        let value = serde_json::to_value(body).expect("序列化请求体失败");

        assert_eq!(value["plan_ids"], json!(["plan_a"]));
        assert_eq!(value["effective_date"], json!(1_707_000_000));
    }

    #[test]
    fn test_social_plan_query_response_deserialize() {
        let value = json!({
            "items": [
                {
                    "id": "sp_1",
                    "name": "社保方案A",
                    "effective_date": 1707000000
                }
            ]
        });
        let response: QueryResponse = serde_json::from_value(value).expect("反序列化响应失败");

        assert_eq!(response.items.len(), 1);
        assert_eq!(response.items[0].id, "sp_1");
    }

    #[test]
    fn test_social_plan_query_validation() {
        let request = QueryRequest::new(create_test_config(), vec![], 1_706_000_000);
        assert!(request.validate().is_err());

        let invalid_date_request =
            QueryRequest::new(create_test_config(), vec!["plan_1".to_string()], 0);
        assert!(invalid_date_request.validate().is_err());

        let valid_request = QueryRequest::new(
            create_test_config(),
            vec!["plan_1".to_string()],
            1_706_000_000,
        );
        assert!(valid_request.validate().is_ok());
    }
}
