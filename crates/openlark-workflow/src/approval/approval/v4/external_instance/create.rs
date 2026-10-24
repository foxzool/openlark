//! 同步三方审批实例（v4）
//!
//! docPath: <https://open.feishu.cn/document/server-docs/approval-v4/external_instance/create>

use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::approval::approval::v4::models::I18nResourceV4;
use crate::common::api_utils::request_serialization_error;

/// 三方审批实例跳转链接（external_instance_link）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExternalInstanceLinkV4 {
    /// PC 端的跳转链接（pc_link 与 mobile_link 至少填一个）
    pub pc_link: String,
    /// 移动端的跳转链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_link: Option<String>,
}

/// 表单数据条目（external_instance_form）
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FormValue {
    /// 表单字段名称（国际化文案 Key）
    pub name: String,
    /// 表单值（国际化文案 Key）
    pub value: String,
}

/// 任务级快捷审批操作配置（action_config）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExternalInstanceActionConfigV4 {
    /// 操作类型（APPROVE/REJECT/任意字符串）
    pub action_type: String,
    /// 操作名称（action_type 为任意字符串时必填，国际化文案 Key）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_name: Option<String>,
    /// 是否需要审批意见
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_need_reason: Option<bool>,
    /// 审批意见是否必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_reason_required: Option<bool>,
    /// 审批意见是否支持上传附件
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_need_attachment: Option<bool>,
}

/// 三方审批任务节点（external_instance_task_node）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExternalInstanceTaskNodeV4 {
    /// 审批实例内审批任务的唯一标识
    pub task_id: String,
    /// 审批人 user_id（user_id 与 open_id 至少传入一个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 审批人 open_id（user_id 与 open_id 至少传入一个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    /// 审批任务名称（国际化文案 Key）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 三方审批跳转链接
    pub links: ExternalInstanceLinkV4,
    /// 任务状态（PENDING/APPROVED/REJECTED/TRANSFERRED/DONE）
    pub status: String,
    /// 扩展字段（JSON 压缩转义字符串，可含 complete_reason）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
    /// 任务创建时间（Unix 毫秒时间戳）
    pub create_time: String,
    /// 任务完成时间（未结束为 0）
    pub end_time: String,
    /// 任务最近更新时间（Unix 毫秒时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 操作上下文
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_context: Option<String>,
    /// 任务级快捷审批操作配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_configs: Option<Vec<ExternalInstanceActionConfigV4>>,
    /// 列表页打开审批任务的方式（BROWSER/SIDEBAR/NORMAL/TRUSTEESHIP）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_method: Option<String>,
    /// 是否不纳入效率统计
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude_statistics: Option<bool>,
    /// 审批节点 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// 节点名称（国际化文案 Key）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_name: Option<String>,
    /// 任务生成类型（EXTERNAL_CONSIGN/DEFAULT）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generate_type: Option<String>,
}

/// 三方审批抄送节点（cc_node）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ExternalInstanceCcNodeV4 {
    /// 审批实例内抄送唯一标识
    pub cc_id: String,
    /// 抄送人 user_id（user_id 与 open_id 至少传入一个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 抄送人 open_id（user_id 与 open_id 至少传入一个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    /// 审批抄送跳转链接
    pub links: ExternalInstanceLinkV4,
    /// 抄送人的阅读状态（READ/UNREAD）
    pub read_status: String,
    /// 扩展字段（JSON 压缩转义字符串）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
    /// 抄送任务名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 抄送发起时间（Unix 毫秒时间戳）
    pub create_time: String,
    /// 抄送最近更新时间（Unix 毫秒时间戳）
    pub update_time: String,
    /// 列表页打开审批任务的方式（BROWSER/SIDEBAR/NORMAL/TRUSTEESHIP）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_method: Option<String>,
}

