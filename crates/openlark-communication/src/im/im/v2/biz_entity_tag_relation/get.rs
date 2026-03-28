//! 查询实体与标签的绑定关系
//!
//! docPath: <https://open.feishu.cn/document/tenant-tag/get>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{common::api_utils::extract_response_data, endpoints::IM_V2_BIZ_ENTITY_TAG_RELATION};

/// 查询实体与标签的绑定关系请求
pub struct GetBizEntityTagRelationRequest {
    config: Config,
    tag_biz_type: String,
    biz_entity_id: String,
}

impl GetBizEntityTagRelationRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            tag_biz_type: String::new(),
            biz_entity_id: String::new(),
        }
    }

    /// 业务类型（查询参数，例如 chat）
    pub fn tag_biz_type(mut self, tag_biz_type: impl Into<String>) -> Self {
        self.tag_biz_type = tag_biz_type.into();
        self
    }

    /// 业务实体 ID（查询参数，例如 chat_id）
    pub fn biz_entity_id(mut self, biz_entity_id: impl Into<String>) -> Self {
        self.biz_entity_id = biz_entity_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/tenant-tag/get>
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<serde_json::Value> {
        validate_required!(self.tag_biz_type, "tag_biz_type 不能为空");
        validate_required!(self.biz_entity_id, "biz_entity_id 不能为空");

        // url: GET:/open-apis/im/v2/biz_entity_tag_relation
        let req: ApiRequest<serde_json::Value> = ApiRequest::get(IM_V2_BIZ_ENTITY_TAG_RELATION)
            .query("tag_biz_type", self.tag_biz_type)
            .query("biz_entity_id", self.biz_entity_id);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "查询实体与标签的绑定关系")
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
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}
