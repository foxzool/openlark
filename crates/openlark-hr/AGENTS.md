# openlark-hr Knowledge Base

**Crate**: HR Services  
**APIs**: 583 个（排除 old；`validate_apis.py --crate openlark-hr`） | **Status**: 生产就绪  
**Coverage**: 招聘 + CoreHR + 考勤 + 薪酬

## OVERVIEW

飞书人力资源模块，涵盖招聘管理、CoreHR、考勤、薪酬等完整 HR 能力。目录 API 数量最多的业务模块（583 个）。

## STRUCTURE

```
src/
├── lib.rs                    # 模块入口
├── hire/                     # 招聘管理 (182 APIs)
│   └── v1/                  # Offer/候选人/面试/职位
├── feishu_people/            # 人员 / CoreHR (269 APIs)
├── attendance/               # 考勤管理 (39 APIs)
├── okr/                      # OKR (37 APIs)
├── compensation_management/  # 薪酬方案 (21 APIs)
├── performance/              # 绩效 (21 APIs)
├── payroll/                  # 薪酬管理 (12 APIs)
├── ehr/                      # 电子 HR (2 APIs)
└── common/                   # 共享代码
    └── api_endpoints.rs
```

## WHERE TO LOOK

| Task | Location | Notes |
|------|----------|-------|
| 招聘 API | `src/hire/v1/` | Offer/Candidate/Interview |
| CoreHR / 人员 | `src/feishu_people/` | 部门/人员管理 |
| 考勤 API | `src/attendance/` | 打卡/排班/请假 |
| 薪酬 API | `src/payroll/` | 工资单/薪资组 |
| 端点常量 | `src/common/api_endpoints.rs` | HR 相关常量 |

## CONVENTIONS

### 领域模型
```rust
// 招聘领域
pub struct Offer { offer_id: String, ... }
pub struct Candidate { candidate_id: String, ... }
pub struct Job { job_id: String, ... }

// CoreHR 领域
pub struct Department { department_id: String, ... }
pub struct Employee { employee_id: String, ... }
```

### API 分组
按业务领域组织子模块：
```rust
// src/hire/v1/mod.rs
pub mod offer;
pub mod candidate;
pub mod interview;
pub mod job;
```

## ANTI-PATTERNS

- ❌ 不要把招聘和 CoreHR 逻辑混在一起（已分离）
- ❌ 不要使用旧的 hire 端点常量（检查 common/api_endpoints.rs）
- ❌ 不要忘记处理分页（HR 数据量大）

## NOTES

- **Feature**: 通过 `hr` feature 启用
- **数据敏感性**: HR 数据敏感，注意权限控制
- **分页**: 大量列表接口需要处理分页
