//! 测试工具（仅单元测试使用）
//!
//! 目标：
//! - 避免并行测试读写环境变量导致的相互污染与随机失败
//! - 用 RAII 保证环境变量在测试结束后可恢复

// 注意：#[cfg(test)] 已在 lib.rs:230 定义，此处移除重复属性
use std::sync::{Mutex, OnceLock};

static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

/// 在闭包执行期间临时设置/移除一组环境变量，并在结束后自动恢复。
///
/// - `vars`: `(key, value)`；当 `value=None` 时表示移除该环境变量。
pub(crate) fn with_env_vars<R>(vars: &[(&str, Option<&str>)], f: impl FnOnce() -> R) -> R {
    let lock = ENV_LOCK.get_or_init(|| Mutex::new(()));
    let _guard = lock.lock().unwrap_or_else(|e| e.into_inner());

    struct EnvRestore {
        saved: Vec<(String, Option<String>)>,
    }

    impl Drop for EnvRestore {
        fn drop(&mut self) {
            for (key, value) in self.saved.drain(..) {
                // Test helpers serialize env access behind ENV_LOCK.
                match value {
                    Some(v) => unsafe { std::env::set_var(key, v) },
                    None => unsafe { std::env::remove_var(key) },
                }
            }
        }
    }

    let saved = vars
        .iter()
        .map(|(key, _)| ((*key).to_string(), std::env::var(key).ok()))
        .collect::<Vec<_>>();

    for (key, value) in vars {
        // Test helpers serialize env access behind ENV_LOCK.
        match value {
            Some(v) => unsafe { std::env::set_var(key, v) },
            None => unsafe { std::env::remove_var(key) },
        }
    }

    // 注意：保证在释放 ENV_LOCK 之前先恢复环境变量（drop 顺序：restore -> _guard）。
    let _restore = EnvRestore { saved };
    f()
}
