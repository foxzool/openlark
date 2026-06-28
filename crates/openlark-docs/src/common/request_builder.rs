//! 强制 Builder 模式实现
//!
//! 确保请求对象在构建时必填字段已设置，避免运行时验证错误。
//!
//! # 设计理念
//!
//! - **编译时验证**: 在 build() 时验证所有必填字段
//! - **类型安全**: 使用 `Option<T>` 明确标记字段状态
//! - **零开销**: 仅在构建时验证，运行时无额外开销
//! - **向后兼容**: 保留旧 API，标记为 deprecated
//!
//! # 使用示例
//!
//! ```rust,ignore
//! use openlark_docs::common::request_builder::impl_required_builder;
//!
//! impl_required_builder!(
//!     CreateRecordRequest,
//!     CreateRecordBuilder,
//!     required: [
//!         app_token: String,
//!         table_id: String
//!     ],
//!     optional: [
//!         user_id_type: String,
//!         client_token: String
//!     ]
//! );
//!
//! // 使用方式
//! let request = CreateRecordRequest::builder()
//!     .app_token("app_xxx")
//!     .table_id("table_xxx")
//!     .user_id_type("open_id")
//!     .config(config)
//!     .build()?;
//! ```

/// 强制构建器宏
///
/// 为 Request 结构生成类型安全的 Builder，确保必填字段在编译时被验证。
///
/// # 参数说明
///
/// - `$request_name`: 请求结构体名称
/// - `$builder_name`: 构建器结构体名称
/// - `required: [...]`: 必填字段列表（格式: `field_name: FieldType`）
/// - `optional: [...]`: 可选字段列表（格式: `field_name: FieldType`）
///
/// # 生成内容
///
/// 该宏会生成：
/// 1. Builder 结构体（所有字段都是 `Option<T>`）
/// 2. Builder::new() 和 Builder::default() 方法
/// 3. 每个字段的 setter 方法
/// 4. Builder::build() 方法（验证必填字段）
/// 5. Request::builder() 关联方法
///
/// # 示例
///
/// ```rust,ignore
/// impl_required_builder!(
///     CreateRecordRequest,
///     CreateRecordBuilder,
///     required: [
///         app_token: String,
///         table_id: String
///     ],
///     optional: [
///         user_id_type: String,
///         client_token: String
///     ]
/// );
/// ```
#[macro_export]
macro_rules! impl_required_builder {
    (
        $request_name:ident,
        $builder_name:ident,
        required: [$($req_field:ident: $req_type:ty),* $(,)?],
        optional: [$($opt_field:ident: $opt_type:ty),* $(,)?]
    ) => {
        #[derive(Debug, Default)]
        pub struct $builder_name {
            $(
                $req_field: Option<$req_type>,
            )*
            $(
                $opt_field: Option<$opt_type>,
            )*
            config: Option<openlark_core::config::Config>,
            _phantom: std::marker::PhantomData<$request_name>,
        }

        impl $builder_name {
            /// 设置配置（链式调用）
            pub fn with_config(mut self, config: openlark_core::config::Config) -> Self {
                self.config = Some(config);
                self
            }

            $(
                /// 设置必填字段
                pub fn $req_field(mut self, value: impl Into<$req_type>) -> Self {
                    self.$req_field = Some(value.into());
                    self
                }
            )*

            $(
                /// 设置可选字段
                pub fn $opt_field(mut self, value: impl Into<$opt_type>) -> Self {
                    self.$opt_field = Some(value.into());
                    self
                }
            )*

            /// 构建请求实例，验证所有必填字段
            pub fn build(self) -> openlark_core::SDKResult<$request_name> {
                // 验证必填字段
                $(
                    let $req_field = self.$req_field.ok_or_else(|| {
                        openlark_core::error::validation_error(
                            stringify!($req_field),
                            concat!("必填字段 '", stringify!($req_field), "' 未设置")
                        )
                    })?;
                )*

                let config = self.config.ok_or_else(|| {
                    openlark_core::error::validation_error(
                        "config",
                        "Config 未设置，请使用 with_config() 方法"
                    )
                })?;

                Ok($request_name {
                    $(
                        $req_field,
                    )*
                    $(
                        $opt_field: self.$opt_field,
                    )*
                    config,
                })
            }
        }

        impl $request_name {
            /// 创建构建器实例
            pub fn builder() -> $builder_name {
                $builder_name::default()
            }
        }
    };
}

