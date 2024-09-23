mod app;
pub use app::ExampleApp;

#[cfg(target_family = "wasm")]
mod web;
