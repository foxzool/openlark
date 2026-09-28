"""Checked-in `api_list_export.csv` row counts.

Dated catalog sync tests pin identities from their issue date. Global size
asserts use these constants so Seams docstrings keep historical numbers.
"""

from __future__ import annotations

# Data rows in root api_list_export.csv (header excluded).
CURRENT_CATALOG_ROW_COUNT = 1751

# Rows with meta.Version != old (crates.md / update_crates_md non-old total).
CURRENT_CATALOG_NON_OLD_ROW_COUNT = 1639
