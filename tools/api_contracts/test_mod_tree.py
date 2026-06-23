"""mod_tree 增量维护测试：多层补、已有跳过、不抹现有、新建。"""

from __future__ import annotations

import shutil
import tempfile
import unittest
from pathlib import Path

from tools.api_contracts.mod_tree import ensure_mod_chain


class EnsureModChainTest(unittest.TestCase):
    def setUp(self) -> None:
        self.root = Path(tempfile.mkdtemp())

    def tearDown(self) -> None:
        shutil.rmtree(self.root, ignore_errors=True)

    def _read(self, rel: str) -> str:
        return (self.root / rel).read_text(encoding="utf-8")

    def test_creates_missing_chain(self):
        # expected_file im/im/v1/message/create.rs，所有 mod.rs 不存在
        actions = ensure_mod_chain(self.root, "im/im/v1/message/create.rs")
        # 应创建 4 层 mod.rs（im, im, v1, message 各层）+ 补 create
        created = [a for a in actions if a.startswith("[CREATE]")]
        self.assertEqual(len(created), 4)
        # 逐层 pub mod 对
        self.assertIn("pub mod im;", self._read("im/mod.rs"))
        self.assertIn("pub mod v1;", self._read("im/im/mod.rs"))
        self.assertIn("pub mod message;", self._read("im/im/v1/mod.rs"))
        self.assertIn("pub mod create;", self._read("im/im/v1/message/mod.rs"))

    def test_skips_existing_pub_mod(self):
        # 预建 message/mod.rs 已含 pub mod create
        msg_mod = self.root / "im" / "im" / "v1" / "message" / "mod.rs"
        msg_mod.parent.mkdir(parents=True, exist_ok=True)
        msg_mod.write_text("//! message\n\npub mod create;\npub mod get;\n", encoding="utf-8")
        actions = ensure_mod_chain(self.root, "im/im/v1/message/create.rs")
        # message/mod.rs 已有 create → 不重复加
        self.assertNotIn("[MOD]", "".join(a for a in actions if "message/mod.rs" in a))
        # 内容不变（无重复 pub mod create）
        content = msg_mod.read_text(encoding="utf-8")
        self.assertEqual(content.count("pub mod create;"), 1)
        self.assertIn("pub mod get;", content)  # 现有保留

    def test_appends_without_destroying_existing(self):
        # message/mod.rs 有 doc + pub use + pub mod，append create 不破坏
        msg_mod = self.root / "im" / "im" / "v1" / "message" / "mod.rs"
        msg_mod.parent.mkdir(parents=True, exist_ok=True)
        original = "//! 消息模块\npub mod get;\npub use get::GetMessageRequest;\n"
        msg_mod.write_text(original, encoding="utf-8")
        ensure_mod_chain(self.root, "im/im/v1/message/create.rs")
        content = msg_mod.read_text(encoding="utf-8")
        # 现有全保留
        self.assertIn("//! 消息模块", content)
        self.assertIn("pub mod get;", content)
        self.assertIn("pub use get::GetMessageRequest;", content)
        # 新增 create
        self.assertIn("pub mod create;", content)

    def test_dry_run_no_write(self):
        actions = ensure_mod_chain(self.root, "a/b/c.rs", dry_run=True)
        self.assertTrue(any("[CREATE]" in a for a in actions))
        # dry_run 不写盘
        self.assertFalse((self.root / "a" / "mod.rs").exists())

    def test_single_segment_no_mod_needed(self):
        # expected_file 无目录（罕见）→ 无 mod.rs 操作
        actions = ensure_mod_chain(self.root, "lonely.rs")
        self.assertEqual(actions, [])


if __name__ == "__main__":
    unittest.main()
