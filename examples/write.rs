pub unsafe extern "C" fn sys_write(sys_arg1: u8, sys_arg2: u8) -> u8 {
    print!("{}", sys_arg1 as char);
    return sys_arg2;
}
