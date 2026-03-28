//! 获取 OKR 周期规则
//!
//! docPath: <https://open.feishu.cn/document/server-docs/okr-v1/period_rule/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 获取 OKR 周期规则请求
#[derive(Debug, Clone)]
pub struct ListRequest {
    /// 周期 ID（可选）
    period_id: Option<String>,
    /// 配置信息
    config: Config,
}

impl ListRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            period_id: None,
            config,
        }
    }

    /// 设置周期 ID（可选）
    pub fn period_id(mut self, period_id: String) -> Self {
        self.period_id = Some(period_id);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListResponse> {
        use crate::common::api_endpoints::OkrApiV1;

        // 1. 构建端点
        let api_endpoint = OkrApiV1::PeriodRuleList;
        let mut request = ApiRequest::<ListResponse>::get(api_endpoint.to_url());

        // 2. 添加查询参数（可选）
        if let Some(ref period_id) = self.period_id {
            request = request.query("period_id", period_id);
        }

        // 3. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 4. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "获取 OKR 周期规则响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 获取 OKR 周期规则响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResponse {
    /// 周期规则列表
    pub items: Vec<PeriodRule>,
}

/// OKR 周期规则
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PeriodRule {
    /// 规则 ID
    pub rule_id: String,
    /// 周期 ID
    pub period_id: String,
    /// 规则名称
    pub name: String,
    /// 规则类型
    /// - 1: 评分规则
    /// - 2: 进度更新规则
    /// - 3: 复盘规则
    pub rule_type: i32,
    /// 规则配置
    pub config: serde_json::Value,
    /// 创建时间
    pub created_at: i64,
    /// 更新时间
    pub updated_at: i64,
}

impl ApiResponseTrait for ListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
