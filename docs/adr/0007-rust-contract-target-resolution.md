# Rust Contract Target 由固定映射解析

Catalog Entry 到 Rust 实现文件的关联统一由 `tools/api_coverage.toml` 和组合时建立的不可变 Rust 源文件快照解析，不再允许调用方全仓搜索或按候选顺序取第一个文件。Canonical、alias 与 rewrite 候选权重相同：唯一存在时得到 Rust Contract Target，无归属、无文件及多文件分别得到 `Unmapped`、`Missing` 与 `Ambiguous`；这牺牲了隐式容错，但换来可重复、可审计且不会误指向实现的合约核对。Alias 与 rewrite 仅作为显式的临时 legacy 规则存在，完成 canonical 路径迁移后必须删除。
