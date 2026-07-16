//! Wiki API 端点目录。

use super::CatalogEndpoint;
use openlark_core::api::{ApiRequest, HttpMethod};

/// Wiki API V1 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum WikiApiV1 {
    /// 搜索Wiki
    NodeSearch,
}

impl WikiApiV1 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            WikiApiV1::NodeSearch => "/open-apis/wiki/v1/nodes/search".to_string(),
        }
    }

    /// 返回配置了稳定请求语义的请求。
    pub fn to_request<R>(&self) -> ApiRequest<R> {
        <Self as CatalogEndpoint>::to_request(self)
    }
}

impl CatalogEndpoint for WikiApiV1 {
    fn to_url(&self) -> String {
        WikiApiV1::to_url(self)
    }

    fn method(&self) -> HttpMethod {
        HttpMethod::Post
    }

    // supported_access_token_types 使用 trait 默认实现（User + Tenant）
}

/// Wiki API V2 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum WikiApiV2 {
    /// 获取知识空间列表
    SpaceList,
    /// 获取知识空间信息
    SpaceGet(String),
    /// 创建知识空间
    SpaceCreate,
    /// 更新知识空间设置
    SpaceSettingUpdate(String),
    /// 获取知识空间节点信息
    SpaceGetNode,
    /// 获取知识空间子节点列表
    SpaceNodeList(String),
    /// 创建知识空间节点
    SpaceNodeCreate(String),
    /// 获取知识空间成员列表
    SpaceMemberList(String),
    /// 添加知识空间成员
    SpaceMemberCreate(String),
    /// 删除知识空间成员
    SpaceMemberDelete(String, String), // space_id, member_id
    /// 移动知识空间节点
    SpaceNodeMove(String, String),
    /// 更新知识空间节点标题
    SpaceNodeUpdateTitle(String, String),
    /// 创建知识空间节点副本
    SpaceNodeCopy(String, String),
    /// 移动云空间文档至知识空间
    SpaceNodeMoveDocsToWiki(String),
    /// 获取任务结果
    TaskGet(String),
}

impl WikiApiV2 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            WikiApiV2::SpaceList => "/open-apis/wiki/v2/spaces".to_string(),
            WikiApiV2::SpaceGet(space_id) => {
                format!("/open-apis/wiki/v2/spaces/{space_id}")
            }
            WikiApiV2::SpaceCreate => "/open-apis/wiki/v2/spaces".to_string(),
            WikiApiV2::SpaceSettingUpdate(space_id) => {
                format!("/open-apis/wiki/v2/spaces/{space_id}/setting")
            }
            WikiApiV2::SpaceGetNode => "/open-apis/wiki/v2/spaces/get_node".to_string(),
            WikiApiV2::SpaceNodeList(space_id) => {
                format!("/open-apis/wiki/v2/spaces/{space_id}/nodes")
            }
            WikiApiV2::SpaceNodeCreate(space_id) => {
                format!("/open-apis/wiki/v2/spaces/{space_id}/nodes")
            }
            WikiApiV2::SpaceMemberList(space_id) => {
                format!("/open-apis/wiki/v2/spaces/{space_id}/members")
            }
            WikiApiV2::SpaceMemberCreate(space_id) => {
                format!("/open-apis/wiki/v2/spaces/{space_id}/members")
            }
            WikiApiV2::SpaceMemberDelete(space_id, member_id) => {
                format!("/open-apis/wiki/v2/spaces/{space_id}/members/{member_id}")
            }
            WikiApiV2::SpaceNodeMove(space_id, node_token) => {
                format!("/open-apis/wiki/v2/spaces/{space_id}/nodes/{node_token}/move")
            }
            WikiApiV2::SpaceNodeUpdateTitle(space_id, node_token) => {
                format!("/open-apis/wiki/v2/spaces/{space_id}/nodes/{node_token}/update_title")
            }
            WikiApiV2::SpaceNodeCopy(space_id, node_token) => {
                format!("/open-apis/wiki/v2/spaces/{space_id}/nodes/{node_token}/copy")
            }
            WikiApiV2::SpaceNodeMoveDocsToWiki(space_id) => {
                format!("/open-apis/wiki/v2/spaces/{space_id}/nodes/move_docs_to_wiki")
            }
            WikiApiV2::TaskGet(task_id) => {
                format!("/open-apis/wiki/v2/tasks/{task_id}")
            }
        }
    }

    /// 返回配置了稳定请求语义的请求。
    pub fn to_request<R>(&self) -> ApiRequest<R> {
        <Self as CatalogEndpoint>::to_request(self)
    }
}

