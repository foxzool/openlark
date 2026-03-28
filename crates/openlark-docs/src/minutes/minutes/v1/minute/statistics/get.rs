//! 获取妙记统计数据
//!
//! docPath: <https://open.feishu.cn/document/server-docs/minutes-v1/minute-statistics/get>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::MinutesApiV1, api_utils::*};

#[derive(Debug, Clone)]
pub struct GetMinuteStatisticsRequest {
    config: Config,
    minute_token: Option<String>,
    user_id_type: Option<String>,
}

impl GetMinuteStatisticsRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            minute_token: None,
            user_id_type: None,
        }
    }

    pub fn minute_token(mut self, minute_token: impl Into<String>) -> Self {
        self.minute_token = Some(minute_token.into());
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    pub async fn execute(self) -> SDKResult<GetMinuteStatisticsResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetMinuteStatisticsResponse> {
        // ===== 参数校验 =====
        let minute_token = self.minute_token.ok_or_else(|| {
            openlark_core::error::validation_error("minute_token", "minute_token 不能为空")
        })?;
        if minute_token.chars().count() != 24 {
            return Err(openlark_core::error::validation_error(
                "minute_token",
                "minute_token 长度必须为 24 字符",
            ));
        }
        if let Some(user_id_type) = &self.user_id_type {
            match user_id_type.as_str() {
                "open_id" | "union_id" | "user_id" => {}
                _ => {
                    return Err(openlark_core::error::validation_error(
                        "user_id_type",
                        "user_id_type 仅支持 open_id/union_id/user_id",
                    ));
                }
            }
        }

        // ===== 构建请求 =====
        let api_endpoint = MinutesApiV1::StatisticsGet(minute_token);
        let mut api_request: ApiRequest<GetMinuteStatisticsResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        if let Some(user_id_type) = &self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type);
        }

        // ===== 发送请求 =====
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试构建器模式
    #[test]
    fn test_get_minute_statistics_builder() {
        let config = Config::default();
        let request = GetMinuteStatisticsRequest::new(config)
            .minute_token("123456789012345678901234")
            .user_id_type("open_id");

        assert_eq!(
            request.minute_token,
            Some("123456789012345678901234".to_string())
        );
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    /// 测试响应数据结构
    #[test]
    fn test_minute_statistics_structure() {
        let stats = MinuteStatistics {
            user_view_count: "10".to_string(),
            page_view_count: "20".to_string(),
            user_view_list: vec![UserViewDetail {
                user_id: "user_123".to_string(),
                view_time: "1679284285000".to_string(),
            }],
        };

        assert_eq!(stats.user_view_count, "10");
        assert_eq!(stats.page_view_count, "20");
        assert_eq!(stats.user_view_list.len(), 1);
    }

    /// 测试响应trait实现
    #[test]
    fn test_response_trait() {
        assert_eq!(
            GetMinuteStatisticsResponse::data_format(),
            ResponseFormat::Data
        );
    }

    /// 测试UserViewDetail结构
    #[test]
    fn test_user_view_detail() {
        let detail = UserViewDetail {
            user_id: "user_abc".to_string(),
            view_time: "1679284285000".to_string(),
        };

        assert_eq!(detail.user_id, "user_abc");
        assert_eq!(detail.view_time, "1679284285000");
    }

    /// 测试有效的minute_token长度
    #[test]
    fn test_valid_minute_token_length() {
        let config = Config::default();
        let valid_token = "a".repeat(24);
        let request = GetMinuteStatisticsRequest::new(config).minute_token(&valid_token);

        assert_eq!(request.minute_token.unwrap().len(), 24);
    }

    /// 测试不同user_id_type
    #[test]
    fn test_different_user_id_types() {
        let config = Config::default();

        let union_id_request =
            GetMinuteStatisticsRequest::new(config.clone()).user_id_type("union_id");
        assert_eq!(union_id_request.user_id_type, Some("union_id".to_string()));

        let user_id_request = GetMinuteStatisticsRequest::new(config).user_id_type("user_id");
        assert_eq!(user_id_request.user_id_type, Some("user_id".to_string()));
    }

    /// 测试空查看列表
    #[test]
    fn test_empty_user_view_list() {
        let stats = MinuteStatistics {
            user_view_count: "0".to_string(),
            page_view_count: "0".to_string(),
            user_view_list: vec![],
        };

        assert!(stats.user_view_list.is_empty());
        assert_eq!(stats.user_view_count, "0");
    }

    /// 测试多个查看记录
    #[test]
    fn test_multiple_user_views() {
        let stats = MinuteStatistics {
            user_view_count: "3".to_string(),
            page_view_count: "10".to_string(),
            user_view_list: vec![
                UserViewDetail {
                    user_id: "user1".to_string(),
                    view_time: "1000".to_string(),
                },
                UserViewDetail {
                    user_id: "user2".to_string(),
                    view_time: "2000".to_string(),
                },
                UserViewDetail {
                    user_id: "user3".to_string(),
                    view_time: "3000".to_string(),
                },
            ],
        };

        assert_eq!(stats.user_view_list.len(), 3);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMinuteStatisticsResponse {
    pub statistics: MinuteStatistics,
}

impl ApiResponseTrait for GetMinuteStatisticsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinuteStatistics {
    pub user_view_count: String,
    pub page_view_count: String,
    #[serde(default)]
    pub user_view_list: Vec<UserViewDetail>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserViewDetail {
    pub user_id: String,
    /// 用户的最近查看时间 timestamp（ms 级别）
    pub view_time: String,
}
