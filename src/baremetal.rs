pub struct Aarch64SysCalls;

extern "C" {
    fn write(byte: &[u8]);
}

impl Aarch64SysCalls {
    pub unsafe fn sys_write(&mut self, bytes: &[u8]) {
        write(bytes);
    }
}
