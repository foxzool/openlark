# API 字段核对工具（verify_api_fields.py）实现计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 产出 `tools/verify_api_fields.py`，能批量扫描代码字段、检测可疑模式，并可选抓飞书文档对比，输出差异报告。

**Architecture:** 三段式流水线——路径解析（复用 validate_apis.py 逻辑）→ 代码字段正则提取 → 可疑模式检测 / 文档抓取对比。双模式：快速（秒级自检）/ 完整（抓文档）。Python 实现，复用现有 fetch_doc.js。

**Tech Stack:** Python 3（标准库 + 正则）、Node.js（fetch_doc.js，仅完整模式）、unittest（回归测试）

**设计文档:** `docs/superpowers/specs/2026-06-16-api-field-verify-tool-design.md`

---

## 文件结构

| 文件 | 责任 | 动作 |
|------|------|------|
| `tools/verify_api_fields.py` | 主工具：CLI + 路径解析 + 字段提取 + 模式检测 + 对比 + 报告 | 创建 |
| `tools/tests/test_verify_api_fields.py` | 回归测试（字段提取 + 模式检测 + 对比逻辑） | 创建 |
| `reports/api_field_verify/` | 输出目录（运行时生成） | 运行时创建 |

工具内部分层（单文件，函数式，按职责分段）：
- `parse_args()` / `main()` — CLI 入口
- `FieldInfo` / `StructFields` — 字段数据模型
- `load_apis_from_csv()` — 读 CSV，复用 validate_apis 的 APIInfo 思路
- `generate_expected_file_path()` — 路径推断（从 validate_apis.py 移植）
- `extract_struct_fields()` — 正则提取 Body/Response struct 字段
- `detect_suspicious_patterns()` — 三类红旗检测
- `parse_doc_fields()` — 解析 fetch_doc.js 抓取的文档文本
- `compare_fields()` — 代码 vs 文档对比
- `generate_report()` — Markdown + JSON 输出

---

### Task 1: 搭建工具骨架与 CSV 加载

**Files:**
- Create: `tools/verify_api_fields.py`
- Test: `tools/tests/test_verify_api_fields.py`

- [ ] **Step 1: 写失败测试——CSV 加载与路径推断**

`tools/tests/test_verify_api_fields.py`:

```python
import importlib.util
import sys
import unittest
from pathlib import Path

MODULE_PATH = Path(__file__).resolve().parents[1] / "verify_api_fields.py"
SPEC = importlib.util.spec_from_file_location("verify_api_fields", MODULE_PATH)
verify_api_fields = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
sys.modules[SPEC.name] = verify_api_fields
SPEC.loader.exec_module(verify_api_fields)


class TestCsvLoading(unittest.TestCase):
    def test_generate_expected_file_path(self):
        """验证路径推断：bizTag/project/version/resource/name.rs"""
        api = verify_api_fields.ApiRecord(
            api_id="1", name="同意", biz_tag="approval", meta_project="approval",
            meta_version="v4", meta_resource="task", meta_name="pass",
            url="POST:/open-apis/approval/v4/tasks/pass", doc_path="", full_path="",
        )
        path = verify_api_fields.generate_expected_file_path(api)
        self.assertEqual(path, "approval/approval/v4/task/pass.rs")

    def test_generate_expected_file_path_with_dotted_resource(self):
        """resource 含 . 时转为 /（如 app.table.record）"""
        api = verify_api_fields.ApiRecord(
            api_id="2", name="创建记录", biz_tag="base", meta_project="bitable",
            meta_version="v1", meta_resource="app.table.record", meta_name="create",
            url="POST:/open-apis/bitable/v1/apps", doc_path="", full_path="",
        )
        path = verify_api_fields.generate_expected_file_path(api)
        self.assertEqual(path, "base/bitable/v1/app/table/record/create.rs")


if __name__ == "__main__":
    unittest.main()
```

- [ ] **Step 2: 运行测试确认失败**

Run: `python3 -m unittest tools.tests.test_verify_api_fields -v`
Expected: FAIL（`module 'verify_api_fields' has no attribute 'ApiRecord'`）

- [ ] **Step 3: 实现最小骨架——数据模型 + 路径推断**

`tools/verify_api_fields.py`（创建，先只含数据模型和路径推断，其余函数后续 task 补）:

