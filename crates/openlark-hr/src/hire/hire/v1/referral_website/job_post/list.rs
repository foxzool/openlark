//! 获取内推官网下职位广告列表
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/referral_website.job_post/list>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error,
    http::Transport,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::hire::hire::common_models::{CodeNameObject, I18nText, IdNameObject};

/// `ListRequest` 请求。
#[derive(Debug, Clone)]
pub struct ListRequest {
    config: Config,
    process_type: Option<i32>,
    page_token: Option<String>,
    page_size: Option<i32>,
    user_id_type: Option<String>,
    department_id_type: Option<String>,
    job_level_id_type: Option<String>,
}

impl ListRequest {
    /// 创建新的请求实例。
    pub fn new(config: Config) -> Self {
        Self {
            config,
            process_type: None,
            page_token: None,
            page_size: None,
            user_id_type: None,
            department_id_type: None,
            job_level_id_type: None,
        }
    }

    /// 设置 `process_type`。
    pub fn process_type(mut self, process_type: i32) -> Self {
        self.process_type = Some(process_type);
        self
    }

    /// 设置分页标记。
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置分页大小。
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置 `department_id_type`。
    pub fn department_id_type(mut self, department_id_type: impl Into<String>) -> Self {
        self.department_id_type = Some(department_id_type.into());
        self
    }

    /// 设置 `job_level_id_type`。
    pub fn job_level_id_type(mut self, job_level_id_type: impl Into<String>) -> Self {
        self.job_level_id_type = Some(job_level_id_type.into());
        self
    }

    /// 执行请求。
    pub async fn execute(self) -> SDKResult<ListResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListResponse> {
        if let Some(page_size) = self.page_size
            && !(1..=10).contains(&page_size)
        {
            return Err(error::validation_error(
                "page_size",
                "page_size 必须在 1-10 之间",
            ));
        }

        let mut request =
            ApiRequest::<ListResponse>::get("/open-apis/hire/v1/referral_websites/job_posts");
        if let Some(process_type) = self.process_type {
            request = request.query("process_type", process_type.to_string());
        }
        if let Some(page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }
        if let Some(department_id_type) = self.department_id_type {
            request = request.query("department_id_type", department_id_type);
        }
        if let Some(job_level_id_type) = self.job_level_id_type {
            request = request.query("job_level_id_type", job_level_id_type);
        }
        let response = Transport::request(request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            error::validation_error(
                "获取内推官网下职位广告列表响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// `JobPostAddress`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct JobPostAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 名称。
    pub name: Option<I18nText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `district` 字段。
    pub district: Option<CodeNameObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `city` 字段。
    pub city: Option<CodeNameObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `state` 字段。
    pub state: Option<CodeNameObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `country` 字段。
    pub country: Option<CodeNameObject>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `ReferralWebsiteJobPostItem`。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ReferralWebsiteJobPostItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标识。
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 标题。
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_id` 字段。
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_code` 字段。
    pub job_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_expire_time` 字段。
    pub job_expire_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_active_status` 字段。
    pub job_active_status: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_process_type` 字段。
    pub job_process_type: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_recruitment_type` 字段。
    pub job_recruitment_type: Option<IdNameObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_department` 字段。
    pub job_department: Option<IdNameObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `job_type` 字段。
    pub job_type: Option<IdNameObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `min_job_level` 字段。
    pub min_job_level: Option<IdNameObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `max_job_level` 字段。
    pub max_job_level: Option<IdNameObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `address` 字段。
    pub address: Option<JobPostAddress>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

/// `ListResponse` 响应。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListResponse {
    #[serde(default)]
    /// 结果项列表。
    pub items: Vec<ReferralWebsiteJobPostItem>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 下一页分页标记。
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    /// 是否还有更多结果。
    pub has_more: Option<bool>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
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
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
