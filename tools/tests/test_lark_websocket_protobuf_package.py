"""Regression coverage for the crates.io packaging seam from issue #605."""

import os
import subprocess
import tarfile
import tempfile
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
CRATE_DIR = ROOT / "crates" / "lark-websocket-protobuf"


class LarkWebsocketProtobufPackageTests(unittest.TestCase):
    def test_packaged_crate_builds_without_protoc(self):
        with tempfile.TemporaryDirectory() as temp_dir:
            temp = Path(temp_dir)
            package_target = temp / "package-target"
            environment = os.environ.copy()
            environment["CARGO_TARGET_DIR"] = str(package_target)

            package = subprocess.run(
                [
                    "cargo",
                    "package",
                    "--manifest-path",
                    str(CRATE_DIR / "Cargo.toml"),
                    "--allow-dirty",
                    "--no-verify",
                    "--offline",
                ],
                cwd=ROOT,
                env=environment,
                check=False,
                capture_output=True,
                text=True,
            )
            self.assertEqual(package.returncode, 0, package.stdout + package.stderr)

            archives = list((package_target / "package").glob("*.crate"))
            self.assertEqual(len(archives), 1, archives)
            unpacked = temp / "unpacked"
            with tarfile.open(archives[0], "r:gz") as archive:
                archive.extractall(unpacked, filter="data")

            package_root = next(unpacked.iterdir())
            environment["CARGO_TARGET_DIR"] = str(temp / "consumer-target")
            environment["PROTOC"] = str(temp / "missing-protoc")
            build = subprocess.run(
                [
                    "cargo",
                    "check",
                    "--manifest-path",
                    str(package_root / "Cargo.toml"),
                    "--offline",
                ],
                cwd=package_root,
                env=environment,
                check=False,
                capture_output=True,
                text=True,
            )
            self.assertEqual(build.returncode, 0, build.stdout + build.stderr)
            self.assertTrue((package_root / "src" / "pbbp2.rs").is_file())
            self.assertFalse((package_root / "build.rs").exists())
            packaged_manifest = (package_root / "Cargo.toml").read_text(encoding="utf-8")
            self.assertNotIn("prost-build", packaged_manifest)


if __name__ == "__main__":
    unittest.main()
