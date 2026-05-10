//! HR 模块测试模板
//!
//! 此文件作为新测试文件的模板参考

use rstest::*;

/// 示例测试模块
#[cfg(test)]
mod template_tests {
    use super::*;

    #[test]
    fn test_example() {
        // 模板示例：实际使用时删除或替换为有意义的测试
        let value = 42;
        assert_eq!(value, 42);
    }

    #[rstest]
    #[case(1)]
    #[case(2)]
    #[case(3)]
    fn test_with_rstest(#[case] value: i32) {
        assert!(value > 0);
    }
}
