/// CCM Sheet API 数据模型
///
/// 提供电子表格操作相关的数据结构，支持工作表管理、
/// 单元格操作、样式设置、数据验证等功能。
use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

/// 读取单个范围请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReadSingleRangeRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 读取范围，例如 "Sheet1!A1:C10"
    pub range: String,
    /// 是否包含格式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_format: Option<bool>,
    /// 值渲染选项
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_render_option: Option<String>,
}

impl ReadSingleRangeRequest {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        use openlark_core::validate_required;
        validate_required!(self.spreadsheet_token, "电子表格token不能为空");
        validate_required!(self.range, "读取范围不能为空");
        Ok(())
    }
}
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        use openlark_core::validate_required;
        validate_required!(self.spreadsheet_token, "电子表格token不能为空");
        validate_required!(self.range, "读取范围不能为空");
        Ok(())
    }
        if self.spreadsheet_token.trim().is_empty() {
            return Err("电子表格token不能为空".to_string());
        }
        if self.range.trim().is_empty() {
            return Err("读取范围不能为空".to_string());
        }
        Ok(())
    }
}

/// 读取单个范围响应
#[derive(Debug, Clone, Serialize, PartialEq, Default)]
pub struct ReadSingleRangeResponse {
    /// 范围数据
    pub value_range: Option<ValueRange>,
    /// 电子表格ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spreadsheet_id: Option<String>,
}

impl<'de> Deserialize<'de> for ReadSingleRangeResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde_json::Value;

        let raw_value = Value::deserialize(deserializer)?;

        // 从原始 JSON 值中提取数据
        let spreadsheet_id = raw_value
            .get("spreadsheetId")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let value_range = raw_value.get("valueRange").and_then(|v| {
            // 手动解析 ValueRange
            let range = v
                .get("range")
                .and_then(|r| r.as_str())
                .map(|s| s.to_string());
            let major_dimension = v
                .get("majorDimension")
                .and_then(|m| m.as_str())
                .map(|s| s.to_string());
            let values = v.get("values").and_then(|vals| vals.as_array()).map(|arr| {
                arr.iter()
                    .filter_map(|row| row.as_array())
                    .map(|row| row.to_vec())
                    .collect::<Vec<_>>()
            });

            if range.is_some() || major_dimension.is_some() || values.is_some() {
                Some(ValueRange {
                    range,
                    major_dimension,
                    values,
                })
            } else {
                None
            }
        });

        Ok(ReadSingleRangeResponse {
            value_range,
            spreadsheet_id,
        })
    }
}

impl ApiResponseTrait for ReadSingleRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 读取多个范围请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReadMultipleRangesRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 读取范围列表
    pub ranges: Vec<String>,
    /// 是否包含格式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_format: Option<bool>,
}

impl ReadMultipleRangesRequest {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        use openlark_core::validate_required;
        validate_required!(self.spreadsheet_token, "电子表格token不能为空");
        if self.ranges.is_empty() {
            return Err(openlark_core::CoreError::validation_msg("读取范围列表不能为空"));
        }
        if self.ranges.len() > 10 {
            return Err(openlark_core::CoreError::validation_msg("读取范围数量不能超过10个"));
        }
        Ok(())
    }
        if self.spreadsheet_token.trim().is_empty() {
            return Err("电子表格token不能为空".to_string());
        }
        if self.ranges.is_empty() {
            return Err("读取范围列表不能为空".to_string());
        }
        if self.ranges.len() > 10 {
            return Err("读取范围数量不能超过10个".to_string());
        }
        Ok(())
    }
}

/// 读取多个范围响应
#[derive(Debug, Clone, Serialize, PartialEq, Default)]
pub struct ReadMultipleRangesResponse {
    /// 范围数据列表
    pub value_ranges: Option<Vec<ValueRange>>,
    /// 电子表格ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spreadsheet_id: Option<String>,
}

