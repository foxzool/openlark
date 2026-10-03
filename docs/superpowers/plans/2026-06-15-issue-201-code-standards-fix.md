# 代码规范整改实施计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 修复 issue #202（unwrap 治理）、#203（validate_required_list! 推广），关闭误报 issue #204/#205

**Architecture:** 两个独立 PR 分别处理 #202（重构 check_env_config 返回 EnvConfig）和 #203（13 处 Vec 字段校验宏替换），#204/#205 通过 issue 评论关闭

**Tech Stack:** Rust, 飞书 SDK, openlark-core 的 validate_required_list! 宏

**关联 spec:** `docs/superpowers/specs/2026-06-15-issue-201-code-standards-fix-design.md`

---

## 文件结构

| Issue | 文件 | 操作 |
|-------|------|------|
| #202 | `crates/openlark-client/src/utils.rs` | 修改：新增 EnvConfig 结构体、重构 check_env_config、改 create_config_from_env、改 diagnose_system |
| #203 | `crates/openlark-hr/src/attendance/.../update.rs` 等 13 处 | 修改：每处 1 行宏替换 |

---

## Task 1: 创建 #202 分支

**Files:** 无（git 操作）

- [ ] **Step 1: 从 main 切出分支**

Run:
```bash
git checkout main
git pull
git checkout -b fix/issue-202-unwrap-treatment
```
Expected: 切换到新分支 `fix/issue-202-unwrap-treatment`

---

## Task 2: 重构 check_env_config 返回 EnvConfig

**Files:**
- Modify: `crates/openlark-client/src/utils.rs:30-91`（check_env_config 定义）
- Modify: `crates/openlark-client/src/utils.rs:100-105`（create_config_from_env）

- [ ] **Step 1: 在 check_env_config 函数前添加 EnvConfig 结构体定义**

在 `crates/openlark-client/src/utils.rs` 中，找到 `pub fn check_env_config() -> Result<()> {`（约 line 30），在其**上方**插入：

```rust
/// 环境变量校验结果
///
/// 由 `check_env_config` 返回，承载已校验的必填凭证，
/// 避免调用方重复读取环境变量。
pub struct EnvConfig {
    /// 飞书应用 App ID（已校验非空）
    pub app_id: String,
    /// 飞书应用 App Secret（已校验非空）
    pub app_secret: String,
}
```

- [ ] **Step 2: 修改 check_env_config 签名与返回值**

将签名 `pub fn check_env_config() -> Result<()> {` 改为：

```rust
pub fn check_env_config() -> Result<EnvConfig> {
```

将函数末尾的 `Ok(())`（约 line 90）改为：

```rust
    Ok(EnvConfig { app_id, app_secret })
```

注意：`app_id` 和 `app_secret` 已在函数体前部读取（line 32, 47），直接复用，无需改动读取逻辑。

- [ ] **Step 3: 修改 create_config_from_env 消费返回值**

将 `crates/openlark-client/src/utils.rs:100-105`：

```rust
pub fn create_config_from_env() -> Result<Config> {
    // 先检查环境变量
    check_env_config()?;

    let app_id = env::var("OPENLARK_APP_ID").unwrap();
    let app_secret = env::var("OPENLARK_APP_SECRET").unwrap();
```

改为：

```rust
pub fn create_config_from_env() -> Result<Config> {
    // 校验环境变量并获取已校验的凭证
    let env_cfg = check_env_config()?;

    let app_id = env_cfg.app_id;
    let app_secret = env_cfg.app_secret;
```

- [ ] **Step 4: 修改 diagnose_system 的 Ok 模式匹配**

将 `crates/openlark-client/src/utils.rs:225-226`：

```rust
    match check_env_config() {
        Ok(()) => {
```

改为：

```rust
    match check_env_config() {
        Ok(_) => {
```

- [ ] **Step 5: 验证编译**

Run:
```bash
cargo build -p openlark-client
```
Expected: 编译通过，无错误

- [ ] **Step 6: 验证 utils.rs 无 env::var unwrap 残留**

Run:
```bash
grep -n "env::var.*\.unwrap()" crates/openlark-client/src/utils.rs
```
Expected: 无输出（line 104-105 的 unwrap 已删除）

- [ ] **Step 7: 运行 client crate 测试**

Run:
```bash
cargo test -p openlark-client
```
Expected: 全部测试通过（14 个 check_env_config 测试 + 2 个 create_config_from_env 测试，断言仅用 is_ok/is_err，签名变更不影响）

---

## Task 3: #202 验证与提交

**Files:** 无（验证与 git 操作）

- [ ] **Step 1: 格式化**

Run:
```bash
just fmt
```
Expected: 无格式错误

- [ ] **Step 2: Clippy 检查**

Run:
```bash
just lint
```
Expected: 无警告（`-Dwarnings` 模式通过）