/// 单据托管回调接入方的接口 URL 地址（trusteeship_urls）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TrusteeshipUrlsV4 {
    /// 获取表单 schema 相关数据的 URL 地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_detail_url: Option<String>,
    /// 获取审批操作区数据的 URL 地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_definition_url: Option<String>,
    /// 获取审批记录相关数据的 URL 地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_node_url: Option<String>,
    /// 进行审批操作时回调的 URL 地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_callback_url: Option<String>,
    /// 获取托管动态数据的 URL 地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pull_business_data_url: Option<String>,
}

/// 托管预缓存策略（trusteeship_instance_cache_config）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TrusteeshipInstanceCacheConfigV4 {
    /// 托管预缓存策略（DISABLE/IMMUTABLE/BY_NODE/BY_USER）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_policy: Option<String>,
    /// 表单是否随国际化改变
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_vary_with_locale: Option<bool>,
    /// 当前使用的表单版本号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form_version: Option<String>,
}

/// 同步三方审批实例请求体（v4）
#[derive(Debug, Clone, Serialize, Default)]
pub struct CreateExternalInstanceBodyV4 {
    /// 审批定义 Code
    pub approval_code: String,
    /// 审批实例状态（PENDING/APPROVED/REJECTED/CANCELED/DELETED/HIDDEN/TERMINATED）
    pub status: String,
    /// 审批实例扩展参数（JSON 压缩转义字符串，单据编号通过 business_key 实现）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
    /// 审批实例唯一标识，自定义设置
    pub instance_id: String,
    /// 审批实例链接信息
    pub links: ExternalInstanceLinkV4,
    /// 审批展示名称（国际化文案 Key）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 用户提交审批时填写的表单数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<Vec<FormValue>>,
    /// 审批发起人 user_id（open_id 与 user_id 需至少传入一个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 审批发起人的用户名（国际化文案 Key）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,
    /// 审批发起人 open_id（open_id 与 user_id 需至少传入一个）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    /// 发起人的部门 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 审批发起人的部门名称（国际化文案 Key）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_name: Option<String>,
    /// 审批发起时间（Unix 毫秒时间戳）
    pub start_time: String,
    /// 审批实例结束时间（未结束为 0）
    pub end_time: String,
    /// 审批实例最近更新时间（Unix 毫秒时间戳）
    pub update_time: String,
    /// 列表页打开审批实例的方式（BROWSER/SIDEBAR/NORMAL/TRUSTEESHIP）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_method: Option<String>,
    /// 更新方式（REPLACE 全量替换 / UPDATE 增量更新，默认 REPLACE）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_mode: Option<String>,
    /// 任务列表（最大 300）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_list: Option<Vec<ExternalInstanceTaskNodeV4>>,
    /// 抄送列表（最大 200）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc_list: Option<Vec<ExternalInstanceCcNodeV4>>,
    /// 国际化文案
    pub i18n_resources: Vec<I18nResourceV4>,
    /// 单据托管认证 token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusteeship_url_token: Option<String>,
    /// 用户标识类型（目前仅支持 user_id）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusteeship_user_id_type: Option<String>,
    /// 单据托管回调接入方的接口 URL 地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusteeship_urls: Option<TrusteeshipUrlsV4>,
    /// 托管预缓存策略
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trusteeship_cache_config: Option<TrusteeshipInstanceCacheConfigV4>,
    /// 资源所在地区（内部统计用，不需要填）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_region: Option<String>,
}

/// 同步三方审批实例响应（v4）
///
/// 官方响应 `data` 内再嵌套一层 `data` 字段（external_instance 回显，
/// 字段与请求体同构），此处以 JSON 透传该回显数据。
#[derive(Debug, Clone, Deserialize, Default)]
pub struct CreateExternalInstanceResponseV4 {
    /// 同步的实例数据（与请求体同构的 external_instance 回显）
    #[serde(default)]
    pub data: Option<serde_json::Value>,
}

/// 同步三方审批实例请求（v4）
#[derive(Debug, Clone)]
pub struct CreateExternalInstanceRequestV4 {
    config: Arc<Config>,
    body: CreateExternalInstanceBodyV4,
}

