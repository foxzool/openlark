## ADDED Requirements

### Requirement: openlark-hr 零资源 accessor 死版本节点 SHALL 删除

当业务 crate 的真实资源为自包含 Config-direct Request struct（如 `CreateGroupRequest::new(config)`，直接持有 `Config`、自带 builder 与 `execute()`，不经版本节点链）时，零资源 accessor 的版本节点 struct SHALL 连同其 returning facade accessor 一并删除，而非保留为 Potemkin 入口。openlark-hr 的 11 个版本节点 struct（`AttendanceV1` / `OkrV1` / `EhrV1` / `HireV1` / `HireV2` / `CorehrV1` / `CorehrV2` / `PayrollV1` / `PerformanceV1` / `PerformanceV2` / `CompensationV1`）每个仅含 `new()` + `config()`、零资源访问器，且零跨 crate 引用、零测试引用，SHALL 删除；8 个 facade struct（`Attendance` / `Okr` / `Ehr` / `Hire` / `Corehr` / `Payroll` / `Performance` / `CompensationManagement`）上返回这 11 个死节点的 `.v1()` / `.v2()` accessor（共 11 个）SHALL 同步删除。本 requirement 构成对 `v1-sub-api-accessors` 现有「非破坏性补全」requirement 的 HR crate 专属例外：HR 因资源导航范式不同（Config-direct Request，非链式 accessor），需要 breaking 删除零引用死节点，而非 platform/ai 的纯加法补全。`okr.v2()` 与 `OkrV2`（`pub type OkrV2 = v2::OkrV2`，有真实资源 accessor 的活类型）不在删除范围。

#### Scenario: 11 个死版本节点 struct 与 returning accessor 移除

- **WHEN** 变更后在 `crates/openlark-hr/src/` 中 grep `struct AttendanceV1\|struct OkrV1\|struct EhrV1\|struct HireV1\|struct HireV2\|struct CorehrV1\|struct CorehrV2\|struct PayrollV1\|struct PerformanceV1\|struct PerformanceV2\|struct CompensationV1`
- **THEN** 命中数为 0；8 个 facade struct 上对应的 11 个 `.v1()`/`.v2()` returning accessor 同步移除

#### Scenario: okr.v2 与 OkrV2 alias 保留

- **WHEN** 变更后检查 `okr/okr/mod.rs` 与 `okr/mod.rs`
- **THEN** 仍存在 `pub type OkrV2 = v2::OkrV2;` 与 `pub fn v2(&self) -> okr::OkrV2`，`okr.v2()` 链可达（`OkrV2` 为有资源 accessor 的活类型，非死节点）

#### Scenario: 真实资源路径不受影响

- **WHEN** 变更后以原有模块路径构造 Config-direct Request，如 `openlark_hr::attendance::attendance::v1::group::CreateGroupRequest::new(config)`
- **THEN** 构造方式与签名不变，编译通过；真实 API 行为（请求/响应/端点）前后一致

#### Scenario: HR crate 编译与测试通过

- **WHEN** 运行 `cargo build -p openlark-hr --all-features` 与 `cargo test -p openlark-hr --all-features`
- **THEN** 均通过；facade struct 的 config 持有者角色与 `HrClient` 字段模式不被破坏

### Requirement: openlark-hr facade doc 指向真实可达路径

openlark-hr 的 facade 文档（`lib.rs` 顶层 doc example）SHALL 指向真实可达的 Config-direct Request 构造路径，不得展示编译失败的链式调用。doc example SHALL 以可编译检查的 doctest 形式（`no_run` 或更强）呈现，确保 advertised 的 API 路径类型/方法真实可达。原有 `client.attendance.v1().group().create()`（`AttendanceV1` 无 `.group()`，靠 `rust,ignore` 跳过编译的谎言）SHALL 改为 Config-direct 构造，如 `CreateGroupRequest::new(client.config().clone()).group_name(...)`。

#### Scenario: doc example 编译检查通过

- **WHEN** 运行 `cargo doc -p openlark-hr` 的 doctest（或 `cargo test --doc -p openlark-hr`）
- **THEN** facade doc example 不再是 `rust,ignore`，以 `no_run`（或更强）形式编译通过，展示的 Config-direct Request 路径真实可达

#### Scenario: doc example 不展示死链调用

- **WHEN** 变更后检查 `crates/openlark-hr/src/lib.rs` 顶层 doc
- **THEN** 不存在 `.v1().group()` 等引用已删除版本节点 accessor 的链式调用，example 仅展示真实可达 API