impl<'de> Deserialize<'de> for ReadMultipleRangesResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde_json::Value;

        let raw_value = Value::deserialize(deserializer)?;

        // 从原始 JSON 值中提取数据
        let spreadsheet_id = raw_value
            .get("spreadsheetId")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let value_ranges = raw_value
            .get("valueRanges")
            .and_then(|vals| vals.as_array())
            .and_then(|arr| {
                let mut ranges = vec![];
                for v in arr {
                    // 手动解析每个 ValueRange
                    let range = v
                        .get("range")
                        .and_then(|r| r.as_str())
                        .map(|s| s.to_string());
                    let major_dimension = v
                        .get("majorDimension")
                        .and_then(|m| m.as_str())
                        .map(|s| s.to_string());
                    let values = v.get("values").and_then(|vals| vals.as_array()).map(|arr| {
                        arr.iter()
                            .filter_map(|row| row.as_array())
                            .map(|row| row.to_vec())
                            .collect::<Vec<_>>()
                    });

                    if range.is_some() || major_dimension.is_some() || values.is_some() {
                        ranges.push(ValueRange {
                            range,
                            major_dimension,
                            values,
                        });
                    }
                }
                if !ranges.is_empty() {
                    Some(ranges)
                } else {
                    None
                }
            });

        Ok(ReadMultipleRangesResponse {
            value_ranges,
            spreadsheet_id,
        })
    }
}

impl ApiResponseTrait for ReadMultipleRangesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 写入单个范围请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WriteSingleRangeRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 写入范围，例如 "Sheet1!A1:C10"
    pub range: String,
    /// 写入的数据
    pub values: Vec<Vec<serde_json::Value>>,
    /// 值输入选项
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_input_option: Option<String>,
}

impl WriteSingleRangeRequest {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        use openlark_core::validate_required;
        validate_required!(self.spreadsheet_token, "电子表格token不能为空");
        validate_required!(self.range, "写入范围不能为空");
        if self.values.is_empty() {
            return Err(openlark_core::CoreError::validation_msg("写入数据不能为空"));
        }
        if self.values.len() > 1000 {
            return Err(openlark_core::CoreError::validation_msg("写入数据行数不能超过1000行"));
        }
        for row in &self.values {
            if row.len() > 1000 {
                return Err(openlark_core::CoreError::validation_msg("写入数据列数不能超过1000列"));
            }
        }
        Ok(())
    }
        if self.spreadsheet_token.trim().is_empty() {
            return Err("电子表格token不能为空".to_string());
        }
        if self.range.trim().is_empty() {
            return Err("写入范围不能为空".to_string());
        }
        if self.values.is_empty() {
            return Err("写入数据不能为空".to_string());
        }
        if self.values.len() > 1000 {
            return Err("写入数据行数不能超过1000行".to_string());
        }
        for row in &self.values {
            if row.len() > 1000 {
                return Err("写入数据列数不能超过1000列".to_string());
            }
        }
        Ok(())
    }
}

/// 写入单个范围响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WriteSingleRangeResponse {
    /// 更新的范围
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_range: Option<String>,
    /// 更新的行数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_rows: Option<i32>,
    /// 更新的列数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_columns: Option<i32>,
    /// 更新的单元格数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_cells: Option<i32>,
}

impl ApiResponseTrait for WriteSingleRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 写入多个范围请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WriteMultipleRangesRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 写入数据列表
    pub data: Vec<WriteData>,
    /// 值输入选项
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_input_option: Option<String>,
}

/// 写入数据结构
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WriteData {
    /// 写入范围
    pub range: String,
    /// 写入的数据
    pub values: Vec<Vec<serde_json::Value>>,
}

    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        use openlark_core::validate_required;
        validate_required!(self.spreadsheet_token, "电子表格token不能为空");
        if self.data.is_empty() {
            return Err(openlark_core::CoreError::validation_msg("写入数据列表不能为空"));
        }
        if self.data.len() > 10 {
            return Err(openlark_core::CoreError::validation_msg("写入数据数量不能超过10个"));
        }
        for write_data in &self.data {
            validate_required!(write_data.range, "写入范围不能为空");
            if write_data.values.is_empty() {
                return Err(openlark_core::CoreError::validation_msg("写入数据不能为空"));
            }
            if write_data.values.len() > 1000 {
                return Err(openlark_core::CoreError::validation_msg("写入数据行数不能超过1000行"));
            }
            for row in &write_data.values {
                if row.len() > 1000 {
                    return Err(openlark_core::CoreError::validation_msg("写入数据列数不能超过1000列"));
                }
            }
        }
        Ok(())
    }
        if self.spreadsheet_token.trim().is_empty() {
            return Err("电子表格token不能为空".to_string());
        }
        if self.data.is_empty() {
            return Err("写入数据列表不能为空".to_string());
        }
        if self.data.len() > 10 {
            return Err("写入数据数量不能超过10个".to_string());
        }
        for write_data in &self.data {
            if write_data.range.trim().is_empty() {
                return Err("写入范围不能为空".to_string());
            }
            if write_data.values.is_empty() {
                return Err("写入数据不能为空".to_string());
            }
            if write_data.values.len() > 1000 {
                return Err("写入数据行数不能超过1000行".to_string());
            }
            for row in &write_data.values {
                if row.len() > 1000 {
                    return Err("写入数据列数不能超过1000列".to_string());
                }
            }
        }
        Ok(())
    }
}

