/// 待补充文档。
pub mod approval;
pub mod district;
/// 待补充文档。
pub mod external_approval;
/// 待补充文档。
pub mod external_instance;
/// 待补充文档。
pub mod external_task;
/// 待补充文档。
pub mod instance;
/// 待补充文档。
pub mod task;

// approval 模块显式导出

pub use approval::{
    CreateApprovalBodyV4, CreateApprovalRequestV4, CreateApprovalResponseV4, GetApprovalRequestV4,
    GetApprovalResponseV4, SubscribeApprovalRequestV4, SubscribeApprovalResponseV4,
    UnsubscribeApprovalRequestV4, UnsubscribeApprovalResponseV4,
};
// external_approval 模块显式导出
pub use external_approval::{
    CreateExternalApprovalBodyV4, CreateExternalApprovalRequestV4,
    CreateExternalApprovalResponseV4, FormField, GetExternalApprovalRequestV4,
    GetExternalApprovalResponseV4,
};
// external_instance 模块显式导出
pub use external_instance::{
    CheckExternalInstanceBodyV4, CheckExternalInstanceRequestV4, CheckExternalInstanceResponseV4,
    CreateExternalInstanceBodyV4, CreateExternalInstanceRequestV4,
    CreateExternalInstanceResponseV4, FormValue,
};
// external_task 模块显式导出
pub use external_task::{
    ExternalTask, ListExternalTaskBodyV4, ListExternalTaskRequestV4, ListExternalTaskResponseV4,
};
// instance 模块显式导出
pub use instance::{
    AddSignBodyV4, AddSignRequestV4, AddSignResponseV4, CancelInstanceBodyV4,
    CancelInstanceRequestV4, CancelInstanceResponseV4, CcInstanceBodyV4, CcInstanceRequestV4,
    CcInstanceResponseV4, CcItemV4, CreateInstanceBodyV4, CreateInstanceCommentBodyV4,
    CreateInstanceCommentRequestV4, CreateInstanceCommentResponseV4, CreateInstanceRequestV4,
    CreateInstanceResponseV4, DeleteInstanceCommentRequestV4, DeleteInstanceCommentResponseV4,
    FlowNode, GetInstanceRequestV4, GetInstanceResponseV4, InstanceComment, InstanceItemV4,
    ListInstanceCommentRequestV4, ListInstanceCommentResponseV4, ListInstanceRequestV4,
    ListInstanceResponseV4, PreviewInstanceBodyV4, PreviewInstanceRequestV4,
    PreviewInstanceResponseV4, QueryInstanceRequestV4, QueryInstanceResponseV4,
    RemoveInstanceCommentRequestV4, RemoveInstanceCommentResponseV4, SearchCcRequestV4,
    SearchCcResponseV4, SpecifiedRollbackBodyV4, SpecifiedRollbackRequestV4,
    SpecifiedRollbackResponseV4,
};
// task 模块显式导出
pub use task::{
    ApproveTaskBodyV4, ApproveTaskRequestV4, ApproveTaskResponseV4, QueryTaskRequestV4,
    QueryTaskResponseV4, RejectTaskBodyV4, RejectTaskRequestV4, RejectTaskResponseV4,
    ResubmitTaskBodyV4, ResubmitTaskRequestV4, ResubmitTaskResponseV4, SearchTaskRequestV4,
    SearchTaskResponseV4, TaskItemV4, TransferTaskBodyV4, TransferTaskRequestV4,
    TransferTaskResponseV4,
};
