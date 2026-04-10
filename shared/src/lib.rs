#![allow(non_snake_case)]

uniffi::setup_scaffolding!("shared");

#[cfg(target_os = "android")]
mod android;
#[cfg(target_os = "android")]
pub use android::*;

// Keep OHOS bindings behind an explicit feature because rustc does not
// recognize `target_os = "ohos"` in stable lint checks.
#[cfg(feature = "ohos")]
mod harmony;
#[cfg(feature = "ohos")]
pub use harmony::*;