impl CreateExternalInstanceRequestV4 {
    /// 创建请求实例。
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: CreateExternalInstanceBodyV4::default(),
        }
    }

    /// 设置审批定义 Code
    pub fn approval_code(mut self, approval_code: impl Into<String>) -> Self {
        self.body.approval_code = approval_code.into();
        self
    }

    /// 设置审批实例状态
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.body.status = status.into();
        self
    }

    /// 设置审批实例扩展参数（JSON 压缩转义字符串）
    pub fn extra(mut self, extra: impl Into<String>) -> Self {
        self.body.extra = Some(extra.into());
        self
    }

    /// 设置审批实例唯一标识
    pub fn instance_id(mut self, instance_id: impl Into<String>) -> Self {
        self.body.instance_id = instance_id.into();
        self
    }

    /// 设置审批实例链接信息
    pub fn links(mut self, links: ExternalInstanceLinkV4) -> Self {
        self.body.links = links;
        self
    }

    /// 设置审批展示名称（国际化文案 Key）
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.body.title = Some(title.into());
        self
    }

    /// 设置表单数据列表
    pub fn form(mut self, form: Vec<FormValue>) -> Self {
        self.body.form = Some(form);
        self
    }

    /// 添加表单数据条目
    pub fn add_form_value(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        let form = self.body.form.get_or_insert_with(Vec::new);
        form.push(FormValue {
            name: name.into(),
            value: value.into(),
        });
        self
    }

    /// 设置审批发起人 user_id
    pub fn user_id(mut self, user_id: impl Into<String>) -> Self {
        self.body.user_id = Some(user_id.into());
        self
    }

    /// 设置审批发起人的用户名（国际化文案 Key）
    pub fn user_name(mut self, user_name: impl Into<String>) -> Self {
        self.body.user_name = Some(user_name.into());
        self
    }

    /// 设置审批发起人 open_id
    pub fn open_id(mut self, open_id: impl Into<String>) -> Self {
        self.body.open_id = Some(open_id.into());
        self
    }

    /// 设置发起人的部门 ID
    pub fn department_id(mut self, department_id: impl Into<String>) -> Self {
        self.body.department_id = Some(department_id.into());
        self
    }

    /// 设置审批发起人的部门名称（国际化文案 Key）
    pub fn department_name(mut self, department_name: impl Into<String>) -> Self {
        self.body.department_name = Some(department_name.into());
        self
    }

    /// 设置审批发起时间（Unix 毫秒时间戳）
    pub fn start_time(mut self, start_time: impl Into<String>) -> Self {
        self.body.start_time = start_time.into();
        self
    }

    /// 设置审批实例结束时间（未结束为 0）
    pub fn end_time(mut self, end_time: impl Into<String>) -> Self {
        self.body.end_time = end_time.into();
        self
    }

    /// 设置审批实例最近更新时间（Unix 毫秒时间戳）
    pub fn update_time(mut self, update_time: impl Into<String>) -> Self {
        self.body.update_time = update_time.into();
        self
    }

    /// 设置列表页打开审批实例的方式
    pub fn display_method(mut self, display_method: impl Into<String>) -> Self {
        self.body.display_method = Some(display_method.into());
        self
    }

    /// 设置更新方式（REPLACE/UPDATE）
    pub fn update_mode(mut self, update_mode: impl Into<String>) -> Self {
        self.body.update_mode = Some(update_mode.into());
        self
    }

    /// 设置任务列表
    pub fn task_list(mut self, task_list: Vec<ExternalInstanceTaskNodeV4>) -> Self {
        self.body.task_list = Some(task_list);
        self
    }

    /// 设置抄送列表
    pub fn cc_list(mut self, cc_list: Vec<ExternalInstanceCcNodeV4>) -> Self {
        self.body.cc_list = Some(cc_list);
        self
    }

    /// 设置国际化文案
    pub fn i18n_resources(mut self, i18n_resources: Vec<I18nResourceV4>) -> Self {
        self.body.i18n_resources = i18n_resources;
        self
    }

    /// 设置单据托管认证 token
    pub fn trusteeship_url_token(mut self, token: impl Into<String>) -> Self {
        self.body.trusteeship_url_token = Some(token.into());
        self
    }

    /// 设置用户标识类型（目前仅支持 user_id）
    pub fn trusteeship_user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.body.trusteeship_user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置单据托管回调接口 URL 地址
    pub fn trusteeship_urls(mut self, trusteeship_urls: TrusteeshipUrlsV4) -> Self {
        self.body.trusteeship_urls = Some(trusteeship_urls);
        self
    }

    /// 设置托管预缓存策略
    pub fn trusteeship_cache_config(mut self, config: TrusteeshipInstanceCacheConfigV4) -> Self {
        self.body.trusteeship_cache_config = Some(config);
        self
    }

    /// 设置资源所在地区
    pub fn resource_region(mut self, resource_region: impl Into<String>) -> Self {
        self.body.resource_region = Some(resource_region.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateExternalInstanceResponseV4> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateExternalInstanceResponseV4> {
        validate_required!(self.body.approval_code, "审批定义 Code 不能为空");
        validate_required!(self.body.status, "审批实例状态不能为空");
        validate_required!(self.body.instance_id, "审批实例唯一标识不能为空");
        validate_required!(self.body.start_time, "审批发起时间不能为空");
        validate_required!(self.body.end_time, "审批实例结束时间不能为空");
        validate_required!(self.body.update_time, "审批实例最近更新时间不能为空");
        validate_required!(self.body.i18n_resources, "国际化文案不能为空");

        let api_endpoint = crate::common::api_endpoints::ApprovalApiV4::ExternalInstanceCreate;
        let mut request =
            ApiRequest::<CreateExternalInstanceResponseV4>::post(api_endpoint.to_url());

        let body_json = serde_json::to_value(&self.body)
            .map_err(|e| request_serialization_error("同步三方审批实例", e))?;

        request = request.body(body_json);

        openlark_core::http::Transport::request_typed(
            request,
            &self.config,
            Some(option),
            "同步三方审批实例",
        )
        .await
    }
}

impl ApiResponseTrait for CreateExternalInstanceResponseV4 {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_external_instance_create_v4_url() {
        let endpoint = crate::common::api_endpoints::ApprovalApiV4::ExternalInstanceCreate;
        assert_eq!(
            endpoint.to_url(),
            "/open-apis/approval/v4/external_instances"
        );
    }

    #[test]
    fn test_create_external_instance_request_builder() {
        let config = Arc::new(
            Config::builder()
                .app_id("test_app")
                .app_secret("test_secret")
                .build(),
        );
        let request = CreateExternalInstanceRequestV4::new(config)
            .approval_code("81D31358-93AF-92D6-7425-01A5D67C4E71")
            .status("PENDING")
            .instance_id("24492654")
            .links(ExternalInstanceLinkV4 {
                pc_link: "https://example.com/pc".to_string(),
                mobile_link: None,
            })
            .start_time("1556468012678")
            .end_time("0")
            .update_time("1556468012678")
            .add_form_value("@i18n@2", "@i18n@3")
            .i18n_resources(vec![I18nResourceV4::default()]);

        assert_eq!(
            request.body.approval_code,
            "81D31358-93AF-92D6-7425-01A5D67C4E71"
        );
        assert_eq!(request.body.status, "PENDING");
        assert_eq!(request.body.instance_id, "24492654");
        assert_eq!(request.body.links.pc_link, "https://example.com/pc");
        assert_eq!(
            request.body.form,
            Some(vec![FormValue {
                name: "@i18n@2".to_string(),
                value: "@i18n@3".to_string(),
            }])
        );
    }
}
