pub struct SysCall {
    pub name: &'static str,
    pub number: u32,
    pub sys_fun: unsafe extern "C" fn(u8, u8) -> u8,
}

impl SysCall {
    pub fn new(sys_name: &'static str, sys_number: u32, sys_fun: unsafe extern "C" fn(u8, u8) -> u8) -> Self {
        return SysCall {
            name: sys_name,
            number: sys_number,
            sys_fun: sys_fun,
        };
    }

    pub unsafe fn call(&self, sys_arg1: u8, sys_arg2: u8) -> u8 {
        return (self.sys_fun)(sys_arg1, sys_arg2);
    }
}
