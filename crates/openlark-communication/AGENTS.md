# openlark-communication Knowledge Base

**Crate**: IM & Contact Services  
**APIs**: 179 个（排除 old；`validate_apis.py --crate openlark-communication`） | **Status**: 生产就绪  
**Coverage**: 即时消息 + 通讯录 + 动态圈 + Aily + 事件

## OVERVIEW

飞书通讯协作模块，提供即时消息（IM）、通讯录管理和群组功能。默认启用的核心模块之一。目录 API 179（另 11 条 old 不计入）。

## STRUCTURE

```
src/
├── lib.rs                    # 模块入口
├── contact/                  # 通讯录 (71 APIs)
│   └── v3/                  # 用户/部门/单位
│       ├── department/      # 部门管理
│       ├── user/            # 用户管理
│       ├── unit/            # 单位管理
│       └── group/           # 用户组
├── im/                       # 即时消息 (74 APIs)
│   └── v1/                  # 消息/群聊/回调
│       ├── message/         # 消息发送/撤回
│       ├── chat/            # 群组管理
│       ├── chat_menu/       # 群菜单
│       └── callback/        # 回调设置
├── aily/                     # Aily (31 APIs)
├── event/                    # 事件 (2 APIs)
├── moments/                  # 动态圈 (1 API)
└── common/                   # 共享代码
```

## WHERE TO LOOK

| Task | Location | Notes |
|------|----------|-------|
| 发送消息 | `src/im/v1/message/` | 文本/图片/卡片/文件 |
| 群组管理 | `src/im/v1/chat/` | 创建/解散/成员管理 |
| 用户管理 | `src/contact/v3/user/` | 用户信息/状态 |
| 部门管理 | `src/contact/v3/department/` | 组织架构 |
| 回调配置 | `src/im/v1/callback/` | 事件订阅 |

## CONVENTIONS

### 消息类型
```rust
// 支持的消息内容类型
pub enum MessageContent {
    Text(String),
    Image { image_key: String },
    Card { card_json: String },
    File { file_key: String },
    // ...
}
```

### 群组操作
```rust
// 链式调用
client.communication.im.v1().chat().create(...)
client.communication.im.v1().chat().members().add(...)
```

### Feature 细分
```rust
// Cargo.toml features
im = []           # 即时消息
contact = []      # 通讯录
group = []        # 群组
```

## ANTI-PATTERNS

- ❌ 不要混淆 contact 和 im 的 API（不同领域）
- ❌ 不要跳过消息内容大小检查
- ❌ 不要忽略回调签名验证

## NOTES

- **默认启用**: 在 `core-services` feature 中
- **消息限制**: 单条消息大小限制，注意检查
- **频率限制**: 发送消息有频率限制，注意节流
