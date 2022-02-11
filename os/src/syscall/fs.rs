use crate::batch::check_current_app_mem;

const FD_STDOUT: usize = 1;

pub fn sys_write(fd: usize, buf: *const u8, len: usize) -> isize {
    match fd {
        FD_STDOUT => {
            if check_current_app_mem(buf as usize, len) {
                let slice = unsafe { core::slice::from_raw_parts(buf, len)};
                let str = core::str::from_utf8(slice).unwrap();
                print!("{}",str);
                len as isize
            } else {
                -1
            }
        },
        _ => {
            error!("Unsupported fd in sys_write!");
            -1
        }
    }
}