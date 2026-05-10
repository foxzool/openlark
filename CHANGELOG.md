# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Compatibility

#### Typed APIs

#### Helpers & Convenience Methods

#### Breaking Changes

#### Deprecations

#### Migration Notes

### Added

### Changed

### Fixed

## [0.16.0] - 2026-05-10

### 🔄 变更

- **style(fmt)**: 统一代码格式化，修复多个文件的格式问题
- **docs(docPath)**: 为 260+ 个 API 文件补充 docPath 官方文档链接
- **refactor(api)**: 删除 explorer/permission v2 的函数式 API，统一使用 Builder 模式
- **refactor(api)**: 统一 platform/helpdesk 40 个文件的 execute() 委托模式，消除代码重复
- **refactor(validate)**: 统一 44 个文件的必填字段校验，使用 validate_required! 宏替换手工校验
- **refactor(types)**: 替换 calendar v4 的 serde_json::Value 为强类型结构体
- **fix(exports)**: 补充 5 个 mod.rs 文件的模型显式导出
- **fix(url)**: 修复 exchange_binding/get.rs 的 API 端点路径拼写错误
- **ci**: 修复 clippy 警告和文档注释缺失
- **build(deps)**: 升级安全相关依赖（tokio-tungstenite、url、reqwest）
- **build(rust)**: 对齐 Rust 2024 / MSRV 1.88

### 🐛 修复

- **fix(ci)**: 修复 CI 持续失败问题（clippy 警告、格式问题）
- **fix(security)**: 添加 max_response_size / ResponseTooLarge HTTP 与 WebSocket 响应大小限制
- **fix(security)**: Token/PII 日志脱敏
- **fix(security)**: path 参数 percent-encoding 安全修复
- **fix(code)**: 生产代码与测试代码 unwrap() 清理
- **fix(auth)**: AuthTokenProvider 多租户缓存 key 修复

## [0.15.0] - 2026-04-05

### 🔄 变更

- **release(root)**: 将工作区版本与根 crate 文案切换到 `0.15.0`
- **docs(release)**: 新增 `0.15` 迁移指南、public API 稳定性策略与正式版发布清障清单
- **ci(examples)**: 为根 README 对齐示例与主推 examples 增加编译校验入口
- **build(metadata)**: 修正 crates.io 元数据中的仓库与文档链接

### 🐛 修复

- **fix(release)**: 修复 GitHub Release 对 RC tag 一律标为正式版的问题
- **fix(docs)**: 修复多个 crate README 中错误的 crate 名、仓库链接和过期版本示例

## [0.15.0-rc.2] - 2026-03-26

### 🔄 变更

- **refactor(root)**: 将 `openlark` 收敛为唯一官方入口 crate，直接导出 `Client`、`ClientBuilder`、`Config`、`RequestOption`、`CoreError` 等高频类型
- **refactor(features)**: 重构根 crate feature 模型，移除面向用户的 `client`/`protocol` 心智，统一为业务 feature、技术 feature 与组合 feature
- **refactor(client)**: 将 `openlark-client` 明确为高级入口，不再作为普通用户的默认接入方式
- **docs(examples)**: 统一 README 和主 examples 到 `openlark` 根入口，修复 `workflow` 示例门槛与实际依赖不一致问题
- **build(lints)**: 将 `workspace.lints` 真正落到各成员 crate，统一工作区 lint 配置

## [0.15.0-rc.1] - 2026-03-17

### ✨ 新增功能

- **feat(webhook)**: 集成 openlark-webhook 模块到工作空间（8 个 API）
  - 自定义机器人、Webhook 事件处理
- **feat(hr)**: 实现 462 个 API (Wave 1-5)，涵盖招聘、CoreHR、考勤、薪酬等模块，总计 562 个 API
- **feat(workflow)**: 完成 workflow 模块 100% API 覆盖（117 个 API）
  - TASK v1 剩余 28 个 API、TASK v2 剩余 24 个 API
  - APPROVAL v4 16 个 API
  - BOARD 模块命名规范修复
- **feat(platform)**: 完成 openlark-platform Transport API 迁移（102 个 API）
- **feat(ai)**: 完成 openlark-ai 模块 27 个 API 实现
- **feat**: 实现缺失的 bizTag API（100% 覆盖）
- **feat(examples)**: 新增长连接 WebSocket Echo 示例并补充测试
- **feat(core)**: 新增测试基础设施模块（testing）
  - `test_runtime()` - 安全的测试运行时
  - `assert_res_ok!`, `assert_err_contains!` 等断言宏