/// 写入多个范围响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct WriteMultipleRangesResponse {
    /// 写入结果列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<Vec<WriteSingleRangeResponse>>,
}

impl ApiResponseTrait for WriteMultipleRangesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 追加数据请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppendDataRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 追加范围
    pub range: String,
    /// 追加的数据
    pub values: Vec<Vec<serde_json::Value>>,
    /// 值输入选项
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_input_option: Option<String>,
    /// 插入数据选项
    #[serde(skip_serializing_if = "Option::is_none")]
    pub insert_data_option: Option<String>,
}

impl AppendDataRequest {
    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        use openlark_core::validate_required;
        validate_required!(self.spreadsheet_token, "电子表格token不能为空");
        validate_required!(self.range, "追加范围不能为空");
        if self.values.is_empty() {
            return Err(openlark_core::CoreError::validation_msg("追加数据不能为空"));
        }
        if self.values.len() > 1000 {
            return Err(openlark_core::CoreError::validation_msg("追加数据行数不能超过1000行"));
        }
        for row in &self.values {
            if row.len() > 1000 {
                return Err(openlark_core::CoreError::validation_msg("追加数据列数不能超过1000列"));
            }
        }
        Ok(())
    }
        if self.spreadsheet_token.trim().is_empty() {
            return Err("电子表格token不能为空".to_string());
        }
        if self.range.trim().is_empty() {
            return Err("追加范围不能为空".to_string());
        }
        if self.values.is_empty() {
            return Err("追加数据不能为空".to_string());
        }
        if self.values.len() > 1000 {
            return Err("追加数据行数不能超过1000行".to_string());
        }
        for row in &self.values {
            if row.len() > 1000 {
                return Err("追加数据列数不能超过1000列".to_string());
            }
        }
        Ok(())
    }
}

/// 追加数据响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AppendDataResponse {
    /// 表格ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub table_id: Option<String>,
    /// 更新的范围
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updates: Option<UpdateInfo>,
}

/// 更新信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct UpdateInfo {
    /// 更新的范围
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_range: Option<String>,
    /// 更新的行数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_rows: Option<i32>,
    /// 更新的列数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_columns: Option<i32>,
    /// 更新的单元格数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_cells: Option<i32>,
}

impl ApiResponseTrait for AppendDataResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 插入行列请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InsertDimensionRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 维度类型：ROWS 或 COLUMNS
    pub dimension: String,
    /// 起始索引
    pub start_index: i32,
    /// 结束索引
    pub end_index: i32,
    /// 是否继承样式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inherit_style_before: Option<bool>,
}

    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        use openlark_core::validate_required;
        validate_required!(self.spreadsheet_token, "电子表格token不能为空");
        if !["ROWS", "COLUMNS"].contains(&self.dimension.as_str()) {
            return Err(openlark_core::CoreError::validation_msg("维度类型必须是ROWS或COLUMNS"));
        }
        if self.start_index < 0 {
            return Err(openlark_core::CoreError::validation_msg("起始索引不能小于0"));
        }
        if self.end_index < self.start_index {
            return Err(openlark_core::CoreError::validation_msg("结束索引不能小于起始索引"));
        }
        let count = self.end_index - self.start_index;
        if count > 5000 {
            return Err(openlark_core::CoreError::validation_msg("插入行列数量不能超过5000"));
        }
        Ok(())
    }
        if self.spreadsheet_token.trim().is_empty() {
            return Err("电子表格token不能为空".to_string());
        }
        if !["ROWS", "COLUMNS"].contains(&self.dimension.as_str()) {
            return Err("维度类型必须是ROWS或COLUMNS".to_string());
        }
        if self.start_index < 0 {
            return Err("起始索引不能小于0".to_string());
        }
        if self.end_index < self.start_index {
            return Err("结束索引不能小于起始索引".to_string());
        }
        let count = self.end_index - self.start_index;
        if count > 5000 {
            return Err("插入行列数量不能超过5000".to_string());
        }
        Ok(())
    }
}