```python
#!/usr/bin/env python3
"""
API 字段核对工具：扫描代码字段、检测可疑模式、可选抓飞书文档对比。

用法：
    快速模式（默认，秒级）：python3 tools/verify_api_fields.py --crate openlark-workflow
    完整模式（抓文档）：    python3 tools/verify_api_fields.py --crate openlark-workflow --fetch-docs

设计文档：docs/superpowers/specs/2026-06-16-api-field-verify-tool-design.md
"""
from __future__ import annotations

import argparse
import csv
import os
import re
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import Dict, List, Optional, Tuple

REPO_ROOT = Path(__file__).resolve().parents[1]
DEFAULT_CSV = REPO_ROOT / "api_list_export.csv"


@dataclass
class ApiRecord:
    """CSV 中一条 API 记录（只保留核对需要的字段）。"""

    api_id: str
    name: str
    biz_tag: str
    meta_project: str
    meta_version: str
    meta_resource: str
    meta_name: str
    url: str
    doc_path: str
    full_path: str

    @property
    def http_method(self) -> str:
        return self.url.partition(":")[0].upper()

    @property
    def endpoint_path(self) -> str:
        return self.url.partition(":")[2]

    @property
    def is_user_level(self) -> bool:
        """用户级接口：文档 fullPath 含 /reference/（新版用户级路径标识）。"""
        return "/reference/" in self.full_path


def generate_expected_file_path(api: ApiRecord) -> str:
    """根据 API 元信息推断 .rs 文件相对路径（移植自 validate_apis.py）。

    规则：bizTag/project/version/resource/name.rs
      - resource 的 . 转为 /
      - name 的 : 转为 _
    """
    resource_path = api.meta_resource.replace(".", "/")
    name_path = api.meta_name.replace(":", "_").rstrip("/")
    return f"{api.biz_tag}/{api.meta_project}/{api.meta_version}/{resource_path}/{name_path}.rs"


def load_apis_from_csv(
    csv_path: Path, filter_tags: Optional[List[str]] = None
) -> List[ApiRecord]:
    """从 CSV 加载 API 记录，可按 bizTag 过滤。跳过 old 版本。"""
    apis: List[ApiRecord] = []
    with open(csv_path, encoding="utf-8-sig", newline="") as f:
        for row in csv.DictReader(f):
            if filter_tags and row.get("bizTag", "") not in filter_tags:
                continue
            if row.get("meta.Version") == "old":
                continue
            apis.append(
                ApiRecord(
                    api_id=row["id"],
                    name=row["name"],
                    biz_tag=row["bizTag"],
                    meta_project=row["meta.Project"],
                    meta_version=row["meta.Version"],
                    meta_resource=row["meta.Resource"],
                    meta_name=row["meta.Name"],
                    url=row["url"],
                    doc_path=row.get("docPath", ""),
                    full_path=row.get("fullPath", ""),
                )
            )
    return apis


def main() -> int:
    """CLI 入口（后续 task 逐步充实）。"""
    parser = argparse.ArgumentParser(description="API 字段核对工具")
    parser.add_argument("--csv", default=str(DEFAULT_CSV), help="API 清单 CSV 路径")
    args = parser.parse_args()
    print(f"📂 CSV: {args.csv}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
```

- [ ] **Step 4: 运行测试确认通过**

Run: `python3 -m unittest tools.tests.test_verify_api_fields -v`
Expected: 2 tests PASS

- [ ] **Step 5: 提交**

```bash
git add tools/verify_api_fields.py tools/tests/test_verify_api_fields.py
git commit -m "feat: verify_api_fields 工具骨架与 CSV 加载/路径推断"
```

---

### Task 2: 代码字段提取（核心）

**Files:**
- Modify: `tools/verify_api_fields.py`（在 `generate_expected_file_path` 后新增提取函数）
- Test: `tools/tests/test_verify_api_fields.py`（新增测试类）

- [ ] **Step 1: 写失败测试——struct 字段提取**

追加到 `tools/tests/test_verify_api_fields.py`（在 `if __name__` 之前插入新类）:

```python
class TestExtractStructFields(unittest.TestCase):
    def test_extract_body_fields_basic(self):
        """提取必填 string 字段。"""
        source = '''
pub struct PassTaskBodyV4 {
    /// 审批实例 Code
    pub instance_code: String,
    /// 审批任务 ID
    pub task_id: String,
}
'''
        structs = verify_api_fields.extract_structs(source)
        self.assertEqual(len(structs), 1)
        s = structs[0]
        self.assertEqual(s.name, "PassTaskBodyV4")
        self.assertEqual(len(s.fields), 2)
        self.assertEqual(s.fields[0].name, "instance_code")
        self.assertTrue(s.fields[0].required)
        self.assertEqual(s.fields[1].name, "task_id")

    def test_extract_optional_and_vec_fields(self):
        """Option 是选填，Vec 是必填数组。"""
        source = '''
pub struct DemoBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    pub user_ids: Vec<String>,
    pub count: i32,
}
'''
        structs = verify_api_fields.extract_structs(source)
        fields = {f.name: f for f in structs[0].fields}
        self.assertFalse(fields["comment"].required)  # Option -> 选填
        self.assertTrue(fields["user_ids"].required)  # Vec -> 必填
        self.assertTrue(fields["count"].required)
        self.assertEqual(fields["user_ids"].type_name, "String")  # Vec<String> -> String

    def test_extract_serde_rename(self):
        """serde rename 属性被记录。"""
        source = '''
pub struct DemoBody {
    #[serde(rename = "type")]
    pub task_type: String,
}
'''
        structs = verify_api_fields.extract_structs(source)
        f = structs[0].fields[0]
        self.assertEqual(f.name, "task_type")
        self.assertEqual(f.rename, "type")

    def test_extract_only_body_and_response(self):
        """只提取名字含 Body 或 Response 的 struct。"""
        source = '''
pub struct PassTaskRequestV4 {
    pub config: Config,
}
pub struct PassTaskBodyV4 {
    pub instance_code: String,
}
pub struct PassTaskResponseV4 {
    pub data: serde_json::Value,
}
'''
        structs = verify_api_fields.extract_structs(source)
        names = [s.name for s in structs]
        self.assertIn("PassTaskBodyV4", names)
        self.assertIn("PassTaskResponseV4", names)
        self.assertNotIn("PassTaskRequestV4", names)  # Request struct 不提取
```

- [ ] **Step 2: 运行测试确认失败**

Run: `python3 -m unittest tools.tests.test_verify_api_fields.TestExtractStructFields -v`
Expected: FAIL（`has no attribute 'extract_structs'`）

- [ ] **Step 3: 实现字段提取**

在 `tools/verify_api_fields.py` 的 `load_apis_from_csv` 之后插入:

