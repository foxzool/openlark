# openlark-docs

飞书开放平台云文档服务模块 - 文档、表格、知识库 API（216 APIs，100% 覆盖，不含旧版本）

## 功能概览

口径：`python3 tools/validate_apis.py --crate openlark-docs`（默认排除 `meta.Version=old`）。CCM 另有 49 条 old，不计入下表。

| 模块（bizTag） | API 数量 | 描述 |
|------|---------|------|
| CCM | 127 | 云文档协同（文档、云盘、表格、知识空间） |
| Base | 52 | 多维表格（bitable） |
| Baike | 27 | 企业百科 / 词典 |
| Minutes | 10 | 会议纪要 |
| **总计** | **216** | **100% 覆盖** |

## 特性

- ✅ **完整覆盖**: 216 个 API，100% 实现覆盖率（排除 old 版本）
- ✅ **类型安全**: 基于 enum 的端点系统，编译时检查
- ✅ **流式 API**: Builder 模式，链式调用
- ✅ **统一入口**: `DocsClient` 作为唯一公开入口
- ✅ **按需编译**: 细粒度 feature，减少编译时间和二进制体积

## 入口定位

- 单业务域 canonical 入口：`openlark_docs::DocsClient`
- 配置入口：`openlark_core::config::Config`
- 业务调用方式：`docs.ccm`、`docs.base`、`docs.baike`、`docs.minutes`
- 如果你需要跨业务域统一客户端，再改用根 crate `openlark` 或高级入口 `openlark-client`

Canonical 公开入口规则见 [`../../docs/PUBLIC_REEXPORT_POLICY.md`](../../docs/PUBLIC_REEXPORT_POLICY.md)。

Docs request/response contract test 范围说明见 [`docs/CONTRACT_TEST_SCOPE.md`](docs/CONTRACT_TEST_SCOPE.md)。
README 对齐的可编译示例见 `examples/docs_readme_examples.rs`。

## 功能选择（Feature Selection）

### 使用方式

在 `Cargo.toml` 中添加所需的 features：

```toml
[dependencies]
openlark-core = "0.15"
openlark-docs = { version = "0.15", features = ["your-features"] }
```

### 可用功能（Features）

#### 核心功能

| Feature | 说明 | 依赖 |
|---------|------|------|
| `core` | 核心功能 | 无 |

#### 云文档协同（CCM）

| Feature | 说明 | 包含的 API |
|---------|------|-----------|
| `ccm` | 完整云文档协同功能 | 174 APIs |
| `ccm-doc` | 文档处理 | 文档创建、获取、更新、删除 |
| `ccm-docx` | DOCX 文档 | DOCX 文档处理 |
| `ccm-drive` | 云盘文件 | 文件上传、下载、删除 |
| `ccm-sheets` | 表格（默认 v3） | 电子表格完整功能 |
| `ccm-sheets-v3` | 表格 v3（推荐） | 稳定版本，现代化 API |
| `ccm-sheets-v2` | 表格 v2（实验性） | 基础表格操作（已弃用） |
| `ccm-wiki` | 知识库 | Wiki 空间、页面管理 |

#### 多维表格（Bitable）

| Feature | 说明 | 包含的 API |
|---------|------|-----------|
| `bitable` | 多维表格 | 49 APIs |
| `base` | 基础服务 | 基础服务支持 |

#### 知识库（Baike）

| Feature | 说明 | 包含的 API |
|---------|------|-----------|
| `baike` | 知识库 | 27 APIs |
| `wiki` | Wiki | Wiki 空间、页面 |

#### 语言服务（Lingo）

| Feature | 说明 | 包含的 API |
|---------|------|-----------|
| `lingo` | 飞书词典 | 词条高亮、实体提取 |

#### 会议纪要（Minutes）

| Feature | 说明 | 包含的 API |
|---------|------|-----------|
| `minutes` | 会议纪要 | 4 APIs |

#### 云文档管理（Docs）

| Feature | 说明 | 包含的 API |
|---------|------|-----------|
| `docs` | 云文档管理 | 文档管理 |

#### 公告管理（Docx）

| Feature | 说明 | 包含的 API |
|---------|------|-----------|
| `docx` | 公告管理 | 公告发布、查询 |

#### 版本支持

| Feature | 说明 |
|---------|------|
| `v1` | API v1 版本 |
| `v2` | API v2 版本 |
| `v3` | API v3 版本 |

#### 功能组合

| Feature | 说明 | 包含的模块 |
|---------|------|-----------|
| `full` | 完整云文档功能 | ccm, bitable, base, baike, minutes, v3 |

#### 向后兼容别名

以下 feature alias 仅保留兼容职责，不再视为新的 canonical 入口：

