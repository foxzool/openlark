import unittest
from pathlib import Path


class OpenlarkWorkflowMissingDocsTests(unittest.TestCase):
    def test_openlark_workflow_cleaned_slices_do_not_suppress_missing_docs(self):
        guarded_roots = [
            Path("crates/openlark-workflow/src/board/board/v1/whiteboard"),
            Path("crates/openlark-workflow/src/v1/task/comment"),
            Path("crates/openlark-workflow/src/v1/task/reminder"),
            Path("crates/openlark-workflow/src/v1/task/follower"),
            Path("crates/openlark-workflow/src/v1/task/collaborator"),
            Path("crates/openlark-workflow/src/v2/comment"),
            Path("crates/openlark-workflow/src/v2/section"),
            Path("crates/openlark-workflow/src/v2/attachment"),
            Path("crates/openlark-workflow/src/task/task/v2/attachment"),
            Path("crates/openlark-workflow/src/v2/custom_field"),
            Path("crates/openlark-workflow/src/v2/custom_field/option"),
            Path("crates/openlark-workflow/src/v2/tasklist/activity_subscription"),
        ]

        for root in guarded_roots:
            for path in root.rglob("*.rs"):
                content = path.read_text(encoding="utf-8")
                self.assertNotIn(
                    "#![allow(missing_docs)]",
                    content,
                    msg=f"{path} should not suppress missing_docs in cleaned workflow slices",
                )
        guarded_files = [
            Path("crates/openlark-workflow/src/v2/task/list.rs"),
            Path("crates/openlark-workflow/src/v2/task/create.rs"),
            Path("crates/openlark-workflow/src/v2/task/remove_dependencies.rs"),
            Path("crates/openlark-workflow/src/v2/task/delete.rs"),
            Path("crates/openlark-workflow/src/v2/task/update.rs"),
            Path("crates/openlark-workflow/src/v2/task/tasklists.rs"),
            Path("crates/openlark-workflow/src/v2/task/uncomplete.rs"),
            Path("crates/openlark-workflow/src/v2/task/get.rs"),
            Path("crates/openlark-workflow/src/v2/task/remove_reminders.rs"),
            Path("crates/openlark-workflow/src/v2/task/add_members.rs"),
            Path("crates/openlark-workflow/src/v2/task/remove_tasklist.rs"),
            Path("crates/openlark-workflow/src/v2/task/remove_members.rs"),
            Path("crates/openlark-workflow/src/v2/task/add_reminders.rs"),
            Path("crates/openlark-workflow/src/v2/task/complete.rs"),
            Path("crates/openlark-workflow/src/v2/task/add_tasklist.rs"),
            Path("crates/openlark-workflow/src/v2/task/add_dependencies.rs"),
            Path("crates/openlark-workflow/src/service.rs"),
            Path("crates/openlark-workflow/src/v2/tasklist/models.rs"),
        ]

        for path in guarded_files:
            content = path.read_text(encoding="utf-8")
            self.assertNotIn(
                "#![allow(missing_docs)]",
                content,
                msg=f"{path} should not suppress missing_docs in cleaned workflow files",
            )
        guarded_files = [
            Path("crates/openlark-workflow/src/v2/tasklist/list.rs"),
            Path("crates/openlark-workflow/src/v2/tasklist/tasks.rs"),
            Path("crates/openlark-workflow/src/v2/tasklist/create.rs"),
            Path("crates/openlark-workflow/src/v2/tasklist/delete.rs"),
            Path("crates/openlark-workflow/src/v2/tasklist/update.rs"),
            Path("crates/openlark-workflow/src/v2/tasklist/get.rs"),
            Path("crates/openlark-workflow/src/v2/tasklist/add_members.rs"),
            Path("crates/openlark-workflow/src/v2/tasklist/remove_members.rs"),
        ]

        for path in guarded_files:
            content = path.read_text(encoding="utf-8")
            self.assertNotIn(
                "#![allow(missing_docs)]",
                content,
                msg=f"{path} should not suppress missing_docs in cleaned workflow files",
            )
        guarded_files = [
            Path("crates/openlark-workflow/src/v1/task/list.rs"),
            Path("crates/openlark-workflow/src/v1/task/create.rs"),
            Path("crates/openlark-workflow/src/v1/task/patch.rs"),
            Path("crates/openlark-workflow/src/v1/task/delete.rs"),
            Path("crates/openlark-workflow/src/v1/task/uncomplete.rs"),
            Path("crates/openlark-workflow/src/v1/task/get.rs"),
            Path("crates/openlark-workflow/src/v1/task/complete.rs"),
        ]

        for path in guarded_files:
            content = path.read_text(encoding="utf-8")
            self.assertNotIn(
                "#![allow(missing_docs)]",
                content,
                msg=f"{path} should not suppress missing_docs in cleaned workflow files",
            )


if __name__ == "__main__":
    unittest.main()
