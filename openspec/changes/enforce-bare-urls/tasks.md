## 1. codegen 渲染器改造（D2，防复发前置）

- [x] 1.1 定位 `tools/api_contracts/codegen_render.py` 中发射 `docPath:` / URL 的函数（调研指向 `_module_doc:69-76`，读源确认所有 URL 发射点）
- [x] 1.2 改发射 `<URL>` 而非裸 URL（如 `//! docPath: <https://...>`）
- [x] 1.3 抽样/单元验证 codegen 渲染输出含 `<URL>`（若 codegen 可离线跑渲染）

## 2. 批量修复现存 1578 裸 URL（D1）

- [x] 2.1 写脚本（sed 或小 Python）包裹 `//!` 行内的裸 `https://`/`http://` token 为 `<...>`，处理：行尾 URL、同一行多 URL、`docPath:` 与 `文档:` 两类标签
- [x] 2.2 跑脚本修全 workspace（16 crate）
- [x] 2.3 闭环验证：`cargo doc --workspace --all-features 2>&1 | grep -c bare_urls` = **0**
- [x] 2.4 `git diff` 抽样人工核对（重点：77 处手写 `文档:` URL、多 URL 行、非 URL 尾随文本未被误伤）

## 3. workspace 级 bare_urls deny（D3）

- [x] 3.1 根 `Cargo.toml` 新增 `[workspace.lints.rustdoc]` 段，声明 `bare_urls = "deny"`
- [x] 3.2 确认 20 业务 crate + 根包经 `[lints] workspace = true` 继承（应已全部继承，sanity check）

## 4. CI doc job 移除压制（D4）

- [ ] 4.1 `.github/workflows/ci.yml:44` 的 doc job `RUSTDOCFLAGS` 移除 `-A rustdoc::bare_urls`
- [ ] 4.2 实测移除后 `cargo doc --workspace --all-features` 是否暴露其他 rustdoc lint（如 broken_intra_doc_links），若有则记录并在 design 阶段决定处理

## 5. 验证（D1-D4 全过）

- [ ] 5.1 `cargo fmt --check` 过
- [ ] 5.2 `cargo doc --workspace --all-features` 过（deny 下 0 warning，exit 0）
- [ ] 5.3 `just lint` 过（clippy `--all-features` 双模式）+ 补跑 `--no-default-features` clippy（CI 同款）
- [ ] 5.4 `cargo build --workspace --all-features` 过
- [ ] 5.5 codegen 渲染器验证：渲染含 URL 的模块 doc，输出 MUST 为 `<URL>` 形态（Spec Req 3）
- [ ] 5.6 msrv `--locked` 验证（`.github/msrv/Cargo.lock`，rustup +1.88），确认无回归
