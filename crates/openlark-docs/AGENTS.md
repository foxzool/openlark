# openlark-docs Knowledge Base

**Crate**: Document Services  
**APIs**: 216 个（排除 old；`validate_apis.py --crate openlark-docs`） | **Status**: 生产就绪  
**Coverage**: 100%

## OVERVIEW

飞书文档服务模块，提供云文档、多维表格、知识库等完整能力。已实现目录内全部 216 个 API（另有 49 条 CCM old，不计入）。

## STRUCTURE

```
src/
├── lib.rs                    # 模块入口
├── ccm/                      # 云文档 (127 APIs，另 49 old)
│   ├── drive/v1/            # 云盘（文件上传下载）
│   ├── doc/v1/              # 旧版文档
│   ├── docx/v1/             # 新版文档
│   ├── sheet/v3/            # 电子表格
│   ├── wiki/v2/             # 知识空间
│   └── ...
├── base/                     # 多维表格 (52 APIs)
│   └── bitable/v1/          # 表格、视图、记录
├── baike/                    # 知识库 (27 APIs)
│   └── v1/                  # 词条、分类
├── minutes/                  # 会议纪要 (10 APIs)
└── common/                   # 共享代码
    ├── api_endpoints.rs     # 端点常量
    └── mod.rs
```

## WHERE TO LOOK

| Task | Location | Notes |
|------|----------|-------|
| 添加云文档 API | `src/ccm/{service}/v*/` | 按服务版本组织 |
| 添加多维表格 API | `src/base/bitable/v1/` | Bitable 相关 |
| 文件上传 | `src/ccm/drive/v1/file/` | UploadAllRequest |
| 知识空间 | `src/ccm/wiki/v2/` | Space/Node 管理 |
| 端点常量 | `src/common/api_endpoints.rs` | 复用常量定义 |

## CONVENTIONS

### API 实现模板
```rust
// src/ccm/drive/v1/file/upload_all.rs
pub struct UploadAllRequest {
    config: Config,
    file_name: String,
    // ... 其他字段
}

impl UploadAllRequest {
    pub fn new(config: Config, ...) -> Self { ... }
    pub fn file_name(mut self, name: String) -> Self { ... }
    
    pub async fn execute(self) -> SDKResult<UploadResponse> {
        validate_required!(self.file_name, "文件名不能为空");
        // ... 实现
    }
}
```

### 模型定义
```rust
// models.rs 或 models/mod.rs
#[derive(Debug, Serialize, Deserialize)]
pub struct FileInfo {
    pub file_token: String,
    pub file_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder: Option<String>,
}
```

### 链式调用入口
```rust
// Client 通过 meta 链访问
client.docs.ccm.drive.v1().file().upload_all(...)
client.docs.base.bitable().app().create(...)
client.docs.ccm.wiki.v2().space().create(...)
```

## ANTI-PATTERNS

- ❌ 不要重复定义端点常量（使用 `common::api_endpoints`）
- ❌ 不要跳过必填字段验证
- ❌ 不要硬编码 URL（使用 `drive.v1.file` 等常量）

## NOTES

- **Feature**: 通过 `docs` feature 启用
- **Token**: 支持 user_access_token 和 tenant_access_token
- **文件上传**: 支持分片上传（大文件）和单次上传（小文件）
