//! 批量获取妙搭应用
//!
//! docPath:

use openlark_core::{
    SDKResult, api::ApiRequest, config::Config, http::Transport, req_option::RequestOption,
};
use std::{collections::HashMap, sync::Arc};

/// 批量获取妙搭应用请求。
#[derive(Debug, Clone)]
pub struct ListSparkAppsRequest {
    config: Arc<Config>,
    query: HashMap<String, String>,
}

impl ListSparkAppsRequest {
    /// 创建新的请求构建器。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            query: HashMap::new(),
        }
    }

    /// 设置查询参数。
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query.insert(key.into(), value.into());
        self
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<serde_json::Value> {
        let mut req = ApiRequest::<serde_json::Value>::get("/open-apis/spark/v1/apps");
        for (key, value) in self.query {
            req = req.query(key, value);
        }
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("批量获取妙搭应用", "响应数据为空")
        })
    }
}
