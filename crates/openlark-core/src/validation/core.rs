//! 核心验证功能

use tracing::warn;

/// 检查字符是否为中文字符
///
/// 使用 Unicode 范围检查来识别中文字符
///
/// # 参数
/// - `c`: 要检查的字符
///
/// # 返回
/// true 如果是中文字符，false 否则
pub fn is_chinese_char(c: char) -> bool {
    // CJK Unified Ideographs (扩展A-F区)
    let ranges = [
        (0x4E00, 0x9FFF),   // CJK Unified Ideographs
        (0x3400, 0x4DBF),   // CJK Unified Ideographs Extension A
        (0x20000, 0x2A6DF), // CJK Unified Ideographs Extension B
        (0x2A700, 0x2B73F), // CJK Unified Ideographs Extension C
        (0x2B740, 0x2B81F), // CJK Unified Ideographs Extension D
        (0x2B820, 0x2CEAF), // CJK Unified Ideographs Extension E
        (0x2CEB0, 0x2EBEF), // CJK Unified Ideographs Extension F
        (0x3000, 0x303F),   // CJK Symbols and Punctuation
        (0x31C0, 0x31EF),   // CJK Strokes
        (0x2F00, 0x2FD5),   // Kangxi Radicals
        (0x2E80, 0x2EFF),   // CJK Radicals Supplement
        (0xF900, 0xFAFF),   // CJK Compatibility Ideographs
        (0x2F800, 0x2FA1F), // CJK Compatibility Ideographs Supplement
    ];

    let code = c as u32;
    ranges
        .iter()
        .any(|&(start, end)| code >= start && code <= end)
}

/// 验证字符串长度，如果超过最大长度则截断
///
/// # 参数
/// - `input`: 输入字符串
/// - `max_len`: 最大长度
/// - `field_name`: 字段名称（用于日志）
///
/// # 返回
/// 验证后的字符串（可能被截断）
pub fn validate_string_length(input: String, max_len: usize, field_name: &str) -> String {
    if input.len() > max_len {
        warn!(
            "字段 {} 超过最大长度 {}，当前长度 {}，将被截断",
            field_name,
            max_len,
            input.len()
        );
        // 安全截断，确保不会截断UTF-8字符
        input.chars().take(max_len).collect()
    } else {
        input
    }
}

/// 验证必填列表字段
///
/// # 参数
/// - `values`: 列表字段
/// - `max_len`: 最大长度限制
/// - `field_name`: 字段名称（用于日志）
///
/// # 返回
/// true 如果字段有效且长度合规，false 否则
pub fn validate_required_list_length(values: &[String], max_len: usize, field_name: &str) -> bool {
    if values.is_empty() {
        warn!("必填列表字段 {} 为空", field_name);
        false
    } else if values.len() > max_len {
        warn!(
            "必填列表字段 {} 长度 {} 超过最大限制 {}",
            field_name,
            values.len(),
            max_len
        );
        false
    } else {
        true
    }
}

/// 验证内容大小
///
/// # 参数
/// - `content`: 内容
/// - `max_size`: 最大大小（字节）
/// - `content_type`: 内容类型（用于日志）
///
/// # 返回
/// true 如果大小有效，false 否则
pub fn validate_content_size(content: &str, max_size: usize, content_type: &str) -> bool {
    let content_size = content.len();
    if content_size > max_size {
        warn!(
            "{} 内容大小 {} 超过最大限制 {}",
            content_type, content_size, max_size
        );
        false
    } else {
        true
    }
}

/// 验证结果
#[derive(Debug, Clone, PartialEq)]
pub enum ValidationResult {
    /// 验证成功
    Valid,
    /// 验证失败，附带错误信息
    Invalid(String),
    /// 需要清理，附带清理后的值
    Sanitized(String),
}

impl ValidationResult {
    /// 检查验证结果是否有效
    pub fn is_valid(&self) -> bool {
        matches!(self, ValidationResult::Valid)
    }