| Feature | 说明 |
|---------|------|
| `cloud-docs` | 云文档协同（别名） |
| `all-cloud-docs` | 所有云文档功能（别名） |

## 快速开始

### 基础使用

```rust,no_run
use openlark_core::{config::Config, SDKResult};
use openlark_docs::DocsClient;
use openlark_docs::ccm::docs::v1::GetDocsContentRequest;

#[tokio::main]
async fn main() -> SDKResult<()> {
    let config = Config::builder()
        .app_id("app_id")
        .app_secret("app_secret")
        .build();
    let docs = DocsClient::new(config);

    // 通过 DocsClient 获取业务域配置，再构建 Request
    let request = GetDocsContentRequest::new("doc_token", "docx", "markdown");
    let doc = openlark_docs::ccm::docs::v1::content::get::get_docs_content(
        request,
        docs.ccm.config(),
        None,
    )
    .await?;
    println!("Document: {:?}", doc);

    Ok(())
}
```

### 多维表格使用

```toml
# Cargo.toml
[dependencies]
openlark-core = "0.15"
openlark-docs = { version = "0.15", features = ["bitable"] }
```

```rust,no_run
use openlark_core::config::Config;
use openlark_docs::base::bitable::v1::GetAppRequest;
use openlark_docs::DocsClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::builder()
        .app_id("app_id")
        .app_secret("app_secret")
        .build();
    let docs = DocsClient::new(config);

    // 访问多维表格（使用 Request 对象方式）
    let app = GetAppRequest::new(docs.base.bitable().config().clone())
        .app_token("app_token")
        .execute()
        .await?;
    println!("App token: {}", app.app.app_token);

    Ok(())
}
```

### 云文档协同使用

```toml
# Cargo.toml
[dependencies]
openlark-core = "0.15"
openlark-docs = { version = "0.15", features = ["ccm"] }
```

```rust,no_run
use openlark_core::config::Config;
use openlark_docs::DocsClient;
use openlark_docs::ccm::drive::v1::file::DownloadFileRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::builder()
        .app_id("app_id")
        .app_secret("app_secret")
        .build();
    let docs = DocsClient::new(config);

    // 访问云文档协同
    let _file = DownloadFileRequest::new(docs.ccm.config().clone(), "file_token")
        .execute()
        .await?;
    // 表格操作请参考 openlark_docs::ccm::sheets 模块

    Ok(())
}
```

### 知识库使用

```toml
# Cargo.toml
[dependencies]
openlark-core = "0.15"
openlark-docs = { version = "0.15", features = ["baike"] }
```

```rust,no_run
use openlark_core::config::Config;
use openlark_docs::baike::baike::v1::GetEntityRequest;
use openlark_docs::DocsClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::builder()
        .app_id("app_id")
        .app_secret("app_secret")
        .build();
    let docs = DocsClient::new(config);

    // 访问知识库（使用 Request 对象方式）
    let entity = GetEntityRequest::new(docs.baike.config().clone(), "entity_id")
        .execute()
        .await?;
    println!("Entity exists: {}", entity.entity.is_some());

    Ok(())
}
```

## 性能优化建议

### 1. 选择性启用功能

仅启用您需要的功能模块：

```toml
# ❌ 不推荐：启用所有功能
openlark-docs = { version = "0.15", features = ["full"] }

# ✅ 推荐：仅启用所需功能
openlark-docs = { version = "0.15", features = ["bitable"] }  # 仅多维表格
```

### 2. 编译优化

```bash
# 发布版本使用更优化的编译配置
cargo build --release

# 检查优化后的二进制大小
ls -lh target/release/
```

### 3. 使用流式 Builder 模式

```rust,ignore
// ✅ 推荐：流式构建器
let request = CreateDocRequest::new(config, doc_body)
    .user_id_type(UserIdType::OpenId)
    .folder_token("folder_token");

// ❌ 不推荐：多次对象创建
let mut request = CreateDocRequest::new(config, doc_body);
request = request.user_id_type(UserIdType::OpenId);
request = request.folder_token("folder_token");
```

## 文档

- [飞书开放平台文档](https://open.feishu.cn/document/home/)
- [API 文档](https://open.feishu.cn/document/server-docs/)
- [SDK 源码](https://github.com/foxzool/openlark)

## 许可证

[Apache-2.0](../../LICENSE-APACHE)

## 贡献

欢迎贡献！请参阅 [贡献指南](../../CONTRIBUTING.md)。

## 支持与反馈

- 问题反馈：[GitHub Issues](https://github.com/foxzool/openlark/issues)
- 功能请求：[GitHub Discussions](https://github.com/foxzool/openlark/discussions)
