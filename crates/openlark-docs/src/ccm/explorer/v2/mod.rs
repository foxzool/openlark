/// CCM Drive Explorer V2 API 模块
///
/// 云盘浏览器API实现，包含8个API：
/// - root_folder_meta: 获取我的空间（根文件夹）元数据
/// - folder_meta: 获取文件夹元数据
/// - file: 新建文件
/// - file_copy: 复制文档
/// - file_docs: 删除Doc
/// - file_spreadsheets: 删除Sheet
/// - folder_children: 获取文件夹下的文档清单
/// - folder: 新建文件夹
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};
use serde::Deserialize;

use crate::common::{
    api_endpoints::{CcmDriveExplorerApiOld, DriveApi},
    api_utils::*,
};

#[derive(Debug, Clone, Deserialize)]
struct DriveListFilesData {
    files: Vec<DriveListFileItem>,
    next_page_token: Option<String>,
    has_more: bool,
}

#[derive(Debug, Clone, Deserialize)]
struct DriveListFileItem {
    token: String,
    name: String,
    r#type: String,
    created_time: Option<String>,
    modified_time: Option<String>,
}

impl ApiResponseTrait for DriveListFilesData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// Explorer v2 模型模块。
pub mod models;

impl ApiResponseTrait for FolderMetaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for CreateFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for CopyFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for GetFolderChildrenResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for CreateFolderResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for DeleteFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone)]
/// 获取根文件夹元信息请求构建器。
pub struct GetRootFolderMetaRequest {
    config: Config,
}

impl GetRootFolderMetaRequest {
    /// 创建获取根文件夹元信息请求。
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 使用默认请求选项执行请求。
    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<FolderMetaResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<FolderMetaResponse> {
        let api_endpoint = CcmDriveExplorerApiOld::RootFolderMeta;
        let api_request: ApiRequest<FolderMetaResponse> = ApiRequest::get(&api_endpoint.to_url());
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取根文件夹元信息")
    }
}

#[derive(Debug, Clone)]
/// 获取文件夹元信息请求构建器。
pub struct GetFolderMetaRequest {
    config: Config,
    folder_token: String,
}

