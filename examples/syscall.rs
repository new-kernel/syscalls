use novusk_syscalls::syscall::SysCall;

mod write;
use write::sys_write;

fn main() {
    println!("Syscall example");

    let new_syscall = SysCall::new("sys_write", 0, sys_write);

    for b in b"Hello, World! From syscall example!\n" {
        unsafe { new_syscall.call(*b, 0); }
    }
}
