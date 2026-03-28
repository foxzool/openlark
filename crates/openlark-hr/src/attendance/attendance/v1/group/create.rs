//! 创建或修改考勤组
//!
//! docPath: <https://open.feishu.cn/document/server-docs/attendance-v1/group/create>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};

use super::models::{
    CreateGroupRequestBody, CreateGroupResponse, OvertimeInfo, ShiftInfo, WorkDayConfig,
};

/// 创建考勤组请求
#[derive(Debug, Clone)]
pub struct CreateGroupRequest {
    /// 考勤组 ID（修改考勤组时必填）
    group_id: Option<String>,
    /// 考勤组名称（必填）
    group_name: String,
    /// 考勤组类型（必填）
    /// - 0: 固定班制
    /// - 1: 排班制
    /// - 2: 自由班制
    group_type: i32,
    /// 参与考勤人员列表
    user_list: Option<Vec<String>>,
    /// 无需考勤人员列表
    excluded_user_list: Option<Vec<String>>,
    /// 考勤负责人列表
    manager_list: Option<Vec<String>>,
    /// 考勤组绑定的部门列表
    dept_list: Option<Vec<String>>,
    /// 考勤组绑定的班次列表（固定班制必填）
    shift_list: Option<Vec<ShiftInfo>>,
    /// 是否允许外勤打卡
    allow_out_punch: Option<bool>,
    /// 外勤打卡是否需要审批
    out_punch_need_approval: Option<bool>,
    /// 是否允许 PC 端打卡
    allow_pc_punch: Option<bool>,
    /// 是否需要拍照打卡
    need_photo: Option<bool>,
    /// 拍照打卡类型
    photo_punch_type: Option<i32>,
    /// 是否允许补卡
    allow_remedy: Option<bool>,
    /// 补卡限制次数
    remedy_limit: Option<i32>,
    /// 补卡限制周期（单位：天）
    remedy_period: Option<i32>,
    /// 工作日设置
    work_day_config: Option<Vec<WorkDayConfig>>,
    /// 加班规则配置
    overtime_info: Option<OvertimeInfo>,
    /// 配置信息
    config: Config,
}

impl CreateGroupRequest {
    /// 创建请求
    pub fn new(config: Config) -> Self {
        Self {
            group_id: None,
            group_name: String::new(),
            group_type: 0,
            user_list: None,
            excluded_user_list: None,
            manager_list: None,
            dept_list: None,
            shift_list: None,
            allow_out_punch: None,
            out_punch_need_approval: None,
            allow_pc_punch: None,
            need_photo: None,
            photo_punch_type: None,
            allow_remedy: None,
            remedy_limit: None,
            remedy_period: None,
            work_day_config: None,
            overtime_info: None,
            config,
        }
    }

    /// 设置考勤组 ID（修改考勤组时必填）
    pub fn group_id(mut self, group_id: String) -> Self {
        self.group_id = Some(group_id);
        self
    }

    /// 设置考勤组名称（必填）
    pub fn group_name(mut self, group_name: String) -> Self {
        self.group_name = group_name;
        self
    }

    /// 设置考勤组类型（必填）
    /// - 0: 固定班制
    /// - 1: 排班制
    /// - 2: 自由班制
    pub fn group_type(mut self, group_type: i32) -> Self {
        self.group_type = group_type;
        self
    }

    /// 设置参与考勤人员列表
    pub fn user_list(mut self, user_list: Vec<String>) -> Self {
        self.user_list = Some(user_list);
        self
    }

    /// 设置无需考勤人员列表
    pub fn excluded_user_list(mut self, excluded_user_list: Vec<String>) -> Self {
        self.excluded_user_list = Some(excluded_user_list);
        self
    }

    /// 设置考勤负责人列表
    pub fn manager_list(mut self, manager_list: Vec<String>) -> Self {
        self.manager_list = Some(manager_list);
        self
    }

    /// 设置考勤组绑定的部门列表
    pub fn dept_list(mut self, dept_list: Vec<String>) -> Self {
        self.dept_list = Some(dept_list);
        self
    }

    /// 设置考勤组绑定的班次列表（固定班制必填）
    pub fn shift_list(mut self, shift_list: Vec<ShiftInfo>) -> Self {
        self.shift_list = Some(shift_list);
        self
    }

    /// 设置是否允许外勤打卡
    pub fn allow_out_punch(mut self, allow_out_punch: bool) -> Self {
        self.allow_out_punch = Some(allow_out_punch);
        self
    }

    /// 设置外勤打卡是否需要审批
    pub fn out_punch_need_approval(mut self, out_punch_need_approval: bool) -> Self {
        self.out_punch_need_approval = Some(out_punch_need_approval);
        self
    }

    /// 设置是否允许 PC 端打卡
    pub fn allow_pc_punch(mut self, allow_pc_punch: bool) -> Self {
        self.allow_pc_punch = Some(allow_pc_punch);
        self
    }

    /// 设置是否需要拍照打卡
    pub fn need_photo(mut self, need_photo: bool) -> Self {
        self.need_photo = Some(need_photo);
        self
    }