```python
@dataclass
class FieldInfo:
    """单个字段信息。"""

    name: str  # Rust 字段名（rename 前的 snake_case）
    type_name: str  # 类型名（Vec<String> -> String，Option<i32> -> i32）
    required: bool  # 是否必填（Option -> False，其余 -> True）
    rename: Optional[str] = None  # serde rename 后的名字，无则 None

    @property
    def effective_name(self) -> str:
        """对比时用的名字：rename 优先。"""
        return self.rename or self.name


@dataclass
class StructFields:
    """一个 struct 提取出的字段集合。"""

    name: str  # struct 名
    fields: List[FieldInfo] = field(default_factory=list)


def extract_structs(source: str) -> List[StructFields]:
    """从 Rust 源码提取 Body/Response struct 的字段。

    只提取名字含 Body 或 Response 的 struct（请求体/响应体），
    跳过 Request struct（那是 builder，不是字段定义）。
    """
    results: List[StructFields] = []
    # 匹配 pub struct Name { ... }，非贪婪到第一个 }
    pattern = re.compile(r"pub\s+struct\s+(\w+)\s*\{([^}]*)\}", re.S)
    for m in pattern.finditer(source):
        name = m.group(1)
        if "Body" not in name and "Response" not in name:
            continue
        body = m.group(2)
        results.append(StructFields(name=name, fields=_extract_fields_from_block(body)))
    return results


def _extract_fields_from_block(block: str) -> List[FieldInfo]:
    """从 struct 体内提取字段列表。"""
    fields: List[FieldInfo] = []
    lines = block.split("\n")
    pending_rename: Optional[str] = None
    for line in lines:
        stripped = line.strip()
        if not stripped:
            continue
        # 收集 serde rename 属性
        rename_match = re.search(r'#\[serde\s*\([^)]*rename\s*=\s*"([^"]+)"', stripped)
        if rename_match:
            pending_rename = rename_match.group(1)
            continue
        # 跳过其他属性行和注释行
        if stripped.startswith("#[") or stripped.startswith("//"):
            continue
        # 匹配 pub field_name: Type,
        field_match = re.match(r"pub\s+(\w+)\s*:\s*(.+?),?\s*$", stripped)
        if not field_match:
            continue
        fname = field_match.group(1)
        raw_type = field_match.group(2).strip().rstrip(",")
        required, type_name = _parse_type(raw_type)
        fields.append(
            FieldInfo(
                name=fname,
                type_name=type_name,
                required=required,
                rename=pending_rename,
            )
        )
        pending_rename = None
    return fields


def _parse_type(raw: str) -> Tuple[bool, str]:
    """解析类型字符串，返回 (是否必填, 规范化类型名)。"""
    raw = raw.strip()
    # Option<T> -> 选填，内部类型
    opt_match = re.match(r"Option<(.+)>$", raw)
    if opt_match:
        inner = opt_match.group(1).strip()
        return False, _unwrap_generic(inner)
    # Vec<T> -> 必填，元素类型
    vec_match = re.match(r"Vec<(.+)>$", raw)
    if vec_match:
        return True, _unwrap_generic(vec_match.group(1).strip())
    # 裸类型
    return True, _unwrap_generic(raw)


def _unwrap_generic(type_str: str) -> str:
    """去掉外层泛型，取核心类型名（Vec<String> -> String，HashMap<K,V> -> K）。"""
    inner_match = re.match(r"\w+<(.+)>$", type_str)
    if inner_match:
        return inner_match.group(1).split(",")[0].strip()
    return type_str
```

- [ ] **Step 4: 运行测试确认通过**

Run: `python3 -m unittest tools.tests.test_verify_api_fields.TestExtractStructFields -v`
Expected: 4 tests PASS

- [ ] **Step 5: 提交**

```bash
git add tools/verify_api_fields.py tools/tests/test_verify_api_fields.py
git commit -m "feat: verify_api_fields 代码字段提取（Body/Response struct 正则解析）"
```

---

### Task 3: 可疑模式检测

**Files:**
- Modify: `tools/verify_api_fields.py`（新增检测函数）
- Test: `tools/tests/test_verify_api_fields.py`（新增测试类）

- [ ] **Step 1: 写失败测试——三类可疑模式**

追加到 `tools/tests/test_verify_api_fields.py`:

