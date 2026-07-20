import re
import tomllib
import unittest
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
WORKFLOW_PATH = ROOT / ".github" / "workflows" / "release.yml"
PUBLISH_SCRIPT_PATH = ROOT / "scripts" / "publish-workspace.sh"


def publishable_workspace_crates():
    root_manifest = tomllib.loads((ROOT / "Cargo.toml").read_text(encoding="utf-8"))
    package_names = {root_manifest["package"]["name"]}
    for member in root_manifest["workspace"]["members"]:
        manifest = tomllib.loads((ROOT / member / "Cargo.toml").read_text(encoding="utf-8"))
        package = manifest["package"]
        if package.get("publish", True) is not False:
            package_names.add(package["name"])
    return package_names


class ReleaseWorkflowTests(unittest.TestCase):
    def test_release_checks_coverage_and_endpoint_contracts(self):
        workflow = WORKFLOW_PATH.read_text(encoding="utf-8")

        self.assertIn("python3 tools/check_typed_coverage_release.py", workflow)
        self.assertIn(
            "python3 tools/validate_api_contracts.py --all-crates --strict endpoint",
            workflow,
        )

    def test_publish_step_covers_every_publishable_workspace_crate(self):
        workflow = WORKFLOW_PATH.read_text(encoding="utf-8")
        self.assertIn("bash scripts/publish-workspace.sh", workflow)

        publish_script = PUBLISH_SCRIPT_PATH.read_text(encoding="utf-8")
        loop = re.search(r"PUBLISH_ORDER=\((?P<crates>.*?)\n\)", publish_script, re.DOTALL)
        self.assertIsNotNone(loop)
        planned = set(re.findall(r'"(openlark(?:-[a-z-]+)?)"', loop.group("crates")))

        self.assertEqual(planned, publishable_workspace_crates())

    def test_publish_step_is_fail_fast(self):
        publish_script = PUBLISH_SCRIPT_PATH.read_text(encoding="utf-8")

        self.assertIn("set -euo pipefail", publish_script)
        self.assertNotRegex(publish_script, r"cargo publish[^\n]+\|\|")
        self.assertIn('grep -q "is already uploaded"', publish_script)


if __name__ == "__main__":
    unittest.main()
