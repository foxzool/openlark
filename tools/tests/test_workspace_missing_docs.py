import subprocess
import unittest
from pathlib import Path


class WorkspaceMissingDocsTests(unittest.TestCase):
    def test_workspace_has_no_missing_docs_warnings(self):
        result = subprocess.run(
            ["cargo", "test", "--workspace", "--all-features", "--no-run"],
            capture_output=True,
            text=True,
            check=False,
        )

        output = result.stdout + result.stderr
        self.assertEqual(result.returncode, 0, msg=output)
        self.assertNotIn("warning: missing documentation for ", output, msg=output)

    def test_workspace_source_files_do_not_use_crate_level_missing_docs_suppressions(self):
        for path in Path("crates").rglob("*.rs"):
            content = path.read_text(encoding="utf-8")
            self.assertNotIn(
                "#![allow(missing_docs)]",
                content,
                msg=f"{path} should not suppress missing_docs warnings in workspace crates",
            )

    def test_workspace_item_level_missing_docs_exception_is_empty(self):
        allow_hits = []
        for path in Path("crates").rglob("*.rs"):
            content = path.read_text(encoding="utf-8")
            if "#[allow(missing_docs)]" in content:
                allow_hits.append(path.as_posix())

        self.assertEqual(
            allow_hits,
            [],
            msg=f"unexpected item-level missing_docs exceptions: {allow_hits}",
        )


if __name__ == "__main__":
    unittest.main()
