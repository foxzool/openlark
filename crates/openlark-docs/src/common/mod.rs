/// 文档服务 API 端点定义。
pub mod api_endpoints;
/// 文档服务 API 通用辅助函数。
pub mod api_utils;
/// 文档服务请求构建辅助模块。
pub mod builders;
/// 文档服务链式调用入口模块。
pub mod chain;
/// 文档服务通用请求构建器模块。
pub mod request_builder;
#[cfg(all(
    test,
    any(
        feature = "baike",
        feature = "lingo",
        feature = "ccm-core",
        feature = "minutes"
    )
))]
pub(crate) mod test_utils;

/// 重新导出常用 API 端点枚举。
pub use api_endpoints::{BaseApiV2, BitableApiV1, MinutesApiV1, SheetsApiV3};

/// 通用常量定义。
pub mod constants {
    /// 默认分页大小。
    pub const DEFAULT_PAGE_SIZE: i32 = 20;
    /// 最大分页大小。
    pub const MAX_PAGE_SIZE: i32 = 100;
    /// 默认超时时间（秒）。
    pub const DEFAULT_TIMEOUT_SECS: u64 = 30;
}

/// 通用类型别名。
pub mod types {
    /// 多维表格应用 Token。
    pub type AppToken = String;
    /// 数据表 ID。
    pub type TableId = String;
    /// 记录 ID。
    pub type RecordId = String;
    /// 表单 ID。
    pub type FormId = String;
    /// 视图 ID。
    pub type ViewId = String;
    /// 字段 ID。
    pub type FieldId = String;
    /// 角色 ID。
    pub type RoleId = String;
    /// 用户 ID。
    pub type UserId = String;
}

/// 批量操作相关类型。
pub mod batch {
    use serde::{Deserialize, Serialize};

    /// 通用批量操作参数。
    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    pub struct BatchCommonParams {
        /// 用户 ID 类型。
        pub user_id_type: Option<String>,
        /// 客户端幂等令牌。
        pub client_token: Option<String>,
    }

    impl BatchCommonParams {
        /// 创建默认批量操作参数。
        pub fn new() -> Self {
            Self::default()
        }

        /// 设置用户 ID 类型。
        pub fn with_user_id_type(mut self, user_id_type: impl ToString) -> Self {
            self.user_id_type = Some(user_id_type.to_string());
            self
        }

        /// 设置客户端幂等令牌。
        pub fn with_client_token(mut self, client_token: impl ToString) -> Self {
            self.client_token = Some(client_token.to_string());
            self
        }
    }

    /// 通用批量请求体。
    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    pub struct BatchCommonBody {
        /// 请求列表。
        pub requests: Vec<serde_json::Value>,
    }

    /// 通用批量操作结果。
    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    pub struct BatchOperationResult {
        /// 成功标识。
        pub success: bool,
        /// 结果列表。
        pub results: Vec<BatchItemResult>,
        /// 错误信息（如果有）。
        pub error: Option<String>,
    }

    impl BatchOperationResult {
        /// 创建成功结果。
        pub fn success() -> Self {
            Self {
                success: true,
                results: Vec::new(),
                error: None,
            }
        }

        /// 创建失败结果。
        pub fn failure(error: impl ToString) -> Self {
            Self {
                success: false,
                results: Vec::new(),
                error: Some(error.to_string()),
            }
        }
    }

    /// 批量操作中的单项结果。
    #[derive(Debug, Clone, Default, Serialize, Deserialize)]
    pub struct BatchItemResult {
        /// 成功标识。
        pub success: bool,
        /// 数据（如果有）。
        pub data: Option<serde_json::Value>,
        /// 错误信息（如果有）。
        pub error: Option<String>,
    }
}

/// 通用特征定义。
pub mod traits {
    /// API 请求基础特征。
    pub trait ApiRequest {
        /// 请求对应的响应类型。
        type Response;

        /// 校验请求参数。
        fn validate(&self) -> openlark_core::SDKResult<()>;
        /// 构建请求路径。
        fn build_path(&self) -> String;
    }

    /// 可分页请求特征。
    pub trait PaginatedRequest {
        /// 设置分页游标。
        fn page_token(self, token: impl Into<String>) -> Self;
        /// 设置分页大小。
        fn page_size(self, size: i32) -> Self;
    }

    /// 可筛选请求特征。
    pub trait FilterableRequest {
        /// 添加一个筛选条件。
        fn add_filter(self, filter: serde_json::Value) -> Self;
    }
}
