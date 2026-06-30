## Why
issue #271 platform app_engine 批（第 7 批，**最后一批**）。前 6 批已归档（63 类型）。app_engine 51 个裸 Builder，全在 apaas 子目录，全最简（无 trait impl/re-export/service）。

## What Changes
- 51 个请求 builder `XxxBuilder` → `XxxRequestBuilder` + `#[deprecated]` alias（放 `#[cfg(test)]` 前）
- 全在 platform/app_engine/apaas/ 子目录
- **BREAKING**（软）：alias 源码兼容

## Capabilities
### New Capabilities
- `platform-app-engine-request-naming`: platform app_engine 子系统请求 builder SHALL 统一 RequestBuilder 后缀。

## Impact
- openlark-platform app_engine/apaas 51 个定义文件改名 + alias。无 re-export/service/trait impl。
