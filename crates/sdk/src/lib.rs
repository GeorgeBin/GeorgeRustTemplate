pub mod ntp {
    pub use template_core::ntp::*;
}

pub mod model {
    pub use template_model::ntp::{NtpRequest, NtpResponse};
}

pub use template_runtime::RuntimeContext;
