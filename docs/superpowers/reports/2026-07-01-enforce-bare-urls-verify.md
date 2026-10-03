# 验证报告：enforce-bare-urls

- Change: enforce-bare-urls（issue #273 Part A 现象 B / A2）
- 分支: feature/20260701/enforce-bare-urls
- base-ref: 9221ca0fd
- HEAD: d4e3cc07a
- verify_mode: full（17 tasks / 1565 文件 / 1 capability）
- 日期: 2026-07-01

## Summary

| 维度 | 状态 |
|------|------|
| Completeness | 17/17 tasks `[x]`，3 Requirements 全覆盖 |
| Correctness | 3/3 spec scenario 通过；D1-D5 决策全部落地 |
| Coherence | 实现与 Design Doc / design.md / proposal 一致；无矛盾 |

**Final Assessment：全检查通过，ready for archive。**

## 1. Completeness

- tasks.md：`grep -c '\- \[ \]'` = **0**（17/17 `[x]`）
- plan：35 step 复选框全 `[x]`（plan line 9 的 `- [ ]` 是 agentic-workers 说明文字，非任务）

### Spec coverage（3 ADDED Requirements）
| Requirement | 验证 |
|---|---|
| Req1 doc comment URL MUST `<>` 包裹 | ✅ `cargo doc --workspace --all-features` bare_urls warning = **0**（迁移前 1580）；新增裸 URL 在 deny 下即时报错 |
| Req2 workspace deny + CI 不压制 | ✅ `[workspace.lints.rustdoc] bare_urls="deny"`（Cargo.toml）；ci.yml `RUSTDOCFLAGS="-D warnings"`（无 `-A`）；全 `.github/` 残留 `-A rustdoc::bare_urls` = 0 |
| Req3 codegen MUST 发射 `<URL>` | ✅ codegen_render.py:73 `//! docPath: <{ir.doc_path}>` + restructure_hr.py:46 `//! docPath: <{doc_path}>` |

## 2. Correctness（design D1-D5）

| 决策 | 落地 |
|---|---|
| D1 脚本批量修 + cargo doc 闭环 | ✅ wrap_bare_urls.py 包裹 1981 URL；闭环 1580→0 |
| D2 codegen 同步改造 | ✅ 两处发射点改 `<URL>`（codegen_render.py:73 + restructure_hr.py:46） |
| D3 workspace deny | ✅ `[workspace.lints.rustdoc] bare_urls="deny"`；21 crate 继承 |
| D4 CI 解压 | ✅ ci.yml 移除 `-A rustdoc::bare_urls`；CI 模拟 Finished |
| D5 capability | ✅ rustdoc-bare-urls spec（3 Requirements） |

## 3. Coherence

- delta spec（3 Req）与 Design Doc 五位一体完全一致，无矛盾
- proposal 目标（清零 1578 + deny 锁死 + codegen 防复发 + CI 解压）全部达成
- 公共 API 无变化（纯 doc comment + lint 配置）

## 4. 验证命令实测证据（build Task 7 + verify 复核，fresh run）

| 命令 | 结果 |
|---|---|
| `cargo fmt --check` | exit 0 |
| `cargo doc --workspace --all-features --no-deps` | warnings=0 bare_urls=0 errors=0（deny 下） |
| `RUSTDOCFLAGS="-D warnings" cargo doc --workspace --all-features` | Finished（CI 模拟，exit 0） |
| `just lint`（clippy --all-features -Dwarnings -A missing_docs） | Finished |
| `cargo clippy --workspace --all-targets --no-default-features -- -D warnings` | Finished |
| `cargo build --workspace --all-features` | Finished |
| `cargo +1.88 check --workspace --all-features --locked`（pinned msrv lockfile） | Finished |

## 5. code review（review_mode=standard，build 阶段执行）

- 审查范围：非机械部分（脚本 regex / codegen / Cargo.toml+CI / 误伤修复完备性）
- 独立复核 cargo doc = 0、deny 全局生效、CI 协同、codegen 覆盖完备、脚本幂等
- **C1 Critical（已修）**：client lib.rs:71-72 的 `` `<https://...`。> `` markdown 损坏（Task 4 审计漏检，cargo doc 对 markdown 损坏盲眼）→ 改 clean autolink `<URL>`（commit d4e3cc07a）。**这是本次最重要的教训**：cargo doc 零警告 ≠ markdown 渲染正确，backtick+标点混排需人工抽样。
- **M1 Minor（接受）**：wrap_bare_urls.py regex 对 backtick+中文标点边界——脚本一次性，codegen 改造后不再批量运行；边界已在 C1 修复人工处理。
- **M2 Minor（接受）**：design doc 风险描述补记上述教训（已在 C1 commit body 记录）。

## 6. 安全检查
- 无硬编码密钥/凭证 ✅
- 无新增 `unsafe` ✅
- 无 `.env`/敏感文件提交 ✅
- 仅 doc comment 文本 + lint 配置，无逻辑/权限变更 ✅

## 7. 执行发现（供复盘）
- **wrap 脚本误伤两轮**：(1) URL 在 `"..."`/`` `...` `` 内时尾随分隔符被吸进 `<>`（16 处，已改进 regex + 就地修正）；(2) backtick code span + 中文句号混排致 `<>` 跨 span（2 处，C1 已修）。两者都暴露 **cargo doc 闭环对 markdown 损坏盲眼**——bare_urls 治理不能只靠 cargo doc，需人工抽样 backtick/标点混排。
- **plan 的 baseline grep 用 `rustdoc::bare_urls` 命中 14（去重 note 行）**，真实 warning 计数用 `this URL is not a hyperlink`（1580）。post-fix 断言 `grep -c bare_urls = 0` 仍有效。

## Final Assessment
全检查通过（Completeness / Correctness / Coherence + 7 项 full 验证 + 安全 + review C1 已修 + M1/M2 接受并记录）。**Ready for archive。**
