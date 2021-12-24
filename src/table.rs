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

    pub fn make_call(&mut self, sys_num: u32, sys_arg1: u8, sys_arg2: u8, sys_arg3: u8) -> u8 {
        let mut syscalls = self.syscalls.as_mut().unwrap();

        let mut ret = 0;

        for n in 0..syscalls.len() {
            if syscalls[n].number == sys_num {
                unsafe { ret = syscalls[n].call(sys_arg1, sys_arg2, sys_arg3); }
                return ret;
            }
        }

        return 255;
    }

    pub fn get_table_info(&self) -> (&str, usize) {
        return (self.systable_name, self.syscalls.as_ref().unwrap().len());
    }

    pub fn get_call_info(&self, num: u32) -> (&str, u32, unsafe extern "C" fn(u8, u8, u8) -> u8) {
        let syscalls = self.syscalls.as_ref().unwrap();

        for n in 0..syscalls.len() {
            if syscalls[n].number == num {
                return (syscalls[n].name, syscalls[n].number, syscalls[n].sys_fun);
            }
        }

        return ("Not Found", 999, sys_empty);
    }
}
