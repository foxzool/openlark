# OpenLark 测试指南

本文档提供 OpenLark SDK 测试的完整指南，包括快速开始、最佳实践和工具使用。

## 📚 目录

- [快速开始](#快速开始)
- [测试组织](#测试组织)
- [测试工具](#测试工具)
- [覆盖率](#覆盖率)
- [最佳实践](#最佳实践)
- [故障排查](#故障排查)

---

## 快速开始

### 运行所有测试

```bash
# 运行工作区所有测试
cargo test --workspace

# 运行特定 crate 的测试
cargo test -p openlark-core
cargo test -p openlark-auth
cargo test -p openlark-webhook
```

### 运行特定测试

```bash
# 运行匹配名称的测试
cargo test test_name

# 运行特定模块的测试
cargo test --lib validation

# 运行集成测试
cargo test --test integration_webhook
```

### 快速检查

```bash
# 格式检查
cargo fmt --check

# Lint 检查
cargo clippy -- -D warnings

# 完整检查（格式 + Lint + 测试）
just check-all
```

---

## 测试组织

### 目录结构

```
.
├── crates/
│   └── openlark-*/tests/     # 每个 crate 的集成测试
├── tests/
│   ├── integration/           # 跨 crate 集成测试
│   ├── property/              # 属性测试
│   └── live/                  # 真实环境测试（需要 .env）
└── benches/                   # 性能基准测试
```

### 测试类型

| 类型 | 位置 | 特点 |
|------|------|------|
| **单元测试** | `src/**/mod.rs` (`#[cfg(test)]`) | 测试内部逻辑 |
| **集成测试** | `crates/*/tests/` | 测试公共 API |
| **属性测试** | `tests/property/` | 随机输入验证 |
| **Live 测试** | `tests/live/` | 真实环境（.env） |

---

## 测试工具

### TestServer（推荐）

统一的 HTTP Mock 服务器封装。

```rust
use openlark_core::testing::prelude::*;
use serde_json::json;

#[tokio::test]
async fn test_example() {
    let server = TestServer::new().await;
    
    // Mock 成功响应
    server.mock_success("/api/v1/test", json!({"code": 0})).await;
    
    // Mock 错误响应
    server.mock_error("/api/v1/error", 400, json!({"code": 99991663})).await;
    
    // Mock 超时
    server.mock_timeout("/api/v1/slow", std::time::Duration::from_secs(10)).await;
    
    // 使用 server.uri() 作为 base_url
    let config = test_config(&server.uri());
    
    // 测试代码...
}
```

**API 参考**：
- `mock_success(route, body)` - 200 OK 响应
- `mock_error(route, code, error)` - 4xx/5xx 错误
- `mock_timeout(route, delay)` - 超时模拟
- `mock_get/put/delete()` - 其他 HTTP 方法
- `mock_with_verification(route, expected, response)` - 带请求体验证

### 测试配置

```rust
use openlark_core::testing::prelude::*;

// 快速创建测试配置
let config = test_config("http://localhost:8080");

// 自定义配置
let config = TestConfigBuilder::new()
    .app_id("custom_app_id")
    .app_secret("custom_secret")
    .base_url("http://custom.url")
    .timeout(Duration::from_secs(30))
    .build();
```

### 断言宏

```rust
use openlark_core::testing::prelude::*;

// Result 断言
let result = some_operation().await;
let value = assert_res_ok!(result, "operation should succeed");
assert_eq!(value.id, "123");

// 错误断言
let result = some_failing_operation().await;
assert_res_err!(result, CoreError::Validation { .. }, "should fail validation");

// 错误消息断言
let err = assert_err_contains!(result, "不能为空", "field validation");

// Option 断言
let opt = Some(42);
let value = assert_some!(opt, "should have value");
assert_none!(opt_none, "should be None");
```

### 参数化测试（rstest）

```rust
use rstest::rstest;

#[rstest]
#[case::success("valid_id", true)]
#[case::empty("", false)]
#[case::whitespace("  ", false)]
#[tokio::test]
async fn test_validation(
    #[case] input: &str,
    #[case] should_pass: bool,
) {
    let result = validate_input(input);
    
    if should_pass {
        assert!(result.is_ok());
    } else {
        assert!(result.is_err());
    }
}
```

---

## 覆盖率

### 生成覆盖率报告

```bash
# 工作区覆盖率
just coverage

# 特定 crate
cargo llvm-cov -p openlark-core --html
cargo llvm-cov -p openlark-auth --html

# 查看缺失行
cargo llvm-cov --workspace --show-missing-lines
```

### 覆盖率报告位置

- **HTML 报告**：`target/llvm-cov/html/index.html`
- **LCOV 格式**：`target/llvm-cov/lcov.info`
- **Per-crate 报告**：`target/llvm-cov/crates/*/`

### 覆盖率门禁

| 指标 | 阈值 | 说明 |
|------|------|------|
| **全局** | 40% | main 分支强制 |
| **openlark-core** | 40% | 警告 |
| **openlark-client** | 35% | 警告 |
| **openlark-auth** | 50% | 警告 |

**查看 CI 覆盖率**：
Coverage CI（行覆盖率）仅在 `push` 到 `main`（含合并后）时运行，PR 期间不触发以避免重复耗时测试。PR 开发请使用本地命令查看覆盖率。
1. 打开仓库 Actions 页面，查看 "Coverage" workflow（仅 main 分支 push 触发）
2. 查看 "Coverage" job 的 Step Summary
3. 下载 "coverage-report" artifact 查看 HTML 报告

PR 期间推荐：`just coverage` 或 `just coverage-check`（本地生成报告与阈值检查）。

---

## 最佳实践

### 1. 使用 TestServer 统一 Mock

✅ **推荐**：
```rust
let server = TestServer::new().await;
server.mock_success("/api/v1/test", json!({"code": 0})).await;
```

❌ **避免**：
```rust
let server = MockServer::start().await;
Mock::given(method("POST"))
    .and(path("/api/v1/test"))
    .respond_with(ResponseTemplate::new(200).set_body_json(json!({...})))
    .mount(&server)
    .await;
```

### 2. 测试三元组（每个 API）

对于每个 API 端点，至少测试：

1. **成功路径**：正常输入，200 OK
2. **参数校验**：空值、边界值、非法值
3. **错误映射**：4xx/5xx 响应

```rust
#[tokio::test]
async fn test_create_user_success() {
    // 成功路径
    let server = TestServer::new().await;
    server.mock_success("/api/v1/users", json!({"code": 0, "data": {"id": "123"}})).await;
    
    let result = create_user("valid_name").await;
    assert_res_ok!(result, "create_user");
}

#[tokio::test]
async fn test_create_user_validation() {
    // 参数校验
    let result = create_user("").await;
    assert_res_err!(result, CoreError::Validation { .. }, "empty name");
}

#[tokio::test]
async fn test_create_user_api_error() {
    // 错误映射
    let server = TestServer::new().await;
    server.mock_error("/api/v1/users", 400, json!({"code": 99991663, "msg": "invalid"})).await;
    
    let result = create_user("valid_name").await;
    assert_res_err!(result, CoreError::Api { .. }, "api error");
}
```

### 3. 模型序列化测试

```rust
#[test]
fn test_user_serialization_roundtrip() {
    let user = User {
        id: "123".to_string(),
        name: "张三".to_string(),
    };
    
    let json = serde_json::to_string(&user).expect("serialize");
    let decoded: User = serde_json::from_str(&json).expect("deserialize");
    
    assert_eq!(decoded.id, user.id);
    assert_eq!(decoded.name, user.name);
}
```

### 4. Feature-gated 测试

```rust
#[cfg(feature = "websocket")]
#[tokio::test]
async fn test_websocket_connection() {
    // 仅在启用 websocket feature 时运行
}

#[tokio::test]
async fn test_always_runs() {
    // 总是运行
}
```

### 5. Live 测试隔离

```rust
#[tokio::test]
#[ignore = "需要设置 OPENLARK_LIVE_TESTS=1"]
async fn test_live_auth() {
    if std::env::var("OPENLARK_LIVE_TESTS").is_err() {
        println!("⚠️  跳过 live 测试");
        return;
    }
    
    dotenvy::dotenv().ok();
    let app_id = std::env::var("APP_ID").expect("APP_ID must be set");
    
    // 真实环境测试...
}
```

---

## 故障排查

### 测试失败

**问题**：测试偶发失败（网络/时序相关）

**解决方案**：
1. 使用 `TestServer` Mock（不依赖外部网络）
2. 增加超时时间
3. 使用 `serial_test` 避免并发冲突

```rust
use serial_test::serial;

#[tokio::test]
#[serial]  // 串行执行
async fn test_concurrent_unsafe() {
    // ...
}
```

### 覆盖率报告不准确

**问题**：覆盖率报告显示 0% 或不准确

**解决方案**：
```bash
# 清理覆盖率数据
cargo llvm-cov clean --workspace

# 重新生成
cargo llvm-cov --workspace --all-features --html
```

### Feature 组合测试失败

**问题**：某些 feature 组合编译失败

**解决方案**：
```bash
# 检查特定 feature
cargo check -p openlark-core --no-default-features
cargo check -p openlark-core --features websocket

# 运行 feature 矩阵测试
just test-features-quick
```

### 找不到测试

**问题**：`cargo test` 没有运行某些测试

**解决方案**：
```bash
# 检查测试是否被 ignore
cargo test -- --include-ignored

# 检查特定测试文件
cargo test --test integration_webhook

# 列出所有测试
cargo test -- --list
```

---

## 开发者清单

每次添加新 API 时，确保：

- [ ] 至少 3 个测试（成功 + 校验 + 错误）
- [ ] 模型序列化/反序列化测试
- [ ] 使用 `TestServer` Mock
- [ ] 参数验证测试（使用 `validate_required!` 宏）
- [ ] Feature-gated 测试（如适用）
- [ ] 覆盖率 >= crate 平均水平

---

## 相关文档

- [测试迁移指南](./docs/TEST_MIGRATION.md) - 如何将现有测试迁移到 `TestServer`
- [API 设计规范](./AGENTS.md) - 代码规范和最佳实践
- [Coverage CI](./.github/workflows/coverage.yml) - CI 配置详情

---

## 快速参考

| 命令 | 说明 |
|------|------|
| `cargo test` | 运行所有测试 |
| `just coverage` | 生成覆盖率报告 |
| `just test-features-quick` | Feature 组合测试 |
| `just check-all` | 完整质量检查 |
| `cargo llvm-cov --html` | HTML 覆盖率报告 |

---

**问题反馈**：如果遇到问题，请在 [GitHub Issues](https://github.com/foxzool/openlark/issues) 提交。