impl CatalogEndpoint for WikiApiV2 {
    fn to_url(&self) -> String {
        WikiApiV2::to_url(self)
    }

    fn method(&self) -> HttpMethod {
        match self {
            Self::SpaceList
            | Self::SpaceGet(_)
            | Self::SpaceGetNode
            | Self::SpaceNodeList(_)
            | Self::SpaceMemberList(_)
            | Self::TaskGet(_) => HttpMethod::Get,
            Self::SpaceCreate
            | Self::SpaceNodeCreate(_)
            | Self::SpaceMemberCreate(_)
            | Self::SpaceNodeMove(_, _)
            | Self::SpaceNodeUpdateTitle(_, _)
            | Self::SpaceNodeCopy(_, _)
            | Self::SpaceNodeMoveDocsToWiki(_) => HttpMethod::Post,
            Self::SpaceSettingUpdate(_) => HttpMethod::Put,
            Self::SpaceMemberDelete(_, _) => HttpMethod::Delete,
        }
    }

    // supported_access_token_types 使用 trait 默认实现（User + Tenant）
}

/// Wiki API 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum WikiApi {
    // Space APIs
    /// 获取知识空间列表
    ListSpaces,
    /// 获取知识空间信息
    GetSpace,
    /// 创建知识空间
    CreateSpace,

    // Space Member APIs
    /// 获取知识空间成员列表
    ListSpaceMembers(String), // space_id
    /// 添加知识空间成员
    CreateSpaceMember(String), // space_id
    /// 删除知识空间成员
    DeleteSpaceMember(String, String), // space_id, member_id

    // Space Setting APIs
    /// 更新知识空间设置
    UpdateSpaceSetting(String), // space_id

    // Space Node APIs
    /// 创建知识空间节点
    CreateSpaceNode(String), // space_id
    /// 获取知识空间节点信息
    GetSpaceNode,
    /// 获取知识空间子节点列表
    ListSpaceNodes,
    /// 移动知识空间节点
    MoveSpaceNode(String, String), // space_id, node_token
    /// 更新知识空间节点标题
    UpdateSpaceNodeTitle(String, String), // space_id, node_token
    /// 创建知识空间节点副本
    CopySpaceNode(String, String), // space_id, node_token
    /// 移动云空间文档至知识空间
    MoveDocsToWiki(String), // space_id

    // Task APIs
    /// 获取任务结果
    GetTask(String), // task_id

    // Node Search API (V1)
    /// 搜索Wiki节点
    SearchNodes,
}

impl WikiApi {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            // Space APIs
            WikiApi::ListSpaces => "/open-apis/wiki/v2/spaces".to_string(),
            WikiApi::GetSpace => "/open-apis/wiki/v2/spaces/get_node".to_string(),
            WikiApi::CreateSpace => "/open-apis/wiki/v2/spaces".to_string(),

            // Space Member APIs
            WikiApi::ListSpaceMembers(space_id) => {
                format!("/open-apis/wiki/v2/spaces/{space_id}/members")
            }
            WikiApi::CreateSpaceMember(space_id) => {
                format!("/open-apis/wiki/v2/spaces/{space_id}/members")
            }
            WikiApi::DeleteSpaceMember(space_id, member_id) => {
                format!("/open-apis/wiki/v2/spaces/{space_id}/members/{member_id}")
            }

            // Space Setting APIs
            WikiApi::UpdateSpaceSetting(space_id) => {
                format!("/open-apis/wiki/v2/spaces/{space_id}/setting")
            }

