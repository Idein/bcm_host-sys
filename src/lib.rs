#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

use std::mem;
use std::ptr;

/// sleep a while
///
/// /opt/vc/include/vcos_platform.h
pub fn vcos_sleep(ms: u32) {
    unsafe {
        let mut ts: timespec = mem::uninitialized();
        ts.tv_sec = (ms / 1000) as i32;
        ts.tv_nsec = (ms % 1000 * 1000000) as i32;
        nanosleep(&ts, ptr::null_mut());
    }
}