    /// 设置拍照打卡类型
    pub fn photo_punch_type(mut self, photo_punch_type: i32) -> Self {
        self.photo_punch_type = Some(photo_punch_type);
        self
    }

    /// 设置是否允许补卡
    pub fn allow_remedy(mut self, allow_remedy: bool) -> Self {
        self.allow_remedy = Some(allow_remedy);
        self
    }

    /// 设置补卡限制次数
    pub fn remedy_limit(mut self, remedy_limit: i32) -> Self {
        self.remedy_limit = Some(remedy_limit);
        self
    }

    /// 设置补卡限制周期（单位：天）
    pub fn remedy_period(mut self, remedy_period: i32) -> Self {
        self.remedy_period = Some(remedy_period);
        self
    }

    /// 设置工作日设置
    pub fn work_day_config(mut self, work_day_config: Vec<WorkDayConfig>) -> Self {
        self.work_day_config = Some(work_day_config);
        self
    }

    /// 设置加班规则配置
    pub fn overtime_info(mut self, overtime_info: OvertimeInfo) -> Self {
        self.overtime_info = Some(overtime_info);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateGroupResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateGroupResponse> {
        use crate::common::api_endpoints::AttendanceApiV1;

        // 1. 验证必填字段
        validate_required!(self.group_name.trim(), "考勤组名称不能为空");

        // 2. 构建端点
        let api_endpoint = AttendanceApiV1::GroupCreate;
        let request = ApiRequest::<CreateGroupResponse>::post(api_endpoint.to_url());

        // 3. 序列化请求体
        let request_body = CreateGroupRequestBody {
            group_id: self.group_id,
            group_name: self.group_name,
            group_type: self.group_type,
            user_list: self.user_list,
            excluded_user_list: self.excluded_user_list,
            manager_list: self.manager_list,
            dept_list: self.dept_list,
            shift_list: self.shift_list,
            allow_out_punch: self.allow_out_punch,
            out_punch_need_approval: self.out_punch_need_approval,
            allow_pc_punch: self.allow_pc_punch,
            need_photo: self.need_photo,
            photo_punch_type: self.photo_punch_type,
            allow_remedy: self.allow_remedy,
            remedy_limit: self.remedy_limit,
            remedy_period: self.remedy_period,
            work_day_config: self.work_day_config,
            overtime_info: self.overtime_info,
        };
        let request = request.body(serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "请求体序列化失败",
                format!("无法序列化请求参数: {}", e),
            )
        })?);

        // 4. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 5. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "创建考勤组响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

impl ApiResponseTrait for CreateGroupResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use openlark_core::config::Config;
    use serde_json::json;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app")
            .app_secret("test_secret")
            .build()
    }

    #[test]
    fn test_create_group_request_builder() {
        let request = CreateGroupRequest::new(create_test_config())
            .group_name("测试考勤组".to_string())
            .group_type(1)
            .allow_pc_punch(true);

        assert_eq!(request.group_name, "测试考勤组");
        assert_eq!(request.group_type, 1);
        assert_eq!(request.allow_pc_punch, Some(true));
    }

    #[test]
    fn test_create_group_request_body_serialize() {
        let request = CreateGroupRequest::new(create_test_config())
            .group_name("研发组".to_string())
            .group_type(0)
            .allow_remedy(true)
            .remedy_limit(3);

        let body = CreateGroupRequestBody {
            group_id: request.group_id,
            group_name: request.group_name,
            group_type: request.group_type,
            user_list: request.user_list,
            excluded_user_list: request.excluded_user_list,
            manager_list: request.manager_list,
            dept_list: request.dept_list,
            shift_list: request.shift_list,
            allow_out_punch: request.allow_out_punch,
            out_punch_need_approval: request.out_punch_need_approval,
            allow_pc_punch: request.allow_pc_punch,
            need_photo: request.need_photo,
            photo_punch_type: request.photo_punch_type,
            allow_remedy: request.allow_remedy,
            remedy_limit: request.remedy_limit,
            remedy_period: request.remedy_period,
            work_day_config: request.work_day_config,
            overtime_info: request.overtime_info,
        };

        let value = serde_json::to_value(body).expect("序列化请求体失败");
        assert_eq!(value["group_name"], json!("研发组"));
        assert_eq!(value["group_type"], json!(0));
        assert_eq!(value["allow_remedy"], json!(true));
        assert_eq!(value["remedy_limit"], json!(3));
    }

    #[test]
    fn test_create_group_response_deserialize() {
        let value = json!({"group_id": "grp_123"});
        let response: CreateGroupResponse =
            serde_json::from_value(value).expect("反序列化响应失败");
        assert_eq!(response.group_id, "grp_123");
    }

    #[test]
    fn test_create_group_validation() {
        let request = CreateGroupRequest::new(create_test_config()).group_name("   ".to_string());
        let result: SDKResult<()> = (|| {
            validate_required!(request.group_name.trim(), "考勤组名称不能为空");
            Ok(())
        })();

        assert!(result.is_err());
    }
}