            // Space Node APIs
            WikiApi::CreateSpaceNode(space_id) => {
                format!("/open-apis/wiki/v2/spaces/{space_id}/nodes")
            }
            WikiApi::GetSpaceNode => "/open-apis/wiki/v2/spaces/get_node".to_string(),
            WikiApi::ListSpaceNodes => "/open-apis/wiki/v2/space.node/list".to_string(),
            WikiApi::MoveSpaceNode(space_id, node_token) => {
                format!("/open-apis/wiki/v2/spaces/{space_id}/nodes/{node_token}/move")
            }
            WikiApi::UpdateSpaceNodeTitle(space_id, node_token) => {
                format!("/open-apis/wiki/v2/spaces/{space_id}/nodes/{node_token}/update_title")
            }
            WikiApi::CopySpaceNode(space_id, node_token) => {
                format!("/open-apis/wiki/v2/spaces/{space_id}/nodes/{node_token}/copy")
            }
            WikiApi::MoveDocsToWiki(space_id) => {
                format!("/open-apis/wiki/v2/spaces/{space_id}/nodes/move_docs_to_wiki")
            }

            // Task APIs
            WikiApi::GetTask(task_id) => {
                format!("/open-apis/wiki/v2/tasks/{task_id}")
            }

            // Node Search API (V1)
            WikiApi::SearchNodes => "/open-apis/wiki/v1/nodes/search".to_string(),
        }
    }

    /// 返回配置了稳定请求语义的请求。
    pub fn to_request<R>(&self) -> ApiRequest<R> {
        <Self as CatalogEndpoint>::to_request(self)
    }
}

impl CatalogEndpoint for WikiApi {
    fn to_url(&self) -> String {
        WikiApi::to_url(self)
    }

    fn method(&self) -> HttpMethod {
        match self {
            Self::ListSpaces
            | Self::GetSpace
            | Self::ListSpaceMembers(_)
            | Self::GetSpaceNode
            | Self::ListSpaceNodes
            | Self::GetTask(_) => HttpMethod::Get,
            Self::CreateSpace
            | Self::CreateSpaceMember(_)
            | Self::CreateSpaceNode(_)
            | Self::MoveSpaceNode(_, _)
            | Self::UpdateSpaceNodeTitle(_, _)
            | Self::CopySpaceNode(_, _)
            | Self::MoveDocsToWiki(_)
            | Self::SearchNodes => HttpMethod::Post,
            Self::UpdateSpaceSetting(_) => HttpMethod::Put,
            Self::DeleteSpaceMember(_, _) => HttpMethod::Delete,
        }
    }

    // supported_access_token_types 使用 trait 默认实现（User + Tenant）
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::api_endpoints::test_support::assert_endpoint_semantics;

    #[test]
    fn wiki_catalog_covers_every_http_method_class() {
        assert_endpoint_semantics(
            WikiApiV1::NodeSearch,
            HttpMethod::Post,
            "/open-apis/wiki/v1/nodes/search",
        );

        assert_endpoint_semantics(
            WikiApiV2::SpaceGet("space".into()),
            HttpMethod::Get,
            "/open-apis/wiki/v2/spaces/space",
        );
        assert_endpoint_semantics(
            WikiApiV2::SpaceCreate,
            HttpMethod::Post,
            "/open-apis/wiki/v2/spaces",
        );
        assert_endpoint_semantics(
            WikiApiV2::SpaceSettingUpdate("space".into()),
            HttpMethod::Put,
            "/open-apis/wiki/v2/spaces/space/setting",
        );
        assert_endpoint_semantics(
            WikiApiV2::SpaceMemberDelete("space".into(), "member".into()),
            HttpMethod::Delete,
            "/open-apis/wiki/v2/spaces/space/members/member",
        );

        assert_endpoint_semantics(
            WikiApi::GetSpace,
            HttpMethod::Get,
            "/open-apis/wiki/v2/spaces/get_node",
        );
        assert_endpoint_semantics(
            WikiApi::CreateSpace,
            HttpMethod::Post,
            "/open-apis/wiki/v2/spaces",
        );
        assert_endpoint_semantics(
            WikiApi::UpdateSpaceSetting("space".into()),
            HttpMethod::Put,
            "/open-apis/wiki/v2/spaces/space/setting",
        );
        assert_endpoint_semantics(
            WikiApi::DeleteSpaceMember("space".into(), "member".into()),
            HttpMethod::Delete,
            "/open-apis/wiki/v2/spaces/space/members/member",
        );
    }
}
