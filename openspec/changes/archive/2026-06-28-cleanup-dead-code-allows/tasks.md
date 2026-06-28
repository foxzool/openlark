# Tasks — cleanup-dead-code-allows

> 已完成。范围扩展：实证移除 allow 后暴露的不止 3 个、而是 ~24 个真死 `config`/死函数（跨 platform/ai/analytics/user/helpdesk/docs/application，含 feature-combo 特定的），统一按 `_config` 改名或 `#[expect]]` 处理。关联 #267；补全工作拆至 #274（platform v1 访问器）/ #275（ai 访问器）/ #276（request builder execute）。

## 1. 调研（build 前）

- [x] 1.1 读 3 个 platform v1 mod.rs 判明 config 无访问器 → D2 方案 C
- [x] 1.2 移除 allow 后三组 feature clippy 暴露 ~24 处真死代码（非预期的 3），全部 `config` 字段或死函数

## 2. 移除 cruft（机械）

- [x] 2.1 缩进感知 sed 批量删除 crates/+src/ 全部 `#[allow(dead_code)]`（含缩进写法，381 文件）
- [x] 2.2 验证 0 残留独立 allow 行；无 inline 变体

## 3. 修正真死字段（方案 C：`_config` + 注释；范围扩至 ~24 处）

- [x] 3.1 platform v1 入口（AdminV1/ApaasV1/DirectoryV1）+ ai 入口（DocumentAiV1/OCR/Speech/Translation）config → _config
- [x] 3.2 request builder（platform admin audit/users、analytics query/user、user settings/preferences）config → _config（入口 struct 的 LIVE config 保留不动）
- [x] 3.3 application/helpdesk service.rs config → _config（feature v1 关闭时 dead）
- [x] 3.4 docs 测试 helper TestRequest + 宏生成 deprecated new()：`#[expect(dead_code)]`（宏硬依赖字段名 config，无法改名）
- [x] 3.5 docs field_types serialize/deserialize_record_fields（pub 无调用者）：`#[expect(dead_code)]`

## 4. 验证

- [x] 4.1 `cargo clippy --workspace --all-targets -- -D warnings`（default）exit 0
- [x] 4.2 `--all-features` exit 0
- [x] 4.3 `--no-default-features` exit 0
- [x] 4.4 `cargo test --workspace` 全绿
- [x] 4.5 grep 终检：HR 0、全 workspace `#[allow(dead_code)]` = 0

## 5. 防复发与收尾

- [x] 5.1 D3：`tools/check_no_dead_code_allows.sh` + ci.yml lint job step + justfile `no-dead-code-allows` recipe；负向测试已验证
- [x] 5.2 CHANGELOG `[Unreleased] > Changed` 记录
- [x] 5.3 拆分 issue：#274（platform v1 访问器）/ #275（ai 访问器）/ #276（request builder execute）；#267 归档时关闭
