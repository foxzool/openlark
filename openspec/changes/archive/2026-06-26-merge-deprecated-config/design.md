## 背景

两个 Config 并存：

| | core::config::Config | client::Config |
|---|---|---|
| 结构 | `Arc<ConfigInner>` 零拷贝 | 普通 struct（pub 字段） |
| 状态 | ✅ 业务 crate 在用 | ⚠️ deprecated 0.17.0 |
| from_env | ❌ | ✅ |
| validate + 白名单 | ❌ | ✅（SSRF 防护） |
| allow_custom_base_url | ❌ | ✅ |
| ConfigSummary | ❌ | ✅ |
| 超时字段 | `req_timeout: Option<Duration>` | `timeout: Duration` |
| headers 字段 | `header` | `headers` |

根 crate `src/lib.rs:31` re-export 的是 deprecated 的 client::Config。

## 高层决策（open 阶段已与用户确认）

1. **core 全吸收**：from_env / validate / base_url 白名单 / allow_custom_base_url / ConfigSummary 全部迁移到 core::Config，成为唯一丰富的 Config。client::Config 移除。
2. **v0.18 移除 client::Config**（breaking）：已 deprecated 一个版本，直接移除。根 crate re-export 改 core::Config。
3. **全量范围**：core + client + 根 crate + examples + 文档/CHANGELOG 一次性闭环。

## 方案大纲

```
core::config::Config（吸收后）
├── ConfigInner += allow_custom_base_url: bool
├── from_env() / load_from_env()       ← 从 client 迁移，适配 Arc 封装
├── validate() + is_known_base_url()   ← SSRF 防护上移到 core
├── ConfigSummary + summary()          ← 从 client 迁移
├── builder() += allow_custom_base_url()
└── （保留）Arc 零拷贝 / with_token_provider / accessors

client crate
├── 移除 config.rs 的 Config/ConfigBuilder/ConfigSummary
├── Client::new(Config) / builder() 衔接 core::Config
└── CoreConfig 别名可保留或移除

根 crate src/lib.rs:31
└── pub use openlark_core::config::Config;   // 改指向 core
```

## 留待 design 阶段（brainstorming）解决的分叉

以下未知项不在 open 阶段定案，进入 comet-design 深度设计：

1. **`builder().build()` 是否引入 validate**：core 当前 build 不校验，client build 校验。吸收后若 core build 默认校验，是行为变化（影响所有 core::Config 用户，含业务 crate）。需权衡「默认安全」vs「不破坏现有 core 用户」。
2. **字段命名统一**：client `timeout`/`headers`（复数）vs core `req_timeout`/`header`。是否在 core 保留 `timeout()`/`headers()` 别名方便迁移？
3. **`ConfigInner` 加 `allow_custom_base_url`** 对 Arc 克隆、`Debug`、`with_token_provider`（逐字段重建 ConfigInner）的影响——需同步所有构造点。
4. **`Client::builder()` 迁移路径**：client crate 的 Client 构造当前依赖 client::Config，如何衔接 core::Config（含/不含 TokenProvider）。
5. **`OPENLARK_TIMEOUT`（client `Duration`）vs core `req_timeout: Option<Duration>`** 语义对齐（默认值差异：client 默认 30s，core 默认 None/永不超时）。

## 非目标

- 不改 core::Config 的 `Arc<ConfigInner>` 零拷贝架构
- 不改业务 crate 已有的 core::Config 用法（保持源码兼容，除非字段名 breaking）
- 不引入新配置能力（仅迁移已有功能）
