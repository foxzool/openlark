/// Bitable V1 应用管理数据模型
///
/// 定义了应用管理相关的数据结构，包括请求和响应模型。
use openlark_core::config::Config;
use serde::{Deserialize, Serialize};

/// 应用服务
///
/// 用于在子模块中实现应用相关的方法。
pub struct AppService {
    /// 配置信息
    pub config: Config,
}

impl AppService {
    /// 创建新的应用服务
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

/// 应用信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct App {
    /// 应用token
    pub app_token: String,
    /// 应用名称
    pub name: String,
    /// 默认数据表ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_table_id: Option<String>,
    /// 应用图标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// 应用链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 文件夹token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
    /// 时区
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<i64>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<i64>,
    /// 是否被删除
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_deleted: Option<bool>,
    /// 应用设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_settings: Option<AppSettings>,
}

/// 应用设置
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppSettings {
    /// 时区
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    /// 语言
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

/// 创建应用请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateAppRequest {
    /// 应用名称
    pub name: String,
    /// 文件夹token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
    /// 时区
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    /// 应用设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_settings: Option<AppSettings>,
}

/// 创建应用响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateAppResponse {
    /// 应用信息
    pub app: App,
}

/// 复制应用请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CopyAppRequest {
    /// 新应用名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 目标文件夹token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_token: Option<String>,
    /// 是否复制内容（true: 不复制内容，false: 复制内容）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub without_content: Option<bool>,
    /// 时区
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
}

/// 复制应用响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CopyAppResponse {
    /// 应用信息
    pub app: App,
}

/// 获取应用响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetAppResponse {
    /// 应用信息
    pub app: App,
}

/// 更新应用请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateAppRequest {
    /// 应用名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 应用图标
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// 应用设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_settings: Option<AppSettings>,
}

/// 更新应用响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateAppResponse {
    /// 应用信息
    pub app: App,
}

/// 删除应用响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteAppResponse {
    /// 是否成功
    pub success: bool,
}

impl CreateAppRequest {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        use openlark_core::validate_required;
        validate_required!(self.name, "应用名称不能为空");

        if self.name.len() > 100 {
            return Err(openlark_core::CoreError::validation_msg(
                "应用名称长度不能超过100个字符",
            ));
        }

        Ok(())
    }
}

impl CopyAppRequest {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        use openlark_core::validate_required;
        if let Some(ref name) = self.name {
            validate_required!(name, "新应用名称不能为空");

            if name.len() > 100 {
                return Err(openlark_core::CoreError::validation_msg(
                    "应用名称长度不能超过100个字符",
                ));
            }
        }

        Ok(())
    }
}

impl UpdateAppRequest {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        use openlark_core::validate_required;
        if let Some(ref name) = self.name {
            validate_required!(name, "应用名称不能为空");

            if name.len() > 100 {
                return Err(openlark_core::CoreError::validation_msg(
                    "应用名称长度不能超过100个字符",
                ));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use serde_json;

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
