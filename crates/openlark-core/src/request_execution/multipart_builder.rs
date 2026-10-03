use crate::error::{CoreError, validation_error};
use reqwest::{RequestBuilder, multipart};
use serde_json::Value;

/// 专门构建 multipart/form-data 请求
pub struct MultipartBuilder;

impl MultipartBuilder {
    /// 构建包含文件的 multipart 表单
    pub fn build_multipart(
        req_builder: RequestBuilder,
        body: &[u8],
        file_data: &[u8],
    ) -> Result<RequestBuilder, CoreError> {
        let json_value = serde_json::from_slice::<Value>(body)?;

        let form_obj = json_value
            .as_object()
            .ok_or_else(|| validation_error("form", "Invalid form data"))?;

        let mut form = multipart::Form::new();

        // 处理文件部分
        form = Self::add_file_part(form, form_obj, file_data)?;

        // 处理其他表单字段
        form = Self::add_form_fields(form, form_obj)?;

        // 设置编码
        form = form.percent_encode_noop();

        Ok(req_builder.multipart(form))
    }

    /// 添加文件部分到表单
    fn add_file_part(
        mut form: multipart::Form,
        form_obj: &serde_json::Map<String, Value>,
        file_data: &[u8],
    ) -> Result<multipart::Form, CoreError> {
        // 兼容两种用法：
        // - "__file_name": 仅用于设置 multipart 文件名，不作为表单字段发送
        // - "file_name": 既是表单字段，也可作为 multipart 文件名
        let file_name = match form_obj.get("__file_name") {
            Some(Value::String(s)) => Some(s.to_string()),
            Some(Value::Null) | None => match form_obj.get("file_name") {
                Some(Value::String(s)) => Some(s.to_string()),
                Some(Value::Null) | None => None,
                Some(_) => {
                    return Err(validation_error(
                        "file_name",
                        "file_name must be a string when provided",
                    ));
                }
            },
            Some(_) => {
                return Err(validation_error(
                    "file_name",
                    "__file_name must be a string when provided",
                ));
            }
        };

        let mut file_part = multipart::Part::bytes(file_data.to_vec());
        if let Some(name) = file_name {
            file_part = file_part.file_name(name);
        }

        form = form.part("file", file_part);
        Ok(form)
    }

