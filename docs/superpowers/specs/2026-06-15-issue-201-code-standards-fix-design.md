# 代码规范整改：Issue #201 子任务实施设计

**日期**: 2026-06-15
**关联 Issue**: #201（父）、#202、#203、#204、#205
**状态**: 已确认，待实施

## 背景

基于 `openlark-code-standards` 技能对 21 个业务 crate 的规范体检，创建了父 issue #201 及 4 个子 issue。在实施前的探索中，对每个 issue 的真实工作量做了精确复核，发现原 issue 描述与代码实际情况存在偏差，本设计基于复核后的准确数据。

## 探索阶段的修正

| Issue | 原描述 | 复核后真实情况 |
|--------|--------|----------------|
| #202 | "非测试代码约 1,731 处 unwrap/expect" | 真实库代码缺陷仅 2 处（`client/utils.rs:104-105`），其余均在 `tests/` 或 `#[cfg(test)]` 内 |
| #203 | "validate_required_list! 仅 23 次，推广不足" | 精确误用 13 处（`Vec` 必填字段用了 `validate_required!`） |
| #204 | "11 个 crate 缺失 prelude.rs" | 14/20 个 crate 已有 prelude（内联 `pub mod prelude` 范式），仅 bot/protocol 2 个缺失且合理 |
| #205 | "`#[doc]` 零散残留 5 处" | 5 处全是合法用法（`#[doc(hidden)]`、宏内动态文档），非风格问题 |

## 实施范围

| Issue | 处理方式 | 工作量 |
|--------|----------|--------|
| #202 | 重构 + breaking change | 1 个 PR，改动 `utils.rs` |
| #203 | 机械替换 | 1 个 PR，改动 13 处 |
| #204 | 关闭（误报） | issue 评论 |
| #205 | 关闭（误报） | issue 评论 |

执行顺序：#202 → #203 → 关闭 #204/#205。

## 分支与 PR 规划

```
main
 ├── fix/issue-202-unwrap-treatment   → PR "fix: 治理非测试代码 unwrap/expect (#202)"
 └── fix/issue-203-validate-list      → PR "refactor: Vec 必填字段统一用 validate_required_list! (#203)"
```

每个 fix 分支从 `main` 切出，完成后开 PR 回 `main`，标题引用 issue 编号触发自动关联。#204/#205 无需分支，直接评论关闭。

## #202 实施细节：重构 check_env_config

### 问题

`openlark-client/src/utils.rs:104-105` 的 `env::var("OPENLARK_APP_ID").unwrap()` 和 `env::var("OPENLARK_APP_SECRET").unwrap()` 违反 AGENTS.md 反模式（库代码禁用 unwrap）。

逻辑上，`create_config_from_env` 在 line 102 先调用 `check_env_config()?`，该函数已校验两个环境变量的存在性与非空。因此 line 104-105 的 unwrap 不会触发，但属冗余读取且违反规范。

### 方案：check_env_config 返回校验后的凭证

引入结构体承载校验结果，消除重复读取：

```rust
/// 环境变量校验结果
pub struct EnvConfig {
    pub app_id: String,
    pub app_secret: String,
}

pub fn check_env_config() -> Result<EnvConfig> {
    // ... 校验逻辑不变 ...
    Ok(EnvConfig { app_id, app_secret })
}
```

### 改动清单

1. **`utils.rs` — 重构 `check_env_config`**
   - 签名：`Result<()>` → `Result<EnvConfig>`
   - 新增 `EnvConfig` 结构体定义
   - body 末尾 `Ok(())` → `Ok(EnvConfig { app_id, app_secret })`
   - 复用函数内已读取的 `app_id`/`app_secret`

2. **`utils.rs:100` — `create_config_from_env` 消费返回值**
   - 删除 line 104-105 的二次 `env::var().unwrap()`
   - 改用 `env_cfg.app_id` / `env_cfg.app_secret`

3. **`utils.rs:225` — `diagnose_system` 适配**
   - `Ok(())` → `Ok(_)`（忽略值，仅关心成功/失败）

4. **`lib.rs` 测试无需改动**
   - 已核实：14 处 `check_env_config` 测试仅用 `result.is_ok()`/`result.is_err()` 断言（`lib.rs:592-715`），签名从 `Result<()>` 改为 `Result<EnvConfig>` 不影响这些断言
   - 2 处 `create_config_from_env` 测试（`lib.rs:722,740`）同样仅断言 ok/err，无需改动

### 兼容性

- `check_env_config` 是 `pub fn`，改返回值是 **breaking change**
- `openlark-client` 当前 0.17.0，AGENTS.md 注明处于 Config 迁移期，`utils` 属内部辅助
- **决策**：接受 breaking change，PR 描述标注 `BREAKING CHANGE: check_env_config 返回 EnvConfig`

### 审查项（保留不改）

