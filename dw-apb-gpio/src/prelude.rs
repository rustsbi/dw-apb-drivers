//! Common GPIO traits for use with `use dw_apb_gpio::prelude::*;`.

pub use embedded_hal::digital::{
    ErrorType as _, InputPin as _, OutputPin as _, StatefulOutputPin as _,
};
