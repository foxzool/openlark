//! 创建 / 更新外部算薪数据
//!
//! docPath: <https://open.feishu.cn/document/server-docs/payroll-v1/datasource_record/save>

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 创建 / 更新外部算薪数据请求
#[derive(Debug, Clone)]
pub struct SaveRequest {
    /// 数据源 ID（必填）
    datasource_id: String,
    /// 员工 ID 列表（必填）
    employee_ids: Vec<String>,
    /// 算薪数据列表（必填）
    records: Vec<DatasourceRecord>,
    /// 配置信息
    config: Config,
}

impl SaveRequest {
    /// 创建请求
    pub fn new(
        config: Config,
        datasource_id: String,
        employee_ids: Vec<String>,
        records: Vec<DatasourceRecord>,
    ) -> Self {
        Self {
            datasource_id,
            employee_ids,
            records,
            config,
        }
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<SaveResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    /// 执行请求（带自定义选项）
    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<SaveResponse> {
        use crate::common::api_endpoints::PayrollApiV1;

        // 1. 验证必填字段
        validate_required!(self.datasource_id.trim(), "datasource_id");
        validate_required!(self.employee_ids, "employee_ids");
        validate_required!(self.records, "records");

        // 2. 构建端点
        let api_endpoint = PayrollApiV1::DatasourceRecordSave;
        let request = ApiRequest::<SaveResponse>::post(api_endpoint.to_url());

        // 3. 构建请求体
        let request_body = SaveRequestBody {
            datasource_id: self.datasource_id,
            employee_ids: self.employee_ids,
            records: self.records,
        };
        let request_body_json = serde_json::to_value(&request_body).map_err(|e| {
            openlark_core::error::validation_error(
                "构建请求体失败",
                format!("序列化请求体失败: {}", e),
            )
        })?;
        let request = request.body(request_body_json);

        // 4. 发送请求
        let response = Transport::request(request, &self.config, Some(option)).await?;

        // 5. 提取响应数据
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error(
                "创建/更新外部算薪数据响应数据为空",
                "服务器没有返回有效的数据",
            )
        })
    }
}

/// 创建 / 更新外部算薪数据请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveRequestBody {
    /// 数据源 ID
    pub datasource_id: String,
    /// 员工 ID 列表
    pub employee_ids: Vec<String>,
    /// 算薪数据列表
    pub records: Vec<DatasourceRecord>,
}

/// 算薪数据记录
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasourceRecord {
    /// 员工 ID
    pub employee_id: String,
    /// 数据项列表
    pub items: Vec<DatasourceRecordItem>,
}

/// 数据项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasourceRecordItem {
    /// 字段名称
    pub field_name: String,
    /// 字段值
    pub value: serde_json::Value,
}

/// 创建 / 更新外部算薪数据响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SaveResponse {
    /// 是否成功
    pub success: bool,
    /// 成功处理的记录数
    pub processed_count: i32,
    /// 失败的记录数
    pub failed_count: i32,
    /// 失败的员工 ID 列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_employee_ids: Option<Vec<String>>,
}

impl ApiResponseTrait for SaveResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
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

    fn build_records() -> Vec<DatasourceRecord> {
        vec![DatasourceRecord {
            employee_id: "emp_1".to_string(),
            items: vec![DatasourceRecordItem {
                field_name: "base_salary".to_string(),
                value: json!(10000),
            }],
        }]
    }

    #[test]
    fn test_payroll_save_request_builder() {
        let request = SaveRequest::new(
            create_test_config(),
            "ds_1".to_string(),
            vec!["emp_1".to_string()],
            build_records(),
        );

        assert_eq!(request.datasource_id, "ds_1");
        assert_eq!(request.employee_ids, vec!["emp_1"]);
        assert_eq!(request.records.len(), 1);
    }

    #[test]
    fn test_payroll_save_request_body_serialize() {
        let body = SaveRequestBody {
            datasource_id: "ds_2".to_string(),
            employee_ids: vec!["emp_2".to_string()],
            records: vec![DatasourceRecord {
                employee_id: "emp_2".to_string(),
                items: vec![DatasourceRecordItem {
                    field_name: "bonus".to_string(),
                    value: json!(888),
                }],
            }],
        };

        let value = serde_json::to_value(body).expect("序列化请求体失败");
        assert_eq!(value["datasource_id"], json!("ds_2"));
        assert_eq!(value["employee_ids"], json!(["emp_2"]));
        assert_eq!(
            value["records"][0]["items"][0]["field_name"],
            json!("bonus")
        );
    }

    #[test]
    fn test_payroll_save_response_deserialize() {
        let value = json!({
            "success": true,
            "processed_count": 1,
            "failed_count": 0,
            "failed_employee_ids": ["emp_x"]
        });
        let response: SaveResponse = serde_json::from_value(value).expect("反序列化响应失败");

        assert!(response.success);
        assert_eq!(response.processed_count, 1);
        assert_eq!(
            response.failed_employee_ids,
            Some(vec!["emp_x".to_string()])
        );
    }

    #[test]
    fn test_payroll_save_validation() {
        let request = SaveRequest::new(create_test_config(), "  ".to_string(), vec![], vec![]);
        let result: SDKResult<()> = (|| {
            validate_required!(request.datasource_id.trim(), "datasource_id");
            validate_required!(request.employee_ids, "employee_ids");
            validate_required!(request.records, "records");
            Ok(())
        })();

        assert!(result.is_err());
    }
}
