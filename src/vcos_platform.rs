use std::mem;
use std::ptr;
use super::*;

pub fn vcos_sleep(ms: u32) {
    unsafe {
        let mut ts: timespec = mem::uninitialized();
        ts.tv_sec = (ms / 1000) as i32;
        ts.tv_nsec = (ms % 1000 * 1000000) as i32;
        nanosleep(&ts, ptr::null_mut());
    }
}