    /// 获取错误信息（如果有）
    pub fn error_message(&self) -> Option<&String> {
        match self {
            ValidationResult::Invalid(msg) => Some(msg),
            _ => None,
        }
    }

    /// 获取清理后的值（如果有）
    pub fn sanitized_value(&self) -> Option<&String> {
        match self {
            ValidationResult::Sanitized(value) => Some(value),
            _ => None,
        }
    }

    /// 将验证结果转换为结果类型
    pub fn into_result(self) -> Result<String, String> {
        match self {
            ValidationResult::Valid => Ok(String::new()),
            ValidationResult::Invalid(msg) => Err(msg),
            ValidationResult::Sanitized(value) => Ok(value),
        }
    }
}

/// 验证构建器特征
pub trait ValidateBuilder {
    /// 最终的输出类型
    type Output;

    /// 添加必填验证
    fn required(self, value: Option<String>, field_name: &str) -> Self;

    /// 添加长度验证
    fn length(self, value: String, min_len: usize, max_len: usize, field_name: &str) -> Self;

    /// 添加自定义验证
    fn custom<F>(self, value: String, validator: F, error_msg: &str) -> Self
    where
        F: FnOnce(&str) -> bool;

    /// 执行验证
    fn validate(&self) -> ValidationResult;

    /// 构建最终结果
    fn build(self) -> Self::Output;
}

/// 默认验证构建器实现
#[derive(Debug)]
pub struct DefaultValidateBuilder {
    value: Option<String>,
    errors: Vec<String>,
}

impl DefaultValidateBuilder {
    /// 创建新的验证构建器
    pub fn new() -> Self {
        Self {
            value: None,
            errors: Vec::new(),
        }
    }

    /// 设置初始值
    pub fn value(mut self, value: String) -> Self {
        self.value = Some(value);
        self
    }
}

impl ValidateBuilder for DefaultValidateBuilder {
    type Output = Result<String, Vec<String>>;

    fn required(mut self, value: Option<String>, field_name: &str) -> Self {
        match value {
            Some(v) => {
                if v.trim().is_empty() {
                    self.errors.push(format!("字段 {field_name} 不能为空"));
                } else {
                    self.value = Some(v);
                }
            }
            None => {
                self.errors.push(format!("字段 {field_name} 不能为空"));
            }
        }
        self
    }

    fn length(mut self, value: String, min_len: usize, max_len: usize, field_name: &str) -> Self {
        if value.len() < min_len {
            self.errors.push(format!(
                "字段 {} 长度 {} 小于最小长度 {}",
                field_name,
                value.len(),
                min_len
            ));
        } else if value.len() > max_len {
            self.errors.push(format!(
                "字段 {} 长度 {} 超过最大长度 {}",
                field_name,
                value.len(),
                max_len
            ));
        } else {
            self.value = Some(validate_string_length(value, max_len, field_name));
        }
        self
    }

    fn custom<F>(mut self, value: String, validator: F, error_msg: &str) -> Self
    where
        F: FnOnce(&str) -> bool,
    {
        if validator(&value) {
            self.value = Some(value);
        } else {
            self.errors.push(error_msg.to_string());
        }
        self
    }

    fn validate(&self) -> ValidationResult {
        if self.errors.is_empty() {
            ValidationResult::Valid
        } else {
            ValidationResult::Invalid(self.errors.join("; "))
        }
    }

    fn build(self) -> Self::Output {
        if self.errors.is_empty() {
            Ok(self.value.unwrap_or_default())
        } else {
            Err(self.errors)
        }
    }
}