- **feat(client)**: 新增 LazyService 延迟初始化工具
- **docs**: 添加 AGENTS.md 项目知识库

### 🔄 变更

- **refactor(docs)**: 简化 API 入口设计，删除 Service 层，统一 Request 模式
- **refactor(docs)**: 将 glob 重导出转换为显式导出（258 → 7 处）
- **perf(docs)**: 优化 Config 传递，使用 `Arc<Config>` 替代 `Config`
- **refactor(meeting)**: 统一 Request 模式，删除冗余 RequestBuilder
- **refactor(hr)**: 统一架构并添加 feature gating 支持
- **refactor(core)**: 为 testing 模块添加 feature gate
- **refactor(core)**: 清理未使用的空 features，将测试依赖移动到 dev-dependencies
- **refactor**: 实现显式导出系统，消除 251+ 个通配符导出
- **style(security)**: 修复命名规范异常，替换硬编码 URL，统一代码风格

### 🐛 修复

- **fix(core)**: 统一 validate_required 语义，支持字符串 trim
- **fix(docs)**: 修复 sheets_v2 数据读取 API 路径
- **fix(docs)**: 修复 Arc<Config> 类型不匹配错误
- **fix(docs)**: 修复 explorer v2 模块编译错误和导出问题
- **fix(hr)**: 添加 CoreHR 缺失的 API 端点定义并修复语法错误
- **fix**: 修复 no-default-features 编译错误
- **fix**: 修复多个 crate 的代码风格和导出
- **fix(examples)**: 修复 examples 编译错误
- **fix(ci)**: 修复 Coverage 工作流覆盖率收集问题（多次迭代修复）

### 🧪 测试

- 大幅提升测试覆盖率至 ~47%
- 为所有主要模块添加测试：docs、workflow、platform、cardkit、hr、meeting、auth、core
- 为 workflow v1/v2 模块添加完整测试套件
- 迁移 44 个测试文件到新框架，消除 144 处 `Runtime::new().unwrap()`

## [0.15.1] - 2025-11-20

### 🔄 架构优化 - openlark-docs 链式调用支持与 API 覆盖率更新

#### ✅ 核心能力增强

- **🔗 openlark-docs 完整链式调用支持** - 通过 openlark-client 提供流畅的链式调用体验
  - **DocsClient 集成** - Client 结构体包含 DocsClient 字段，启用 `docs` feature 即可使用
  - **完整链式调用路径** - `client.docs.ccm.drive.v1()`, `client.docs.ccm.sheets.v3()`, `client.docs.base.bitable()` 等
  - **类型安全** - 编译时验证所有链式调用路径

#### 📊 openlark-docs API 覆盖率验证

- **✅ 254 个 API，100% 完成** - 全面验证 openlark-docs 的 API 实现情况
  - **✅ 零未完成标记** - 无 TODO/FIXME/unimplemented! 标记
  - **✅ 代码质量优异** - 所有实现文件经过验证，零编译警告

#### 📈 模块实现状态详情

| 模块 | API 数量 | 已实现 | 未实现 | 完成率 |
|------|---------|--------|--------|--------|
| **CCM** | 174 | 174 | 0 | 100% |
| **BASE** | 49 | 49 | 0 | 100% |
| **BAIKE** | 27 | 27 | 0 | 100% |
| **MINUTES** | 4 | 4 | 0 | 100% |

#### 🏗️ 架构优化

- **链式调用架构** - DocsClient 通过字段链式访问所有子服务
  - **模块化设计** - ccm, base, baike, minutes 清晰的功能分层
  - **类型安全接口** - 提供服务访问器方法，如 `client.docs.ccm.drive.v1()`
  - **配置透传** - 支持从 DocsClient 获取底层服务

#### 📋 文档与示例

- **链式调用文档** - 详细的链式调用路径和使用示例
- **API 验证报告** - `docs/API_COVERAGE_REPORT.md` 提供详细的实现状态
- **模块级文档** - 每个 AGENTS.md 提供模块特定的使用指南

#### 🔧 技术债务清理

- **零未完成标记** - 扫描所有 API 文件，无 TODO/FIXME/unimplemented! 标记
- **代码质量优秀** - 所有实现文件通过编译和 lint 检查
- **架构清晰** - 严格的模块划分和命名规范

#### 📊 性能与质量

- **编译性能** - 默认功能 0.6s，全功能验证 0.37s
- **零警告构建** - 所有模块通过 clippy 检查
- **测试覆盖** - 核心功能完整测试覆盖
