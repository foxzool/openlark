# Tasks

## 1. platform admin v1（AdminV1，6 操作集合 + 2 facade，浅）

- [x] 1.1 为 admin v1 操作集合子模块建 service 入口类型：`badge`/`badge_image`/`password`/`admin_dept_stat`/`admin_user_stat`/`audit_info` 各建 `XxxService { config }` + 暴露叶子 builder 构造方法（如 `badge().create()`）
- [x] 1.2 facade `audit.rs`/`users.rs` 复用已有 `AuditApi`/`UsersApi`，不新建类型
- [x] 1.3 `AdminV1`：`_config` → `config`，装 `pub fn badge()/badge_image()/password()/admin_dept_stat()/admin_user_stat()/audit_info()/audit()/users()` 访问器
- [x] 1.4 补 access 测试（仿 `test_spark_v1_directory_access`），链式到叶子 builder + facade 访问器
- [x] 1.5 admin 范围 `cargo clippy -W dead_code` + `cargo fmt` 通过

## 2. platform directory v1（DirectoryV1，8 子模块，浅）

- [x] 2.1 为 directory v1 子模块建 service：`department`/`departments`/`users`/`employee`/`sync`/`collaboration_share_entity`/`collaboration_tenant`/`collaboration_rule`
- [x] 2.2 `DirectoryV1`：`_config` → `config`，装访问器
- [x] 2.3 access 测试链到叶子 builder
- [x] 2.4 directory 范围 clippy + fmt 通过

## 3. platform apaas v1（ApaasV1，8 顶层 + 深嵌套）

- [x] 3.1 顶层 8 service：`app`/`approval_task`/`approval_instance`/`application`/`user_task`/`seat_activity`/`seat_assignment`/`workspace`
- [ ] 3.2 `application` 深嵌套逐级 service：`object`→`record`、`role`→`member`、`record_permission`→`member`、`environment_variable`/`function`/`flow`/`audit_log`
- [ ] 3.3 `workspace` 嵌套 service：`table`/`view`/`enum_mod`
- [x] 3.4 `ApaasV1`：`_config` → `config`，装顶层访问器
- [ ] 3.5 深链 access 测试：`application().object().record()` 等走到叶子 builder
- [ ] 3.6 apaas 范围 clippy + fmt 通过（含深嵌套无 dead_code）

## 4. 全局验证 + 闭环

- [ ] 4.1 `cargo fmt --check`（workspace）
- [ ] 4.2 `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- [ ] 4.3 `cargo clippy -W dead_code` 于 openlark-platform，无新增告警
- [ ] 4.4 grep 确认 3 个 platform 入口无 `_config` 遗留
- [ ] 4.5 闭环 cleanup-dead-code-allows：移除 3 个 platform 入口的"待装访问器"reserved 注释
