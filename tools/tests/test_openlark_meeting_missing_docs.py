import unittest
from pathlib import Path


class OpenlarkMeetingMissingDocsTests(unittest.TestCase):
    def test_openlark_meeting_cleaned_slices_do_not_suppress_missing_docs(self):
        guarded_roots = [
            Path("crates/openlark-meeting/src/meeting_room"),
            Path("crates/openlark-meeting/src/calendar/calendar/v4/exchange_binding"),
            Path("crates/openlark-meeting/src/calendar/calendar/v4/freebusy"),
            Path("crates/openlark-meeting/src/calendar/calendar/v4/timeoff_event"),
            Path("crates/openlark-meeting/src/calendar/calendar/v4/calendar/acl"),
            Path("crates/openlark-meeting/src/calendar/calendar/v4/calendar/event/meeting_chat"),
            Path("crates/openlark-meeting/src/calendar/calendar/v4/calendar/event/meeting_minute"),
            Path("crates/openlark-meeting/src/calendar/calendar/v4/calendar/event/attendee/chat_member"),
            Path("crates/openlark-meeting/src/vc/vc/v1/reserve_config/admin"),
            Path("crates/openlark-meeting/src/vc/vc/v1/reserve_config/disable_inform"),
            Path("crates/openlark-meeting/src/vc/vc/v1/reserve_config/form"),
            Path("crates/openlark-meeting/src/vc/vc/v1/scope_config"),
            Path("crates/openlark-meeting/src/vc/vc/v1/report"),
            Path("crates/openlark-meeting/src/vc/vc/v1/room_config"),
            Path("crates/openlark-meeting/src/vc/vc/v1/scope_config"),
            Path("crates/openlark-meeting/src/vc/vc/v1/report"),
            Path("crates/openlark-meeting/src/vc/vc/v1/export"),
            Path("crates/openlark-meeting/src/vc/vc/v1/meeting/recording"),
            Path("crates/openlark-meeting/src/vc/vc/v1/export"),
            Path("crates/openlark-meeting/src/vc/vc/v1/resource_reservation_list"),
            Path("crates/openlark-meeting/src/vc/vc/v1/participant_list"),
            Path("crates/openlark-meeting/src/vc/vc/v1/participant_quality_list"),
            Path("crates/openlark-meeting/src/vc/vc/v1/meeting_list"),
            Path("crates/openlark-meeting/src/vc/vc/v1/room"),
            Path("crates/openlark-meeting/src/vc/vc/v1/room_level"),
        ]

        guarded_files = [
            Path("crates/openlark-meeting/src/calendar/calendar/v4/setting/generate_caldav_conf.rs"),
            Path("crates/openlark-meeting/src/calendar/calendar/v4/calendar/models.rs"),
            Path("crates/openlark-meeting/src/calendar/calendar/v4/responses_new.rs"),
            Path("crates/openlark-meeting/src/vc/vc/v1/reserve/get_active_meeting.rs"),
            Path("crates/openlark-meeting/src/vc/vc/v1/responses.rs"),
        ]

        for root in guarded_roots:
            for path in root.rglob("*.rs"):
                content = path.read_text(encoding="utf-8")
                self.assertNotIn(
                    "#![allow(missing_docs)]",
                    content,
                    msg=f"{path} should not suppress missing_docs in cleaned meeting slices",
                )

        for path in guarded_files:
            content = path.read_text(encoding="utf-8")
            self.assertNotIn(
                "#![allow(missing_docs)]",
                content,
                msg=f"{path} should not suppress missing_docs in cleaned meeting files",
            )


if __name__ == "__main__":
    unittest.main()
