#!/usr/bin/env bash
# 检查业务 crate 的 Cargo.toml 不直接声明 reqwest 依赖（issue #270 边界防复发）。
#
# 架构约定：业务 crate 须经 core 的 openlark_core::http::Transport<T>::request()
# 发起 HTTP 请求，不在各自 Cargo.toml 中声明 reqwest、不在源码使用 reqwest 类型。
# 唯一例外白名单（见 ARCHITECTURE.md「Transport HTTP 边界」）：
#   - openlark-core    抽象本体（Transport 定义在此）
#   - openlark-client  客户端装配 + websocket feature
#   - openlark-webhook by-design 性能例外（无鉴权推送器，进程级共享 reqwest::Client 复用连接池，#214）
#
# 被 justfile (just reqwest-boundary) 与 .github/workflows/ci.yml (lint job) 调用。
set -euo pipefail
# 空 glob 不展开成字面量（crates/ 为空时循环体不执行，直接 PASS，避免 stderr 噪音）
shopt -s nullglob

# 例外白名单（精确枚举，不依赖运行时推断）
ALLOW=(openlark-core openlark-client openlark-webhook)

hits=""
for toml in crates/*/Cargo.toml; do
  crate=$(basename "$(dirname "$toml")")
  # 跳过白名单 crate
  for a in "${ALLOW[@]}"; do
    [ "$crate" = "$a" ] && continue 2
  done
  # 业务 crate 的 Cargo.toml 任何 section 出现 reqwest = ... 即违规
  # （依赖声明在 [dependencies] 即属边界泄漏，无需区分 dev/build section）
  if grep -qE '^[[:space:]]*reqwest[[:space:]]*=' "$toml"; then
    hits="${hits}${toml}"$'\n'
  fi
done

if [ -n "$hits" ]; then
  echo "❌ 业务 crate 直接声明了 reqwest 依赖（须经 core Transport 发请求，见 ARCHITECTURE.md）：" >&2
  printf '%s' "$hits" >&2
  exit 1
fi

echo "✅ 业务 crate Cargo.toml 无 reqwest 直接依赖（HTTP 边界由 core Transport 收口）"
