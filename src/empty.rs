// It's very unlikely that any architecture will have more than a hundred syscall nums
pub const SYS_EMPTY: u32 = 999;

#[no_mangle]
pub unsafe extern "C" fn sys_empty(sys_arg1: u8, sys_arg2: u8, sys_arg3: u8) -> u8 {
    12
}
