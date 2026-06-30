# Tasks — unify-platform-app-engine-request-naming（#271 app_engine 批，最后一批）
## 1. 重命名 + alias
- [x] 1.1 51 定义文件 struct+impl+测试→RequestBuilder；#[cfg(test)] 前加 #[deprecated] alias
## 2. 验证
- [x] 2.1 build --all-features + clippy×3 + test + fmt + grep
## 3. CHANGELOG
- [x] 3.1 CHANGELOG v0.18 breaking 段记录
