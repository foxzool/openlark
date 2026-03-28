//! 绑定标签到群
//!
//! docPath: <https://open.feishu.cn/document/tenant-tag/create-2>

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::{
        api_utils::{extract_response_data, serialize_params},
        models::EmptyData,
    },
    endpoints::IM_V2_BIZ_ENTITY_TAG_RELATION,
    im::im::v2::biz_entity_tag_relation::models::BizEntityTagRelationBody,
};

/// 绑定标签到群请求
pub struct CreateBizEntityTagRelationRequest {
    config: Config,
}

impl CreateBizEntityTagRelationRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// docPath: <https://open.feishu.cn/document/tenant-tag/create-2>
    pub async fn execute(self, body: BizEntityTagRelationBody) -> SDKResult<EmptyData> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: BizEntityTagRelationBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EmptyData> {
        validate_required!(body.tag_biz_type, "tag_biz_type 不能为空");
        validate_required!(body.biz_entity_id, "biz_entity_id 不能为空");

        // url: POST:/open-apis/im/v2/biz_entity_tag_relation
        let req: ApiRequest<EmptyData> = ApiRequest::post(IM_V2_BIZ_ENTITY_TAG_RELATION)
            .body(serialize_params(&body, "绑定标签到群")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "绑定标签到群")
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