```python
class TestDetectSuspiciousPatterns(unittest.TestCase):
    def test_user_level_with_user_id_field(self):
        """用户级接口的 Body 含 user_id -> 警告。"""
        api = verify_api_fields.ApiRecord(
            api_id="1", name="同意", biz_tag="approval", meta_project="approval",
            meta_version="v4", meta_resource="task", meta_name="pass",
            url="POST:/open-apis/approval/v4/tasks/pass", doc_path="",
            full_path="/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/task/pass",
        )
        structs = [
            verify_api_fields.StructFields(
                name="PassTaskBodyV4",
                fields=[
                    verify_api_fields.FieldInfo("user_id", "String", True),
                    verify_api_fields.FieldInfo("instance_code", "String", True),
                ],
            )
        ]
        source = "pub fn execute() {}"  # 无 validate_required_list
        issues = verify_api_fields.detect_suspicious_patterns(api, structs, source)
        # 应检测到 user_id 警告
        user_id_issues = [i for i in issues if "user_id" in i.detail]
        self.assertEqual(len(user_id_issues), 1)
        self.assertEqual(user_id_issues[0].severity, "warning")

    def test_vec_field_without_validate_required_list(self):
        """Body 有 Vec 字段但源码无 validate_required_list -> 警告。"""
        api = verify_api_fields.ApiRecord(
            api_id="2", name="抄送", biz_tag="approval", meta_project="approval",
            meta_version="v4", meta_resource="instance", meta_name="add_cc",
            url="POST:/open-apis/approval/v4/instances/add_cc", doc_path="", full_path="",
        )
        structs = [
            verify_api_fields.StructFields(
                name="AddCcBody",
                fields=[verify_api_fields.FieldInfo("cc_user_ids", "String", True)],
            )
        ]
        source = "validate_required!(self.body.instance_code)"  # 无 _list
        issues = verify_api_fields.detect_suspicious_patterns(api, structs, source)
        vec_issues = [i for i in issues if "validate_required_list" in i.detail]
        self.assertTrue(len(vec_issues) >= 1)

    def test_get_with_empty_response(self):
        """GET 查询接口 Response 无字段 -> 提示。"""
        api = verify_api_fields.ApiRecord(
            api_id="3", name="详情", biz_tag="approval", meta_project="approval",
            meta_version="v4", meta_resource="instance", meta_name="detail",
            url="GET:/open-apis/approval/v4/instances/detail", doc_path="", full_path="",
        )
        structs = [verify_api_fields.StructFields(name="DetailResponse", fields=[])]
        issues = verify_api_fields.detect_suspicious_patterns(api, structs, "")
        empty_resp = [i for i in issues if "Response" in i.detail or "响应" in i.detail]
        self.assertTrue(len(empty_resp) >= 1)
        self.assertEqual(empty_resp[0].severity, "info")
```

- [ ] **Step 2: 运行测试确认失败**

Run: `python3 -m unittest tools.tests.test_verify_api_fields.TestDetectSuspiciousPatterns -v`
Expected: FAIL（`has no attribute 'detect_suspicious_patterns'`）

- [ ] **Step 3: 实现可疑模式检测**

在 `tools/verify_api_fields.py` 的 `_unwrap_generic` 之后插入:

```python
@dataclass
class FieldIssue:
    """一个检测到的字段问题。"""

    severity: str  # "error" | "warning" | "info"
    category: str  # 问题类别标识
    detail: str  # 人可读的描述


def detect_suspicious_patterns(
    api: ApiRecord, structs: List[StructFields], source: str
) -> List[FieldIssue]:
    """检测不抓文档就能发现的字段问题（三类红旗）。

    红旗依据：
      1. 用户级接口 Body 含 user_id/approval_code（用户级从 token 推断，不应有）
      2. Vec 字段缺 validate_required_list! 校验
      3. GET 查询接口 Response 为空（可能漏建响应字段）
    """
    issues: List[FieldIssue] = []

    # 收集所有 Body struct 的字段
    body_structs = [s for s in structs if "Body" in s.name]

    # 红旗 1：用户级接口含 user_id / approval_code
    if api.is_user_level:
        for s in body_structs:
            for f in s.fields:
                if f.name in ("user_id", "approval_code"):
                    issues.append(
                        FieldIssue(
                            severity="warning",
                            category="user_level_extra_field",
                            detail=(
                                f"用户级接口 {s.name} 含 {f.name} 字段——"
                                "用户级接口的操作者身份从 user_access_token 推断，"
                                "请求体通常不应含此字段"
                            ),
                        )
                    )

    # 红旗 2：Vec 字段缺 validate_required_list!
    has_validate_list = "validate_required_list!" in source
    for s in body_structs:
        vec_fields = [f for f in s.fields if f.name.endswith("_ids") or f.name.endswith("_user_ids")]
        # 启发式：字段名暗示数组（_ids）但源码没用 list 校验
        if vec_fields and not has_validate_list:
            for f in vec_fields:
                issues.append(
                    FieldIssue(
                        severity="warning",
                        category="missing_validate_required_list",
                        detail=(
                            f"Body {s.name} 的 {f.name} 疑似数组字段，"
                            "但 execute_with_options 未使用 validate_required_list! 校验"
                        ),
                    )
                )

    # 红旗 3：GET 查询接口 Response 为空
    if api.http_method == "GET":
        resp_structs = [s for s in structs if "Response" in s.name]
        for s in resp_structs:
            if not s.fields:
                issues.append(
                    FieldIssue(
                        severity="info",
                        category="empty_get_response",
                        detail=(
                            f"GET 接口 {s.name} 无响应字段——"
                            "查询接口通常应返回数据，可能漏建响应体"
                        ),
                    )
                )

    return issues
```

- [ ] **Step 4: 运行测试确认通过**

Run: `python3 -m unittest tools.tests.test_verify_api_fields.TestDetectSuspiciousPatterns -v`
Expected: 3 tests PASS

- [ ] **Step 5: 提交**

```bash
git add tools/verify_api_fields.py tools/tests/test_verify_api_fields.py
git commit -m "feat: verify_api_fields 可疑模式检测（用户级字段/Vec校验/GET空响应）"
```

---

### Task 4: 快速模式串联 + 报告生成

**Files:**
- Modify: `tools/verify_api_fields.py`（充实 main，加报告生成）
- Test: `tools/tests/test_verify_api_fields.py`（端到端快速模式测试）

- [ ] **Step 1: 写失败测试——快速模式端到端**

追加到 `tools/tests/test_verify_api_fields.py`:

