# lark-websocket-protobuf

飞书/Lark WebSocket 客户端使用的 protobuf 消息定义。

`pbbp2.proto` 对应的 Rust 源码已预生成并包含在发布包中。依赖本 crate 的项目只需正常执行
`cargo build`，不需要安装 `protoc` 或设置 `PROTOC`。

公开类型保持在以下模块路径：

```rust
use lark_websocket_protobuf::pbbp2::{Frame, Header};
```

`protos/` 是协议源文件；修改协议时，维护者应重新生成并提交 `src/pbbp2.rs`，并在发布前运行
仓库中的打包回归测试，确认 crates.io 包在没有 `protoc` 的环境中仍可构建。
