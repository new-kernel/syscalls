#![no_std]

#[cfg(feature = "baremetal_aarch64")]
mod baremetal;
#[cfg(feature = "baremetal_aarch64")]
pub use baremetal::Aarch64SysCalls;

#[cfg(feature = "novusk_uefi")]
mod novusk_uefi;
#[cfg(feature = "novusk_uefi")]
pub use novusk_uefi::;
