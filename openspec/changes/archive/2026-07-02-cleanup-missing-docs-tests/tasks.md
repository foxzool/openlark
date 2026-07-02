## 1. D1：删冗余 `has_no_warnings`

- [x] 1.1 **删 8 整文件**（仅含 has_no_warnings）：`test_openlark_{ai,analytics,application,auth,cardkit,client,core,webhook}_missing_docs.py`。
- [x] 1.2 **9 文件删 `has_no_warnings` 方法、留结构变体**：`test_openlark_{communication,docs,helpdesk,hr,mail,meeting,platform,protocol,workflow}_missing_docs.py`（移除 `test_*_has_no_missing_docs_warnings` 方法 + 不再需要的 `subprocess` import；保留 `do_not_suppress`/`mod_roots`/`cleaned_slices`/`v1_root` 结构变体 + 其所需 import）。
- [x] 1.3 保留 `test_openlark_workflow_narrow_missing_docs.py` 不动（本就只有结构变体）。
- [x] 1.4 自验：`grep -rn 'has_no_missing_docs_warnings' tools/tests/` 输出为空（18 处全删）；`ls tools/tests/test_openlark_*_missing_docs.py` 剩 10 文件（9 改 + workflow_narrow）。

## 2. D2：接 10 结构变体进 CI

- [x] 2.1 `.github/workflows/ci.yml` 在 `test_workspace_missing_docs`（ci.yml:114，#1 加）旁，加一行 `python3 -m unittest` 跑 10 模块（缩进照抄 ci.yml:113-114）：
  `tools.tests.test_openlark_communication_missing_docs tools.tests.test_openlark_docs_missing_docs tools.tests.test_openlark_helpdesk_missing_docs tools.tests.test_openlark_hr_missing_docs tools.tests.test_openlark_mail_missing_docs tools.tests.test_openlark_meeting_missing_docs tools.tests.test_openlark_platform_missing_docs tools.tests.test_openlark_protocol_missing_docs tools.tests.test_openlark_workflow_missing_docs tools.tests.test_openlark_workflow_narrow_missing_docs`
- [x] 2.2 yaml 语法：`python3 -c "import yaml; yaml.safe_load(open('.github/workflows/ci.yml'))"` OK。

## 3. 验证

- [x] 3.1 **本地跑 10 结构变体全绿**：`python3 -m unittest`（D2 那行）exit 0，确认 10 测试 pass。
- [x] 3.2 **workspace missing_docs 仍 0**（#1 守门不变）：`cargo doc --workspace --all-features 2>&1 | grep -c 'missing documentation for'` = 0。
- [x] 3.3 `cargo fmt --check` + `just lint`（测试文件改动不影响 Rust lint）通过。
- [x] 3.4 **无残留死测试**：`ls tools/tests/test_openlark_*_missing_docs.py | wc -l` = 10；无仅含 has_no_warnings 的文件。