```python
class TestQuickModeReport(unittest.TestCase):
    def test_run_quick_mode_on_temp_files(self):
        """用临时 CSV + 临时 .rs 文件跑快速模式，生成报告。"""
        import tempfile

        with tempfile.TemporaryDirectory() as tmpdir:
            tmpdir = Path(tmpdir)
            # 构造一个 CSV（单条用户级 API，含 user_id 红旗）
            csv_file = tmpdir / "apis.csv"
            csv_file.write_text(
                "id,name,bizTag,meta.Project,meta.Version,meta.Resource,meta.Name,"
                "detail,chargingMethod,fullDose,fullPath,url,orderMark,supportAppTypes,"
                "tags,updateTime,isCharge,meta.Type,docPath\n"
                '1,同意,approval,approval,v4,task,pass,x,none,true,'
                '/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/task/pass,'
                'POST:/open-apis/approval/v4/tasks/pass,1,"[]",[],0,false,1,\n',
                encoding="utf-8",
            )
            # 构造对应的 .rs 文件
            src_dir = tmpdir / "src" / "approval" / "approval" / "v4" / "task"
            src_dir.mkdir(parents=True)
            (src_dir / "pass.rs").write_text(
                "pub struct PassTaskBodyV4 {\n"
                "    pub user_id: String,\n"
                "    pub instance_code: String,\n"
                "}\n"
                "pub struct PassTaskResponseV4 {}\n",
                encoding="utf-8",
            )
            out_md = tmpdir / "report.md"
            out_json = tmpdir / "summary.json"

            report = verify_api_fields.run_quick_mode(
                csv_path=csv_file,
                src_root=tmpdir / "src",
                output_md=out_md,
                output_json=out_json,
            )

            # 报告应包含 user_id 警告
            self.assertIn("user_id", report)
            self.assertTrue(out_md.exists())
            self.assertTrue(out_json.exists())
            import json
            data = json.loads(out_json.read_text(encoding="utf-8"))
            self.assertEqual(data["total_apis"], 1)
            self.assertGreaterEqual(data["apis_with_issues"], 1)
```

- [ ] **Step 2: 运行测试确认失败**

Run: `python3 -m unittest tools.tests.test_verify_api_fields.TestQuickModeReport -v`
Expected: FAIL（`has no attribute 'run_quick_mode'`）

- [ ] **Step 3: 实现快速模式 + 报告生成**

在 `tools/verify_api_fields.py` 的 `detect_suspicious_patterns` 之后、`main` 之前插入:

```python
@dataclass
class ApiFieldReport:
    """单个 API 的核对结果。"""

    api: ApiRecord
    file_path: str
    file_exists: bool
    structs: List[StructFields]
    issues: List[FieldIssue]


def run_quick_mode(
    csv_path: Path,
    src_root: Path,
    output_md: Optional[Path] = None,
    output_json: Optional[Path] = None,
    filter_tags: Optional[List[str]] = None,
) -> str:
    """快速模式：扫描代码字段 + 可疑模式检测，不抓文档。返回报告文本。"""
    apis = load_apis_from_csv(csv_path, filter_tags)
    reports: List[ApiFieldReport] = []

    for api in apis:
        rel_path = generate_expected_file_path(api)
        full_path = src_root / rel_path
        if not full_path.exists():
            reports.append(
                ApiFieldReport(
                    api=api, file_path=rel_path, file_exists=False,
                    structs=[], issues=[],
                )
            )
            continue
        source = full_path.read_text(encoding="utf-8")
        structs = extract_structs(source)
        issues = detect_suspicious_patterns(api, structs, source)
        reports.append(
            ApiFieldReport(
                api=api, file_path=rel_path, file_exists=True,
                structs=structs, issues=issues,
            )
        )

    md = _render_report(reports, mode="quick")
    if output_md:
        output_md.parent.mkdir(parents=True, exist_ok=True)
        output_md.write_text(md, encoding="utf-8")
    if output_json:
        _write_summary_json(reports, output_json, mode="quick")
    return md


def _render_report(reports: List[ApiFieldReport], mode: str) -> str:
    """渲染 Markdown 报告。"""
    import datetime

    total = len(reports)
    found = [r for r in reports if r.file_exists]
    missing = [r for r in reports if not r.file_exists]
    with_issues = [r for r in found if r.issues]

    lines = [
        "# API 字段核对报告",
        "",
        f"**生成时间**: {datetime.datetime.now().strftime('%Y-%m-%d %H:%M')}",
        f"**模式**: {mode}",
        "",
        "## 一、总体统计",
        "",
        "| 指标 | 数量 |",
        "|------|------|",
        f"| 核对 API 数 | {total} |",
        f"| 文件存在 | {len(found)} |",
        f"| 文件缺失 | {len(missing)} |",
        f"| 有问题 | {len(with_issues)} |",
        "",
    ]

    if with_issues:
        lines.append("## 二、问题详情（按严重度）")
        lines.append("")
        for sev, label in [("error", "🔴 硬错误"), ("warning", "🟡 警告"), ("info", "🟢 提示")]:
            sev_issues = [
                (r, i) for r in with_issues for i in r.issues if i.severity == sev
            ]
            if not sev_issues:
                continue
            lines.append(f"### {label}（{len(sev_issues)}）")
            lines.append("")
            lines.append("| API | 文件 | 问题 |")
            lines.append("|-----|------|------|")
            for r, i in sev_issues:
                lines.append(f"| {r.api.name} | `{r.file_path}` | {i.detail} |")
            lines.append("")

    if missing:
        lines.append("## 三、文件缺失（无法核对）")
        lines.append("")
        for r in missing:
            lines.append(f"- {r.api.name}: `{r.file_path}`")
        lines.append("")

    return "\n".join(lines)


def _write_summary_json(reports: List[ApiFieldReport], path: Path, mode: str) -> None:
    """写机器可读的 JSON 汇总。"""
    import json

    with_issues = sum(1 for r in reports if r.issues)
    data = {
        "mode": mode,
        "total_apis": len(reports),
        "apis_with_issues": with_issues,
        "apis": [
            {
                "id": r.api.api_id,
                "name": r.api.name,
                "url": r.api.url,
                "file": r.file_path,
                "file_exists": r.file_exists,
                "issues": [
                    {"severity": i.severity, "category": i.category, "detail": i.detail}
                    for i in r.issues
                ],
            }
            for r in reports
        ],
    }
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, ensure_ascii=False, indent=2), encoding="utf-8")
```

