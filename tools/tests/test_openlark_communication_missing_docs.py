import unittest
from pathlib import Path


class OpenlarkCommunicationMissingDocsTests(unittest.TestCase):
    def test_openlark_communication_cleaned_slices_do_not_suppress_missing_docs(self):
        guarded_roots = [
            Path("crates/openlark-communication/src/endpoints"),
        ]
        guarded_files = [
            Path("crates/openlark-communication/src/common/chain.rs"),
            Path("crates/openlark-communication/src/moments/moments/v1/post/get.rs"),
            Path("crates/openlark-communication/src/aily/aily/v1/aily_session/create.rs"),
            Path("crates/openlark-communication/src/aily/aily/v1/aily_session/delete.rs"),
            Path("crates/openlark-communication/src/aily/aily/v1/aily_session/update.rs"),
            Path("crates/openlark-communication/src/aily/aily/v1/aily_session/get.rs"),
            Path("crates/openlark-communication/src/aily/aily/v1/aily_session/aily_message/list.rs"),
            Path("crates/openlark-communication/src/aily/aily/v1/aily_session/aily_message/create.rs"),
            Path("crates/openlark-communication/src/aily/aily/v1/aily_session/aily_message/get.rs"),
            Path("crates/openlark-communication/src/aily/aily/v1/aily_session/run/list.rs"),
            Path("crates/openlark-communication/src/aily/aily/v1/aily_session/run/cancel.rs"),
            Path("crates/openlark-communication/src/aily/aily/v1/aily_session/run/create.rs"),
            Path("crates/openlark-communication/src/aily/aily/v1/aily_session/run/get.rs"),
            Path("crates/openlark-communication/src/aily/aily/v1/app/data_asset/list.rs"),
            Path("crates/openlark-communication/src/aily/aily/v1/app/data_asset/create.rs"),
            Path("crates/openlark-communication/src/aily/aily/v1/app/data_asset/delete.rs"),
            Path("crates/openlark-communication/src/aily/aily/v1/app/data_asset/get.rs"),
            Path("crates/openlark-communication/src/aily/aily/v1/app/data_asset/upload_file.rs"),
            Path("crates/openlark-communication/src/aily/aily/v1/app/skill/start.rs"),
            Path("crates/openlark-communication/src/aily/aily/v1/app/skill/list.rs"),
            Path("crates/openlark-communication/src/aily/aily/v1/app/skill/get.rs"),
            Path("crates/openlark-communication/src/aily/aily/v1/app/data_asset_tag/list.rs"),
            Path("crates/openlark-communication/src/aily/aily/v1/app/knowledge/ask.rs"),
            Path("crates/openlark-communication/src/contact/contact/v3/unit/bind_department.rs"),
            Path("crates/openlark-communication/src/contact/contact/v3/unit/create.rs"),
            Path("crates/openlark-communication/src/contact/contact/v3/unit/delete.rs"),
            Path("crates/openlark-communication/src/contact/contact/v3/unit/get.rs"),
            Path("crates/openlark-communication/src/contact/contact/v3/unit/list.rs"),
            Path("crates/openlark-communication/src/contact/contact/v3/unit/list_department.rs"),
            Path("crates/openlark-communication/src/contact/contact/v3/unit/models.rs"),
            Path("crates/openlark-communication/src/contact/contact/v3/unit/patch.rs"),
            Path("crates/openlark-communication/src/contact/contact/v3/unit/unbind_department.rs"),
            Path("crates/openlark-communication/src/contact/contact/v3/user/batch_get_id.rs"),
            Path("crates/openlark-communication/src/contact/contact/v3/user/create.rs"),
            Path("crates/openlark-communication/src/contact/contact/v3/user/delete.rs"),
            Path("crates/openlark-communication/src/contact/contact/v3/user/find_by_department.rs"),
            Path("crates/openlark-communication/src/contact/contact/v3/user/models.rs"),
            Path("crates/openlark-communication/src/contact/contact/v3/user/patch.rs"),
            Path("crates/openlark-communication/src/contact/contact/v3/user/update.rs"),
            Path("crates/openlark-communication/src/im/im/v1/message/reaction/create.rs"),
            Path("crates/openlark-communication/src/im/im/v1/message/reaction/delete.rs"),
            Path("crates/openlark-communication/src/im/im/v1/message/reaction/list.rs"),
            Path("crates/openlark-communication/src/im/im/v1/message/reaction/models.rs"),
            Path("crates/openlark-communication/src/im/im/v1/message/read_users.rs"),
            Path("crates/openlark-communication/src/im/im/v1/message/push_follow_up.rs"),
            Path("crates/openlark-communication/src/im/im/v1/message/urgent_app.rs"),
            Path("crates/openlark-communication/src/im/im/v1/message/urgent_phone.rs"),
            Path("crates/openlark-communication/src/im/im/v1/message/urgent_sms.rs"),
            Path("crates/openlark-communication/src/im/im/v1/message/resource/get.rs"),
            Path("crates/openlark-communication/src/im/im/v1/message/create.rs"),
            Path("crates/openlark-communication/src/im/im/v1/message/get.rs"),
            Path("crates/openlark-communication/src/im/im/v1/message/list.rs"),
            Path("crates/openlark-communication/src/im/im/v1/message/delete.rs"),
            Path("crates/openlark-communication/src/im/im/v1/message/patch.rs"),
            Path("crates/openlark-communication/src/im/im/v1/message/update.rs"),
            Path("crates/openlark-communication/src/im/im/v1/message/forward.rs"),
            Path("crates/openlark-communication/src/im/im/v1/message/merge_forward.rs"),
            Path("crates/openlark-communication/src/im/im/v1/message/reply.rs"),
            Path("crates/openlark-communication/src/im/im/v1/message/models.rs"),
            Path("crates/openlark-communication/src/im/im/v1/chat/announcement/get.rs"),
            Path("crates/openlark-communication/src/im/im/v1/chat/announcement/patch.rs"),
            Path("crates/openlark-communication/src/im/im/v1/chat/announcement/models.rs"),
            Path("crates/openlark-communication/src/im/im/v1/chat/moderation/get.rs"),
            Path("crates/openlark-communication/src/im/im/v1/chat/moderation/update.rs"),
            Path("crates/openlark-communication/src/im/im/v1/chat/moderation/models.rs"),
            Path("crates/openlark-communication/src/im/im/v1/chat/managers/add_managers.rs"),
            Path("crates/openlark-communication/src/im/im/v1/chat/managers/delete_managers.rs"),
            Path("crates/openlark-communication/src/im/im/v1/chat/managers/models.rs"),
            Path("crates/openlark-communication/src/im/im/v1/chat/menu_tree/create.rs"),
            Path("crates/openlark-communication/src/im/im/v1/chat/menu_tree/delete.rs"),
            Path("crates/openlark-communication/src/im/im/v1/chat/menu_tree/get.rs"),
            Path("crates/openlark-communication/src/im/im/v1/chat/menu_tree/models.rs"),
            Path("crates/openlark-communication/src/im/im/v1/chat/menu_tree/sort.rs"),
            Path("crates/openlark-communication/src/im/im/v1/chat/menu_item/patch.rs"),
            Path("crates/openlark-communication/src/im/im/v1/chat/top_notice/put_top_notice.rs"),
            Path("crates/openlark-communication/src/im/im/v1/chat/top_notice/delete_top_notice.rs"),
            Path("crates/openlark-communication/src/im/im/v1/chat/models.rs"),
            Path("crates/openlark-communication/src/im/im/v1/chat/tab/create.rs"),
            Path("crates/openlark-communication/src/im/im/v1/chat/tab/delete_tabs.rs"),
            Path("crates/openlark-communication/src/im/im/v1/chat/tab/list_tabs.rs"),
            Path("crates/openlark-communication/src/im/im/v1/chat/tab/models.rs"),
            Path("crates/openlark-communication/src/im/im/v1/chat/tab/sort_tabs.rs"),
            Path("crates/openlark-communication/src/im/im/v1/chat/tab/update_tabs.rs"),
            Path("crates/openlark-communication/src/im/im/v1/chat/members/create.rs"),
            Path("crates/openlark-communication/src/im/im/v1/chat/members/delete.rs"),
            Path("crates/openlark-communication/src/im/im/v1/chat/members/get.rs"),
            Path("crates/openlark-communication/src/im/im/v1/chat/members/is_in_chat.rs"),
            Path("crates/openlark-communication/src/im/im/v1/chat/members/me_join.rs"),
            Path("crates/openlark-communication/src/im/im/v1/chat/members/models.rs"),
        ]

        for root in guarded_roots:
            for path in root.rglob("*.rs"):
                content = path.read_text(encoding="utf-8")
                self.assertNotIn(
                    "#![allow(missing_docs)]",
                    content,
                    msg=f"{path} should not suppress missing_docs in cleaned communication slices",
                )

        for path in guarded_files:
            content = path.read_text(encoding="utf-8")
            self.assertNotIn(
                "#![allow(missing_docs)]",
                content,
                msg=f"{path} should not suppress missing_docs in cleaned communication files",
            )


if __name__ == "__main__":
    unittest.main()
