/// This file is only store some configurations, no func.
///
/// And this file's lines is very less, we'll add some
///
/// Options later.

/// The version, a &str.
pub const VERSION: &str = "v0.9.2 (Build 9192)";

/// The root directory.
/// 
/// It'll be different in each platforms.
#[cfg(target_os = "linux")]
pub const ROOTDIR: &str = "/";

#[cfg(target_os = "windows")]
pub const ROOTDIR: &str = "C:\\mcospkg";
