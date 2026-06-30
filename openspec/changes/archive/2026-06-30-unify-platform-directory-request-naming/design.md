## Context
#271 platform directory 批。模式 5 批验证。21 个全无 trait impl/re-export/service → 最简。

## Decisions
1. Builder→RequestBuilder（#271 既定方向）
2. #[deprecated] alias（放 #[cfg(test)] 前）
3. 无 re-export/service → 仅定义文件改
4. CollaborationTenantListBuilder 不撞名（不同模块路径，platform 根不 re-export）

## Risks
alias 放 #[cfg(test)] 前；push 前 cargo fmt --check。
