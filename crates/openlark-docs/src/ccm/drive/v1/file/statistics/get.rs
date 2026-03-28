//! 获取文件统计信息

//!

//! 获取文件统计信息，包括文档阅读人数、次数和点赞数。

//!

//! docPath: <https://open.feishu.cn/document/server-docs/docs/drive-v1/file/get>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 获取文件统计信息请求

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct GetFileStatisticsRequest {
    /// 文件token
    pub file_token: String,

    /// 文件类型
    pub file_type: String,
}

impl GetFileStatisticsRequest {
    /// 创建获取文件统计信息请求
    ///
    /// # 参数
    ///
    /// * `file_token` - 文件token
    ///
    /// * `file_type` - 文件类型（doc/sheet/mindnote/bitable/wiki/file/docx）
    pub fn new(file_token: impl Into<String>, file_type: impl Into<String>) -> Self {
        Self {
            file_token: file_token.into(),

            file_type: file_type.into(),
        }
    }
}

/// 文件统计信息

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct FileStatistics {
    /// 文档历史访问人数，同一用户（user_id）多次访问按一次计算
    pub uv: i32,

    /// 文档历史访问次数，同一用户（user_id）多次访问按多次计算，但同一用户在间隔在半小时内访问两次视为一次访问
    pub pv: i32,

    /// 文档历史点赞总数。`-1` 表示对应的文档类型不支持点赞
    pub like_count: i32,

    /// 时间戳（单位：秒）
    pub timestamp: i32,

    /// 今日新增文档访问人数
    pub uv_today: i32,

    /// 今日新增文档访问次数
    pub pv_today: i32,

    /// 今日新增文档点赞数
    pub like_count_today: i32,
}

/// 获取文件统计信息响应（data）

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct GetFileStatisticsResponse {
    /// 文档 token
    pub file_token: String,

    /// 文档类型
    pub file_type: String,

    /// 文档统计信息
    pub statistics: FileStatistics,
}

impl ApiResponseTrait for GetFileStatisticsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取文件统计信息
///
/// 获取文件统计信息，包括文档阅读人数、次数和点赞数。
pub async fn get_file_statistics(
    request: GetFileStatisticsRequest,

    config: &Config,

    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<GetFileStatisticsResponse> {
    // ===== 参数校验 =====
    if request.file_token.is_empty() {
        return Err(openlark_core::error::validation_error(
            "file_token",
            "file_token 不能为空",
        ));
    }

    if request.file_type.is_empty() {
        return Err(openlark_core::error::validation_error(
            "file_type",
            "file_type 不能为空",
        ));
    }

    match request.file_type.as_str() {
        "doc" | "sheet" | "mindnote" | "bitable" | "wiki" | "file" | "docx" => {}

        _ => {
            return Err(openlark_core::error::validation_error(
                "file_type",
                "file_type 仅支持 doc/sheet/mindnote/bitable/wiki/file/docx",
            ));
        }
    }

    // ===== 构建请求 =====
    let url = DriveApi::GetFileStatistics(request.file_token.clone()).to_url();

    let mut api_request: ApiRequest<GetFileStatisticsResponse> = ApiRequest::get(&url);

    api_request = api_request.query("file_type", &request.file_type);

    // ===== 发送请求 =====
    let response = Transport::request(api_request, config, option).await?;

    extract_response_data(response, "获取文件统计信息")
}

#[cfg(test)]
mod tests {

    use super::*;

    /// 测试构建器模式
    #[test]

    fn test_get_file_statistics_request_builder() {
        let request = GetFileStatisticsRequest::new("file_token", "doc");

        assert_eq!(request.file_token, "file_token");

        assert_eq!(request.file_type, "doc");
    }

    /// 测试统计数据结构
    #[test]

    fn test_file_statistics_structure() {
        let stats = FileStatistics {
            uv: 10,

            pv: 15,

            like_count: 2,

            timestamp: 1627367349,

            uv_today: 1,

            pv_today: 1,

            like_count_today: 1,
        };

        assert_eq!(stats.uv, 10);

        assert_eq!(stats.pv, 15);

        assert_eq!(stats.like_count, 2);
    }

    /// 测试响应trait实现
    #[test]

    fn test_response_trait() {
        assert_eq!(
            GetFileStatisticsResponse::data_format(),
            ResponseFormat::Data
        );
    }

    /// 测试sheet类型统计
    #[test]
    fn test_sheet_file_type() {
        let request = GetFileStatisticsRequest::new("sheet_token", "sheet");

        assert_eq!(request.file_type, "sheet");
    }

    /// 测试bitable类型统计
    #[test]
    fn test_bitable_file_type() {
        let request = GetFileStatisticsRequest::new("bitable_token", "bitable");

        assert_eq!(request.file_type, "bitable");
    }

    /// 测试不支持点赞的文档
    #[test]
    fn test_unsupported_like_count() {
        let stats = FileStatistics {
            uv: 100,
            pv: 200,
            like_count: -1, // 表示不支持点赞
            timestamp: 1627367349,
            uv_today: 5,
            pv_today: 10,
            like_count_today: 0,
        };

        assert_eq!(stats.like_count, -1);
    }

    /// 测试今日统计数据
    #[test]
    fn test_today_statistics() {
        let stats = FileStatistics {
            uv: 100,
            pv: 200,
            like_count: 10,
            timestamp: 1627367349,
            uv_today: 5,
            pv_today: 8,
            like_count_today: 2,
        };

        assert_eq!(stats.uv_today, 5);
        assert_eq!(stats.pv_today, 8);
        assert_eq!(stats.like_count_today, 2);
    }
}
