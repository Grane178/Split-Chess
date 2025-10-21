
pub mod web;

// 为WASM目标导出web模块
#[cfg(target_arch = "wasm32")]
pub use web::*;