`openlark-webhook/src/common/signature.rs:15,29` 和 `openlark-core/src/testing/mock_context.rs:28,58` 的 `.expect()`：
- 均带详尽 SAFETY 注释，逻辑上不可能失败
- 符合 Rust 惯例（HMAC 密钥长度、系统时间、测试运行时）
- 在 #202 issue 评论说明这类属可接受例外

## #203 实施细节：推广 validate_required_list!

### 问题

13 处 `Vec` 必填字段误用 `validate_required!`（仅校验非空），未校验长度上限。应改用 `validate_required_list!`（校验非空 + 长度上限）。

### MAX_LEN 取值依据

按仓库现有惯例（23 处 `validate_required_list!` 的取值）：

| 字段类型 | MAX_LEN | 依据 |
|----------|---------|------|
| `user_ids: Vec<String>` | 50 | `user_flow/query.rs:91`、`user_task/query.rs:91`、`batch.rs:108` 统一用 50 |
| `employee_ids: Vec<String>` | 50 | 同 user_ids 惯例 |
| `records: Vec<Record>` / `Vec<DatasourceRecord>` | 100 | `upload_report.rs:50` 用 100 |
| `field_ids: Vec<String>` | 50 | ID 列表惯例 |
| `shifts: Vec<...>` | 100 | 批量操作惯例（参考 reports 100） |
| `texts: Vec<String>` | 50 | 文本列表，保守取 50 |
| `file: Vec<u8>` | 1 | 单文件二进制，参考 `file/upload.rs:54` 用 1 |

### 13 处改动清单

| # | 文件 | 字段 | MAX_LEN |
|---|------|------|---------|
| 1 | `hr/attendance/v1/user_stats_view/update.rs:63` | `field_ids` | 50 |
| 2 | `hr/attendance/v1/user_daily_shift/batch_create_temp.rs:43` | `shifts` | 100 |
| 3 | `hr/attendance/v1/user_daily_shift/batch_create.rs:43` | `shifts` | 100 |
| 4 | `hr/attendance/v1/user_setting/query.rs:43` | `user_ids` | 50 |
| 5 | `hr/payroll/v1/datasource_record/save.rs:58` | `employee_ids` | 50 |
| 6 | `hr/payroll/v1/datasource_record/save.rs:59` | `records` | 100 |
| 7 | `hr/performance/v1/stage_task/find_by_user_list.rs:56` | `user_ids` | 50 |
| 8 | `hr/performance/v2/additional_information/query.rs:56` | `user_ids` | 50 |
| 9 | `docs/bitable/v1/app/table/record/batch_create.rs:120` | `records` | 100 |
| 10 | `docs/bitable/v1/app/table/record/batch_update.rs:114` | `records` | 100 |
| 11 | `ai/translation/v1/text/translate.rs:32` | `texts` | 50 |
| 12 | `ai/document_ai/v1/bank_card/recognize.rs:30` | `file` | 1 |
| 13 | `platform/admin/v1/badge/grant/create.rs:56` | `user_ids` | 50 |

### 改动模式

```rust
// 改前
validate_required!(self.field_ids, "field_ids 不能为空");

// 改后
validate_required_list!(self.field_ids, 50, "field_ids 不能为空且不能超过 50 个");
```

错误消息包含上限值，便于用户理解失败原因，与现有惯例一致（如 `user_flow/query.rs:91` 的 `"用户 ID 列表不能为空且不能超过 50 个"`）。

### 不在范围

- 不新增测试（lint+test 验证不新增测试）
- 不改动已有的 23 处 `validate_required_list!`（已规范）
- 不改动非必填的 `Option<Vec<T>>` 字段（无需校验）

## #204 / #205 关闭处理

### #204（prelude）

评论说明：
- 真实情况：14/20 crate 已有 prelude（内联 `pub mod prelude { ... }` 范式）
- `openlark-bot`：webhook 场景机器人辅助，无独立 Client/Service 入口，不需要 prelude
- `openlark-protocol`：底层 WebSocket 协议定义，类型经 `openlark_core` 间接消费，不需要独立 prelude
- 原 issue 基于错误的"11 个缺失"判断，关闭

### #205（文档属性）

评论说明：
- 5 处 `#[doc]` 全部合法：
  - `client/src/config.rs:84`、`lib.rs:456,460,467` — `#[doc(hidden)]` 隐藏内部 re-export（标准实践）
  - `client/src/client/macros.rs:45` — `#[doc = $doc]` 宏内动态文档（标准实践）
- `//!` 模块注释已是主流（1692 处），无风格不一致，关闭

## 验证流程

每个 PR 遵循相同验证步骤：

```bash
just fmt    # 格式化
just lint   # Clippy 检查
just test   # 测试不回归
```

### PR 验收标准

| PR | 验收点 |
|----|--------|
| #202 | `utils.rs` 无 `env::var.*unwrap`；14 个 `check_env_config` 测试通过；`create_config_from_env` 测试通过 |
| #203 | 13 处全部替换为 `validate_required_list!`；`just test` 通过 |

### 不验证项

- 不新增测试
- 不跑 `just check-all`（留作合并前最终检查）
