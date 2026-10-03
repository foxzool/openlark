import unittest
from pathlib import Path


class OpenlarkPlatformMissingDocsTests(unittest.TestCase):
    def test_v1_root_modules_do_not_use_missing_docs_allow(self):
        for path in Path("crates/openlark-platform/src").rglob("*.rs"):
            content = path.read_text(encoding="utf-8")
            self.assertNotIn("#![allow(missing_docs)]", content, msg=f"{path} still suppresses missing_docs warnings")


if __name__ == "__main__":
    unittest.main()
