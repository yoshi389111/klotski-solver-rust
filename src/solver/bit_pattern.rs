#[cfg(feature = "impl_u128")]
mod impl_u128;

#[cfg(feature = "impl_u16x5")]
mod impl_u16x5;

#[cfg(feature = "impl_u16x5")]
pub use impl_u16x5::*;

#[cfg(feature = "impl_u128")]
pub use impl_u128::*;
