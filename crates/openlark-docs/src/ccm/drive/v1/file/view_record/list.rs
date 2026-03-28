//! 获取文件查看记录

//!

//! 获取文件的访问记录列表。

//!

//! docPath: <https://open.feishu.cn/document/server-docs/docs/drive-v1/file-view_record/list>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};

use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 获取文件查看记录请求

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct GetFileViewRecordsRequest {
    /// 文件token
    pub file_token: String,

    /// 文件类型
    pub file_type: String,

    /// 分页大小（1~50）
    pub page_size: i32,

    /// 分页标记（可选）
    pub page_token: Option<String>,

    /// 返回的访问者 ID 的类型（默认 open_id）
    pub viewer_id_type: Option<String>,
}

impl GetFileViewRecordsRequest {
    /// 创建获取文件查看记录请求
    ///
    /// # 参数
    ///
    /// * `file_token` - 文件token
    ///
    /// * `file_type` - 文件类型（doc/docx/sheet/bitable/mindnote/wiki/file）
    ///
    /// * `page_size` - 分页大小（1~50）
    pub fn new(
        file_token: impl Into<String>,

        file_type: impl Into<String>,

        page_size: i32,
    ) -> Self {
        Self {
            file_token: file_token.into(),

            file_type: file_type.into(),

            page_size,

            page_token: None,

            viewer_id_type: None,
        }
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());

        self
    }

    /// 设置访问者 ID 类型
    pub fn viewer_id_type(mut self, viewer_id_type: impl Into<String>) -> Self {
        self.viewer_id_type = Some(viewer_id_type.into());

        self
    }
}

/// 获取文件访问记录响应（data）

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct GetFileViewRecordsResponse {
    /// 访问记录列表

    #[serde(default)]
    pub items: Vec<ViewRecord>,

    /// 分页标记，当 has_more 为 true 时返回

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,

    /// 是否还有更多项
    pub has_more: bool,
}

impl ApiResponseTrait for GetFileViewRecordsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 访问记录

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct ViewRecord {
    /// 访问者 ID
    pub viewer_id: String,

    /// 访问者姓名
    pub name: String,

    /// 访问者头像的 URL
    pub avatar_url: String,

    /// 最近访问时间。Unix 时间戳，单位为秒
    pub last_view_time: String,
}

/// 获取文件查看记录
///
/// 获取文件的访问记录列表。
pub async fn get_file_view_records(
    request: GetFileViewRecordsRequest,

    config: &Config,

    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<GetFileViewRecordsResponse> {
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
        "doc" | "docx" | "sheet" | "bitable" | "mindnote" | "wiki" | "file" => {}

        _ => {
            return Err(openlark_core::error::validation_error(
                "file_type",
                "file_type 仅支持 doc/docx/sheet/bitable/mindnote/wiki/file",
            ));
        }
    }

    if !(1..=50).contains(&request.page_size) {
        return Err(openlark_core::error::validation_error(
            "page_size",
            "page_size 必须在 1~50 之间",
        ));
    }

    // ===== 构建请求 =====
    let url = DriveApi::ListFileViewRecords(request.file_token.clone()).to_url();

    let mut api_request: ApiRequest<GetFileViewRecordsResponse> = ApiRequest::get(&url);

    api_request = api_request.query("page_size", &request.page_size.to_string());

    if let Some(page_token) = &request.page_token {
        api_request = api_request.query("page_token", page_token);
    }

    api_request = api_request.query("file_type", &request.file_type);

    if let Some(viewer_id_type) = &request.viewer_id_type {
        match viewer_id_type.as_str() {
            "user_id" | "union_id" | "open_id" => {}

            _ => {
                return Err(openlark_core::error::validation_error(
                    "viewer_id_type",
                    "viewer_id_type 仅支持 user_id/union_id/open_id",
                ));
            }
        }

        api_request = api_request.query("viewer_id_type", viewer_id_type);
    }

    // ===== 发送请求 =====
    let response = Transport::request(api_request, config, option).await?;

    extract_response_data(response, "获取文件访问记录")
}

#[cfg(test)]
mod tests {

    use super::*;

    /// 测试构建器模式
    #[test]

    fn test_get_file_view_records_request_builder() {
        let request = GetFileViewRecordsRequest::new("file_token", "docx", 10)
            .page_token("token123")
            .viewer_id_type("open_id");

        assert_eq!(request.file_token, "file_token");

        assert_eq!(request.file_type, "docx");

        assert_eq!(request.page_size, 10);

        assert_eq!(request.page_token, Some("token123".to_string()));

        assert_eq!(request.viewer_id_type, Some("open_id".to_string()));
    }

    /// 测试访问记录数据结构
    #[test]

    fn test_view_record_structure() {
        let record = ViewRecord {
            viewer_id: "ou_xxx".to_string(),

            name: "zhangsan".to_string(),

            avatar_url: "https://foo.icon.com/xxxx".to_string(),

            last_view_time: "1679284285".to_string(),
        };

        assert_eq!(record.name, "zhangsan".to_string());

        assert_eq!(record.last_view_time, "1679284285".to_string());
    }

    /// 测试响应trait实现
    #[test]

    fn test_response_trait() {
        assert_eq!(
            GetFileViewRecordsResponse::data_format(),
            ResponseFormat::Data
        );
    }

    /// 测试不同file_type
    #[test]
    fn test_different_file_types() {
        let sheet_request = GetFileViewRecordsRequest::new("sheet_token", "sheet", 20);
        assert_eq!(sheet_request.file_type, "sheet");

        let wiki_request = GetFileViewRecordsRequest::new("wiki_token", "wiki", 15);
        assert_eq!(wiki_request.file_type, "wiki");
    }

    /// 测试边界page_size
    #[test]
    fn test_page_size_boundaries() {
        let min_request = GetFileViewRecordsRequest::new("file_token", "docx", 1);
        assert_eq!(min_request.page_size, 1);

        let max_request = GetFileViewRecordsRequest::new("file_token", "docx", 50);
        assert_eq!(max_request.page_size, 50);
    }

    /// 测试不同viewer_id_type
    #[test]
    fn test_viewer_id_types() {
        let user_id_request =
            GetFileViewRecordsRequest::new("file_token", "docx", 10).viewer_id_type("user_id");
        assert_eq!(user_id_request.viewer_id_type, Some("user_id".to_string()));

        let union_id_request =
            GetFileViewRecordsRequest::new("file_token", "docx", 10).viewer_id_type("union_id");
        assert_eq!(
            union_id_request.viewer_id_type,
            Some("union_id".to_string())
        );
    }

    /// 测试响应分页信息
    #[test]
    fn test_response_pagination() {
        let response = GetFileViewRecordsResponse {
            items: vec![],
            page_token: Some("next_token".to_string()),
            has_more: true,
        };

        assert!(response.has_more);
        assert_eq!(response.page_token, Some("next_token".to_string()));
    }

    /// 测试无更多数据场景
    #[test]
    fn test_no_more_data() {
        let response = GetFileViewRecordsResponse {
            items: vec![],
            page_token: None,
            has_more: false,
        };

        assert!(!response.has_more);
        assert!(response.page_token.is_none());
    }
}
