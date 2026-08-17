# HR 域 typed coverage 优先级清单

## 当前结论

基于当前覆盖率报告（#571 selective P2 之后）：

- `python3 tools/validate_apis.py --crate openlark-hr`

当前 `openlark-hr` 状态为：

- API 总数：583
- 已实现：583（含 path noise 匹配）
- 真缺口（未实现）：0
- 完成率：100.0%
- 路径噪音匹配：25（全部为 OKR v2 layout）

历史「CoreHR 最后 5 个缺口」与「OKR v2 25 个缺口」在 denoise 后均已清零：

| 历史缺口族 | 终端结论 | 证据 |
|------------|----------|------|
| CoreHR 时间轴 / 流程（曾记 5 项） | 已在磁盘有 strict 实现（0.19 后） | 当前 `true_missing=0` |
| OKR v2（25 项） | **path noise**：CSV resource `okr.*` vs 磁盘 `okr/okr/v2/*` | `implementation_path_rewrites` |

## 默认频率顺序

如果未来 HR 域再次出现**真缺口**，默认按以下顺序补齐：

1. **组织与时间轴查询**
   - company / location / department / employee timeline
2. **员工状态变更**
   - probation、入转调离等高频变更
3. **流程模板与流程发起**
   - query_flow_data_template / process_start
4. **OKR / 绩效尾部能力**（仅当 CSV 新增且无 on-disk leaf）
5. **其余尾部能力**

## 说明

- 当前 HR 域的关键点不是「还有很多没做」，而是**保持 denoise 分类可信**，避免把 layout 噪音重新记成实现工单。
- 详见 `docs/typed-api-coverage.md` §2.3（#571）。
