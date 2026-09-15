# 🤝 贡献指南

欢迎参与 openlark 项目！本指南将帮助你了解如何为项目做出贡献。

---

## 🎯 贡献方式

### 💻 代码贡献
- **新功能开发**: 实现新的API接口或服务模块
- **Bug修复**: 修复已知问题和缺陷
- **性能优化**: 提升代码执行效率
- **重构改进**: 改善代码结构和可维护性

### 📚 文档贡献
- **文档完善**: 改进现有文档的准确性和完整性
- **示例代码**: 添加新的使用示例和最佳实践
- **翻译工作**: 协助文档的多语言化
- **教程创作**: 编写入门和进阶教程

### 🐛 问题反馈
- **Bug报告**: 报告发现的问题和缺陷
- **功能建议**: 提出新功能需求和改进建议
- **用户体验**: 反馈使用过程中的体验问题
- **文档问题**: 指出文档中的错误或不清晰之处

---

## 🚀 快速开始

### 1. 环境准备
```bash
# 克隆项目
git clone https://github.com/foxzool/openlark.git
cd openlark

# 安装依赖（edition 2024，MSRV 1.88+）
cargo build --workspace --all-features

# 运行测试
cargo test --workspace --all-features
```

### 2. 开发环境配置
```bash
# 复制环境配置
cp .env-example .env

# 可选：安装 just，然后用 justfile 中的目标
cargo install just
just --list  # 查看可用命令
```

若未安装 `just`，可直接使用等价 cargo 命令（见 [`AGENTS.md`](AGENTS.md) Cursor Cloud 说明）。

### 3. 代码规范检查
```bash
# 格式化代码
just fmt

# 运行代码检查
just lint

# 运行完整检查
just check-all
```

---

## 📋 开发流程

### 🔄 标准流程
1. **Fork项目** → 在GitHub上fork原项目
2. **创建分支** → 基于main分支创建功能分支
3. **开发代码** → 遵循代码规范进行开发
4. **测试验证** → 确保所有测试通过
5. **提交PR** → 提交Pull Request并描述变更

### 🌿 分支规范
- **main**: 主分支，稳定版本
- **feature/xxx**: 新功能分支
- **fix/xxx**: Bug修复分支
- **docs/xxx**: 文档更新分支
- **refactor/xxx**: 重构分支

### 💬 提交信息规范
遵循[约定式提交](https://www.conventionalcommits.org/zh-hans/)格式：

```
<类型>[可选的作用域]: <描述>

[可选的正文]

[可选的脚注]
```

#### 提交类型
- **feat**: 新功能 ✨
- **fix**: Bug修复 🐛
- **docs**: 文档更新 📝
- **style**: 代码格式化 🎨
- **refactor**: 重构 ♻️
- **perf**: 性能优化 ⚡
- **test**: 测试相关 ✅
- **chore**: 构建过程或辅助工具的变动 🔧

#### 示例
```bash
feat(im): 添加消息撤回功能
fix(auth): 修复token刷新逻辑错误
docs(api): 更新认证接口文档
```

---

## 🛠️ 开发指南

### 📁 项目结构

当前仓库是 **Cargo workspace**（不是早期单体 `src/service/` 布局）。概览：

```
openlark/
├── crates/                 # 业务与基础设施 crates（openlark-*）
├── src/                    # 根 crate `openlark` 导出
├── examples/               # 可编译公开示例
├── docs/                   # 维护者文档（索引：docs/README.md）
├── tests/                  # 集成测试
├── tools/                  # 开发/校验脚本
└── AGENTS.md               # 结构、约定与常用命令
```

新增 API / 模块布局的权威说明见 [`AGENTS.md`](AGENTS.md) 与 [`docs/README.md`](docs/README.md)。

运行时生成的覆盖率等报告写在 gitignored 的 `reports/` 下，不作为仓库文档入口。

### 🎯 新服务模块开发

按业务域在对应 `crates/openlark-<domain>/` 下按版本目录添加 API（`v1/`、`v2/` 等），请求使用 Builder 模式，模型用 Serde，校验统一走 `CoreError` / `validate_required!`。完整模板见：

- [`.agents/skills/openlark-api/`](.agents/skills/openlark-api/)
- [`docs/api-implementation-template.md`](docs/api-implementation-template.md)
- [`docs/API_DESIGN_SPECIFICATION.md`](docs/API_DESIGN_SPECIFICATION.md)

文档与测试约定见 [`TESTING.md`](TESTING.md)；公开示例见 [`examples/README.md`](examples/README.md)。

---

## ✅ 质量标准

### 📊 代码质量要求
- **编译通过**: 所有代码必须能够正常编译
- **测试覆盖**: 新功能必须包含测试用例
- **文档完整**: 公开API必须有完整文档
- **格式规范**: 代码必须通过格式化检查
- **Clippy检查**: 必须通过Clippy静态检查

### 📝 文档质量要求
- **中文优先**: 面向中国开发者的中文文档
- **示例完整**: 每个功能都要有使用示例
- **测试可运行**: 文档中的代码示例必须可以编译
- **结构清晰**: 遵循统一的文档结构规范

### 🔍 PR审查标准
- **功能正确**: 实现符合需求和预期
- **代码质量**: 遵循项目代码规范
- **测试充分**: 有合适的测试覆盖
- **文档完善**: 包含必要的文档更新
- **向后兼容**: 不破坏现有API兼容性

---

## 🛡️ 安全注意事项

### 🔐 敏感信息处理
- **永不提交**: 不要提交API密钥、令牌等敏感信息
- **使用环境变量**: 通过.env文件管理配置
- **示例数据**: 使用虚拟数据作为示例

### 🧪 测试数据安全
- **沙箱环境**: 使用测试环境进行开发
- **数据隔离**: 避免使用生产环境数据
- **权限最小化**: 使用最小必要权限

---

## 🤝 社区规范

### 💬 交流原则
- **友善尊重**: 保持友善和建设性的交流
- **技术导向**: 专注于技术问题和解决方案
- **包容开放**: 欢迎不同背景的贡献者
- **耐心帮助**: 积极帮助新手和初学者

### 📋 行为准则
- 使用包容性语言
- 尊重不同观点和经验
- 优雅地接受建设性批评
- 专注于对社区最有利的事情

---

## 📞 获取帮助

### 🔗 联系方式
- **GitHub Issues**: [提出问题](https://github.com/foxzool/openlark/issues)
- **GitHub Discussions**: [社区讨论](https://github.com/foxzool/openlark/discussions)

### 📚 学习资源
- **API文档**: [docs.rs/openlark](https://docs.rs/openlark)
- **示例代码**: [examples目录](examples/)
- **维护者文档索引**: [docs/README.md](docs/README.md)
- **Rust学习**: [Rust官方文档](https://doc.rust-lang.org/)

---

## 🎉 致谢

感谢你对 openlark 项目的关注和贡献！每一个贡献都让这个项目变得更好。

**贡献类型**: 🌟 所有贡献都同样重要  
**响应时间**: ⚡ 我们会尽快回复你的贡献  
**社区氛围**: 🤝 友善、包容、技术导向

---

*让我们一起构建更好的飞书开放平台 Rust SDK！*