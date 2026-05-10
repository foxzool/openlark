//! 妙搭和飞书用户 ID 转换
//!
//! 文档：内部文档接口 `spark-v1/directory-user/id_convert`
//! docPath:

use crate::common::{
    api_utils::{extract_response_data, serialize_params},
    constants::endpoints::SPARK_V1_DIRECTORY_USER_ID_CONVERT,
};
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// 用户 ID 转换类型
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum UserIdConvertType {
    /// 妙搭用户 ID 转飞书开放平台 Open ID
    ForceUserIdToFeishuOpenId = 10,
    /// 妙搭用户 ID 转飞书开放平台 Union ID
    ForceUserIdToFeishuUnionId = 11,
    /// 飞书开放平台 Open ID 转妙搭用户 ID
    FeishuOpenIdToForceUserId = 20,
    /// 飞书开放平台 Union ID 转妙搭用户 ID
    FeishuUnionIdToForceUserId = 21,
}

/// 妙搭和飞书用户 ID 转换 Builder
#[derive(Debug, Clone)]
pub struct DirectoryUserIdConvertBuilder {
    config: Config,
    id_convert_type: UserIdConvertType,
    ids: Vec<String>,
}

impl DirectoryUserIdConvertBuilder {
    /// 创建新的 Builder
    pub fn new(config: Config, id_convert_type: UserIdConvertType) -> Self {
        Self {
            config,
            id_convert_type,
            ids: Vec::new(),
        }
    }

    /// 设置转换类型
    pub fn id_convert_type(mut self, id_convert_type: UserIdConvertType) -> Self {
        self.id_convert_type = id_convert_type;
        self
    }

    /// 添加单个待转换 ID
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.ids.push(id.into());
        self
    }

    /// 批量设置待转换 ID
    pub fn ids(mut self, ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.ids = ids.into_iter().map(Into::into).collect();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DirectoryUserIdConvertResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 使用请求选项执行请求
    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<DirectoryUserIdConvertResponse> {
        let request = DirectoryUserIdConvertRequest {
            id_convert_type: self.id_convert_type,
            ids: self.ids,
        };

        let req: ApiRequest<DirectoryUserIdConvertResponse> =
            ApiRequest::post(SPARK_V1_DIRECTORY_USER_ID_CONVERT)
                .body(serialize_params(&request, "转换妙搭和飞书用户 ID")?);
        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "转换妙搭和飞书用户 ID")
    }
}

/// 用户 ID 转换请求
#[derive(Debug, Clone, Serialize)]
struct DirectoryUserIdConvertRequest {
    /// ID 转换类型
    #[serde(rename = "id_convert_type")]
    id_convert_type: UserIdConvertType,
    /// 待转换 ID 列表
    #[serde(rename = "ids", skip_serializing_if = "Vec::is_empty")]
    ids: Vec<String>,
}

/// 单条 ID 映射
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DirectoryUserIdMapItem {
    /// 源 ID
    #[serde(rename = "source_id")]
    pub source_id: String,
    /// 目标 ID
    #[serde(rename = "target_id")]
    pub target_id: String,
}

/// 妙搭和飞书用户 ID 转换响应
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DirectoryUserIdConvertResponse {
    /// ID 映射结果
    #[serde(default, rename = "items")]
    pub items: Vec<DirectoryUserIdMapItem>,
}

impl ApiResponseTrait for DirectoryUserIdConvertResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_basic() {
        let config = Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build();

        let request = DirectoryUserIdConvertBuilder::new(
            config,
            UserIdConvertType::ForceUserIdToFeishuOpenId,
        )
        .id("123456789837364")
        .id("223456789837364");

        assert_eq!(request.ids.len(), 2);
        assert_eq!(
            request.id_convert_type,
            UserIdConvertType::ForceUserIdToFeishuOpenId
        );
    }

    #[test]
    fn test_request_serialization() {
        let request = DirectoryUserIdConvertRequest {
            id_convert_type: UserIdConvertType::FeishuUnionIdToForceUserId,
            ids: vec!["ou_xxx".to_string()],
        };

        let value = serde_json::to_value(&request).unwrap();
        assert_eq!(value["id_convert_type"], 21);
        assert_eq!(value["ids"][0], "ou_xxx");
    }

    #[test]
    fn test_response_deserialization() {
        let json = r#"{
            "items": [
                {
                    "source_id": "123445678933432",
                    "target_id": "ou_1234cdjhjfedgfhgdhy3884"
                }
            ]
        }"#;

        let response: DirectoryUserIdConvertResponse =
            serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(response.items.len(), 1);
        assert_eq!(response.items[0].source_id, "123445678933432");
        assert_eq!(response.items[0].target_id, "ou_1234cdjhjfedgfhgdhy3884");
    }
}
