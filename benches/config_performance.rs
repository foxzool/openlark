//! 配置相关的轻量性能基准
//!
//! 说明：
//! - 该基准用于 CI 中的 `cargo bench --bench config_performance`，避免 “无 bench 目标” 导致工作流失败
//! - 选择 `openlark_core::config::Config` 的构建与克隆路径，确保不依赖外部环境变量与网络
#![allow(missing_docs)]

use criterion::{Criterion, black_box, criterion_group, criterion_main};
use openlark_core::config::Config;

fn bench_config_build(c: &mut Criterion) {
    c.bench_function("config/build_default", |b| {
        b.iter(|| {
            let cfg = Config::builder()
                .app_id("bench_app_id")
                .app_secret("bench_app_secret")
                .build();
            black_box(cfg)
        })
    });
}

fn bench_config_clone_arc(c: &mut Criterion) {
    let cfg = Config::builder()
        .app_id("bench_app_id")
        .app_secret("bench_app_secret")
        .build();

    c.bench_function("config/clone_arc", |b| {
        b.iter(|| {
            let c1 = cfg.clone();
            let c2 = cfg.clone();
            let c3 = cfg.clone();
            let c4 = cfg.clone();
            black_box((c1, c2, c3, c4))
        })
    });
}

criterion_group!(benches, bench_config_build, bench_config_clone_arc);
criterion_main!(benches);
