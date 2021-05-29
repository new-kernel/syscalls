extern crate uefi;
use uefi::proto::console::text::Input;

pub struct NovuskSysCalls;

extern "C" {
    fn write(bytes: &[u8]);
    fn input() -> &mut Input;
    fn reboot() -> !;
    fn shutdown() -> !;
}

#[derive(Copy, Clone, PartialEq)]
pub enum SysCalls {
    Write,
    Reboot,
    Shutdown,
}

impl NovuskSysCalls {
    pub unsafe fn sys_write(&mut self, bytes: &[u8]) {
        write(bytes);
    }

    pub unsafe fn sys_input(&mut self) -> &mut Input {
        return input();
    }

    pub unsafe fn sys_reboot(&mut self) -> ! {
        reboot()
    }

    pub unsafe fn sys_shutdown(&mut self) -> ! {
        shutdown()
    }
}

pub unsafe fn syscall(sysnum: SysCalls, bytes_arg: &[u8], str_arg: &str) {
    let mut syscalls = NovuskSysCalls;
    if sysnum == SysCalls::Write {
        syscalls.sys_write(bytes_arg);
    } else if sysnum == SysCalls::Reboot {
        syscalls.sys_reboot()
    } else if sysnum == SysCalls::Shutdown {
        syscalls.sys_shutdown()
    } else {
        return;
    }
}