/// 插入行列响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct InsertDimensionResponse {
    /// 电子表格ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spreadsheet_id: Option<String>,
}

impl ApiResponseTrait for InsertDimensionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除行列请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteDimensionRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 维度类型：ROWS 或 COLUMNS
    pub dimension: String,
    /// 起始索引
    pub start_index: i32,
    /// 结束索引
    pub end_index: i32,
}

    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        use openlark_core::validate_required;
        validate_required!(self.spreadsheet_token, "电子表格token不能为空");
        if !["ROWS", "COLUMNS"].contains(&self.dimension.as_str()) {
            return Err(openlark_core::CoreError::validation_msg("维度类型必须是ROWS或COLUMNS"));
        }
        if self.start_index < 0 {
            return Err(openlark_core::CoreError::validation_msg("起始索引不能小于0"));
        }
        if self.end_index < self.start_index {
            return Err(openlark_core::CoreError::validation_msg("结束索引不能小于起始索引"));
        }
        let count = self.end_index - self.start_index;
        if count > 5000 {
            return Err(openlark_core::CoreError::validation_msg("删除行列数量不能超过5000"));
        }
        Ok(())
    }
        if self.spreadsheet_token.trim().is_empty() {
            return Err("电子表格token不能为空".to_string());
        }
        if !["ROWS", "COLUMNS"].contains(&self.dimension.as_str()) {
            return Err("维度类型必须是ROWS或COLUMNS".to_string());
        }
        if self.start_index < 0 {
            return Err("起始索引不能小于0".to_string());
        }
        if self.end_index < self.start_index {
            return Err("结束索引不能小于起始索引".to_string());
        }
        let count = self.end_index - self.start_index;
        if count > 5000 {
            return Err("删除行列数量不能超过5000".to_string());
        }
        Ok(())
    }
}

/// 删除行列响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct DeleteDimensionResponse {
    /// 电子表格ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spreadsheet_id: Option<String>,
}

impl ApiResponseTrait for DeleteDimensionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量更新工作表请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchUpdateSheetRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 更新操作列表
    pub requests: Vec<BatchUpdateRequest>,
    /// 是否包含格式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_format: Option<bool>,
}

/// 批量更新操作
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchUpdateRequest {
    /// 操作类型
    #[serde(rename = "type")]
    pub request_type: String,
    /// 操作参数
    pub params: serde_json::Value,
}

    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        use openlark_core::validate_required;
        validate_required!(self.spreadsheet_token, "电子表格token不能为空");
        if self.requests.is_empty() {
            return Err(openlark_core::CoreError::validation_msg("更新操作列表不能为空"));
        }
        if self.requests.len() > 100 {
            return Err(openlark_core::CoreError::validation_msg("更新操作数量不能超过100个"));
        }
        Ok(())
    }
        if self.spreadsheet_token.trim().is_empty() {
            return Err("电子表格token不能为空".to_string());
        }
        if self.requests.is_empty() {
            return Err("更新操作列表不能为空".to_string());
        }
        if self.requests.len() > 100 {
            return Err("更新操作数量不能超过100个".to_string());
        }
        Ok(())
    }
}

/// 批量更新工作表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BatchUpdateSheetResponse {
    /// 更新结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<Vec<serde_json::Value>>,
    /// 电子表格ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spreadsheet_id: Option<String>,
}

impl ApiResponseTrait for BatchUpdateSheetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取表格元数据请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetSheetMetaRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 是否包含格式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_format: Option<bool>,
    /// 是否包含数据验证
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_data_validation: Option<bool>,
}

    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        use openlark_core::validate_required;
        validate_required!(self.spreadsheet_token, "电子表格token不能为空");
        Ok(())
    }
        if self.spreadsheet_token.trim().is_empty() {
            return Err("电子表格token不能为空".to_string());
        }
        Ok(())
    }
}

/// 获取表格元数据响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GetSheetMetaResponse {
    /// 表格属性
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<SheetProperties>,
    /// 工作表列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheets: Option<Vec<SheetInfo>>,
    /// 电子表格ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spreadsheet_id: Option<String>,
}

impl ApiResponseTrait for GetSheetMetaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 表格属性
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SheetProperties {
    /// 表格标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    /// 修改时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modified_time: Option<String>,
    /// 时区
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    /// 语言环境
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
}

