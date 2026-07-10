//! 知识库（FAQ）模块
//!
//! 提供客服知识库相关的 API。

pub mod create;
/// 删除接口。
pub mod delete;
/// 获取接口。
pub mod get;
/// image 模块。
pub mod image;
/// 列表接口。
pub mod list;
/// 更新接口。
pub mod patch;
/// search 模块。
pub mod search;

use openlark_core::config::Config;
use std::sync::Arc;

/// 知识库服务
#[derive(Clone)]
pub struct Faq {
    config: Arc<Config>,
}

impl Faq {
    /// 创建新的知识库服务实例
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 获取知识库列表
    pub fn list(&self) -> list::ListFaqRequest {
        list::ListFaqRequest::new(self.config.clone())
    }

    /// 创建知识库
    pub fn create(&self) -> create::CreateFaqRequest {
        create::CreateFaqRequest::new(self.config.clone())
    }

    /// 获取指定知识库
    pub fn get(&self, id: impl Into<String>) -> get::GetFaqRequest {
        get::GetFaqRequest::new(self.config.clone(), id.into())
    }

    /// 更新指定知识库
    pub fn patch(&self, id: impl Into<String>) -> patch::PatchFaqRequest {
        patch::PatchFaqRequest::new(self.config.clone(), id.into())
    }

    /// 删除指定知识库
    pub fn delete(&self, id: impl Into<String>) -> delete::DeleteFaqRequest {
        delete::DeleteFaqRequest::new(self.config.clone(), id.into())
    }

    /// 搜索知识库
    pub fn search(&self) -> search::SearchFaqRequest {
        search::SearchFaqRequest::new(self.config.clone())
    }

    /// 获取知识库图片
    pub fn image(
        &self,
        id: impl Into<String>,
        image_key: impl Into<String>,
    ) -> image::GetFaqImageRequest {
        image::GetFaqImageRequest::new(self.config.clone(), id.into(), image_key.into())
    }
}

pub use create::{CreateFaqRequest, CreateFaqRequestBuilder};
pub use delete::{DeleteFaqRequest, DeleteFaqRequestBuilder};
pub use get::{GetFaqRequest, GetFaqRequestBuilder};
pub use image::{GetFaqImageRequest, GetFaqImageRequestBuilder};
pub use list::{ListFaqRequest, ListFaqRequestBuilder};
pub use patch::{PatchFaqRequest, PatchFaqRequestBuilder};
pub use search::{SearchFaqRequest, SearchFaqRequestBuilder};
