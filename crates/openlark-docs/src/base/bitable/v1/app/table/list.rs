//! Bitable 列出数据表API
//!
//! docPath: <https://open.feishu.cn/document/server-docs/docs/bitable-v1/app-table/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::SDKResult,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};

/// 列出数据表请求
#[derive(Debug, Clone)]
pub struct ListTablesRequest {
    /// 多维表格的 app_token
    app_token: String,
    /// 分页大小
    page_size: Option<i32>,
    /// 分页标记
    page_token: Option<String>,
    /// 配置信息
    config: Config,
}

impl ListTablesRequest {
    /// 创建列出数据表请求
    pub fn new(config: Config) -> Self {
        Self {
            app_token: String::new(),
            page_size: None,
            page_token: None,
            config,
        }
    }

    /// 设置应用token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size.min(100)); // 限制最大100
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListTablesResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListTablesResponse> {
        // === 必填字段验证 ===
        validate_required!(self.app_token.trim(), "app_token");

        // === 边界值验证 ===
        if let Some(page_size) = self.page_size {
            if page_size <= 0 {
                return Err(openlark_core::error::validation_error(
                    "page_size",
                    "分页大小必须大于0",
                ));
            }
        }

        // 🚀 使用新的enum+builder系统生成API端点
        // 替代传统的字符串拼接方式，提供类型安全和IDE自动补全
        use crate::common::api_endpoints::BitableApiV1;
        let api_endpoint = BitableApiV1::TableList(self.app_token.clone());

        // 创建API请求 - 使用类型安全的URL生成
        let mut api_request: ApiRequest<ListTablesResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        // 构建查询参数
        if let Some(page_size) = self.page_size {
            api_request = api_request.query("page_size", &page_size.to_string());
        }

        if let Some(ref page_token) = self.page_token {
            api_request = api_request.query("page_token", page_token);
        }

        // 发送请求
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

/// 数据表信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TableInfo {
    /// 数据表的ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_id: Option<String>,
    /// 数据表的版本号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<i32>,
    /// 数据表的名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// 列出数据表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListTablesResponse {
    /// 是否还有更多项
    #[serde(default)]
    pub has_more: bool,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 总数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i32>,
    /// 数据表列表
    #[serde(default)]
    pub items: Vec<TableInfo>,
}

impl ApiResponseTrait for ListTablesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::testing::prelude::test_runtime;

    #[test]
    fn test_empty_app_token() {
        let config = Config::default();
        let request = ListTablesRequest::new(config).app_token("".to_string());
        let rt = test_runtime();
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("app_token"));
    }

    #[test]
    fn test_invalid_page_size() {
        let config = Config::default();
        let request = ListTablesRequest::new(config)
            .app_token("app_token".to_string())
            .page_size(0);
        let rt = test_runtime();
        let result = rt.block_on(request.execute());
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.to_string().contains("page_size"));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(ListTablesResponse::data_format(), ResponseFormat::Data);
    }
}