然后充实 `main()`:

```python
def main() -> int:
    parser = argparse.ArgumentParser(description="API 字段核对工具")
    parser.add_argument("--csv", default=str(DEFAULT_CSV), help="API 清单 CSV 路径")
    parser.add_argument("--crate", help="指定单个 crate（如 openlark-workflow）")
    parser.add_argument("--all-crates", action="store_true", help="核对所有 crate")
    parser.add_argument("--fetch-docs", action="store_true", help="完整模式：抓飞书文档对比（慢）")
    parser.add_argument("--output-dir", default="reports/api_field_verify", help="报告输出目录")
    args = parser.parse_args()

    csv_path = Path(args.csv)
    out_dir = Path(args.output_dir)

    # 确定 src 根目录和 bizTag 过滤
    if args.crate:
        src_root = REPO_ROOT / "crates" / args.crate / "src"
        filter_tags = _load_crate_tags(args.crate)
        crate_label = args.crate
    else:
        src_root = REPO_ROOT / "crates"
        filter_tags = None
        crate_label = "all"

    print(f"📂 CSV: {csv_path}")
    print(f"📁 源码根: {src_root}")
    print(f"🏷️  过滤 bizTag: {filter_tags or '(全部)'}")

    if args.fetch_docs:
        print("🐌 完整模式（抓文档）——Task 5 实现")
        # 完整模式在 Task 5 接入
        return _run_full_mode(csv_path, src_root, out_dir, crate_label, filter_tags)

    # 快速模式
    print("⚡ 快速模式（代码自检）")
    md = run_quick_mode(
        csv_path=csv_path,
        src_root=src_root,
        output_md=out_dir / f"{crate_label}.md",
        output_json=out_dir / "summary.json",
        filter_tags=filter_tags,
    )
    print(f"✅ 报告: {out_dir / f'{crate_label}.md'}")
    return 0


def _load_crate_tags(crate: str) -> Optional[List[str]]:
    """从 tools/api_coverage.toml 读 crate 的 biz_tags。"""
    import tomllib  # Python 3.11+

    toml_path = REPO_ROOT / "tools" / "api_coverage.toml"
    if not toml_path.exists():
        return None
    with open(toml_path, "rb") as f:
        data = tomllib.load(f)
    crate_cfg = data.get("crates", {}).get(crate, {})
    return crate_cfg.get("biz_tags")


def _run_full_mode(csv_path, src_root, out_dir, crate_label, filter_tags):
    """完整模式占位（Task 5 实现）。"""
    print("⚠️ 完整模式尚未实现，见 Task 5")
    return 1
```

- [ ] **Step 4: 运行测试确认通过**

Run: `python3 -m unittest tools.tests.test_verify_api_fields.TestQuickModeReport -v`
Expected: PASS

- [ ] **Step 5: 实际跑一次快速模式验证**

Run: `python3 tools/verify_api_fields.py --crate openlark-workflow`
Expected: 生成 `reports/api_field_verify/openlark-workflow.md`，能看到之前修正的 approval 接口无 user_id 警告

- [ ] **Step 6: 提交**

```bash
git add tools/verify_api_fields.py tools/tests/test_verify_api_fields.py
git commit -m "feat: verify_api_fields 快速模式串联 + Markdown/JSON 报告"
```

---

### Task 5: 完整模式（文档抓取与对比）

**Files:**
- Modify: `tools/verify_api_fields.py`（实现 `_run_full_mode` + 文档解析 + 对比）
- Test: `tools/tests/test_verify_api_fields.py`（文档解析 + 对比测试）

- [ ] **Step 1: 写失败测试——文档字段解析与对比**

追加到 `tools/tests/test_verify_api_fields.py`:

```python
class TestParseDocFields(unittest.TestCase):
    def test_parse_request_body_fields(self):
        """从文档文本提取 POST 请求体字段。"""
        doc_text = """Request header
Parameter Type Required Description

instance_code

string

Yes

审批实例 Code

task_id

string

Yes

任务 ID

Request example
"""
        fields = verify_api_fields.parse_doc_request_fields(doc_text, method="POST")
        names = {f.name for f in fields}
        self.assertIn("instance_code", names)
        self.assertIn("task_id", names)
        required_map = {f.name: f.required for f in fields}
        self.assertTrue(required_map["instance_code"])

    def test_parse_response_fields_from_example(self):
        """从响应示例 JSON 提取响应字段名。"""
        doc_text = '''Response body example
{
    "code": 0,
    "data": {
        "definition_name": "请假",
        "status": "PENDING",
        "tasks": [{"id": "1"}]
    }
}
Error code
'''
        fields = verify_api_fields.parse_doc_response_fields(doc_text)
        self.assertIn("definition_name", fields)
        self.assertIn("status", fields)
        self.assertIn("tasks", fields)


class TestCompareFields(unittest.TestCase):
    def test_compare_finds_missing_and_extra(self):
        """对比代码字段与文档字段，找出缺失和多余。"""
        code_fields = [
            verify_api_fields.FieldInfo("instance_code", "String", True),
            verify_api_fields.FieldInfo("user_id", "String", True),  # 多余
        ]
        doc_fields = [
            verify_api_fields.FieldInfo("instance_code", "String", True),
            verify_api_fields.FieldInfo("task_id", "String", True),  # 代码缺失
        ]
        diff = verify_api_fields.compare_fields(code_fields, doc_fields)
        self.assertIn("task_id", diff.missing)  # 文档有代码无
        self.assertIn("user_id", diff.extra)  # 代码有文档无
        self.assertIn("instance_code", diff.matched)
```

