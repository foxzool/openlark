# Triage Labels

The skills speak in terms of five canonical triage roles. This file maps those roles to the actual label strings used in this repo's issue tracker.

| Label in mattpocock/skills | Label in our tracker | Meaning                                  |
| -------------------------- | -------------------- | ---------------------------------------- |
| `needs-triage`             | `needs-triage`       | Maintainer needs to evaluate this issue  |
| `needs-info`               | `needs-info`         | Waiting on reporter for more information |
| `ready-for-agent`          | `ready-for-agent`    | Fully specified, ready for an AFK agent  |
| `ready-for-human`          | `ready-for-human`    | Requires human implementation            |
| `wontfix`                  | `wontfix`            | Will not be actioned                     |

When a skill mentions a role (e.g. "apply the AFK-ready triage label"), use the corresponding label string from this table.

Edit the right-hand column to match whatever vocabulary you actually use.

## Category roles

The `triage` skill also speaks in terms of two **category** roles — `bug` (something is broken) and `enhancement` (new feature or improvement). Every triaged issue should carry exactly one category role **and** one state role.

This repo expresses category through its existing `type:` taxonomy rather than the canonical `bug` / `enhancement` names, keeping category orthogonal to the state machine above and consistent with the project's conventions:

| Canonical category | This repo's label | When to use |
| --- | --- | --- |
| `bug` | `bug` | 缺陷修复（仓库已有 `bug` label） |
| `enhancement` | `type:feat` | 新功能或公开 API 增强（如补 accessor） |
| `enhancement` | `type:refactor` | 重构、清理、死码移除 |
| `enhancement` | `type:docs` / `type:test` / `type:ci` / `type:policy` | 文档 / 测试 / CI / 策略类改进 |

当 skill 提到 "apply a category role" 时，按上表选 `type:` 标签（`bug` 类用独立 `bug` label）。state 与 category 正交：一个 issue 同时带一个 state（最上面的表）+ 一个 category（本表）。
