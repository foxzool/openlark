---
comet_change: fix-missing-docs-lint-consistency
role: technical-design
canonical_spec: openspec
---

# fix-missing-docs-lint-consistency 技术设计

> 关联 OpenSpec change：`openspec/changes/fix-missing-docs-lint-consistency/`（canonical spec 以 OpenSpec 为准）。
> 范围：issue #273 Part A1 **最小版**——missing_docs lint 执行一致性 + 安全 outlier 收编。
> analytics / Python 死测试 / codegen / 占位 doc 治理均为 Non-Goals（另案）。

## 1. 背景与现状

missing_docs lint 跨 5 层治理，执行层不一致：

| 层 | 现状 | 一致性 |
|----|------|--------|
| `[workspace.lints.rust]`（Cargo.toml:63） | `missing_docs = "warn"` | 基线 |
| `just lint`（justfile:14） | `... -Dwarnings -A missing_docs` | ❌ 放过（与 CI 相反） |
| CI lint job（ci.yml:91） | `RUSTFLAGS="-D warnings"`，无 `-A` | ✅ 强制 |
| 源码 `#![deny]` | security(lib.rs:88)、client 死注释(lib.rs:238) | ❌ 冗余/残留 |
| 源码 `#![allow]` | analytics(lib.rs:35) | ❌ 隐藏（另案） |

净效果：「同一份代码，just 绿、CI 红」——开发者本地无法复现 CI 的 missing_docs 失败。

调研另发现 17 个 missing_docs Python 测试不在 CI 跑（死测试）、1057 行占位 doc 撑着 0 警告——均不在本 change 范围。

## 2. 实现方案（3 处零风险改动）

### D1: `just lint` 移除 `-A missing_docs`，对齐 CI

`justfile:14`：

```just
# before
cargo clippy --workspace --all-targets --all-features -- -Dwarnings -A missing_docs
# after
cargo clippy --workspace --all-targets --all-features -- -Dwarnings
```

CI ci.yml:91（`RUSTFLAGS="-D warnings"` 无 `-A`）是事实标准，just 对齐。移除后 missing_docs 现状 0 警告（其他 crate 全文档化 + analytics 自带 allow 压制），`just lint` 仍通过。

### D2: 移除 security `#![deny(missing_docs)]`（回落 warn）

`crates/openlark-security/src/lib.rs:88` 的 `#![deny]` 删除。security 当前以 deny 编译通过 → 全文档化 → 回落 workspace warn 后 missing_docs 仍 0。统一到 `[workspace.lints.rust]` 单一来源。

### D3: 移除 client 死注释

`crates/openlark-client/src/lib.rs:238` 的 `//#![deny(missing_docs)]  // 暂时禁用以完成基本编译` 删除（被注释掉无作用，纯技术债）。

### D4: 保留 protocol item 级例外（不动）

`crates/openlark-protocol/src/lib.rs:9` 的 `#[allow(missing_docs)]` 是 item 级（vendored pbbp2 生成模块），`test_workspace_missing_docs.py:37` allowlist 唯一条目，保留。

## 3. 决策依据

- **D1**：消除「本地绿 CI 红」复现盲区；CI 是事实标准
- **D2**：security deny 是冗余升级（workspace 已 warn），移除统一治理点；零风险（全文档化）
- **D3**：死注释清理，无行为变化
- **D4**：protocol 例外已登记，保留

## 4. 范围外（诚实限制 / 另案）

- **analytics `#![allow(missing_docs)]`**（lib.rs:35）：隐藏未文档化项，移除须回补 analytics doc → 独立 change
- **17 个 missing_docs Python 测试不在 CI**（死测试）：加入 CI 或删除 → 独立决策
- **codegen `tools/codegen.py:185` 的 `-A missing_docs`**：codegen 闭环范围
- **1057 行占位 doc**（`/// 待补充文档。`）：独立 doc 质量治理 change
- **不升 workspace `missing_docs = "deny"`**：保持 warn

spec `lint-execution-consistency` 已用边界条款声明 analytics 不在本 requirement 范围，不 over-claim。

## 5. 测试策略

| 验证项 | 命令 | 预期 |
|--------|------|------|
| **核心断言** | `cargo clippy -p openlark-security --all-features -- -Dwarnings` | Finished（移除 deny 后 0 警告） |
| just 对齐 | `just lint`（移除 -A 后） | Finished（与 CI 一致） |
| missing_docs 仍 0 | `cargo doc --workspace --all-features` | 0 warning |
| clippy ndf | `cargo clippy --workspace --all-targets --no-default-features -- -D warnings` | Finished |
| build | `cargo build --workspace --all-features` | Finished |
| MSRV | `cargo +1.88 check --locked`（pinned） | Finished |
| outlier 清理 | `grep -rn 'deny(missing_docs)' crates/openlark-security crates/openlark-client` | 空 |
| 格式 | `cargo fmt --check` | exit 0 |

## 6. 风险

- **[移除 security deny 暴露警告]** → Mitigation：security 当前 deny 编译通过 = 全文档化；build 阶段 `cargo clippy -p openlark-security -Dwarnings` 验证。
- 其余改动零风险（配置值修改 + 死注释删除）。
