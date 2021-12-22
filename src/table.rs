use alloc::vec::Vec;
use crate::empty::*;
use crate::syscall::SysCall;

pub struct SysCallTable {
    pub systable_name: &'static str,
    pub syscalls: Option<Vec<SysCall>>,
}

impl SysCallTable {
    pub const fn new() -> Self {
        return SysCallTable {
            systable_name: "Novusk System call Table",
            syscalls: None,
        };
    }

    pub fn start_init(&mut self) {
        self.syscalls = Some(vec![SysCall::new("sys_empty", SYS_EMPTY, sys_empty)]);
    }

    pub fn set_name(&mut self, name: &'static str) {
        self.systable_name = name;
    }

    pub fn add_syscall(&mut self, syscall: SysCall) {
        self.syscalls.as_mut().unwrap().push(syscall);
    }

    pub fn make_call(&mut self, sys_num: u32, sys_arg1: u8, sys_arg2: u8) -> u8 {
        let mut syscalls = self.syscalls.as_mut().unwrap();

        let mut ret = 0;

        for n in 0..syscalls.len() {
            if syscalls[n].number == sys_num {
                unsafe { ret = syscalls[n].call(sys_arg1, sys_arg2); }
                break;
            }
        }

        return ret;
    }
}