/// 工作表信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SheetInfo {
    /// 工作表ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_id: Option<String>,
    /// 工作表标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 工作表类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_type: Option<String>,
    /// 网格线是否可见
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grid_properties: Option<GridProperties>,
}

/// 网格属性
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct GridProperties {
    /// 行数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row_count: Option<i32>,
    /// 列数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column_count: Option<i32>,
    /// 是否显示网格线
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_grid_lines: Option<bool>,
}

/// 值范围数据
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ValueRange {
    /// 范围
    pub range: Option<String>,
    /// 主要维度
    pub major_dimension: Option<String>,
    /// 值
    pub values: Option<Vec<Vec<serde_json::Value>>>,
}

impl Serialize for ValueRange {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("ValueRange", 3)?;
        if let Some(ref range) = self.range {
            state.serialize_field("range", range)?;
        }
        if let Some(ref dimension) = self.major_dimension {
            state.serialize_field("majorDimension", dimension)?;
        }
        if let Some(ref values) = self.values {
            state.serialize_field("values", values)?;
        }
        state.end()
    }
}

/// 设置单元格样式请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SetCellStyleRequest {
    /// 电子表格token
    pub spreadsheet_token: String,
    /// 设置范围
    pub range: String,
    /// 样式设置
    pub style: CellStyle,
}

/// 单元格样式
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CellStyle {
    /// 背景颜色
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<Color>,
    /// 文本颜色
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_color: Option<Color>,
    /// 字体大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub font_size: Option<i32>,
    /// 字体加粗
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bold: Option<bool>,
    /// 字体斜体
    #[serde(skip_serializing_if = "Option::is_none")]
    pub italic: Option<bool>,
    /// 字体下划线
    #[serde(skip_serializing_if = "Option::is_none")]
    pub underline: Option<bool>,
    /// 水平对齐
    #[serde(skip_serializing_if = "Option::is_none")]
    pub horizontal_alignment: Option<String>,
    /// 垂直对齐
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vertical_alignment: Option<String>,
}

/// 颜色
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Color {
    /// 红色分量 (0-255)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub red: Option<f32>,
    /// 绿色分量 (0-255)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub green: Option<f32>,
    /// 蓝色分量 (0-255)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue: Option<f32>,
    /// alpha通道 (0-1)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alpha: Option<f32>,
}

    /// 验证请求参数
    pub fn validate(&self) -> openlark_core::SDKResult<()> {
        use openlark_core::validate_required;
        validate_required!(self.spreadsheet_token, "电子表格token不能为空");
        validate_required!(self.range, "设置范围不能为空");
        Ok(())
    }
        if self.spreadsheet_token.trim().is_empty() {
            return Err("电子表格token不能为空".to_string());
        }
        if self.range.trim().is_empty() {
            return Err("设置范围不能为空".to_string());
        }
        Ok(())
    }
}

/// 设置单元格样式响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SetCellStyleResponse {
    /// 电子表格ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spreadsheet_id: Option<String>,
}

impl ApiResponseTrait for SetCellStyleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

    #[test]
    fn test_read_single_range_request_validation() {
        let valid_request = ReadSingleRangeRequest {
            spreadsheet_token: "token123".to_string(),
            range: "Sheet1!A1:C10".to_string(),
            include_format: Some(true),
            value_render_option: Some("FORMATTED_VALUE".to_string()),
        };
        assert!(valid_request.validate().is_ok());

        let empty_token_request = ReadSingleRangeRequest {
            spreadsheet_token: "".to_string(),
            range: "Sheet1!A1:C10".to_string(),
            include_format: None,
            value_render_option: None,
        };
        assert!(empty_token_request.validate().is_err());

        let empty_range_request = ReadSingleRangeRequest {
            spreadsheet_token: "token123".to_string(),
            range: "".to_string(),
            include_format: None,
            value_render_option: None,
        };
        assert!(empty_range_request.validate().is_err());
    }

    #[test]
    fn test_read_single_range_response_deserialization() {
        let json = r#"{
            "valueRange": {
                "range": "Sheet1!A1:C10",
                "values": [["1", "2", "3"], ["4", "5", "6"]]
            },
            "spreadsheetId": "spreadsheet_id_123"
        }"#;
        let response: ReadSingleRangeResponse =
            serde_json::from_str(json).expect("JSON 反序列化失败");
        assert!(response.value_range.is_some());
        assert_eq!(
            response.spreadsheet_id,
            Some("spreadsheet_id_123".to_string())
        );
    }
}
