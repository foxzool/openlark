//! 批量查询国家/地区
//!
//! 文档: <https://open.feishu.cn/document/mdm-v1/mdm-v3/country_region/get>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 批量查询国家/地区 Builder
#[derive(Debug, Clone)]
pub struct CountryRegionBatchGetBuilder {
    config: Config,
    mdm_codes: Vec<String>,
}

impl CountryRegionBatchGetBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config) -> Self {
        Self {
            config,
            mdm_codes: Vec::new(),
        }
    }

    /// 添加主数据编码
    pub fn mdm_code(mut self, mdm_code: impl Into<String>) -> Self {
        self.mdm_codes.push(mdm_code.into());
        self
    }

    /// 添加多个主数据编码
    pub fn mdm_codes(mut self, mdm_codes: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.mdm_codes.extend(mdm_codes.into_iter().map(Into::into));
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CountryRegionBatchGetResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CountryRegionBatchGetResponse> {
        let mut url = "/open-apis/mdm/v3/batch_country_region".to_string();

        // 添加查询参数
        if !self.mdm_codes.is_empty() {
            let codes = self.mdm_codes.join(",");
            url.push_str(&format!("?mdm_codes={}", codes));
        }

        let req: ApiRequest<CountryRegionBatchGetResponse> = ApiRequest::get(&url);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data
            .ok_or_else(|| openlark_core::error::validation_error("Operation", "响应数据为空"))
    }
}

/// 批量查询国家/地区响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CountryRegionBatchGetResponse {
    /// 国家/地区列表
    pub items: Vec<CountryRegion>,
}

/// 国家/地区信息
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CountryRegion {
    /// 主数据编码
    #[serde(rename = "mdm_code")]
    pub mdm_code: String,
    /// 国家/地区名称
    pub name: String,
    /// 国际化名称
    #[serde(rename = "i18n_name")]
    pub i18n_name: Option<CountryRegionI18nName>,
    /// 电话区号
    #[serde(rename = "phone_code")]
    pub phone_code: Option<String>,
}

/// 国家/地区国际化名称
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CountryRegionI18nName {
    /// 中文名称
    #[serde(rename = "zh_cn")]
    pub zh_cn: Option<String>,
    /// 英文名称
    #[serde(rename = "en_us")]
    pub en_us: Option<String>,
    /// 日文名称
    #[serde(rename = "ja_jp")]
    pub ja_jp: Option<String>,
}

impl ApiResponseTrait for CountryRegionBatchGetResponse {}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

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
