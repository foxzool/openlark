//! 应用对象相关 API

pub mod oql_query;
pub mod record;
pub mod search;

pub use oql_query::OqlQueryRequestBuilder;
pub use record::batch_create as record_batch_create;
pub use record::batch_delete as record_batch_delete;
pub use record::batch_query as record_batch_query;
pub use record::batch_update as record_batch_update;
pub use record::create as record_create;
pub use record::delete as record_delete;
pub use record::patch as record_patch;
pub use record::query as record_query;
pub use search::RecordSearchRequestBuilder;
