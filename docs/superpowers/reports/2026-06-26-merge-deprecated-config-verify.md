# 验证报告：merge-deprecated-config

- Change: merge-deprecated-config
- Date: 2026-06-26
- verify_mode: full
- branch: merge-deprecated-config
- base..HEAD: 13676a2b..HEAD（26 files, +1351 -950）

## 结论：PASS

## 1. Fresh 技术验证（this-message evidence）

| 检查 | 命令 | 结果 |
|---|---|---|
| 测试 | `cargo test --workspace --all-features` | 84 组 `test result: ok`，0 failed |
| Lint | `cargo clippy --workspace --all-targets --all-features` | 干净（无 warning/error） |
| 编译 | `cargo check --workspace --all-targets --all-features` | 全绿（无 error） |
| 残留 | `rg 'openlark_client::Config\b\|pub struct Config\b' crates/openlark-client/src` | 无命中 |

## 2. Spec 合规验证（full，7 项）

1. **tasks.md 全部完成**：11/11 `[x]` ✓
2. **实现符合 design.md 5 分叉决策** ✓
   - 分叉 1：`builder().build()` 不校验 + `Config::validate()` + `from_env` 内部校验
   - 分叉 2：core 命名（`req_timeout` / `header`），不保留 client 别名
   - 分叉 3：`ConfigInner` 加 `allow_custom_base_url`，同步 Default/Debug/with_token_provider/build/new 所有构造点
   - 分叉 4：`ClientBuilder` 走 `ClientBuildConfig → build_core_config → Client::new(core::Config)`
   - 分叉 5：`req_timeout` 默认 `None`；`OPENLARK_TIMEOUT` → `req_timeout(Some(Duration))`
3. **实现符合 Design Doc** ✓
4. **能力规格场景全部通过** ✓（见 §3）
5. **proposal.md 目标满足** ✓（消除 client::Config / core 全吸收 / v0.18 breaking / 根 crate re-export core）
6. **delta spec 与 design doc 无矛盾** ✓
7. **Design Doc 可定位** ✓ `docs/superpowers/specs/2026-06-26-merge-deprecated-config-design.md`

## 3. Delta Spec Scenario 覆盖（TDD 测试映射）

### Requirement: 环境变量加载 Config
- 识别 `OPENLARK_*` → `test_config_from_env_reads_all_vars`（含 TIMEOUT→req_timeout(Some) 断言）
- from_env 不阻塞无效配置 → `test_config_from_env_invalid_does_not_block`

### Requirement: Config 校验与 base_url 白名单 SSRF 防护
- 白名单域名通过 → `test_config_validate_whitelist_ok`
- 非白名单拒绝（错误提示 `allow_custom_base_url`） → `test_config_validate_non_whitelist_rejected`
- `allow_custom_base_url` 豁免 → `test_config_validate_allow_custom_exempts_whitelist`
- app_id / app_secret 空 → `test_config_validate_empty_app_id` / `_empty_app_secret`
- `build()` 不自动校验 → `test_config_build_does_not_validate`

### Requirement: Config 配置摘要
- 不含敏感信息（结构体 + Display） → `test_config_summary_secret_not_leaked`

### Requirement: Config 支持 allow_custom_base_url
- 默认 false → `test_config_allow_custom_base_url_default`
- builder 设置 true → `test_config_builder_allow_custom_base_url`
- Arc 操作一致 → `with_token_provider` 同步（T1）+ `test_config_clone`

### REMOVED: openlark_client::Config
- 引用编译失败 → `config.rs` 删除 + `rg` 无残留 + `is_known_base_url` 提升 core pub

## 4. Code Review（review_mode=standard）

Code Reviewer 结论：**Ready for verify**，无 Critical。

| Issue | 级别 | 处理 |
|---|---|---|
| I-1 验证逻辑双份（`client_build_config::validate` vs `core::Config::validate`） | Important（非阻塞） | **接受为 follow-up**：reviewer 判定当前行为一致；统一委托超本次「消除 client::Config 类型」核心范围，记为独立清理项 |
| I-2 残留 `allow(deprecated)`（ws_client / lib.rs test / websocket_echo_bot） | Important | **已修复**（移除 3 处，cargo check 无 deprecated warning） |
| M-3 from_env 文档缺 validate 提示 | Minor | **已修复** |
| M-1 validate warn 冗余 / M-2 make_mut 性能 | Minor | 记录（make_mut 独占引用零成本，reqwest::Client clone 廉价，无性能陷阱） |

## Assessment

**PASS** — 实现完整、spec 合规、fresh 技术验证全绿。无 CRITICAL/IMPORTANT 未决项（I-1 已接受为 follow-up 并记录影响范围）。可进入分支处理 + archive。