- [ ] **Step 2: 运行测试确认失败**

Run: `python3 -m unittest tools.tests.test_verify_api_fields.TestParseDocFields TestCompareFields -v`
Expected: FAIL（`has no attribute 'parse_doc_request_fields'`）

- [ ] **Step 3: 实现文档解析与对比**

在 `tools/verify_api_fields.py` 的 `_write_summary_json` 之后插入:

```python
@dataclass
class FieldDiff:
    """代码字段与文档字段的对比结果。"""

    matched: List[str] = field(default_factory=list)  # 两边都有
    missing: List[str] = field(default_factory=list)  # 文档有、代码无
    extra: List[str] = field(default_factory=list)  # 代码有、文档无


def parse_doc_request_fields(doc_text: str, method: str) -> List[FieldInfo]:
    """从文档 innerText 提取请求体/查询参数字段。

    POST: Request body（第2次出现）→ Request example
    GET:  Query parameters → Request example
    """
    if method == "POST":
        section = _extract_section(doc_text, "Request body", "Request example", occurrence=2)
    else:
        section = _extract_section(doc_text, "Query parameters", "Request example", occurrence=1)
    if not section:
        return []
    return _parse_param_table(section)


def parse_doc_response_fields(doc_text: str) -> List[str]:
    """从响应示例 JSON 提取字段名集合。

    Response body 的 data 子字段在折叠区拿不到，从示例 JSON 反推。
    返回 data 内部的字段名列表。
    """
    section = _extract_section(doc_text, "Response body example", "Error code", occurrence=1)
    if not section:
        return []
    # 提取所有 "field": 的字段名（排除外层 code/msg/data）
    names = re.findall(r'"([a-z_]+)"\s*:', section)
    return [n for n in names if n not in ("code", "msg", "data")]


def _extract_section(text: str, start: str, end: str, occurrence: int = 1) -> str:
    """提取 start（第 occurrence 次出现）到 end 之间的文本。"""
    parts = text.split(start)
    if len(parts) <= occurrence:
        return ""
    chunk = start.join(parts[occurrence:])
    end_idx = chunk.find(end)
    if end_idx < 0:
        return chunk
    return chunk[:end_idx]


def _parse_param_table(section: str) -> List[FieldInfo]:
    """解析参数表（参数名/类型/必填交错成行）。"""
    lines = [l.strip() for l in section.split("\n")]
    results: List[FieldInfo] = []
    banned = {
        "parameter", "type", "required", "description", "authorization",
        "content", "value", "example",
    }
    i = 0
    while i < len(lines):
        line = lines[i]
        # 候选参数名：snake_case，非 banned 词
        if re.fullmatch(r"[a-z][a-z0-9_]*", line) and line not in banned and len(line) >= 2:
            # 往后找 Yes/No
            for j in range(i + 1, min(i + 8, len(lines))):
                if lines[j] in ("Yes", "No"):
                    required = lines[j] == "Yes"
                    results.append(FieldInfo(name=line, type_name="", required=required))
                    i = j + 1
                    break
            else:
                i += 1
        else:
            i += 1
    return results


def compare_fields(
    code_fields: List[FieldInfo], doc_fields: List[FieldInfo]
) -> FieldDiff:
    """对比代码字段与文档字段。"""
    code_names = {f.effective_name for f in code_fields}
    doc_names = {f.effective_name for f in doc_fields}
    return FieldDiff(
        matched=sorted(code_names & doc_names),
        missing=sorted(doc_names - code_names),
        extra=sorted(code_names - doc_names),
    )
```

- [ ] **Step 4: 运行测试确认通过**

Run: `python3 -m unittest tools.tests.test_verify_api_fields.TestParseDocFields TestCompareFields -v`
Expected: 3 tests PASS

- [ ] **Step 5: 实现完整模式串联（_run_full_mode）**

替换 `_run_full_mode` 占位函数:

