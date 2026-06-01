use super::Client;
use crate::{Result, error::with_operation_context};

/// 客户端错误处理扩展特征
pub trait ClientErrorHandling {
    /// 处理错误并添加客户端上下文
    fn handle_error<T>(&self, result: Result<T>, operation: &str) -> Result<T>;
    /// 处理异步错误并添加客户端上下文
    fn handle_async_error<'a, T, F>(
        &'a self,
        f: F,
        operation: &'a str,
    ) -> impl std::future::Future<Output = Result<T>> + Send + 'a
    where
        T: Send + 'a,
        F: std::future::Future<Output = Result<T>> + Send + 'a;
}

impl ClientErrorHandling for Client {
    fn handle_error<T>(&self, result: Result<T>, operation: &str) -> Result<T> {
        with_operation_context(result, operation, "Client")
    }

    async fn handle_async_error<'a, T, F>(&'a self, f: F, operation: &'a str) -> Result<T>
    where
        T: Send + 'a,
        F: std::future::Future<Output = Result<T>> + Send + 'a,
    {
        let result = f.await;
        with_operation_context(result, operation, "Client")
    }
}
