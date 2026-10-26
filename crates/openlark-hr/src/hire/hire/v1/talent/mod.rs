//! Hire v1 talent 资源模块
//!
//! 包含候选人相关 API：
//! - 获取候选人列表
//! - 获取候选人信息
//! - 综合创建候选人
//! - 综合更新候选人
//! - 批量获取候选人 ID
//! - 将人才加入/移除文件夹
//! - 操作人才标签
//! - 获取候选人入职状态
//! - 候选人外部信息管理

/// add_to_folder 子模块。
pub mod add_to_folder;
/// batch_get_id 子模块。
pub mod batch_get_id;
/// combined_create 子模块。
pub mod combined_create;
/// combined_update 子模块。
pub mod combined_update;
/// external_info 子模块。
pub mod external_info;
/// get 子模块。
pub mod get;
/// list 子模块。
pub mod list;
/// models 子模块。
pub mod models;
/// onboard_status 子模块。
pub mod onboard_status;
/// remove_to_folder 子模块。
pub mod remove_to_folder;
/// tag 子模块。
pub mod tag;

// Re-export 公共类型
/// 添加人才到文件夹相关类型。
pub use add_to_folder::{AddToFolderRequest, AddToFolderResponse};
/// 批量获取人才 ID 请求类型。
pub use batch_get_id::BatchGetIdRequest;
/// 综合创建人才请求类型。
pub use combined_create::CombinedCreateRequest;
/// 综合更新人才请求类型。
pub use combined_update::CombinedUpdateRequest;
/// 获取人才详情请求类型。
pub use get::GetRequest;
/// 查询人才列表请求类型。
pub use list::ListRequest;
/// 人才资源通用模型。
pub use models::{
    BatchGetIdRequestBody, BatchGetIdResponse, CombinedCreateRequestBody, CombinedCreateResponse,
    CombinedUpdateRequestBody, CombinedUpdateResponse, GetRequestBody, GetResponse,
    ListRequestBody, ListResponse, Talent, TalentCombinedAwardInfo, TalentCombinedBasicInfo,
    TalentCombinedCareerInfo, TalentCombinedEducationInfo, TalentCombinedLanguageInfo,
    TalentCombinedProjectInfo, TalentCombinedSnsInfo, TalentCombinedWorkInfo, TalentContact,
    TalentCustomizedDataObjectValue, TalentCustomizedDataObjectValueChild, TalentEducation,
    TalentIdInfo, TalentIdentificationInfo, TalentSelfEvaluation, TalentWorkExperience,
};
/// 人才入职状态相关类型。
pub use onboard_status::{OnboardStatusRequest, OnboardStatusResponse};
/// 从文件夹移除人才相关类型。
pub use remove_to_folder::{RemoveToFolderRequest, RemoveToFolderResponse};
/// 人才标签操作相关类型。
pub use tag::{TagRequest, TagResponse};
