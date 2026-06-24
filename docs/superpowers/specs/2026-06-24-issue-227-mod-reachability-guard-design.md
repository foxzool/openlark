# Issue #227 设计 — mod 可达性 CI 守卫（防「写了但没接通」的死代码）

- **Issue**: [#227 ci: 加 mod 可达性守卫，防止「写了但没接通」的死代码](https://github.com/foxzool/openlark/issues/227)
- **日期**: 2026-06-24
- **状态**: 待实现

## 背景

PR #225 修复前，`openlark-communication` 的整个 `event/` 目录（含既有 `outbound_ip/list.rs`）
**未在 `lib.rs` 声明**，是死代码——从未被编译。Rust 编译器的 `dead_code` lint **抓不到**这类问题：
`pub mod` 声明的缺失让目录文件成为「孤儿」，但编译器对 `pub` 项不发 dead_code 警告
（假设外部 crate 可能用到）。因此需要**结构性守卫**：以编译器实际编译的文件清单为基准，
比对 `src/` 目录树，找出从未被任何 mod 链引用的孤儿文件。

## 检测方法：cargo-driven（dep-info 基准）

**核心思想**：用 `cargo rustc -p <crate> --lib --all-features -- --emit=dep-info` 生成
rustc 实际编译的源文件清单（Makefile 格式 `.d` 文件），作为 ground truth；再与 `src/`
下所有 `.rs` 文件比对，差集即为「孤儿文件」。

### 为什么 dep-info 可靠

- 用编译器自身的模块解析：完美处理 `#[path]` 属性、inline `mod {}` 块、`mod.rs` vs
  `X.rs`（Rust 2018 惯例）、`#[cfg(feature)]` 门控——**零误报**。
- `--all-features` 下所有 cfg 门都打开，故「可达性」问题是良定义的：任一 feature 组合
  下都编译不到的文件才是真孤儿。
- 已在本仓库实测：main 分支上 `event/event/v1/outbound_ip/list.rs`（死代码）的
  dep-info 计数 = 0，而 `endpoints/event.rs`（已挂载的常量文件）计数 = 1。差异
  336 (src) − 329 (compiled) = 7 个孤儿候选。

### 已验证的关键事实

- 全部 20 个 `openlark-*` crate 都有 `src/lib.rs`（`--lib` 安全）。
- dep 文件路径确定性：`target/debug/lib<crate_name_with_underscores>.d`
  （`openlark-communication` → `libopenlark_communication.d`）。
- `.d` 文件第 1 行格式：`<rlib path>: <src1.rs> <src2.rs> ...`（空格分隔，可能换行续行）。
- CI 已有多处 `--all-features` 调用（`ci.yml:35/57/107/233`），加一步即可。

## 设计

### 落盘路径

```
tools/check_mod_reachability.py          # 守卫脚本
tools/tests/test_check_mod_reachability.py  # 单测（unittest，对齐现有 tools/tests 风格）
```

### 脚本契约

```python
# 输入：无参数（默认扫全 workspace），或 --crate <name> 单 crate，或 --baseline-only（不构建，复用现有 .d）
# 输出：stdout 人类可读报告；退出码 0 = 无孤儿，1 = 发现孤儿
# 用法：
#   python3 tools/check_mod_reachability.py            # 全 workspace（CI 用）
#   python3 tools/check_mod_reachability.py --crate openlark-communication
```

### 核心流程

1. **枚举 crate**：扫 `crates/openlark-*/`，过滤出有 `src/lib.rs` 的（预期 20 个）。
   支持 `--crate` 限定单个。
2. **取编译基准**（每个 crate）：
   - 运行 `cargo rustc -p <crate> --lib --all-features -- --emit=dep-info`
     （`--all-features` 确保所有 cfg 门打开）。
   - 解析 `target/debug/lib<crate>.d`：第 1 行冒号后、空格分隔的路径中，
     过滤出属于本 crate `src/` 的 `.rs` 文件 → `compiled` 集合。
3. **取目录基准**：`find <crate>/src -name '*.rs'` → `on_disk` 集合。
   - 排除 `target/`、`tests/`、`examples/`、`benches/`（仅查 `src/`）。
4. **差集**：`orphans = on_disk - compiled`。
   - 排除已知豁免：`mod.rs` 若父目录无任何文件被编译则整目录算孤儿（已隐含在差集中）。
5. **报告 + 退出码**：有孤儿 → 列出文件 + 退出码 1；无 → 退出码 0。

### 容错与稳健性

- **dep 文件路径**：用 glob `target/debug/lib<crate>*.d`（部分 crate 名含连字符→下划线），
  取最新一个；找不到则报错并提示重新构建。
- **`--all-features` 失败**：若某 crate 全量编译失败（真实编译错误），脚本不应误报孤儿——
  捕获 cargo 非 0 退出，报「crate 编译失败，跳过」并最终退出码 1（让 CI 先修编译错误）。
- **CI 增量构建**：`.d` 文件随 `cargo build --all-features` 自然生成；CI 步骤可先跑
  `cargo build --workspace --all-features`（复用现有缓存）再跑守卫，守卫用 `--baseline-only`
  复用已生成的 `.d`，避免重复编译。
- **跨平台路径**：dep 文件路径是绝对路径；脚本用 `Path.resolve()` 归一化后比对。

### CI 集成

集成点：`.github/workflows/ci.yml` 的 **`lint` job**（第 87-109 行）。该 job 已运行
`cargo clippy --workspace --lib --all-features`（第 107 行），会为每个 crate 的 lib 生成
`.d` 文件。在该步骤**之后**追加 Python setup + 守卫两步：

```yaml
      - name: Run clippy (all features)
        run: cargo clippy --workspace --lib --all-features -- -D warnings
      - uses: actions/setup-python@v5
        with:
          python-version: "3.12"
      - name: Check mod reachability (no orphan src files)
        run: |
          python3 -m unittest tools.tests.test_check_mod_reachability
          python3 tools/check_mod_reachability.py
```

`clippy --lib --all-features` 与守卫的 `cargo rustc --lib --all-features --emit=dep-info`
共用同一 feature 矩阵与构建缓存，`.d` 已存在时守卫的 dep-info 调用近乎瞬时（只重写 `.d`，
不重编译）。守卫放 lint job 而非 api-contracts job，是因为它逻辑上属于「代码结构门禁」。

### 单测设计（`tools/tests/test_check_mod_reachability.py`）

对齐现有 `test_compare_api_catalogs.py` 风格（importlib 加载 + unittest）：
- `test_parse_dep_info`：给定一段 `.d` 文件内容，断言正确提取 `.rs` 路径集合。
- `test_diff_finds_orphan`：`on_disk = {a.rs, b.rs, c.rs}`、`compiled = {a.rs, b.rs}`
  → orphan = {c.rs}。
- `test_no_orphan_returns_zero_diff`：两集相等 → 空 orphan 集。
- `test_excludes_non_src_files`：dep 里的 `core` crate 路径（非本 crate）被正确过滤。

脚本纯函数（`parse_dep_info`、`diff_orphans`）与 IO 分离，便于单测无网络/无 cargo。

## 验证（Definition of Done）

- [ ] `tools/check_mod_reachability.py` 实现 + 4 个单测通过
- [ ] 本地跑全 workspace：main 分支上**必须**报出 `event/` 孤儿（回归守卫验证）
- [ ] 切到 PR #225 分支：`event/` 孤儿消失（守卫随修复转绿）
- [ ] CI 步骤接入 `ci.yml`
- [ ] 开 PR 关闭 #227

## 风险与边界

- **已知孤儿存量（重大）**：全仓扫描发现 **375 个孤儿文件，横跨 16 个 crate**
  （workflow 127 / application 90 / meeting 42 / platform 31 …）。远超 #225 的单一 event/ 案例。
  采用 **baseline allowlist 策略**：`tools/mod_reachability_allowlist.txt` 记录当前 375 个存量，
  CI 只对**新增**孤儿失败（防回归），存量由各 crate 修复后从 allowlist 删除逐步收敛。
  - 生成：`python3 tools/check_mod_reachability.py --update-allowlist`
  - 脚本默认读 `tools/mod_reachability_allowlist.txt`；可 `--allowlist <path>` 覆盖。
- **不检测**：非 `src/` 目录（tests/examples/benches）、inline `mod {}` 内的死代码
  （那是 dead_code lint 的职责，与本守卫正交）。
- **性能**：全 workspace 20 crate × dep-info 解析，秒级；复用 build 缓存不重复编译。

## 后续（不在 #227 范围）

为孤儿最多的 crate 开独立 issue 清理死代码（workflow/application/meeting 优先），
每修一个 crate 就从 allowlist 删对应行，最终 allowlist 收敛到空。

## 不在本次范围内（YAGNI）

- 不检测单 feature 组合下的孤儿（`--all-features` 已覆盖「任一组合可达」语义）。
- 不做 mod 声明语法校验（那是 rustc 的职责）。
- 不修复存量孤儿（本 issue 仅加守卫；存量修复由对应 PR 各自负责）。
