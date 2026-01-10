uniffi::setup_scaffolding!("shared");

#[cfg(target_os = "android")]
mod android;
#[cfg(target_os = "android")]
pub use android::*;

#[cfg(target_os = "ohos")]
mod harmony;
#[cfg(target_os = "ohos")]
pub use harmony::*;
