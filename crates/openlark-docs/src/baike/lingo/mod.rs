/// Lingo 模块
pub mod v1;

// 使用通配符导出所有子模块
// v1 模块显式导出
pub use v1::{
    Abbreviation, BaikeImage, Classification, ClassificationFilter, ClassificationRef,
    CreateDraftRequest, CreateDraftResp, CreateEntityRequest, CreateEntityResp,
    DeleteEntityRequest, DeleteEntityResp, DisplayStatus, DownloadFileRequest, Draft,
    DraftEntityBody, DraftEntityInput, DraftUpdateEntityBody, DraftUpdateEntityInput, Entity,
    EntityBody, EntityInput, GetEntityRequest, GetEntityResp, HighlightEntityBody,
    HighlightEntityRequest, HighlightEntityResp, I18nClsName, I18nEntryDesc, Language, LingoDraft,
    LingoEntity, ListClassificationRequest, ListClassificationResp, ListEntityRequest,
    ListEntityResp, ListRepoRequest, ListRepoResp, MatchEntityBody, MatchEntityRequest,
    MatchEntityResp, MatchInfo, OuterInfo, Phrase, RelatedChat, RelatedDoc, RelatedLink,
    RelatedMeta, RelatedOncall, RelatedUser, Repo, SearchEntityBody, SearchEntityRequest,
    SearchEntityResp, Span, Statistics, Term, TermType, UpdateDraftRequest, UpdateDraftResp,
    UpdateEntityRequest, UpdateEntityResp, UploadFileRequest, UploadFileResp, UserIdType,
};
