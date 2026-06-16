//! 访问记录 API
//!
//! 提供门禁访问记录的查询和人脸识别照片下载功能。

use openlark_core::error::api_error;
use std::sync::Arc;

/// 访问记录服务
#[derive(Debug)]
pub struct AccessRecordsService {
    config: Arc<crate::models::SecurityConfig>,
}

impl AccessRecordsService {
    /// 创建新的访问记录服务实例
    pub fn new(config: Arc<crate::models::SecurityConfig>) -> Self {
        Self { config }
    }

    /// 获取访问记录列表
    pub fn list(&self) -> ListAccessRecordsBuilder {
        ListAccessRecordsBuilder {
            config: self.config.clone(),
            user_id_filter: None,
            device_id_filter: None,
            start_time: None,
            end_time: None,
            access_result_filter: None,
            page_size: Some(20),
            page_token: None,
            sort_field: Some("access_time".to_string()),
            sort_direction: Some("desc".to_string()),
        }
    }

    /// 获取访问记录的人脸识别照片
    pub fn get_access_photo(&self) -> GetAccessPhotoBuilder {
        GetAccessPhotoBuilder {
            config: self.config.clone(),
            access_record_id: String::new(),
        }
    }
}

/// 获取访问记录列表构建器
#[derive(Debug)]
pub struct ListAccessRecordsBuilder {
    config: Arc<crate::models::SecurityConfig>,
    user_id_filter: Option<String>,
    device_id_filter: Option<String>,
    start_time: Option<crate::models::Timestamp>,
    end_time: Option<crate::models::Timestamp>,
    access_result_filter: Option<crate::models::acs::AccessResult>,
    page_size: Option<i32>,
    page_token: Option<String>,
    sort_field: Option<String>,
    sort_direction: Option<String>,
}

impl ListAccessRecordsBuilder {
    /// 设置用户ID过滤
    pub fn user_id_filter(mut self, user_id: impl Into<String>) -> Self {
        self.user_id_filter = Some(user_id.into());
        self
    }

    /// 设置设备ID过滤
    pub fn device_id_filter(mut self, device_id: impl Into<String>) -> Self {
        self.device_id_filter = Some(device_id.into());
        self
    }

    /// 设置开始时间
    pub fn start_time(mut self, start_time: crate::models::Timestamp) -> Self {
        self.start_time = Some(start_time);
        self
    }

    /// 设置结束时间
    pub fn end_time(mut self, end_time: crate::models::Timestamp) -> Self {
        self.end_time = Some(end_time);
        self
    }

    /// 设置时间范围
    pub fn time_range(
        mut self,
        start_time: crate::models::Timestamp,
        end_time: crate::models::Timestamp,
    ) -> Self {
        self.start_time = Some(start_time);
        self.end_time = Some(end_time);
        self
    }

    /// 设置最近N天的记录
    pub fn last_days(mut self, days: i64) -> Self {
        use chrono::Utc;
        let now = Utc::now().timestamp();
        let start_time = now - days * 24 * 3600;
        self.start_time = Some(start_time);
        self.end_time = Some(now);
        self
    }

    /// 设置最近N小时的记录
    pub fn last_hours(mut self, hours: i64) -> Self {
        use chrono::Utc;
        let now = Utc::now().timestamp();
        let start_time = now - hours * 3600;
        self.start_time = Some(start_time);
        self.end_time = Some(now);
        self
    }

    /// 设置访问结果过滤
    pub fn access_result_filter(mut self, access_result: crate::models::acs::AccessResult) -> Self {
        self.access_result_filter = Some(access_result);
        self
    }

    /// 设置页面大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 设置排序字段
    pub fn sort_field(mut self, sort_field: impl Into<String>) -> Self {
        self.sort_field = Some(sort_field.into());
        self
    }

    /// 设置排序方向
    pub fn sort_direction(mut self, sort_direction: impl Into<String>) -> Self {
        self.sort_direction = Some(sort_direction.into());
        self
    }

    /// 设置按时间升序排列
    pub fn sort_by_time_asc(mut self) -> Self {
        self.sort_field = Some("access_time".to_string());
        self.sort_direction = Some("asc".to_string());
        self
    }

    /// 设置按时间降序排列
    pub fn sort_by_time_desc(mut self) -> Self {
        self.sort_field = Some("access_time".to_string());
        self.sort_direction = Some("desc".to_string());
        self
    }

