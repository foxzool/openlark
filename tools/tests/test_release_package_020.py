"""Assert 0.20.0 package identity surfaces meet acceptance seams (#573).

Seams under test (public release surfaces, not implementation details):
- workspace / publishable package version identity via cargo metadata / Cargo.toml
- primary install / banner docs show 0.20.0
- CHANGELOG has a dated ## [0.20.0] section (GitHub Release body source)
- migration-guide current-version banner points at 0.20 and covers Baike break
"""

from __future__ import annotations

import re
import subprocess
import tomllib
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
TARGET_VERSION = "0.20.0"


def _cargo_package_versions() -> dict[str, str]:
    """Return package name → version for all workspace packages named openlark*."""
    proc = subprocess.run(
        ["cargo", "metadata", "--format-version", "1", "--no-deps"],
        cwd=ROOT,
        check=True,
        capture_output=True,
        text=True,
    )
    import json

    data = json.loads(proc.stdout)
    out: dict[str, str] = {}
    for pkg in data["packages"]:
        name = pkg["name"]
        if name == "openlark" or name.startswith("openlark-"):
            out[name] = pkg["version"]
    return out


class ReleasePackage020Tests(unittest.TestCase):
    def test_workspace_package_identity_is_0_20_0(self) -> None:
        root_manifest = tomllib.loads((ROOT / "Cargo.toml").read_text(encoding="utf-8"))
        self.assertEqual(
            root_manifest["workspace"]["package"]["version"],
            TARGET_VERSION,
            "workspace.package.version must be 0.20.0",
        )
        # Root package inherits via version.workspace = true (no inline version key).
        self.assertTrue(
            root_manifest["package"].get("version") in (None, TARGET_VERSION)
            or root_manifest["package"].get("version", {}).get("workspace") is True
            or root_manifest["package"].get("version") == TARGET_VERSION,
        )

        versions = _cargo_package_versions()
        self.assertIn("openlark", versions)
        for name, version in versions.items():
            if name == "openlark-capability-unique":
                # internal trybuild helper (publish=false); not part of crates.io cut
                continue
            self.assertEqual(
                version,
                TARGET_VERSION,
                f"{name} must publish as {TARGET_VERSION}",
            )

    def test_primary_docs_surfaces_show_0_20_0(self) -> None:
        surfaces = [
            ROOT / "README.md",
            ROOT / "AGENTS.md",
            ROOT / "crates" / "openlark-client" / "README.md",
            ROOT / "examples" / "01_getting_started" / "README.md",
            ROOT / "RELEASE_NOTES.md",
        ]
        for path in surfaces:
            text = path.read_text(encoding="utf-8")
            self.assertIn(
                TARGET_VERSION,
                text,
                f"{path.relative_to(ROOT)} must show {TARGET_VERSION}",
            )

    def test_changelog_has_dated_0_20_0_section(self) -> None:
        text = (ROOT / "CHANGELOG.md").read_text(encoding="utf-8")
        # Keep a Changelog dated heading; release.yml extracts this section.
        self.assertIsNotNone(
            re.search(
                rf"^## \[{re.escape(TARGET_VERSION)}\] - \d{{4}}-\d{{2}}-\d{{2}}\s*$",
                text,
                flags=re.MULTILINE,
            ),
            f"CHANGELOG must have dated ## [{TARGET_VERSION}] heading",
        )
        # Unreleased must not still hold the 0.20 narrative primary content.
        unreleased = text.split("## [0.20.0]", 1)[0]
        self.assertNotIn("BaikeApiV1", unreleased)
        self.assertIn("BaikeApiV1", text)
        self.assertIn("#573", text)  # packaging ticket referenced in freeze note

        # Upgrade-first subsection order (checklist D / changelog-compatibility-categories):
        # Changed (Breaking) before Added within the 0.20 section.
        section = text.split("## [0.20.0]", 1)[1].split("## [", 1)[0]
        breaking_at = section.find("### Changed (Breaking)")
        added_at = section.find("### Added")
        self.assertNotEqual(breaking_at, -1, "0.20 section must have ### Changed (Breaking)")
        self.assertNotEqual(added_at, -1, "0.20 section must have ### Added")
        self.assertLess(
            breaking_at,
            added_at,
            "Breaking subsection must precede Added (upgrade-impact first)",
        )

    def test_migration_guide_covers_0_20(self) -> None:
        text = (ROOT / "docs" / "migration-guide.md").read_text(encoding="utf-8")
        self.assertIn("0.20.0", text)
        self.assertRegex(text, r"# OpenLark 0\.20")
        self.assertIn("BaikeApiV1", text)
        self.assertIn("LingoApiV1", text)


if __name__ == "__main__":
    unittest.main()
