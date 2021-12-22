use novusk_syscalls::syscall::SysCall;
use novusk_syscalls::table::SysCallTable;

mod write;
use write::sys_write;

// Get the table defining it in an extern block
#[no_mangle]
static mut SYSCALL_TABLE: SysCallTable = SysCallTable::new();

fn main() {
    println!("Syscall Table example");

    unsafe {
        SYSCALL_TABLE.start_init();
        SYSCALL_TABLE.set_name("Novusk System call Example Table");

        println!("Adding write system call...");
        SYSCALL_TABLE.add_syscall(SysCall::new("sys_write", 1, sys_write));

        println!("\nUsing \"{}\" as a system call table", SYSCALL_TABLE.systable_name);
        println!("Empty: {}", SYSCALL_TABLE.make_call(999, 0, 0));
        println!("\nWrite: {}", SYSCALL_TABLE.make_call(1, b'A', 0));
    }
}