    /// 发送请求获取访问记录列表
    pub async fn send(
        self,
    ) -> crate::SecurityResult<crate::models::PageResponse<crate::models::acs::AccessRecord>> {
        let url = format!("{}/open-apis/acs/v1/access_records", self.config.base_url);

        let mut query_params = Vec::new();

        if let Some(user_id_filter) = &self.user_id_filter {
            query_params.push(format!("user_id_filter={user_id_filter}"));
        }
        if let Some(device_id_filter) = &self.device_id_filter {
            query_params.push(format!("device_id_filter={device_id_filter}"));
        }
        if let Some(start_time) = self.start_time {
            query_params.push(format!("start_time={start_time}"));
        }
        if let Some(end_time) = self.end_time {
            query_params.push(format!("end_time={end_time}"));
        }
        if let Some(access_result_filter) = &self.access_result_filter {
            let result_str = match access_result_filter {
                crate::models::acs::AccessResult::Success => "success",
                crate::models::acs::AccessResult::Failed => "failed",
                crate::models::acs::AccessResult::Timeout => "timeout",
            };
            query_params.push(format!("access_result_filter={result_str}"));
        }
        if let Some(page_size) = self.page_size {
            query_params.push(format!("page_size={page_size}"));
        }
        if let Some(page_token) = &self.page_token {
            query_params.push(format!("page_token={page_token}"));
        }
        if let Some(sort_field) = &self.sort_field {
            query_params.push(format!("sort_field={sort_field}"));
        }
        if let Some(sort_direction) = &self.sort_direction {
            query_params.push(format!("sort_direction={sort_direction}"));
        }

        if !query_params.is_empty() {
            let url_with_params = format!("{}?{}", url, query_params.join("&"));
            let response = reqwest::Client::new()
                .get(&url_with_params)
                .header(
                    "Authorization",
                    format!("Bearer {}", get_app_token(&self.config).await?),
                )
                .header("Content-Type", "application/json")
                .send()
                .await?;

            if response.status().is_success() {
                let api_response: crate::models::ApiResponse<
                    crate::models::PageResponse<crate::models::acs::AccessRecord>,
                > = response.json().await?;
                match api_response.data {
                    Some(records) => Ok(records),
                    None => Err(api_error(
                        api_response.code as u16,
                        "/acs/v1/access_records",
                        &api_response.msg,
                        None,
                    )),
                }
            } else {
                Err(api_error(
                    response.status().as_u16(),
                    "/acs/v1/access_records",
                    format!(
                        "HTTP {}: {}",
                        response.status(),
                        response.text().await.unwrap_or_default()
                    ),
                    None,
                ))
            }
        } else {
            let response = reqwest::Client::new()
                .get(&url)
                .header(
                    "Authorization",
                    format!("Bearer {}", get_app_token(&self.config).await?),
                )
                .header("Content-Type", "application/json")
                .send()
                .await?;

            if response.status().is_success() {
                let api_response: crate::models::ApiResponse<
                    crate::models::PageResponse<crate::models::acs::AccessRecord>,
                > = response.json().await?;
                match api_response.data {
                    Some(records) => Ok(records),
                    None => Err(api_error(
                        api_response.code as u16,
                        "/acs/v1/access_records",
                        &api_response.msg,
                        None,
                    )),
                }
            } else {
                Err(api_error(
                    response.status().as_u16(),
                    "/acs/v1/access_records",
                    format!(
                        "HTTP {}: {}",
                        response.status(),
                        response.text().await.unwrap_or_default()
                    ),
                    None,
                ))
            }
        }
    }
}

/// 获取访问记录人脸识别照片构建器
#[derive(Debug)]
pub struct GetAccessPhotoBuilder {
    config: Arc<crate::models::SecurityConfig>,
    access_record_id: String,
}

impl GetAccessPhotoBuilder {
    /// 设置访问记录ID
    pub fn access_record_id(mut self, access_record_id: impl Into<String>) -> Self {
        self.access_record_id = access_record_id.into();
        self
    }

    /// 发送请求获取访问记录的人脸识别照片
    pub async fn send(self) -> crate::SecurityResult<Vec<u8>> {
        let url = format!(
            "{}/open-apis/acs/v1/access_records/{}/access_photo",
            self.config.base_url, self.access_record_id
        );

        let response = reqwest::Client::new()
            .get(&url)
            .header(
                "Authorization",
                format!("Bearer {}", get_app_token(&self.config).await?),
            )
            .send()
            .await?;

        if response.status().is_success() {
            let photo_data = response.bytes().await?;
            Ok(photo_data.to_vec())
        } else {
            Err(api_error(
                response.status().as_u16(),
                "/acs/v1/access_records",
                format!("HTTP: {}", response.status()),
                None,
            ))
        }
    }
}

