// 启用所有的 Clippy lint 和 Rust 2018 特性的警告
#![warn(clippy::all, rust_2018_idioms)]

// 引入 app 模块
mod app;
// 从 app 模块中公开 CoproApp
pub use app::CoproApp;