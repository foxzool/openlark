#!/usr/bin/env bash
# 检查非测试代码中没有 #[allow(dead_code)] / #![allow(dead_code)]（issue #267 防复发）
# 被 justfile (just no-dead-code-allows) 与 .github/workflows/ci.yml (lint job) 调用。
set -euo pipefail

# 已知的 inner-attribute（crate/mod 级）dead_code 抑制 —— 掩盖 ~109 处死代码
# （hr 85 / core 12 / mail 7 等）。清理工作 tracked in #277；清理完成后移除本排除项。
KNOWN_INNER_DEBT=$(cat <<'EOF'
crates/openlark-mail/src/lib.rs
crates/openlark-bot/src/lib.rs
crates/openlark-hr/src/endpoints/mod.rs
crates/openlark-docs/src/ccm/explorer/explorer/mod.rs
crates/openlark-core/src/observability.rs
crates/openlark-core/src/query_params.rs
crates/openlark-core/src/request_builder/header_builder.rs
EOF
)

# 匹配独立成行的 #[allow(dead_code)] 与 #![allow(dead_code)]（含缩进）。
# 排除 tests/ 目录与 #[cfg(test)] 测试 mod；排除上方已登记的 #277 known-debt 文件。
# #[expect(dead_code)] 不被匹配（它是受控的预期死代码，非 blanket 抑制）。
hits=$(grep -rn --include='*.rs' -E '^[[:space:]]*#!?\[allow\(dead_code\)\][[:space:]]*$' crates/ src/ \
  | grep -v '/tests/' \
  | grep -v '#\[cfg(test)\]' \
  | grep -vFf <(printf '%s\n' "$KNOWN_INNER_DEBT") \
  || true)

if [ -n "$hits" ]; then
  echo "❌ 发现非测试代码中的 #[allow(dead_code)]（应删除或改 _ 前缀 / #[expect(dead_code)]）：" >&2
  echo "$hits" >&2
  exit 1
fi

echo "✅ 无非测试 #[allow(dead_code)] 残留（#277 的 7 处 inner-attribute 已登记为例外）"
