# Issue #41 静态检查闸门使用说明

`tools/issue41_guardrail.py` 用于持续检查 canonical leaf 执行范式（issue #41 / #585），
阻止同域新增第二套执行风格。

默认扫描业务 crate（`DEFAULT_TARGET_CRATES`）：

- `openlark-docs`
- `openlark-meeting`
- `openlark-communication`
- `openlark-hr`

## 检查项

| Code | Severity | 含义 |
|------|----------|------|
| `E001` | ERROR | 存在 `execute()` 但缺失 `execute_with_options()` |
| `E002` | ERROR | `execute()` 未委托 `execute_with_options(RequestOption::default())` |
| `E003` | ERROR | 业务 API 文件中出现 `Transport::request(..., None)`（option 未透传） |
| `W001` | WARN | 启发式判断存在必填场景，但未发现已识别的必填校验模式 |

Canonical leaf shape（项目策略）：

1. Request 持有 `Config`
2. `execute` 委托 `execute_with_options`
3. `RequestOption` 透传到 `Transport`
4. 校验在网络 I/O 之前

## Enforcement ladder（强制阶梯）

Phased so the first hard-gate PR stays green without monorepo rewrite (#585).

### Tier 1 — hard in CI（当前）

- **Scope**: `DEFAULT_TARGET_CRATES` above（已对 E001/E002/E003 清零）
- **Rules**: ERROR codes only (`E001` / `E002` / `E003`)
- **Flags**: `python3 tools/issue41_guardrail.py` — **no** `--strict-warn`
- **CI**: `.github/workflows/ci.yml` → `lint` job →
  `Leaf paradigm hard gate (issue41 guardrail; #585)`
- **Pin**: `tools/tests/test_issue41_guardrail.py` fails if the CI step, critical
  flags, default crate set, or this ladder section is removed

Intentional paradigm violations in enforced scope fail CI. Agents cannot silently
reintroduce execute-only / option-dropping leaves there.

### Tier 2 — deferred historical debt（未 hard-gate）

| Bucket | Status | Notes |
|--------|--------|-------|
| `W001` on Tier-1 crates | warn-only | 7 residual heuristics; do not flip `--strict-warn` until cleared |
| `openlark-ai` | deferred | ~23× `E002` historical |
| `openlark-helpdesk` | deferred | ~45× `E002` historical |
| `openlark-platform` | deferred | ~6× `E002` (+ optional W001) |
| `openlark-webhook` | deferred | `E001`/`E002` residual |
| Other ERROR-clean crates outside default set | optional promote | e.g. mail/application/cardkit — expand `DEFAULT_TARGET_CRATES` + CI pin together |

Promotion rule: clean E00x on a crate (or clear a W001 class), then expand
`DEFAULT_TARGET_CRATES` / enable `--strict-warn` **and** update the inventory pin
in the same change. Never lower the gate to force green.

## 运行方式

```bash
python3 tools/issue41_guardrail.py
```

如果你在仓库根目录，也可以使用 just 入口：

```bash
just issue41-guardrail
```

Unit / inventory tests (same as CI pin):

```bash
python3 -m unittest tools.tests.test_issue41_guardrail
```

## 常用参数

只扫指定 crate：

```bash
python3 tools/issue41_guardrail.py --crates openlark-meeting
```

将警告也作为失败（**local only until Tier-2 W001 cleared**；CI 不得默认启用）：

```bash
python3 tools/issue41_guardrail.py --strict-warn
```

## 退出码

- 默认：存在 `ERROR` 时退出码为 `1`
- 启用 `--strict-warn`：存在 `ERROR` 或 `WARN` 时退出码为 `1`

## Fixture / golden coverage

`tools/tests/test_issue41_guardrail.py` covers at least:

- missing `execute_with_options` → `E001`
- `execute` not delegating → `E002`
- option ignored (`Transport::request(..., None)`) → `E003`
- clean golden leaf → no ERROR
