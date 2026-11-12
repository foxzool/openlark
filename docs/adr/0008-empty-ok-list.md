# Empty-ok List 用单独的校验声明

必填列表能否为空，不再写在中文注释里，也不改 `validate_required!` / `validate_required_list!` 的现有含义。`validate_allow_empty_list!` 是这条事实的唯一声明：无上限时运行时不拒绝任何长度，带上限时只拒绝超长。快速检测和 issue41 guardrail 只认这次调用。

`validate_required!` 对 `Vec` 仍表示空即非法。`validate_required_list!` 仍表示非空且不超过上限。Empty-ok List 单独成一条，因为把空合法塞进前两条会改变已经在用的调用。

不选注释短语解析。检测器曾经用「允许空数组」等中文说法豁免缺校验，运行时却仍可能拒绝空列表。

不选给 `validate_required_list!` 加模式参数。那条宏的调用方已经依赖「空即非法」。

无上限形式不带错误文案，因为没有失败。带上限的错误只描述超长。元素是否空白、是否重复，不在这条宏里。

这次不让 codegen 发射它：合约 IR 没有「允许空数组」或列表上限。不把这条事实放进 `rust_source` 或 Official Document Evidence。`skip_serializing_if = Vec::is_empty` 仍表示空列表不出现在线上，不是 Empty-ok List。注释里写了上限、但运行时还没检查的，这次也不新增客户端拒绝。
