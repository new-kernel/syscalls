#![no_std]

#[macro_use] extern crate alloc;

pub(crate) mod empty;
pub mod syscall;
pub mod table;

pub use syscall::SysCall;
pub use table::SysCallTable;
