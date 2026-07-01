## 1. 规范文档落盘

- [ ] 1.1 创建 `docs/FEATURE_NAMING_CONVENTION.md`：写明「模块 feature 为主、版本 feature 为单 API 版本 crate 合法例外」原则；附三套方案现状清单（A 单版本门控 / B 纯模块门控 / C 混合）、6 个单版本 crate 合法例外清单，并显式纠正 issue #272 对 `openlark-bot` v4 的误判（v4 门控 live 代码）
- [ ] 1.2 在 `AGENTS.md` 的 CONVENTIONS 段补一行指向 `docs/FEATURE_NAMING_CONVENTION.md`

## 2. 移除 5 个混合 crate 的死版本链

- [ ] 2.1 `openlark-platform/Cargo.toml`：移除 `v1`/`v2`/`v3`/`v4` 定义，`full` 去掉 `"v4"`
- [ ] 2.2 `openlark-analytics/Cargo.toml`：移除 `v1`/`v2`/`v3`/`v4` 定义，`full` 去掉 `"v4"`
- [ ] 2.3 `openlark-security/Cargo.toml`：移除 `v1`/`v2`/`v3` 定义，`full` 去掉 `"v3"`
- [ ] 2.4 `openlark-docs/Cargo.toml`：移除 `v1`/`v2`/`v3` 定义，`full` 去掉 `"v3"`
- [ ] 2.5 `openlark-user/Cargo.toml`：移除 `v2`/`v3`/`v4` 定义（**保留 live 的 `v1`**），`full` 去掉 `"v4"`

## 3. 同步下游 docs feature 引用

- [ ] 3.1 `crates/openlark-client/Cargo.toml`：`docs` feature 移除 `"openlark-docs/v2"`、`"openlark-docs/v3"` 引用
- [ ] 3.2 根 `Cargo.toml`：workspace `docs` feature 移除 `"openlark-docs/v2"`、`"openlark-docs/v3"`；按 Q2 决策处理 `docs-sheets-v2`/`docs-sheets-v3`（退化为 `docs-ccm` 别名或移除）

## 4. 验证

- [ ] 4.1 `just fmt` + `cargo fmt --check` 通过
- [ ] 4.2 `just lint` 通过（CI 双模式：`--no-default-features` 与 `--all-features` 均过，确认无悬空 feature 引用）
- [ ] 4.3 `just build` + `just test` 通过
- [ ] 4.4 核实 `docs/FEATURE_MATRIX.md` 是否需同步更新移除的 feature（Q3）
