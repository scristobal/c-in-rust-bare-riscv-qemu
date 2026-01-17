#![cfg_attr(target_os = "none", no_std)]

#[cfg(feature = "qemu")]
pub mod qemu;

pub mod some_c;
