//! Platform abstraction stubs.
//!
//! These are intentionally small and honest.
//! Codex should grow them milestone by milestone.

pub trait ScreenCapturer {
    fn start(&mut self) -> Result<(), String>;
    fn capture_frame(&mut self) -> Result<Vec<u8>, String>;
    fn stop(&mut self) -> Result<(), String>;
}

pub trait InputInjector {
    fn move_pointer(&mut self, x: i32, y: i32) -> Result<(), String>;
    fn mouse_button(&mut self, button: u8, down: bool) -> Result<(), String>;
    fn key_event(&mut self, key_code: u32, down: bool) -> Result<(), String>;
}

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub mod linux;