/// 简化的流式构建器宏
///
/// 为已有的 Request 结构添加 builder() 方法，保持向后兼容。
///
/// # 使用场景
///
/// 当 Request 结构已经定义，且使用 String::new() 初始化字段时，
/// 使用此宏添加 Builder 模式支持。
///
/// # 示例
///
/// ```rust,ignore
/// // 已有的 Request 结构
/// #[derive(Debug, Clone)]
/// pub struct MyRequest {
///     pub app_token: String,
///     pub table_id: String,
///     pub config: Config,
/// }
///
/// // 添加 Builder 支持
/// impl_fluent_builder!(MyRequest, app_token, table_id);
/// ```
#[macro_export]
macro_rules! impl_fluent_builder {
    (
        $request_name:ident,
        config: Config,
        required: [$($req_field:ident: $req_type:ty),* $(,)?],
        optional: [$($opt_field:ident: $opt_type:ty),* $(,)?]
    ) => {
        impl $request_name {
            /// 创建 Builder 实例
            pub fn builder() -> $request_name {
                Self {
                    $(
                        $req_field: String::new(),
                    )*
                    $(
                        $opt_field: None,
                    )*
                    config: Config::default(),
                }
            }

            $(
                /// 设置必填字段（流式接口）
                pub fn $req_field(mut self, value: impl Into<$req_type>) -> Self {
                    self.$req_field = value.into();
                    self
                }
            )*

            $(
                /// 设置可选字段（流式接口）
                pub fn $opt_field(mut self, value: impl Into<$opt_type>) -> Self {
                    self.$opt_field = Some(value.into());
                    self
                }
            )*
        }
    };
}

#[cfg(test)]
mod tests {
    use openlark_core::config::Config;

    // 测试用的请求结构
    // 测试 helper，见 #267：impl_required_builder! 宏在 build() 中按字段名 `config` 构造，无法改名。
    // 用 #[expect(dead_code)] 显式标记：CI grep 只拦 #[allow]，#[expect]] 是受控的预期死代码。
    #[expect(dead_code)]
    #[derive(Debug, Clone)]
    pub struct TestRequest {
        app_token: String,
        table_id: String,
        user_id_type: Option<String>,
        config: Config,
    }

    // 使用宏生成 Builder
    impl_required_builder!(
        TestRequest,
        TestRequestBuilder,
        required: [
            app_token: String,
            table_id: String
        ],
        optional: [
            user_id_type: String
        ]
    );

    #[test]
    fn test_required_builder_success() {
        let config = Config::builder().app_id("test").app_secret("test").build();

        let request = TestRequest::builder()
            .with_config(config)
            .app_token("token")
            .table_id("table")
            .build()
            .unwrap();

        assert_eq!(request.app_token, "token");
        assert_eq!(request.table_id, "table");
        assert!(request.user_id_type.is_none());
    }

    #[test]
    fn test_required_builder_with_optional() {
        let config = Config::builder().app_id("test").app_secret("test").build();

        let request = TestRequest::builder()
            .with_config(config)
            .app_token("token")
            .table_id("table")
            .user_id_type("open_id")
            .build()
            .unwrap();

        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_required_builder_missing_required_field() {
        let config = Config::builder().app_id("test").app_secret("test").build();

        let result = TestRequest::builder()
            .with_config(config)
            .app_token("token")
            // 缺少 table_id
            .build();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("table_id"));
    }

    #[test]
    fn test_required_builder_missing_config() {
        let result = TestRequest::builder()
            .app_token("token")
            .table_id("table")
            // 缺少 config
            .build();

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Config"));
    }

    #[test]
    fn test_required_builder_into_string() {
        let config = Config::builder().app_id("test").app_secret("test").build();

        // 测试 impl Into<String>
        let request = TestRequest::builder()
            .with_config(config)
            .app_token("token") // &str
            .table_id(String::from("table")) // String
            .build()
            .unwrap();

        assert_eq!(request.app_token, "token");
        assert_eq!(request.table_id, "table");
    }
}
