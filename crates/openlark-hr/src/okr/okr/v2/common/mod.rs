//! okr/v2 跨叶共享 domain struct。
//!
//! 收纳在多个叶子（如 objective/get、cycle/objective/list 等）中
//! 重复出现的同一飞书实体的 typed 表示，避免逐字重复定义（Shotgun Surgery）。

pub mod models;