- [ ] **Step 3: 提交**

Run:
```bash
git add crates/openlark-client/src/utils.rs
git commit -m "fix: 重构 check_env_config 返回 EnvConfig，消除 unwrap (#202)

- 新增 EnvConfig 结构体承载已校验的 app_id/app_secret
- create_config_from_env 消费返回值，删除 2 处 env::var().unwrap()
- diagnose_system 适配 Ok(_) 模式

BREAKING CHANGE: check_env_config 返回值从 Result<()> 改为 Result<EnvConfig>"
```
Expected: 提交成功

---

## Task 4: 创建 #202 PR

**Files:** 无（GitHub 操作）

- [ ] **Step 1: 推送分支**

Run:
```bash
git push -u origin fix/issue-202-unwrap-treatment
```
Expected: 推送成功

- [ ] **Step 2: 创建 PR**

Run:
```bash
gh pr create --repo foxzool/openlark \
  --base main \
  --head fix/issue-202-unwrap-treatment \
  --title "fix: 治理非测试代码 unwrap/expect (#202)" \
  --body "## 改动

重构 \`check_env_config\` 返回 \`EnvConfig\` 结构体，消除 \`create_config_from_env\` 中的 2 处 \`env::var().unwrap()\`。

## 探索发现

原 issue 描述的 '1731 处非测试 unwrap' 经精确复核，真实库代码缺陷仅 2 处（\`utils.rs:104-105\`），其余均在 \`tests/\` 目录或 \`#[cfg(test)]\` 模块内。

webhook/core 中的 \`.expect()\` 带 SAFETY 注释（HMAC 密钥长度、系统时间、测试运行时），逻辑上不可能失败，保留不改。

## Breaking Change

\`check_env_config\` 返回值从 \`Result<()>\` 改为 \`Result<EnvConfig>\`。\`openlark-client\` 处于 0.17.x Config 迁移期，\`utils\` 属内部辅助。

Closes #202" \
  --label "scope:quality,type:refactor,priority:p0,area:hr"
```
Expected: PR 创建成功，返回 PR URL

---

## Task 5: 创建 #203 分支

**Files:** 无（git 操作）

- [ ] **Step 1: 从 main 切出分支**

Run:
```bash
git checkout main
git checkout -b fix/issue-203-validate-list
```
Expected: 切换到新分支 `fix/issue-203-validate-list`

---

## Task 6: 替换 HR crate 的 8 处 Vec 字段校验

**Files:**
- Modify: `crates/openlark-hr/src/attendance/attendance/v1/user_stats_view/update.rs:63`
- Modify: `crates/openlark-hr/src/attendance/attendance/v1/user_daily_shift/batch_create_temp.rs:43`
- Modify: `crates/openlark-hr/src/attendance/attendance/v1/user_daily_shift/batch_create.rs:43`
- Modify: `crates/openlark-hr/src/attendance/attendance/v1/user_setting/query.rs:43`
- Modify: `crates/openlark-hr/src/payroll/payroll/v1/datasource_record/save.rs:58-59`
- Modify: `crates/openlark-hr/src/performance/performance/v1/stage_task/find_by_user_list.rs:56`
- Modify: `crates/openlark-hr/src/performance/performance/v2/additional_information/query.rs:56`

- [ ] **Step 1: 替换 user_stats_view/update.rs:63**

找到（约 line 63）：
```rust
validate_required!(self.field_ids, "field_ids");
```
改为：
```rust
validate_required_list!(self.field_ids, 50, "field_ids 不能为空且不能超过 50 个");
```

- [ ] **Step 2: 替换 user_daily_shift/batch_create_temp.rs:43**

找到（约 line 43）：
```rust
validate_required!(self.shifts, "shifts");
```
改为：
```rust
validate_required_list!(self.shifts, 100, "shifts 不能为空且不能超过 100 个");
```

- [ ] **Step 3: 替换 user_daily_shift/batch_create.rs:43**

找到（约 line 43）：
```rust
validate_required!(self.shifts, "shifts");
```
改为：
```rust
validate_required_list!(self.shifts, 100, "shifts 不能为空且不能超过 100 个");
```

- [ ] **Step 4: 替换 user_setting/query.rs:43**

找到（约 line 43）：
```rust
validate_required!(self.user_ids, "user_ids");
```
改为：
```rust
validate_required_list!(self.user_ids, 50, "user_ids 不能为空且不能超过 50 个");
```

- [ ] **Step 5: 替换 datasource_record/save.rs:58-59（2 处）**

找到（约 line 58-59）：
```rust
validate_required!(self.employee_ids, "employee_ids");
validate_required!(self.records, "records");
```
改为：
```rust
validate_required_list!(self.employee_ids, 50, "employee_ids 不能为空且不能超过 50 个");
validate_required_list!(self.records, 100, "records 不能为空且不能超过 100 个");
```

