//! 将人才从指定文件夹移除
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/talent/remove_to_folder>

use openlark_core::{
    api::{ApiResponseTrait, ResponseFormat},
    config::Config,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 将人才从指定文件夹移除请求
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct RemoveToFolderRequest {
    /// 配置信息
    config: Config,
    // TODO: 添加请求字段
}

impl RemoveToFolderRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            // TODO: 初始化字段
        }
    }

    // TODO: 添加字段 setter 方法

    /// 执行请求
    pub async fn execute(self) -> SDKResult<RemoveToFolderResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        _option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<RemoveToFolderResponse> {
        // TODO: 实现 API 调用逻辑
        todo!("实现 将人才从指定文件夹移除 API 调用")
    }
}

/// 将人才从指定文件夹移除响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RemoveToFolderResponse {
    /// 响应数据
    ///
    /// TODO: 根据官方文档添加具体字段
    pub data: Value,
}

impl ApiResponseTrait for RemoveToFolderResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use openlark_core::testing::prelude::TestConfigBuilder;

    #[test]
    fn test_remove_to_folder_request_builder_new() {
        let request = RemoveToFolderRequest::new(TestConfigBuilder::new().build());
        let _ = request;
    }
}
