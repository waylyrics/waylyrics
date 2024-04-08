#[cfg(target_os = "unix")]
pub use unix::*;
#[cfg(target_os = "unix")]
mod unix;

#[cfg(target_os = "windows")]
pub use windows::*;
#[cfg(target_os = "windows")]
mod windows;
