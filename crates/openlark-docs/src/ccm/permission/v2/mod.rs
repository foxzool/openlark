/// CCM Drive Permission V2 API 模块
///
/// 文档权限管理API实现，包含3个API构建器：
/// - CheckMemberPermissionRequest: 判断协作者是否有某权限
/// - TransferOwnerRequest: 转移拥有者
/// - GetPublicPermissionRequest: 获取云文档权限设置V2
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required,
};

use crate::common::{api_endpoints::PermissionApiOld, api_utils::*};

/// 权限接口模型模块。
pub mod models;

impl ApiResponseTrait for CheckMemberPermissionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for TransferOwnerResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ApiResponseTrait for GetPublicPermissionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone)]
/// 检查协作者权限请求构建器。
pub struct CheckMemberPermissionRequest {
    config: Config,
    params: CheckMemberPermissionParams,
}

impl CheckMemberPermissionRequest {
    /// 创建检查协作者权限请求。
    pub fn new(config: Config, params: CheckMemberPermissionParams) -> Self {
        Self { config, params }
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<CheckMemberPermissionResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CheckMemberPermissionResponse> {
        validate_required!(self.params.obj_token.trim(), "文件Token不能为空");
        validate_required!(self.params.permission.trim(), "权限类型不能为空");

        let api_endpoint = PermissionApiOld::MemberPermitted;
        let api_request: ApiRequest<CheckMemberPermissionResponse> =
            ApiRequest::post(&api_endpoint.to_url())
                .body(serialize_params(&self.params, "检查成员权限")?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "检查成员权限")
    }
}

#[derive(Debug, Clone)]
/// 转移文档拥有者请求构建器。
pub struct TransferOwnerRequest {
    config: Config,
    params: TransferOwnerParams,
}

impl TransferOwnerRequest {
    /// 创建转移拥有者请求。
    pub fn new(config: Config, params: TransferOwnerParams) -> Self {
        Self { config, params }
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<TransferOwnerResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<TransferOwnerResponse> {
        validate_required!(self.params.obj_token.trim(), "文件Token不能为空");
        validate_required!(self.params.member_id.trim(), "新拥有者用户ID不能为空");
        validate_required!(self.params.member_id_type.trim(), "用户ID类型不能为空");

        let api_endpoint = PermissionApiOld::MemberTransfer;
        let api_request: ApiRequest<TransferOwnerResponse> =
            ApiRequest::post(&api_endpoint.to_url())
                .body(serialize_params(&self.params, "转移拥有者")?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "转移拥有者")
    }
}

#[derive(Debug, Clone)]
/// 获取公开权限设置请求构建器。
pub struct GetPublicPermissionRequest {
    config: Config,
    params: GetPublicPermissionParams,
}

impl GetPublicPermissionRequest {
    /// 创建获取公开权限设置请求。
    pub fn new(config: Config, params: GetPublicPermissionParams) -> Self {
        Self { config, params }
    }

    /// 使用默认请求选项执行请求。
    pub async fn execute(self) -> SDKResult<GetPublicPermissionResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetPublicPermissionResponse> {
        validate_required!(self.params.obj_token.trim(), "文件Token不能为空");

        let api_endpoint = PermissionApiOld::Public;
        let api_request: ApiRequest<GetPublicPermissionResponse> =
            ApiRequest::post(&api_endpoint.to_url())
                .body(serialize_params(&self.params, "获取公开权限设置")?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取公开权限设置")
    }
}

// API函数已经在模块中定义，不需要重复导出

/// 重新导出权限接口模型。
pub use models::{
    CheckMemberPermissionParams, CheckMemberPermissionResponse, GetPublicPermissionParams,
    GetPublicPermissionResponse, PermissionCheckResult, PublicPermission, TransferOwnerParams,
    TransferOwnerResponse, TransferResult, UserInfo,
};