- [ ] **Step 6: 替换 stage_task/find_by_user_list.rs:56**

找到（约 line 56）：
```rust
validate_required!(self.user_ids, "user_ids");
```
改为：
```rust
validate_required_list!(self.user_ids, 50, "user_ids 不能为空且不能超过 50 个");
```

- [ ] **Step 7: 替换 additional_information/query.rs:56**

找到（约 line 56）：
```rust
validate_required!(self.user_ids, "user_ids");
```
改为：
```rust
validate_required_list!(self.user_ids, 50, "user_ids 不能为空且不能超过 50 个");
```

- [ ] **Step 8: 验证 HR crate 编译**

Run:
```bash
cargo build -p openlark-hr
```
Expected: 编译通过

---

## Task 7: 替换 docs/ai/platform crate 的 5 处 Vec 字段校验

**Files:**
- Modify: `crates/openlark-docs/src/base/bitable/v1/app/table/record/batch_create.rs:120`
- Modify: `crates/openlark-docs/src/base/bitable/v1/app/table/record/batch_update.rs:114`
- Modify: `crates/openlark-ai/src/ai/translation/v1/text/translate.rs:32`
- Modify: `crates/openlark-ai/src/ai/document_ai/v1/bank_card/recognize.rs:30`
- Modify: `crates/openlark-platform/src/admin/admin/v1/badge/grant/create.rs:56`

- [ ] **Step 1: 替换 docs batch_create.rs:120**

找到（约 line 120）：
```rust
validate_required!(self.records, "records");
```
改为：
```rust
validate_required_list!(self.records, 100, "records 不能为空且不能超过 100 个");
```

- [ ] **Step 2: 替换 docs batch_update.rs:114**

找到（约 line 114）：
```rust
validate_required!(self.records, "records");
```
改为：
```rust
validate_required_list!(self.records, 100, "records 不能为空且不能超过 100 个");
```

- [ ] **Step 3: 替换 ai translate.rs:32**

找到（约 line 32）：
```rust
validate_required!(self.texts, "texts 不能为空");
```
改为：
```rust
validate_required_list!(self.texts, 50, "texts 不能为空且不能超过 50 个");
```

- [ ] **Step 4: 替换 ai bank_card/recognize.rs:30**

找到（约 line 30）：
```rust
validate_required!(self.file, "file 不能为空");
```
改为：
```rust
validate_required_list!(self.file, 1, "file 不能为空");
```

注：`file: Vec<u8>` 是单文件二进制，MAX_LEN=1 表示恰好一个文件，沿用 `file/upload.rs:54` 的惯例。

- [ ] **Step 5: 替换 platform badge/grant/create.rs:56**

找到（约 line 56）：
```rust
validate_required!(self.user_ids, "用户ID列表不能为空");
```
改为：
```rust
validate_required_list!(self.user_ids, 50, "用户ID列表不能为空且不能超过 50 个");
```

- [ ] **Step 6: 验证全部编译**

Run:
```bash
cargo build -p openlark-docs -p openlark-ai -p openlark-platform
```
Expected: 编译通过

---

## Task 8: #203 验证与提交

**Files:** 无（验证与 git 操作）

- [ ] **Step 1: 确认 13 处全部替换完成**

Run:
```bash
python3 << 'EOF'
import os, re
from collections import Counter
suspects = []
for root, dirs, files in os.walk('crates'):
    for f in files:
        if not f.endswith('.rs'): continue
        path = os.path.join(root, f)
        if '/tests/' in path: continue
        try:
            with open(path) as fh:
                lines = fh.read().split('\n')
        except: continue
        vec_fields = {}
        for line in lines:
            m = re.match(r'\s*pub\s+(\w+):\s*(Vec<[^>]+>)', line)
            if m: vec_fields[m.group(1)] = m.group(2)
        for i, line in enumerate(lines, 1):
            m = re.search(r'validate_required!\(self\.(\w+)', line)
            if m and m.group(1) in vec_fields:
                suspects.append(f"{path}:{i} self.{m.group(1)}")
print(f"剩余 Vec 字段误用 validate_required!: {len(suspects)} 处")
for s in suspects: print(f"  {s}")
EOF
```
Expected: 输出 "剩余 Vec 字段误用 validate_required!: 0 处"

- [ ] **Step 2: 格式化**

Run:
```bash
just fmt
```
Expected: 无格式错误

- [ ] **Step 3: Clippy 检查**

Run:
```bash
just lint
```
Expected: 无警告

- [ ] **Step 4: 运行受影响 crate 的测试**

Run:
```bash
cargo test -p openlark-hr -p openlark-docs -p openlark-ai -p openlark-platform
```
Expected: 全部测试通过（现有校验测试不回归）

