import unittest
from pathlib import Path


class OpenlarkHrMissingDocsTests(unittest.TestCase):
    def test_openlark_hr_files_do_not_suppress_missing_docs(self):
        for path in Path("crates/openlark-hr").rglob("*.rs"):
            content = path.read_text(encoding="utf-8")
            self.assertNotIn(
                "#![allow(missing_docs)]",
                content,
                msg=f"{path} should not suppress missing_docs in hr crate",
            )


if __name__ == "__main__":
    unittest.main()