    /// 添加普通表单字段
    fn add_form_fields(
        mut form: multipart::Form,
        form_obj: &serde_json::Map<String, Value>,
    ) -> Result<multipart::Form, CoreError> {
        for (key, value) in form_obj.iter() {
            // 跳过 __file_name 字段和 null 值
            if key == "__file_name" || value == &Value::Null {
                continue;
            }

            form = match value {
                Value::String(s) => form.text(key.to_string(), s.to_string()),
                Value::Number(n) => form.text(key.to_string(), n.to_string()),
                Value::Bool(b) => form.text(key.to_string(), b.to_string()),
                Value::Array(arr) => {
                    // 对于数组，转换为 JSON 字符串
                    form.text(key.to_string(), serde_json::to_string(arr)?)
                }
                Value::Object(obj) => {
                    // 对于对象，转换为 JSON 字符串
                    form.text(key.to_string(), serde_json::to_string(obj)?)
                }
                Value::Null => continue,
            };
        }

        Ok(form)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::CoreError;
    use reqwest::Client;
    use serde_json::json;

    fn create_test_request_builder() -> reqwest::RequestBuilder {
        Client::new().post("https://test.api.feishu.cn/upload")
    }

    #[test]
    fn test_build_multipart_simple_file() {
        let body_json = json!({
            "file_name": "test.txt",
            "parent_type": "folder",
            "parent_node": "fldr123"
        });
        let body = serde_json::to_vec(&body_json).unwrap();
        let file_data = b"Hello, World!";

        let req_builder = create_test_request_builder();
        let result = MultipartBuilder::build_multipart(req_builder, &body, file_data);

        assert!(result.is_ok());
    }

    #[test]
    fn test_build_multipart_missing_file_name() {
        let body_json = json!({
            "parent_type": "folder",
            "parent_node": "fldr123"
        });
        let body = serde_json::to_vec(&body_json).unwrap();
        let file_data = b"Hello, World!";

        let req_builder = create_test_request_builder();
        let result = MultipartBuilder::build_multipart(req_builder, &body, file_data);

        assert!(result.is_ok());
    }

    #[test]
    fn test_build_multipart_invalid_json() {
        let invalid_body = b"not valid json";
        let file_data = b"Hello, World!";

        let req_builder = create_test_request_builder();
        let result = MultipartBuilder::build_multipart(req_builder, invalid_body, file_data);

        assert!(result.is_err());
        match result {
            Err(CoreError::Serialization { .. }) => {}
            _ => panic!("Expected DeserializeError"),
        }
    }

    #[test]
    fn test_build_multipart_non_object_json() {
        let body_json = json!(["not", "an", "object"]);
        let body = serde_json::to_vec(&body_json).unwrap();
        let file_data = b"Hello, World!";

        let req_builder = create_test_request_builder();
        let result = MultipartBuilder::build_multipart(req_builder, &body, file_data);

        assert!(result.is_err());
        if let Err(CoreError::Validation { message, .. }) = result {
            assert!(message.contains("Invalid form data"));
        } else {
            panic!("Expected BadRequest error");
        }
    }

    #[test]
    fn test_build_multipart_with_multiple_fields() {
        let body_json = json!({
            "file_name": "document.pdf",
            "parent_type": "folder",
            "parent_node": "fldr456",
            "description": "Important document",
            "size": 1024,
            "is_public": true
        });
        let body = serde_json::to_vec(&body_json).unwrap();
        let file_data = b"PDF content here";

        let req_builder = create_test_request_builder();
        let result = MultipartBuilder::build_multipart(req_builder, &body, file_data);

        assert!(result.is_ok());
    }

    #[test]
    fn test_build_multipart_with_null_values() {
        let body_json = json!({
            "file_name": "test.txt",
            "parent_type": "folder",
            "parent_node": "fldr789",
            "optional_field": null,
            "description": "Test file"
        });
        let body = serde_json::to_vec(&body_json).unwrap();
        let file_data = b"Test content";

        let req_builder = create_test_request_builder();
        let result = MultipartBuilder::build_multipart(req_builder, &body, file_data);

        assert!(result.is_ok());
    }

    #[test]
    fn test_build_multipart_with_array_field() {
        let body_json = json!({
            "file_name": "data.json",
            "parent_type": "folder",
            "parent_node": "fldr101",
            "tags": ["important", "work", "project"],
            "metadata": {
                "created_by": "user123",
                "version": 1
            }
        });
        let body = serde_json::to_vec(&body_json).unwrap();
        let file_data = b"JSON content";

        let req_builder = create_test_request_builder();
        let result = MultipartBuilder::build_multipart(req_builder, &body, file_data);

        assert!(result.is_ok());
    }

    #[test]
    fn test_build_multipart_empty_file() {
        let body_json = json!({
            "file_name": "empty.txt",
            "parent_type": "folder",
            "parent_node": "fldr202"
        });
        let body = serde_json::to_vec(&body_json).unwrap();
        let file_data = b"";

        let req_builder = create_test_request_builder();
        let result = MultipartBuilder::build_multipart(req_builder, &body, file_data);

        assert!(result.is_ok());
    }

    #[test]
    fn test_build_multipart_large_file() {
        let body_json = json!({
            "file_name": "large_file.bin",
            "parent_type": "folder",
            "parent_node": "fldr303"
        });
        let body = serde_json::to_vec(&body_json).unwrap();

        // Create a 1MB file
        let file_data = vec![0u8; 1024 * 1024];

        let req_builder = create_test_request_builder();
        let result = MultipartBuilder::build_multipart(req_builder, &body, &file_data);

        assert!(result.is_ok());
    }

    #[test]
    fn test_add_file_part_success() {
        let mut form_data = serde_json::Map::new();
        form_data.insert("file_name".to_string(), json!("test.txt"));
        form_data.insert("other_field".to_string(), json!("value"));

        let form = reqwest::multipart::Form::new();
        let file_data = b"file content";

        let result = MultipartBuilder::add_file_part(form, &form_data, file_data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_add_file_part_missing_file_name() {
        let form_data = serde_json::Map::new();
        let form = reqwest::multipart::Form::new();
        let file_data = b"file content";

        let result = MultipartBuilder::add_file_part(form, &form_data, file_data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_add_file_part_non_string_file_name() {
        let mut form_data = serde_json::Map::new();
        form_data.insert("file_name".to_string(), json!(123)); // Number instead of string

        let form = reqwest::multipart::Form::new();
        let file_data = b"file content";

        let result = MultipartBuilder::add_file_part(form, &form_data, file_data);
        assert!(result.is_err());

        if let Err(CoreError::Validation { message, .. }) = result {
            assert!(message.contains("file_name must be a string"));
        }
    }

    #[test]
    fn test_add_form_fields_various_types() {
        let mut form_data = serde_json::Map::new();
        form_data.insert("string_field".to_string(), json!("text_value"));
        form_data.insert("number_field".to_string(), json!(42));
        form_data.insert("boolean_field".to_string(), json!(true));
        form_data.insert("null_field".to_string(), json!(null));
        form_data.insert("file_name".to_string(), json!("test.txt")); // Should be skipped

        let array_value = json!(["item1", "item2", "item3"]);
        form_data.insert("array_field".to_string(), array_value);

        let object_value = json!({"key": "value", "nested": {"inner": "data"}});
        form_data.insert("object_field".to_string(), object_value);

        let form = reqwest::multipart::Form::new();
        let result = MultipartBuilder::add_form_fields(form, &form_data);

        assert!(result.is_ok());
    }

    #[test]
    fn test_add_form_fields_empty_data() {
        let form_data = serde_json::Map::new();
        let form = reqwest::multipart::Form::new();

        let result = MultipartBuilder::add_form_fields(form, &form_data);
        assert!(result.is_ok());
    }

    #[test]
    fn test_add_form_fields_only_null_values() {
        let mut form_data = serde_json::Map::new();
        form_data.insert("field1".to_string(), json!(null));
        form_data.insert("field2".to_string(), json!(null));
        form_data.insert("file_name".to_string(), json!(null)); // Should be skipped

        let form = reqwest::multipart::Form::new();
        let result = MultipartBuilder::add_form_fields(form, &form_data);

        assert!(result.is_ok());
    }

    #[test]
    fn test_add_form_fields_special_characters() {
        let mut form_data = serde_json::Map::new();
        form_data.insert("unicode_field".to_string(), json!("测试内容"));
        form_data.insert("special_chars".to_string(), json!("@#$%^&*()"));
        form_data.insert("quotes".to_string(), json!("\"quoted text\""));
        form_data.insert("newlines".to_string(), json!("line1\nline2\nline3"));

        let form = reqwest::multipart::Form::new();
        let result = MultipartBuilder::add_form_fields(form, &form_data);

        assert!(result.is_ok());
    }

    #[test]
    fn test_build_multipart_with_complex_nested_data() {
        let body_json = json!({
            "file_name": "complex.json",
            "parent_type": "folder",
            "parent_node": "fldr999",
            "metadata": {
                "author": {
                    "name": "John Doe",
                    "id": "user123"
                },
                "permissions": ["read", "write", "delete"],
                "settings": {
                    "auto_backup": true,
                    "compression": "gzip",
                    "retention_days": 30
                }
            },
            "tags": ["json", "complex", "test"]
        });
        let body = serde_json::to_vec(&body_json).unwrap();
        let file_data = b"Complex JSON content";

        let req_builder = create_test_request_builder();
        let result = MultipartBuilder::build_multipart(req_builder, &body, file_data);

        assert!(result.is_ok());
    }

    #[test]
    fn test_build_multipart_file_name_with_special_characters() {
        let body_json = json!({
            "file_name": "测试文件 @#$%.txt",
            "parent_type": "folder",
            "parent_node": "fldr888"
        });
        let body = serde_json::to_vec(&body_json).unwrap();
        let file_data = b"File with special name";

        let req_builder = create_test_request_builder();
        let result = MultipartBuilder::build_multipart(req_builder, &body, file_data);

        assert!(result.is_ok());
    }

    #[test]
    fn test_multipart_builder_integration() {
        // Test the complete multipart building process
        let body_json = json!({
            "file_name": "integration_test.pdf",
            "parent_type": "bitable",
            "parent_node": "bitable_id_123",
            "description": "Integration test file",
            "size": 2048,
            "is_encrypted": false,
            "upload_metadata": {
                "source": "api_test",
                "timestamp": "2024-01-01T00:00:00Z"
            }
        });

        let body = serde_json::to_vec(&body_json).unwrap();
        let file_data = b"PDF integration test content";

        let req_builder = create_test_request_builder();
        let result = MultipartBuilder::build_multipart(req_builder, &body, file_data);

        assert!(result.is_ok());

        // Verify the result is a proper RequestBuilder
        let request_builder = result.unwrap();

        // We can't easily inspect the multipart form without building the request,
        // but we can verify it's still a valid RequestBuilder
        assert!(format!("{request_builder:?}").contains("RequestBuilder"));
    }
}
