## Context
#271 最后一批（app_engine 51）。模式 6 批验证。全最简（无 re-export/service/trait impl）。

## Decisions
1. Builder→RequestBuilder（#271 既定方向）
2. #[deprecated] alias（放 #[cfg(test)] 前）
3. 无 re-export/service → 仅定义文件改

## Risks
alias 放 #[cfg(test)] 前；push 前 cargo fmt --check。51 个量大但机械化脚本统一处理。
