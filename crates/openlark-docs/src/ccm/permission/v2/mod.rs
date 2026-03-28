/// CCM Drive Permission V2 API 模块
///
/// 文档权限管理API实现，包含3个API：
/// - member_permitted: 判断协作者是否有某权限
/// - member_transfer: 转移拥有者
/// - public: 获取云文档权限设置V2
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};

use crate::common::{api_endpoints::PermissionApiOld, api_utils::*};

// 导出模型定义
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

/// 判断协作者是否有某权限
///
/// 根据filetoken判断当前登录用户是否具有某权限。
/// docPath: /document/server-docs/historic-version/docs/drive/permission/querying-if-a-collaborator-has-a-specific-permission
/// doc: <https://open.feishu.cn/document/server-docs/historic-version/docs/drive/permission/querying-if-a-collaborator-has-a-specific-permission>
pub async fn check_member_permission(
    config: &Config,
    params: CheckMemberPermissionParams,
) -> SDKResult<CheckMemberPermissionResponse> {
    // 验证必填字段
    validate_required!(params.obj_token.trim(), "文件Token不能为空");
    validate_required!(params.permission, "权限类型不能为空");

    // 使用enum+builder系统生成API端点
    let api_endpoint = PermissionApiOld::MemberPermitted;

    // 创建API请求
    let api_request: ApiRequest<CheckMemberPermissionResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "检查成员权限")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "检查成员权限")
}

/// 转移拥有者
///
/// 根据文档信息和用户信息转移文档的所有者。
/// docPath: /document/server-docs/historic-version/docs/drive/permission/transfer-ownership
/// doc: <https://open.feishu.cn/document/server-docs/historic-version/docs/drive/permission/transfer-ownership>
pub async fn transfer_owner(
    config: &Config,
    params: TransferOwnerParams,
) -> SDKResult<TransferOwnerResponse> {
    // 验证必填字段
    validate_required!(params.obj_token.trim(), "文件Token不能为空");
    validate_required!(params.member_id.trim(), "新拥有者用户ID不能为空");
    validate_required!(params.member_id_type.trim(), "用户ID类型不能为空");

    // 使用enum+builder系统生成API端点
    let api_endpoint = PermissionApiOld::MemberTransfer;

    // 创建API请求
    let api_request: ApiRequest<TransferOwnerResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "转移拥有者")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "转移拥有者")
}

/// 获取云文档权限设置V2
///
/// 根据filetoken获取文档的公共设置。
/// docPath: /document/server-docs/historic-version/docs/drive/permission/get-document-sharing-settings-v2
/// doc: <https://open.feishu.cn/document/server-docs/historic-version/docs/drive/permission/get-document-sharing-settings-v2>
pub async fn get_public_permission(
    config: &Config,
    params: GetPublicPermissionParams,
) -> SDKResult<GetPublicPermissionResponse> {
    // 验证必填字段
    validate_required!(params.obj_token.trim(), "文件Token不能为空");

    // 使用enum+builder系统生成API端点
    let api_endpoint = PermissionApiOld::Public;

    // 创建API请求
    let api_request: ApiRequest<GetPublicPermissionResponse> =
        ApiRequest::post(&api_endpoint.to_url())
            .body(serialize_params(&params, "获取公开权限设置")?);

    // 发送请求并提取响应数据
    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "获取公开权限设置")
}

// API函数已经在模块中定义，不需要重复导出

// 重新导出模型
// models 模块显式导出
pub use models::{
    CheckMemberPermissionParams, CheckMemberPermissionResponse, GetPublicPermissionParams,
    GetPublicPermissionResponse, PermissionCheckResult, PublicPermission, TransferOwnerParams,
    TransferOwnerResponse, TransferResult, UserInfo,
};
