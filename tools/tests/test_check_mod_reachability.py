import importlib.util
import sys
import unittest
from pathlib import Path

MODULE_PATH = Path(__file__).resolve().parents[1] / "check_mod_reachability.py"
SPEC = importlib.util.spec_from_file_location("check_mod_reachability", MODULE_PATH)
mod = importlib.util.module_from_spec(SPEC)
assert SPEC.loader is not None
sys.modules[SPEC.name] = mod
SPEC.loader.exec_module(mod)


class TestParseDepInfo(unittest.TestCase):
    def test_parses_src_files_on_first_line(self):
        src_dir = Path("/repo/crates/openlark-fake/src")
        content = (
            "/repo/target/debug/libopenlark_fake.rlib: "
            "/repo/crates/openlark-fake/src/lib.rs "
            "/repo/crates/openlark-fake/src/a.rs "
            "/repo/crates/openlark-core/src/lib.rs\n"
        )
        result = mod.parse_dep_info(content, src_dir)
        names = {p.name for p in result}
        self.assertEqual(names, {"lib.rs", "a.rs"})

    def test_excludes_other_crates_and_non_rs(self):
        src_dir = Path("/repo/crates/openlark-fake/src")
        content = (
            "/repo/target/debug/libopenlark_fake.rlib: "
            "/repo/crates/openlark-fake/src/lib.rs "
            "/repo/crates/openlark-core/src/lib.rs "
            "/repo/crates/openlark-fake/Cargo.toml\n"
        )
        result = mod.parse_dep_info(content, src_dir)
        self.assertEqual({p.name for p in result}, {"lib.rs"})

    def test_handles_line_continuation(self):
        src_dir = Path("/repo/crates/openlark-fake/src")
        content = (
            "lib.rlib: /repo/crates/openlark-fake/src/lib.rs \\\n"
            "  /repo/crates/openlark-fake/src/b.rs\n"
        )
        result = mod.parse_dep_info(content, src_dir)
        self.assertEqual({p.name for p in result}, {"lib.rs", "b.rs"})


class TestDiffOrphans(unittest.TestCase):
    def test_finds_orphan(self):
        a = Path("/x/a.rs")
        b = Path("/x/b.rs")
        c = Path("/x/c.rs")
        result = mod.diff_orphans([a, b, c], [a, b])
        self.assertEqual(result, {c})

    def test_no_orphan_when_equal(self):
        f = Path("/x/a.rs")
        self.assertEqual(mod.diff_orphans([f], [f]), set())


class TestIsTestOnlyModule(unittest.TestCase):
    """#[cfg(test)] mod <name>; 挂载的文件应被识别为测试专用（非真孤儿）。"""

    def test_detects_cfg_test_mod(self):
        """`#[cfg(test)]` 下一行 `mod tests;` → 视为测试专用。"""
        import tempfile

        with tempfile.TemporaryDirectory() as d:
            root = Path(d)
            (root / "parent.rs").write_text(
                "#[cfg(test)]\nmod tests;\n", encoding="utf-8"
            )
            orphan = root / "tests.rs"
            self.assertTrue(mod.is_test_only_module(orphan, root))

    def test_detects_pub_mod_under_cfg_test(self):
        """`#[cfg(test)] pub mod mock_server;` → 视为测试专用。"""
        import tempfile

        with tempfile.TemporaryDirectory() as d:
            root = Path(d)
            (root / "mod.rs").write_text(
                "pub mod assertions;\n\n#[cfg(test)]\npub mod mock_server;\n",
                encoding="utf-8",
            )
            orphan = root / "mock_server.rs"
            self.assertTrue(mod.is_test_only_module(orphan, root))

    def test_non_test_mod_is_not_test_only(self):
        """普通 `mod foo;`（无 cfg(test)）→ 不是测试专用。"""
        import tempfile

        with tempfile.TemporaryDirectory() as d:
            root = Path(d)
            (root / "parent.rs").write_text("mod foo;\n", encoding="utf-8")
            orphan = root / "foo.rs"
            self.assertFalse(mod.is_test_only_module(orphan, root))

    def test_wrong_name_not_test_only(self):
        """`#[cfg(test)] mod other;` 不匹配 `tests.rs` → 不是测试专用。"""
        import tempfile

        with tempfile.TemporaryDirectory() as d:
            root = Path(d)
            (root / "parent.rs").write_text(
                "#[cfg(test)]\nmod other;\n", encoding="utf-8"
            )
            orphan = root / "tests.rs"
            self.assertFalse(mod.is_test_only_module(orphan, root))


if __name__ == "__main__":
    unittest.main()