impl GetFolderMetaRequest {
    /// 创建获取文件夹元信息请求。
    pub fn new(config: Config, folder_token: impl Into<String>) -> Self {
        Self {
            config,
            folder_token: folder_token.into(),
        }
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<FolderMetaResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<FolderMetaResponse> {
        validate_required!(self.folder_token.trim(), "文件夹Token不能为空");

        let api_endpoint = CcmDriveExplorerApiOld::FolderMeta(self.folder_token);
        let api_request: ApiRequest<FolderMetaResponse> = ApiRequest::get(&api_endpoint.to_url());
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取文件夹元信息")
    }
}

#[derive(Debug, Clone)]
/// 新建文件请求构建器。
pub struct CreateFileRequest {
    config: Config,
    folder_token: String,
    params: CreateFileParams,
}

impl CreateFileRequest {
    /// 创建新建文件请求。
    pub fn new(config: Config, folder_token: impl Into<String>, params: CreateFileParams) -> Self {
        Self {
            config,
            folder_token: folder_token.into(),
            params,
        }
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<CreateFileResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CreateFileResponse> {
        validate_required!(self.folder_token.trim(), "文件夹Token不能为空");
        validate_required!(self.params.title.trim(), "文件标题不能为空");
        validate_required!(self.params.parent_type.trim(), "文件类型不能为空");

        let api_endpoint = CcmDriveExplorerApiOld::File(self.folder_token);
        let api_request: ApiRequest<CreateFileResponse> = ApiRequest::post(&api_endpoint.to_url())
            .body(serialize_params(&self.params, "新建文件")?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "新建文件")
    }
}

#[derive(Debug, Clone)]
/// 复制文件请求构建器。
pub struct CopyFileRequest {
    config: Config,
    file_token: String,
    params: CopyFileParams,
}

impl CopyFileRequest {
    /// 创建复制文件请求。
    pub fn new(config: Config, file_token: impl Into<String>, params: CopyFileParams) -> Self {
        Self {
            config,
            file_token: file_token.into(),
            params,
        }
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<CopyFileResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<CopyFileResponse> {
        validate_required!(self.file_token.trim(), "文件Token不能为空");
        validate_required!(self.params.folder_token.trim(), "目标文件夹Token不能为空");

        let api_endpoint = CcmDriveExplorerApiOld::FileCopy(self.file_token);
        let api_request: ApiRequest<CopyFileResponse> = ApiRequest::post(&api_endpoint.to_url())
            .body(serialize_params(&self.params, "复制文档")?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "复制文档")
    }
}

#[derive(Debug, Clone)]
/// 删除 Doc 请求构建器。
pub struct DeleteDocRequest {
    config: Config,
    doc_token: String,
}

impl DeleteDocRequest {
    /// 创建删除 Doc 请求。
    pub fn new(config: Config, doc_token: impl Into<String>) -> Self {
        Self {
            config,
            doc_token: doc_token.into(),
        }
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<DeleteFileResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DeleteFileResponse> {
        validate_required!(self.doc_token.trim(), "文档Token不能为空");

        let api_endpoint = CcmDriveExplorerApiOld::FileDocs(self.doc_token);
        let api_request: ApiRequest<DeleteFileResponse> =
            ApiRequest::delete(&api_endpoint.to_url());

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "删除Doc")
    }
}

#[derive(Debug, Clone)]
/// 删除 Sheet 请求构建器。
pub struct DeleteSheetRequest {
    config: Config,
    spreadsheet_token: String,
}

impl DeleteSheetRequest {
    /// 创建删除 Sheet 请求。
    pub fn new(config: Config, spreadsheet_token: impl Into<String>) -> Self {
        Self {
            config,
            spreadsheet_token: spreadsheet_token.into(),
        }
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<DeleteFileResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DeleteFileResponse> {
        validate_required!(self.spreadsheet_token.trim(), "表格Token不能为空");

        let api_endpoint = CcmDriveExplorerApiOld::FileSpreadsheets(self.spreadsheet_token);
        let api_request: ApiRequest<DeleteFileResponse> =
            ApiRequest::delete(&api_endpoint.to_url());

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "删除Sheet")
    }
}

#[derive(Debug, Clone)]
/// 获取文件夹子项列表请求构建器。
pub struct GetFolderChildrenRequest {
    config: Config,
    folder_token: String,
    params: Option<GetFolderChildrenParams>,
}

impl GetFolderChildrenRequest {
    /// 创建获取文件夹子项列表请求。
    pub fn new(
        config: Config,
        folder_token: impl Into<String>,
        params: Option<GetFolderChildrenParams>,
    ) -> Self {
        Self {
            config,
            folder_token: folder_token.into(),
            params,
        }
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<GetFolderChildrenResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetFolderChildrenResponse> {
        validate_required!(self.folder_token.trim(), "文件夹Token不能为空");

        let mut api_request: ApiRequest<DriveListFilesData> =
            ApiRequest::get(&DriveApi::ListFiles.to_url()).query("folder_token", self.folder_token);

        if let Some(params) = self.params {
            api_request =
                api_request.query_opt("page_size", params.page_size.map(|v| v.to_string()));
            api_request = api_request.query_opt("page_token", params.page_token);
            if let Some(doc_type) = params.doc_type {
                api_request = api_request.query("type", doc_type);
            }
        }

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        let data: DriveListFilesData = extract_response_data(response, "获取文件夹子项")?;

        let items = data
            .files
            .into_iter()
            .map(|item| {
                let doc_type = item.r#type;
                let is_folder = doc_type == "folder";
                FileItem {
                    file_token: item.token,
                    title: item.name,
                    doc_type,
                    is_folder,
                    create_time: item
                        .created_time
                        .as_deref()
                        .unwrap_or("0")
                        .parse::<i64>()
                        .unwrap_or(0),
                    update_time: item
                        .modified_time
                        .as_deref()
                        .unwrap_or("0")
                        .parse::<i64>()
                        .unwrap_or(0),
                }
            })
            .collect::<Vec<_>>();

        Ok(GetFolderChildrenResponse {
            data: Some(FolderChildrenData {
                items,
                has_more: data.has_more,
                page_token: data.next_page_token,
            }),
        })
    }
}

#[derive(Debug, Clone)]
/// 新建文件夹请求构建器。
pub struct CreateFolderRequest {
    config: Config,
    folder_token: String,
    params: CreateFolderParams,
}

impl CreateFolderRequest {
    /// 创建新建文件夹请求。
    pub fn new(
        config: Config,
        folder_token: impl Into<String>,
        params: CreateFolderParams,
    ) -> Self {
        Self {
            config,
            folder_token: folder_token.into(),
            params,
        }
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<CreateFolderResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CreateFolderResponse> {
        validate_required!(self.folder_token.trim(), "父文件夹Token不能为空");
        validate_required!(self.params.title.trim(), "文件夹标题不能为空");

        let api_endpoint = CcmDriveExplorerApiOld::Folder(self.folder_token);
        let api_request: ApiRequest<CreateFolderResponse> =
            ApiRequest::post(&api_endpoint.to_url())
                .body(serialize_params(&self.params, "新建文件夹")?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "新建文件夹")
    }
}

/// 获取我的空间（根文件夹）元数据
///
/// 获取 "我的空间" 的元信息。
/// docPath: /document/server-docs/docs/drive-v1/folder/get-root-folder-meta
// API函数已经在模块中定义，不需要重复导出

/// 重新导出 Explorer v2 模型。
pub use models::{
    CopyFileParams, CopyFileResponse, CopyResult, CreateFileParams, CreateFileResponse,
    CreateFolderParams, CreateFolderResponse, DeleteFileResponse, DeleteResult, FileInfo, FileItem,
    FolderChildrenData, FolderMeta, FolderMetaResponse, GetFolderChildrenParams,
    GetFolderChildrenResponse, NewFolderInfo, UserInfo,
};
