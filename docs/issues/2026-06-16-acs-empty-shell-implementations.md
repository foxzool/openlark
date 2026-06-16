# Issue: openlark-security acs（智能门禁）子模块接口为空壳实现

**日期**: 2026-06-16
**发现方式**: `verify_api_fields.py --crate openlark-security` 完整模式抽查
**严重度**: 🔴 高（写操作接口完全不可用）
**影响 crate**: `openlark-security`
**影响 bizTag**: `acs`

## 问题描述

acs（智能门禁）子模块的 API 实现是**空壳**——Request struct 没有任何字段、没有 Body 定义、没有 builder 方法，`execute_with_options` 直接发送空请求。

```rust
// 典型空壳实现（rule_external/create.rs）
pub struct CreateRuleExternalRequest {
    config: Arc<Config>,
    // ← 文档要求必填的 rule 字段完全缺失
}

pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<...> {
    let path = format!("/open-apis/acs/v1/rule_external");
    let req = ApiRequest::post(&path);
    // ← 没有 .body()，发送空请求
    Transport::request(req, &self.config, Some(option)).await?
}
```

## 影响范围

CSV 定义了 14 个 acs API，代码文件涉及 45 个（含 `acs/acs/v1/` 和 `security/acs/v1/` 两套重复路径）。

### 写操作（5 个，🔴 完全不可用）

这些接口需要请求体，空壳导致发空请求被服务端拒绝：

| API | URL | 缺失字段 |
|-----|-----|---------|
| 创建或更新权限组 | `POST /open-apis/acs/v1/rule_external` | `rule`（含 name/device_ids/user_ids 等子字段）|
| 设备绑定权限组 | `POST /open-apis/acs/v1/rule_external/device_bind` | `device_id`(必填)、`rule_ids`(必填) |
| 修改用户部分信息 | `PATCH /open-apis/acs/v1/users/:user_id` | 用户信息字段 |
| 上传人脸图片 | `PUT /open-apis/acs/v1/users/:user_id/face` | 人脸图片数据 |
| 添加访客 | `POST /open-apis/acs/v1/visitors` | 访客信息字段 |

### 读操作（9 个，🟡 能调通但拿不到数据）

GET 接口空壳影响较小（至少 HTTP 请求能发出去），但 Response 也是空壳，反序列化后用户读不到数据：

| API | URL | 问题 |
|-----|-----|------|
| 获取门禁记录列表 | `GET /open-apis/acs/v1/access_records` | Response 无字段 |
| 下载开门人脸图片 | `GET /open-apis/acs/v1/access_records/:id/access_photo` | Response 无字段 |
| 获取门禁设备列表 | `GET /open-apis/acs/v1/devices` | Response 无字段 |
| 获取单个用户信息 | `GET /open-apis/acs/v1/users/:user_id` | Response 无字段 |
| 获取用户列表 | `GET /open-apis/acs/v1/users` | Response 无字段 |
| 获取权限组信息 | `GET /open-apis/acs/v1/rule_external` | Response 无字段 |
| 删除权限组 | `DELETE /open-apis/acs/v1/rule_external` | — |
| 删除访客 | `DELETE /open-apis/acs/v1/visitors/:visitor_id` | — |

## 根因分析

这些接口是**脚手架式批量生成**的——为了把覆盖率刷到 100%（`validate_apis.py` 只检查文件是否存在），生成了 struct 骨架但从未实现字段。覆盖率报告显示 `openlark-security: 27/27 = 100%`，但实际可用率为 0。

## 重复路径问题

代码存在两套路径，可能是历史迁移遗留：
- `crates/openlark-security/src/acs/acs/v1/...`（14 个文件）
- `crates/openlark-security/src/security/acs/v1/...`（31 个文件）

需确认哪套是 `validate_apis.py` 路径推断实际指向的，清理另一套。

## 建议修复方案

### 优先级 1：写操作（5 个）

每个接口需要：
1. 用 `fetch_doc.js` 抓飞书文档（fullPath 见 CSV）
2. 提取请求体字段，建强类型 Body struct
3. 补 builder 方法
4. 补 Response 字段（完整建模）
5. 补 `validate_required` / `validate_required_list` 校验
6. 单测

参考已修复的 approval v4 用户级接口（`openlark-workflow`）的实现模式。

### 优先级 2：读操作（9 个）

补 Response 字段（从文档响应示例提取），让用户能读到返回数据。

### 优先级 3：清理重复路径

确认并删除冗余的一套代码路径。

## 验证方法

修复后用工具核验：
```bash
# 快速模式（字段自检）
python3 tools/verify_api_fields.py --crate openlark-security

# 完整模式（抓文档对比）
python3 tools/verify_api_fields.py --crate openlark-security --fetch-docs
```

## 关联

- 发现工具：`tools/verify_api_fields.py`（快速模式检不出空壳，完整模式才能发现）
- 字段核对技能：`.agents/skills/openlark-api-field-verify/`
- 实现规范：`Skill(openlark-api)`