/// 获取应用访问令牌的辅助函数
async fn get_app_token(config: &crate::models::SecurityConfig) -> crate::SecurityResult<String> {
    // 使用 SecurityConfig 的 get_app_access_token 方法获取真实的 token
    config.get_app_access_token().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    fn create_test_config() -> Arc<crate::models::SecurityConfig> {
        Arc::new(crate::models::SecurityConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
        })
    }

    #[test]
    fn test_access_records_service_creation() {
        let config = create_test_config();
        let service = AccessRecordsService::new(config);
        assert_eq!(service.config.app_id, "test_app_id");
    }

    #[test]
    fn test_list_access_records_builder_defaults() {
        let config = create_test_config();
        let service = AccessRecordsService::new(config);
        let builder = service.list();
        assert_eq!(builder.page_size, Some(20));
        assert_eq!(builder.page_token, None);
        assert_eq!(builder.sort_field, Some("access_time".to_string()));
        assert_eq!(builder.sort_direction, Some("desc".to_string()));
    }

    #[test]
    fn test_list_access_records_builder_with_filters() {
        let config = create_test_config();
        let service = AccessRecordsService::new(config);
        let builder = service
            .list()
            .user_id_filter("user_123")
            .device_id_filter("device_456")
            .access_result_filter(crate::models::acs::AccessResult::Success)
            .page_size(50);

        assert_eq!(builder.user_id_filter, Some("user_123".to_string()));
        assert_eq!(builder.device_id_filter, Some("device_456".to_string()));
        assert!(builder.access_result_filter.is_some());
        assert_eq!(builder.page_size, Some(50));
    }

    #[test]
    fn test_list_access_records_builder_time_range() {
        let config = create_test_config();
        let service = AccessRecordsService::new(config);
        let builder = service.list().time_range(1000000, 2000000);

        assert_eq!(builder.start_time, Some(1000000));
        assert_eq!(builder.end_time, Some(2000000));
    }

    #[test]
    fn test_list_access_records_builder_last_days() {
        let config = create_test_config();
        let service = AccessRecordsService::new(config);
        let builder = service.list().last_days(7);

        assert!(builder.start_time.is_some());
        assert!(builder.end_time.is_some());
        assert!(builder.end_time.unwrap() > builder.start_time.unwrap());
    }

    #[test]
    fn test_list_access_records_builder_last_hours() {
        let config = create_test_config();
        let service = AccessRecordsService::new(config);
        let builder = service.list().last_hours(24);

        assert!(builder.start_time.is_some());
        assert!(builder.end_time.is_some());
        assert!(builder.end_time.unwrap() > builder.start_time.unwrap());
    }

    #[test]
    fn test_list_access_records_sort_methods() {
        let config = create_test_config();
        let service = AccessRecordsService::new(config);

        // 测试时间升序
        let builder_asc = service.list().sort_by_time_asc();
        assert_eq!(builder_asc.sort_field, Some("access_time".to_string()));
        assert_eq!(builder_asc.sort_direction, Some("asc".to_string()));

        // 测试时间降序
        let builder_desc = service.list().sort_by_time_desc();
        assert_eq!(builder_desc.sort_field, Some("access_time".to_string()));
        assert_eq!(builder_desc.sort_direction, Some("desc".to_string()));
    }

    #[test]
    fn test_get_access_photo_builder() {
        let config = create_test_config();
        let service = AccessRecordsService::new(config);
        let builder = service.get_access_photo().access_record_id("record_123");
        assert_eq!(builder.access_record_id, "record_123");
    }

    #[test]
    fn test_access_result_variants() {
        // 测试不同的访问结果
        let success = crate::models::acs::AccessResult::Success;
        let failed = crate::models::acs::AccessResult::Failed;
        let timeout = crate::models::acs::AccessResult::Timeout;

        // 验证可以设置过滤
        let config = create_test_config();
        let service = AccessRecordsService::new(config);

        let _ = service.list().access_result_filter(success);
        let _ = service.list().access_result_filter(failed);
        let _ = service.list().access_result_filter(timeout);
    }
}
