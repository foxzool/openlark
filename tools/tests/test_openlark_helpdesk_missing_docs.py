import unittest
from pathlib import Path


class OpenlarkHelpdeskMissingDocsTests(unittest.TestCase):
    def test_openlark_helpdesk_mod_roots_do_not_suppress_missing_docs(self):
        for path in Path("crates/openlark-helpdesk/src").rglob("mod.rs"):
            content = path.read_text(encoding="utf-8")
            self.assertNotIn(
                "#![allow(missing_docs)]",
                content,
                msg=f"{path} should not suppress missing_docs at module root",
            )


if __name__ == "__main__":
    unittest.main()
