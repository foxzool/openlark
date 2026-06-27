#!/usr/bin/env bash
# 检查非测试代码中没有 #[allow(dead_code)]（issue #267 防复发）
# 被 justfile (just no-dead-code-allows) 与 .github/workflows/ci.yml (lint job) 调用。
set -euo pipefail

# 匹配独立成行的 #[allow(dead_code)]（含缩进），排除 tests/ 目录与 #[cfg(test)] 测试 mod。
# 注：纯 grep 无法精确判断某行是否在 #[cfg(test)] mod 块内（跨行），
# 故用 grep -v 排除紧邻 cfg(test) 的 allow（罕见）。当前仓库基线为绿；
# 若未来测试 mod 内出现 allow 误报，再迭代为脚本级块判断。
hits=$(grep -rn --include='*.rs' -E '^[[:space:]]*#\[allow\(dead_code\)\][[:space:]]*$' crates/ src/ \
  | grep -v '/tests/' \
  | grep -v '#\[cfg(test)\]' \
  || true)

if [ -n "$hits" ]; then
  echo "❌ 发现非测试代码中的 #[allow(dead_code)]（应删除或改 _ 前缀 / #[expect(dead_code)]）：" >&2
  echo "$hits" >&2
  exit 1
fi

echo "✅ 无非测试 #[allow(dead_code)] 残留"