impl Default for DefaultValidateBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_chinese_char() {
        assert!(is_chinese_char('你'));
        assert!(is_chinese_char('好'));
        assert!(is_chinese_char('世'));
        assert!(is_chinese_char('界'));
        assert!(!is_chinese_char('a'));
        assert!(!is_chinese_char('1'));
        assert!(!is_chinese_char('!'));
    }

    #[test]
    fn test_validate_string_length() {
        let long_string = "这是一个很长的字符串用于测试截断功能".to_string();
        let result = validate_string_length(long_string.clone(), 10, "测试字段");
        assert_eq!(result.chars().count(), 10); // 确保字符数正好是10
        assert!(result.len() <= 30); // 字节数可能更多（UTF-8）

        let short_string = "短".to_string();
        let result = validate_string_length(short_string, 10, "测试字段");
        assert_eq!(result, "短");
    }

    #[test]
    fn test_validate_content_size() {
        let content = "这是一个测试内容";
        assert!(validate_content_size(content, 30, "测试内容")); // 30 > 25字节
        assert!(!validate_content_size(content, 20, "测试内容")); // 20 < 25字节
    }

    #[test]
    fn test_validation_result() {
        let valid = ValidationResult::Valid;
        assert!(valid.is_valid());
        assert_eq!(valid.error_message(), None);
        assert_eq!(valid.sanitized_value(), None);

        let invalid = ValidationResult::Invalid("错误".to_string());
        assert!(!invalid.is_valid());
        assert_eq!(invalid.error_message(), Some(&"错误".to_string()));
        assert_eq!(invalid.sanitized_value(), None);

        let sanitized = ValidationResult::Sanitized("清理后".to_string());
        assert!(!sanitized.is_valid());
        assert_eq!(sanitized.error_message(), None);
        assert_eq!(sanitized.sanitized_value(), Some(&"清理后".to_string()));
    }

    #[test]
    fn test_default_validate_builder() {
        let builder = DefaultValidateBuilder::new();

        // 成功案例
        let result = builder
            .value("测试值".to_string())
            .length("测试值".to_string(), 1, 10, "测试字段")
            .build();
        assert!(result.is_ok());

        // 失败案例
        let result = DefaultValidateBuilder::new()
            .value("这是一个很长的测试值".to_string())
            .length("这是一个很长的测试值".to_string(), 1, 5, "测试字段")
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_is_chinese_char_unicode_boundaries() {
        let chinese_boundaries = [
            (0x4E00, 0x9FFF),
            (0x3400, 0x4DBF),
            (0x20000, 0x2A6DF),
            (0x2A700, 0x2B73F),
            (0x2B740, 0x2B81F),
            (0x2B820, 0x2CEAF),
            (0x2CEB0, 0x2EBEF),
            (0x3000, 0x303F),
            (0x31C0, 0x31EF),
            (0x2F00, 0x2FD5),
            (0x2E80, 0x2EFF),
            (0xF900, 0xFAFF),
            (0x2F800, 0x2FA1F),
        ];

        for (start, end) in chinese_boundaries {
            let start_char = char::from_u32(start).expect("start must be valid char");
            let end_char = char::from_u32(end).expect("end must be valid char");
            assert!(is_chinese_char(start_char));
            assert!(is_chinese_char(end_char));
        }

        assert!(!is_chinese_char(
            char::from_u32(0x4DFF).expect("valid char")
        ));
        assert!(!is_chinese_char(
            char::from_u32(0xA000).expect("valid char")
        ));
        assert!(!is_chinese_char('A'));
        assert!(!is_chinese_char('😀'));
    }

    #[test]
    fn test_validate_string_length_utf8_truncation() {
        let input = "A你😀B".to_string();
        let result = validate_string_length(input, 3, "utf8字段");
        assert_eq!(result, "A你😀");
        assert!(result.is_char_boundary(result.len()));

        let empty_result = validate_string_length("任何值".to_string(), 0, "零长度字段");
        assert_eq!(empty_result, "");

        let ascii_exact = validate_string_length("abcd".to_string(), 4, "ascii字段");
        assert_eq!(ascii_exact, "abcd");
    }

    #[test]
    fn test_validate_required_list_length_boundaries() {
        let empty: Vec<String> = vec![];
        assert!(!validate_required_list_length(&empty, 3, "列表字段"));

        let one = vec!["a".to_string()];
        assert!(validate_required_list_length(&one, 1, "列表字段"));

        let max = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        assert!(validate_required_list_length(&max, 3, "列表字段"));

        let overflow = vec![
            "a".to_string(),
            "b".to_string(),
            "c".to_string(),
            "d".to_string(),
        ];
        assert!(!validate_required_list_length(&overflow, 3, "列表字段"));

        let non_empty_when_max_zero = vec!["a".to_string()];
        assert!(!validate_required_list_length(
            &non_empty_when_max_zero,
            0,
            "列表字段"
        ));
    }

    #[test]
    fn test_validate_content_size_various_sizes() {
        assert!(validate_content_size("", 0, "空内容"));
        assert!(!validate_content_size("a", 0, "单字符内容"));

        let ascii = "abcd";
        assert!(validate_content_size(ascii, 4, "ascii内容"));
        assert!(!validate_content_size(ascii, 3, "ascii内容"));

        let utf8 = "你a";
        assert!(validate_content_size(utf8, 4, "utf8内容"));
        assert!(!validate_content_size(utf8, 3, "utf8内容"));

        let kb_content = "x".repeat(1024);
        assert!(validate_content_size(&kb_content, 1024, "1KB内容"));
        assert!(!validate_content_size(&kb_content, 1023, "1KB内容"));
    }

    #[test]
    fn test_default_validate_builder_all_methods() {
        let ok_builder = DefaultValidateBuilder::default()
            .value("seed".to_string())
            .required(Some("abc".to_string()), "字段A")
            .length("abc".to_string(), 1, 10, "字段A")
            .custom("abc".to_string(), |v| v.contains('b'), "自定义校验失败");

        assert!(ok_builder.validate().is_valid());
        assert_eq!(ok_builder.build(), Ok("abc".to_string()));

        let required_none = DefaultValidateBuilder::new().required(None, "字段B");
        assert!(matches!(
            required_none.validate(),
            ValidationResult::Invalid(_)
        ));
        assert_eq!(
            required_none.build(),
            Err(vec!["字段 字段B 不能为空".to_string()])
        );

        let required_blank =
            DefaultValidateBuilder::new().required(Some("   ".to_string()), "字段C");
        assert!(matches!(
            required_blank.validate(),
            ValidationResult::Invalid(_)
        ));

        let length_too_short =
            DefaultValidateBuilder::new().length("ab".to_string(), 3, 5, "字段D");
        assert!(matches!(
            length_too_short.validate(),
            ValidationResult::Invalid(_)
        ));

        let length_too_long =
            DefaultValidateBuilder::new().length("abcdef".to_string(), 1, 5, "字段E");
        assert!(matches!(
            length_too_long.validate(),
            ValidationResult::Invalid(_)
        ));

        let custom_fail =
            DefaultValidateBuilder::new().custom("abc".to_string(), |v| v.len() > 10, "必须大于10");
        assert!(matches!(
            custom_fail.validate(),
            ValidationResult::Invalid(_)
        ));
    }

    #[test]
    fn test_validation_result_into_result_variants() {
        assert_eq!(ValidationResult::Valid.into_result(), Ok(String::new()));
        assert_eq!(
            ValidationResult::Sanitized("clean".to_string()).into_result(),
            Ok("clean".to_string())
        );
        assert_eq!(
            ValidationResult::Invalid("bad".to_string()).into_result(),
            Err("bad".to_string())
        );
    }

    #[test]
    fn test_validate_string_length_empty_string() {
        let result = validate_string_length(String::new(), 8, "空字符串字段");
        assert_eq!(result, "");
    }

    #[test]
    fn test_validate_required_list_length_exact_match() {
        let values = vec!["x".to_string(), "y".to_string()];
        assert!(validate_required_list_length(&values, 2, "列表字段"));
    }

    #[test]
    fn test_validate_content_size_multibyte_edge() {
        let content = "你";
        assert!(validate_content_size(content, 3, "中文内容"));
        assert!(!validate_content_size(content, 2, "中文内容"));
    }

    #[test]
    fn test_default_validate_builder_value_only_build() {
        let result = DefaultValidateBuilder::new()
            .value("预设值".to_string())
            .build();
        assert_eq!(result, Ok("预设值".to_string()));
    }

    // ===== 新增测试用例，提升覆盖率至 70%+ =====

    #[test]
    fn test_is_chinese_char_cjk_extension_c_d() {
        // CJK Unified Ideographs Extension C (0x2A700 - 0x2B73F)
        assert!(is_chinese_char(char::from_u32(0x2A700).unwrap()));
        assert!(is_chinese_char(char::from_u32(0x2A800).unwrap()));
        assert!(is_chinese_char(char::from_u32(0x2B73F).unwrap()));

        // CJK Unified Ideographs Extension D (0x2B740 - 0x2B81F)
        assert!(is_chinese_char(char::from_u32(0x2B740).unwrap()));
        assert!(is_chinese_char(char::from_u32(0x2B800).unwrap()));
        assert!(is_chinese_char(char::from_u32(0x2B81F).unwrap()));

        // 边界外（Extension D之前）
        assert!(!is_chinese_char(char::from_u32(0x2A6FF).unwrap()));
    }

    #[test]
    fn test_is_chinese_char_cjk_extension_e_f() {
        // CJK Unified Ideographs Extension E (0x2B820 - 0x2CEAF)
        assert!(is_chinese_char(char::from_u32(0x2B820).unwrap()));
        assert!(is_chinese_char(char::from_u32(0x2B900).unwrap()));
        assert!(is_chinese_char(char::from_u32(0x2CEAF).unwrap()));

        // CJK Unified Ideographs Extension F (0x2CEB0 - 0x2EBEF)
        assert!(is_chinese_char(char::from_u32(0x2CEB0).unwrap()));
        assert!(is_chinese_char(char::from_u32(0x2D000).unwrap()));
        assert!(is_chinese_char(char::from_u32(0x2EBEF).unwrap()));

        // 边界外（Extension F之后）
        assert!(!is_chinese_char(char::from_u32(0x2EBF0).unwrap()));
    }

    #[test]
    fn test_is_chinese_char_cjk_symbols_punctuation() {
        // CJK Symbols and Punctuation (0x3000 - 0x303F)
        assert!(is_chinese_char('　')); // 全角空格 0x3000
        assert!(is_chinese_char('、')); // 顿号 0x3001
        assert!(is_chinese_char('。')); // 句号 0x3002
        assert!(is_chinese_char(char::from_u32(0x303F).unwrap())); // 边界

        // 边界外
        assert!(!is_chinese_char(char::from_u32(0x3040).unwrap()));
    }

    #[test]
    fn test_is_chinese_char_cjk_strokes_kangxi() {
        // CJK Strokes (0x31C0 - 0x31EF)
        assert!(is_chinese_char(char::from_u32(0x31C0).unwrap()));
        assert!(is_chinese_char(char::from_u32(0x31E0).unwrap()));
        assert!(is_chinese_char(char::from_u32(0x31EF).unwrap()));

        // Kangxi Radicals (0x2F00 - 0x2FD5)
        assert!(is_chinese_char(char::from_u32(0x2F00).unwrap()));
        assert!(is_chinese_char(char::from_u32(0x2FA0).unwrap()));
        assert!(is_chinese_char(char::from_u32(0x2FD5).unwrap()));

        // CJK Radicals Supplement (0x2E80 - 0x2EFF)
        assert!(is_chinese_char(char::from_u32(0x2E80).unwrap()));
        assert!(is_chinese_char(char::from_u32(0x2EC0).unwrap()));
        assert!(is_chinese_char(char::from_u32(0x2EFF).unwrap()));
    }

    #[test]
    fn test_is_chinese_char_cjk_compatibility() {
        // CJK Compatibility Ideographs (0xF900 - 0xFAFF)
        assert!(is_chinese_char(char::from_u32(0xF900).unwrap()));
        assert!(is_chinese_char(char::from_u32(0xFA00).unwrap()));
        assert!(is_chinese_char(char::from_u32(0xFAFF).unwrap()));

        // CJK Compatibility Ideographs Supplement (0x2F800 - 0x2FA1F)
        assert!(is_chinese_char(char::from_u32(0x2F800).unwrap()));
        assert!(is_chinese_char(char::from_u32(0x2F900).unwrap()));
        assert!(is_chinese_char(char::from_u32(0x2FA1F).unwrap()));

        // 边界外
        assert!(!is_chinese_char(char::from_u32(0x2FA20).unwrap()));
    }

    #[test]
    fn test_validate_string_length_multibyte_boundary() {
        // 测试多字节字符在边界截断的情况
        // 注意：validate_string_length 使用 chars().take() 是按字符数截取，不是字节数
        let chinese = "中文字符测试".to_string();

        // 截取2个字符
        let result = validate_string_length(chinese.clone(), 2, "中文测试");
        assert_eq!(result.chars().count(), 2);
        assert_eq!(result, "中文");

        // 截取4个字符
        let result = validate_string_length(chinese.clone(), 4, "中文测试");
        assert_eq!(result.chars().count(), 4);
        assert_eq!(result, "中文字符");

        // 截取6个字符
        let result = validate_string_length(chinese.clone(), 6, "中文测试");
        assert_eq!(result.chars().count(), 6);
        assert_eq!(result, "中文字符测试");

        // 测试 emoji（每个emoji是1个字符）
        let emoji = "A😀B😁C".to_string();
        let result = validate_string_length(emoji, 5, "emoji测试");
        assert_eq!(result, "A😀B😁C"); // 5个字符正好

        // 测试 emoji 截断
        let emoji = "A😀B😁C".to_string();
        let result = validate_string_length(emoji, 3, "emoji测试");
        assert_eq!(result, "A😀B"); // 3个字符
    }

    #[test]
    fn test_validate_string_length_exact_boundary() {
        // 测试精确边界条件
        let s = "ABC".to_string();

        // 正好等于最大长度
        let result = validate_string_length(s.clone(), 3, "边界测试");
        assert_eq!(result, "ABC");

        // 超过1字节
        let result = validate_string_length(s.clone(), 2, "边界测试");
        assert_eq!(result, "AB");

        // 超过很多
        let result = validate_string_length(s.clone(), 0, "边界测试");
        assert_eq!(result, "");
    }

    #[test]
    fn test_validate_required_list_length_zero_max() {
        // 测试 max_len = 0 的情况
        let empty: Vec<String> = vec![];
        assert!(!validate_required_list_length(&empty, 0, "零列表"));

        // 有元素但max_len为0
        let with_items = vec!["a".to_string()];
        assert!(!validate_required_list_length(&with_items, 0, "零列表"));
    }

    #[test]
    fn test_validate_required_list_length_large_lists() {
        // 测试大列表
        let large: Vec<String> = (0..100).map(|i| format!("item{i}")).collect();
        assert!(validate_required_list_length(&large, 100, "大列表"));
        assert!(validate_required_list_length(&large, 200, "更大列表"));
        assert!(!validate_required_list_length(&large, 50, "小列表"));
        assert!(!validate_required_list_length(&large, 99, "刚好少一个"));
    }

    #[test]
    fn test_validate_required_list_length_single_item() {
        // 测试单元素列表
        let single = vec!["only".to_string()];
        assert!(validate_required_list_length(&single, 1, "单元素"));
        assert!(validate_required_list_length(&single, 5, "宽松限制"));
        assert!(!validate_required_list_length(&single, 0, "零限制"));
    }

    #[test]
    fn test_validate_content_size_mb_sizes() {
        // 测试 MB 级别的内容大小
        let content_1mb = "x".repeat(1024 * 1024);
        assert!(validate_content_size(&content_1mb, 1024 * 1024, "1MB内容"));
        assert!(!validate_content_size(
            &content_1mb,
            1024 * 1024 - 1,
            "1MB-1内容"
        ));

        let content_10mb = "x".repeat(10 * 1024 * 1024);
        assert!(validate_content_size(
            &content_10mb,
            10 * 1024 * 1024,
            "10MB内容"
        ));
        assert!(!validate_content_size(
            &content_10mb,
            10 * 1024 * 1024 - 1,
            "10MB-1内容"
        ));
    }

    #[test]
    fn test_validate_content_size_empty_and_small() {
        // 测试空内容和小内容
        assert!(validate_content_size("", 0, "空内容0限制"));
        assert!(validate_content_size("", 1, "空内容1限制"));
        assert!(validate_content_size("a", 1, "单字节1限制"));
        assert!(!validate_content_size("a", 0, "单字节0限制"));

        // 测试中文内容
        assert!(validate_content_size("你好", 6, "中文6字节")); // "你好" = 6字节
        assert!(!validate_content_size("你好", 5, "中文5字节"));
    }

    #[test]
    fn test_default_validate_builder_chain_multiple_validations() {
        // 测试多验证链式调用
        let builder = DefaultValidateBuilder::new()
            .required(Some("start".to_string()), "字段1")
            .length("mid".to_string(), 2, 10, "字段2")
            .custom("end".to_string(), |v| v.len() < 10, "自定义校验");

        // 注意：由于builder的消费性质，最后设置的value会覆盖之前的
        // 这里需要理解builder的行为：每次调用都消耗self并返回新builder
        let result = builder.build();
        assert!(result.is_ok());
    }

    #[test]
    fn test_default_validate_builder_multiple_errors() {
        // 测试多错误累积
        let builder = DefaultValidateBuilder::new()
            .required(None, "字段1")
            .required(Some("".to_string()), "字段2")
            .length("a".to_string(), 5, 10, "字段3");

        let result = builder.build();
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(errors.len() >= 2); // 应该有多个错误
    }

    #[test]
    fn test_default_validate_builder_custom_validator() {
        // 测试自定义验证器的各种情况
        let email_regex = |v: &str| v.contains('@') && v.contains('.');

        // 通过的情况
        let result = DefaultValidateBuilder::new()
            .custom("test@example.com".to_string(), email_regex, "无效的邮箱")
            .build();
        assert!(result.is_ok());

        // 失败的情况
        let result = DefaultValidateBuilder::new()
            .custom("invalid-email".to_string(), email_regex, "无效的邮箱")
            .build();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), vec!["无效的邮箱"]);
    }

    #[test]
    fn test_default_validate_builder_length_exact_min_max() {
        // 测试长度验证的精确边界
        // 正好等于最小值
        let result = DefaultValidateBuilder::new()
            .length("abc".to_string(), 3, 10, "长度测试")
            .build();
        assert!(result.is_ok());

        // 正好等于最大值
        let result = DefaultValidateBuilder::new()
            .length("abcdefghij".to_string(), 1, 10, "长度测试")
            .build();
        assert!(result.is_ok());

        // 小于最小值
        let result = DefaultValidateBuilder::new()
            .length("ab".to_string(), 3, 10, "长度测试")
            .build();
        assert!(result.is_err());

        // 大于最大值
        let result = DefaultValidateBuilder::new()
            .length("abcdefghijk".to_string(), 1, 10, "长度测试")
            .build();
        assert!(result.is_err());
    }

    #[test]
    fn test_validation_result_edge_cases() {
        // 测试 ValidationResult 的边缘情况
        let valid = ValidationResult::Valid;
        assert!(valid.is_valid());
        assert_eq!(valid.error_message(), None);
        assert_eq!(valid.sanitized_value(), None);
        assert_eq!(valid.into_result(), Ok(String::new()));

        // 空错误消息
        let invalid_empty = ValidationResult::Invalid("".to_string());
        assert!(!invalid_empty.is_valid());
        assert_eq!(invalid_empty.error_message(), Some(&"".to_string()));

        // 空清理值
        let sanitized_empty = ValidationResult::Sanitized("".to_string());
        assert!(!sanitized_empty.is_valid());
        assert_eq!(sanitized_empty.sanitized_value(), Some(&"".to_string()));
        assert_eq!(sanitized_empty.into_result(), Ok("".to_string()));
    }
}