```python
def _run_full_mode(csv_path, src_root, out_dir, crate_label, filter_tags):
    """完整模式：抓文档对比字段（慢）。"""
    import subprocess

    apis = load_apis_from_csv(csv_path, filter_tags)
    fetch_script = REPO_ROOT / ".agents" / "skills" / "openlark-api-field-verify" / "scripts" / "fetch_doc.js"
    if not fetch_script.exists():
        print(f"❌ 找不到抓取脚本: {fetch_script}")
        return 1

    reports: List[ApiFieldReport] = []
    doc_cache = out_dir / "doc_cache"
    doc_cache.mkdir(parents=True, exist_ok=True)
    failed: List[Tuple[str, str]] = []

    for idx, api in enumerate(apis, 1):
        rel_path = generate_expected_file_path(api)
        full_path = src_root / rel_path
        if not full_path.exists():
            continue

        source = full_path.read_text(encoding="utf-8")
        structs = extract_structs(source)
        issues = detect_suspicious_patterns(api, structs, source)

        # 抓文档
        if api.full_path:
            url = "https://open.feishu.cn" + api.full_path
            doc_file = doc_cache / f"{api.api_id}.txt"
            if not doc_file.exists():  # 简单 resume：文件存在则跳过
                try:
                    subprocess.run(
                        ["node", str(fetch_script), url, str(doc_file)],
                        check=True, capture_output=True, timeout=90,
                    )
                except (subprocess.CalledProcessError, subprocess.TimeoutExpired) as e:
                    failed.append((api.api_id, str(e)[:80]))
                    doc_text = ""
                else:
                    doc_text = doc_file.read_text(encoding="utf-8") if doc_file.exists() else ""
            else:
                doc_text = doc_file.read_text(encoding="utf-8")

            # 对比字段
            if doc_text:
                doc_req = parse_doc_request_fields(doc_text, api.http_method)
                code_body = next((s.fields for s in structs if "Body" in s.name), [])
                if doc_req and code_body:
                    diff = compare_fields(code_body, doc_req)
                    if diff.missing:
                        issues.append(FieldIssue("error", "missing_field",
                                                  f"请求体缺字段: {', '.join(diff.missing)}"))
                    if diff.extra:
                        issues.append(FieldIssue("warning", "extra_field",
                                                  f"请求体多余字段: {', '.join(diff.extra)}"))
        else:
            doc_text = ""

        reports.append(ApiFieldReport(
            api=api, file_path=rel_path, file_exists=True,
            structs=structs, issues=issues,
        ))
        print(f"⏳ [{idx}/{len(apis)}] {api.name} ({len(issues)} 问题)")

    md = _render_report(reports, mode="full")
    (out_dir / f"{crate_label}.md").write_text(md, encoding="utf-8")
    _write_summary_json(reports, out_dir / "summary.json", mode="full")

    if failed:
        print(f"⚠️ {len(failed)} 个文档抓取失败，详见 failed.json")
        import json
        (out_dir / "failed.json").write_text(
            json.dumps(failed, ensure_ascii=False, indent=2), encoding="utf-8"
        )
    print(f"✅ 报告: {out_dir / f'{crate_label}.md'}")
    return 0
```

- [ ] **Step 6: 手动验证（单 API）**

Run: `python3 tools/verify_api_fields.py --crate openlark-workflow --fetch-docs`（观察前几个，Ctrl+C 中断即可，验证能抓取对比）
Expected: 能看到进度输出和字段对比

- [ ] **Step 7: 提交**

```bash
git add tools/verify_api_fields.py tools/tests/test_verify_api_fields.py
git commit -m "feat: verify_api_fields 完整模式（文档抓取对比 + 字段差异）"
```

---

### Task 6: 文档更新与收尾

**Files:**
- Modify: `tools/verify_api_fields.py`（补 `--max-workers`/`--resume` 参数，文档化）
- Modify: `.agents/skills/openlark-api-field-verify/SKILL.md`（指向新工具）

- [ ] **Step 1: 补 CLI 参数（max-workers / resume）**

在 `main()` 的 argparse 部分追加参数:

```python
    parser.add_argument("--max-workers", type=int, default=1, help="文档抓取并发数（默认 1，防限流）")
    parser.add_argument("--resume", action="store_true", help="跳过已抓取的文档（按缓存文件判断）")
    parser.add_argument("--api-id", help="只核对单个 API（调试用）")
```

在 `main()` 开头加 api-id 过滤:

```python
    if args.api_id:
        filter_tags = None  # api-id 模式不过滤 bizTag
        # 后续 load 时按 id 过滤（在 _run_full_mode / run_quick_mode 内处理）
```

- [ ] **Step 2: 更新 field-verify 技能，指向新工具**

在 `.agents/skills/openlark-api-field-verify/SKILL.md` 的"批量抓取"部分后追加:

```markdown
### 批量自动化（推荐用于全量核对）

对于多接口/全 crate 核对，使用工具脚本而非手动逐个：

```bash
# 快速模式：全仓代码自检（秒级）
python3 tools/verify_api_fields.py --all-crates

# 完整模式：单个 crate 抓文档对比
python3 tools/verify_api_fields.py --crate openlark-workflow --fetch-docs
```

工具自动完成路径解析、字段提取、文档抓取、差异对比，输出 `reports/api_field_verify/` 报告。
```

- [ ] **Step 3: 跑完整测试套件**

Run: `python3 -m unittest tools.tests.test_verify_api_fields -v`
Expected: 全部 PASS（约 12 个测试）

- [ ] **Step 4: 提交**

```bash
git add tools/verify_api_fields.py .agents/skills/openlark-api-field-verify/SKILL.md
git commit -m "feat: verify_api_fields 补全 CLI 参数 + 更新技能指向工具"
```

---

## 实现顺序总结

1. Task 1: 骨架 + CSV 加载 + 路径推断（可独立测试）
2. Task 2: 字段提取（核心，可独立测试）
3. Task 3: 可疑模式检测（依赖 Task 2，可独立测试）
4. Task 4: 快速模式串联 + 报告（依赖 1-3，端到端可用）
5. Task 5: 完整模式文档对比（依赖 1-3，可独立测试解析/对比逻辑）
6. Task 6: 收尾（CLI 补全 + 技能更新）

Task 1-4 完成后工具的快速模式即可投入使用（最高价值，秒级全仓扫描）。
