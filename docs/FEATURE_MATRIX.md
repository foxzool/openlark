# OpenLark Feature Matrix

本文档描述 OpenLark 的 feature 组合测试策略和维护指南。

## 背景

OpenLark 采用模块化架构，提供 50+ 个 feature flags 支持按需编译。为确保跨模块组合在发布前后不会出现回归，我们将常用 feature 组合纳入 CI 自动化校验。

## 目标

- 覆盖 `no-default-features`、默认 features、常见业务组合和 `all-features`
- 为新增 feature 或 feature 重构提供回归保护
- 明确 feature 组合清单的维护策略

## 当前常见组合矩阵

| 组合 | 目的 | 当前回归信号 |
|------|------|--------------|
| `no-default-features` | 验证最小构建不依赖业务模块 | 根 crate `src/lib.rs` 的最小构建测试 + CI matrix |
| `auth,communication` | 最常见消息/通讯录接入 | 根 crate communication 入口测试 + CI matrix |
| `auth,docs-bitable` | Docs helper / README 主路径 | 根 crate docs-bitable helper 测试 + public examples compile-check |
| `auth,docs-drive` | Drive 上传/下载主路径 | 根 crate docs-drive helper 测试 + CI matrix |
| `essential` | 推荐业务组合（auth + communication + docs） | 根 crate `essential` 组合测试 + CI matrix |
| `enterprise` | 企业级主组合（essential + security + hr + workflow） | 根 crate `enterprise` 组合测试 + CI matrix |
| `webhook-full` | 自定义机器人卡片 + 签名组合 | 根 crate webhook-full 组合测试 + CI matrix |
| `communication,websocket` | 长连接消息能力 | CI matrix + public examples compile-check |

## 回归测试入口

本仓库当前有两层 feature 组合回归保护：

1. **根 crate 组合契约测试**
   - 位置：`src/lib.rs`
   - 作用：验证常见组合下的公开入口与关键 helper 类型是否可用
2. **CI feature matrix**
   - 位置：`.github/workflows/ci.yml`
   - 作用：对常见组合执行 `cargo build/test --lib`

本地快速回归命令：

```bash
just test-features-quick
```

完整组合回归命令：

```bash
just test-features
```

## 高级矩阵的覆盖与完成证据

`Feature Matrix Testing` 的 `individual`、`core` 分别在独立 job 执行，
每个 job 上限 30 分钟；计划生成与测试共用 20 分钟预算，剩余时间供工具安装、
缓存恢复与报告上传使用。任一命令非零退出、启动失败、被信号终止或超时，
对应 job 都失败；日志和 summary 中的文字不会改变退出状态。

- **individual**：固定使用 cargo-hack 0.6.45，保留
  `--each-feature --exclude-features websocket --lib` 的选择规则，以根包 `openlark`
  为目标。先用 `--print-command-list` 获取计划，经格式与根 manifest 校验后，
  直接逐项执行这份计划，不再次枚举、不筛掉默认项。无法完整解析或为空的计划直接失败。
  `websocket` 只从主动单项中排除；其他 feature 的传递依赖仍按 Cargo 原有语义激活。
- **core**：根包 `openlark` 的 `auth,docs,communication,hr,ai,websocket`
  的空集、6 个单项和 15 个两两组合，共 **22 个逻辑项**。
  全部使用 `--no-default-features --lib`，保留因依赖关系而等效的逻辑项，
  不再固定开启六项后枚举其他业务 feature。历史 430 项不作为覆盖基准。
- **默认与全 feature**：individual 原有 `default` 项保留；既有显式矩阵的 `auth`
  项继续覆盖当前默认配置（`default = ["auth"]`），workspace 的 all-features 测试
  继续承担全 feature 覆盖。core 不额外加入 all-features，既有显式矩阵保持不变。

本地使用与 CI 相同的入口（Python 3.11+；individual 另需 cargo-hack 0.6.45）：

```bash
python3 tools/run_feature_matrix.py individual --report reports/feature-matrix/individual.json
python3 tools/run_feature_matrix.py core --report reports/feature-matrix/core.json
python3 -m unittest tools.tests.test_feature_matrix -v
```

`--timeout-seconds` 可调整整个 phase 的执行预算，默认 1200 秒，不会为每条命令重置。
超时会结束该命令的子进程组；失败后提前结束，其余项保留为 `pending`。
每项执行前后均更新 JSON，其中记录实际 argv、状态和退出码；summary 列出计划数、
已结束数、通过数、失败数、未运行数及耗时。只有每项都以 0 退出才报告 `passed`。
`completed` 包含已经失败的项，不能当作通过数。

每个 job 均上传 `feature-matrix-individual` 或 `feature-matrix-core` artifact，
其中包含与 run ID、测试 revision 关联的 JSON。验收时必须核对两个 phase 的完整计划
与实际通过结果，以及同次 CI 的 workspace all-features 测试。取消或硬终止可能使报告
停留在 `running`，或无法上传；缺失、未完成的报告均不能作为完整通过的证据。

## 相关文件

- [ci.yml](../.github/workflows/ci.yml) - CI 配置，包含 feature 组合测试
- [feature-matrix.yml](../.github/workflows/feature-matrix.yml) - 高级 feature 矩阵测试
- [Cargo.toml](../Cargo.toml) - Feature 定义
- [src/lib.rs](../src/lib.rs) - 根 crate 的组合契约测试
- [justfile](../justfile) - 本地 feature 回归入口

## 参考

- Issue #47 - 添加 CI checks 用于常用 feature 组合
- [Cargo Features 文档](https://doc.rust-lang.org/cargo/reference/features.html)
- [cargo-hack 文档](https://github.com/taiki-e/cargo-hack)
