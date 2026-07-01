//! 获取职位上的招聘人员信息
//!
//! docPath: <https://open.feishu.cn/document/server-docs/hire-v1/job.manager/get>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::hire::hire::common_models::JobRecruiterRecord;

/// 获取职位上的招聘人员信息请求
#[derive(Debug, Clone)]
pub struct GetRequest {
    job_id: String,
    manager_id: String,
    /// 配置信息
    config: Config,
}

impl GetRequest {
    /// 创建请求
    pub fn new(config: Config, job_id: String) -> Self {
        Self {
            job_id,
            manager_id: String::new(),
            config,
        }
    }

    /// 设置 `manager_id`。
    pub fn manager_id(mut self, manager_id: String) -> Self {
        self.manager_id = manager_id;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 使用指定请求选项执行请求。
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetResponse> {
        validate_required!(self.job_id.trim(), "职位 ID 不能为空");
        validate_required!(self.manager_id.trim(), "招聘人员 ID 不能为空");

        let request = ApiRequest::<GetResponse>::get(format!(
            "/open-apis/hire/v1/jobs/{}/managers/{}",
            self.job_id, self.manager_id
        ));
        let response = Transport::request(request, &self.config, Some(option)).await?;

        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "获取职位上的招聘人员信息响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 获取职位上的招聘人员信息响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    /// `manager` 字段。
    pub manager: Option<JobRecruiterRecord>,
    #[serde(default, flatten)]
    /// 扩展字段。
    pub extra: HashMap<String, Value>,
}

impl ApiResponseTrait for GetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}
