# 设计审查报告（模板）

> 用法：复制本模板到 PR 描述或 Issue；把“证据”补齐到具体 `path:line`。

## 结论概览

- [ ] 结论 1（1 句话）
- [ ] 结论 2（1 句话）
- [ ] 结论 3（1 句话）

## 范围与约束

- 审查范围：`<crate/目录/模块>`
- 目标：`<一致性/可用性/可维护性/编译体积/性能/兼容性>`
- Breaking change：`允许 / 不允许`
- 相关 feature：`<features...>`
- 运行验证：`cargo check ...` / `cargo test ...`（可选贴输出摘要）

## 接口完成度（排除 meta.Version=old）

> 数据来源建议使用：`python3 tools/validate_apis.py --crate <crate>`（默认跳过 old）。

- 统计口径：`api_list_export.csv` 中 `meta.Version != old`
- 完成度判定：严格按落盘路径命名规范对比（见 `tools/validate_apis.py`）

| 维度 | 预期 API | 已实现 | 未实现 | 完成率 |
|---|---:|---:|---:|---:|
| 总计 | `<N>` | `<N>` | `<N>` | `<N>%` |

（可选）按 bizTag：

| bizTag | 预期 API | 已实现 | 未实现 | 完成率 |
|---|---:|---:|---:|---:|
| `<bizTag>` | `<N>` | `<N>` | `<N>` | `<N>%` |

## 问题清单（P0~P3）

> 规则：每条必须包含：现象 → 证据 → 影响 → 建议

### P0（阻塞/高风险）

1. **现象**：`<一句话>`
   - **证据**：`path/to/file.rs:123`
   - **影响**：`<对用户/对稳定性/对兼容性的影响>`
   - **建议**：`<可落地改造动作>`

### P1（重要/高收益）

1. **现象**：`<一句话>`
   - **证据**：`path/to/file.rs:123`
   - **影响**：`<...>`
   - **建议**：`<...>`

### P2（中优先级/一致性/债务）

1. **现象**：`<一句话>`
   - **证据**：`path/to/file.rs:123`
   - **影响**：`<...>`
   - **建议**：`<...>`

### P3（低优先级/清理/文档）

1. **现象**：`<一句话>`
   - **证据**：`path/to/file.rs:123`
   - **影响**：`<...>`
   - **建议**：`<...>`

## 收敛方案（必须选 1 套）

### 方案 A：Request 自持 Config（流式 Builder）

- 适用原因：`<为什么适合该 crate>`
- 核心规范：
  - `Request::new(Config)` 持有 `Config`
  - 对外统一 `execute()` / `execute_with_options(RequestOption)`
  - Service 只做“分组/版本入口”，不承载网络逻辑
- 迁移策略：
  - 第 1 步：`<试点模块>`
  - 第 2 步：`<逐步迁移>`
  - 兼容：旧入口 `deprecated` + 给替代路径

### 方案 B：Builder → build(Request) → execute(Service)

- 适用原因：`<为什么适合该 crate>`
- 核心规范：
  - Builder 只拼 Request，不做网络
  - Service 持有 Config，统一执行与注入点
  - 使用 `openlark_core::trait_system::ExecutableBuilder`（trait 定义在 `crates/openlark-core/src/trait_system/executable_builder.rs:11`；业务 crate 通过宏 impl，如 `crates/openlark-meeting/src/common/macros.rs`）
  - ⚠️ trait 名是 `ExecutableBuilder`（非单字母 `n`），引用写完整路径
- 迁移策略：
  - 第 1 步：`<试点模块>`
  - 第 2 步：`<逐步迁移>`
  - 兼容：旧入口 `deprecated` + 给替代路径

## 可执行 TODO（<= 10 条）

1) P0：`<动作>`（涉及：`path/to/file.rs`）
2) P0：`<动作>`（涉及：`path/to/file.rs`）
3) P1：`<动作>`（涉及：`path/to/file.rs`）
4) P1：`<动作>`（涉及：`path/to/file.rs`）
5) P2：`<动作>`（涉及：`path/to/file.rs`）

## 风险与回滚

- 风险：`<Breaking/行为变化/feature gating 变化>`
- 回滚策略：`<如何回滚>`