- [ ] **Step 5: 提交**

Run:
```bash
git add crates/openlark-hr crates/openlark-docs crates/openlark-ai crates/openlark-platform
git commit -m "refactor: Vec 必填字段统一用 validate_required_list! (#203)

替换 13 处 Vec 字段的 validate_required! 为 validate_required_list!，
补充长度上限校验（user_ids=50, records=100, file=1 等）。

涉及 crate: hr(8) docs(2) ai(2) platform(1)"
```
Expected: 提交成功

---

## Task 9: 创建 #203 PR

**Files:** 无（GitHub 操作）

- [ ] **Step 1: 推送分支**

Run:
```bash
git push -u origin fix/issue-203-validate-list
```
Expected: 推送成功

- [ ] **Step 2: 创建 PR**

Run:
```bash
gh pr create --repo foxzool/openlark \
  --base main \
  --head fix/issue-203-validate-list \
  --title "refactor: Vec 必填字段统一用 validate_required_list! (#203)" \
  --body "## 改动

将 13 处 \`Vec\` 必填字段的 \`validate_required!\` 替换为 \`validate_required_list!\`，补充长度上限校验。

## MAX_LEN 取值依据

按仓库现有 23 处 \`validate_required_list!\` 的惯例：
- \`user_ids\`/\`employee_ids\`/\`field_ids\`: 50
- \`records\`/\`shifts\`: 100
- \`texts\`: 50
- \`file: Vec<u8>\`: 1（单文件二进制）

## 涉及 crate

- openlark-hr: 8 处
- openlark-docs: 2 处
- openlark-ai: 2 处
- openlark-platform: 1 处

Closes #203" \
  --label "scope:api,type:refactor,priority:p1"
```
Expected: PR 创建成功

---

## Task 10: 关闭误报 issue #204

**Files:** 无（GitHub 操作）

- [ ] **Step 1: 在 #204 发表分析评论**

Run:
```bash
gh issue comment 204 --repo foxzool/openlark --body "## 探索复核结论

经精确复核，原 issue 基于错误的判断，实际现状如下：

**14/20 个 crate 已有 prelude**（采用内联 \`pub mod prelude { ... }\` 范式，非独立 \`prelude.rs\` 文件）：
\`\`\`
openlark-ai / analytics / application / auth / cardkit / client /
communication / core / docs / helpdesk / hr / mail / meeting /
platform / security / user / webhook / workflow
\`\`\`

**仅 2 个 crate 无 prelude，且属合理设计**：
- \`openlark-bot\`：webhook 场景的机器人辅助，无独立 Client/Service 入口
- \`openlark-protocol\`：底层 WebSocket 协议定义，类型经 \`openlark_core\` 间接消费

原 issue 描述的 '11 个 crate 缺失' 判断有误。关闭此 issue。"
```
Expected: 评论发表成功

- [ ] **Step 2: 关闭 #204**

Run:
```bash
gh issue close 204 --repo foxzool/openlark --reason "not planned"
```
Expected: issue 关闭

---

## Task 11: 关闭误报 issue #205

**Files:** 无（GitHub 操作）

- [ ] **Step 1: 在 #205 发表分析评论**

Run:
```bash
gh issue comment 205 --repo foxzool/openlark --body "## 探索复核结论

经精确复核，全仓 5 处 \`#[doc]\` 全部是合法用法，非风格残留：

| 位置 | 用法 | 说明 |
|------|------|------|
| \`client/src/config.rs:84\` | \`#[doc(hidden)]\` | 隐藏内部 re-export，标准实践 |
| \`client/src/lib.rs:456,460,467\` | \`#[doc(hidden)]\` | 隐藏内部 re-export，标准实践 |
| \`client/src/client/macros.rs:45\` | \`#[doc = \$doc]\` | 宏内动态生成文档，标准实践 |

\`//!\` 模块注释已是仓库主流（1692 处），不存在风格不一致问题。原 issue 的判断有误，关闭。"
```
Expected: 评论发表成功

- [ ] **Step 2: 关闭 #205**

Run:
```bash
gh issue close 205 --repo foxzool/openlark --reason "not planned"
```
Expected: issue 关闭

---

## Task 12: 更新父 issue #201 状态

**Files:** 无（GitHub 操作）

- [ ] **Step 1: 在 #201 评论汇总进展**

Run:
```bash
gh issue comment 201 --repo foxzool/openlark --body "## 整改进展

- #202：已开 PR（重构 check_env_config 返回 EnvConfig）
- #203：已开 PR（13 处 Vec 字段校验宏替换）
- #204：关闭（误报，14/20 crate 已有 prelude）
- #205：关闭（误报，#[doc] 全是合法用法）

父 issue 将在 #202、#203 两个 PR 合并后关闭。"
```
Expected: 评论发表成功
