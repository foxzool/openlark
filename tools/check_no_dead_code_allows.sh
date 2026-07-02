#!/usr/bin/env bash
# 检查非测试代码中没有 #[allow(dead_code)] / #![allow(dead_code)]（issue #267 / #277 防复发）
# 被 justfile (just no-dead-code-allows) 与 .github/workflows/ci.yml (lint job) 调用。
set -euo pipefail

# 匹配独立成行的 #[allow(dead_code)] 与 #![allow(dead_code)]（含缩进）。
# 排除 tests/ 目录与 #[cfg(test)] 测试 mod。
# #[expect(dead_code)] 不被匹配（它是受控的预期死代码，非 blanket 抑制）。
hits=$(grep -rn --include='*.rs' -E '^[[:space:]]*#!?\[allow\(dead_code\)\][[:space:]]*$' crates/ src/ \
  | grep -v '/tests/' \
  | grep -v '#\[cfg(test)\]' \
  || true)

if [ -n "$hits" ]; then
  echo "❌ 发现非测试代码中的 #[allow(dead_code)] / #![allow(dead_code)]（应删除或改 _ 前缀 / #[expect(dead_code)]）：" >&2
  echo "$hits" >&2
  exit 1
fi

echo "✅ 无非测试 #[allow(dead_code)] / #![allow(dead_code)] 残留"
