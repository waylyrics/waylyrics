#[cfg(unix)]
pub use unix::*;
#[cfg(unix)]
mod unix;

#[cfg(target_os = "windows")]
pub use windows::*;
#[cfg(target_os = "windows")]
mod windows;
