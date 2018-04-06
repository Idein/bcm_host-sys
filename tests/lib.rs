extern crate bcm_host_sys;
extern crate env_logger;
#[macro_use]
extern crate log;

use std::mem;
use std::ptr;
use std::os::raw as ctype;
use std::ffi::CStr;

use bcm_host_sys as sys;

#[test]
fn strcpy_offset() {
    let mut buffer: [ctype::c_char; 32] = [b'\0'; 32];
    let ptr = buffer.as_mut_ptr();
    let mut offset = 0;
    let from = |lit| CStr::from_bytes_with_nul(lit).unwrap().as_ptr();
    unsafe {
        offset = sys::vcos_safe_strcpy(ptr, from(b"get_mem\0"), buffer.len(), offset);
        assert_eq!(7, offset);
        offset = sys::vcos_safe_strcpy(ptr, from(b" \0"), buffer.len(), offset);
        assert_eq!(8, offset);
        offset = sys::vcos_safe_strcpy(ptr, from(b"gpu\0"), buffer.len(), offset);
        assert_eq!(11, offset);
    }
}

/// # Summary
/// Getting size of gpu memory.
/// This test works as same as `vcgencmd get_mem gpu`.
#[test]
fn get_mem_gpu() {
    env_logger::init().ok();
    unsafe {
        sys::vcos_init();
        let mut vchi_instance: sys::VCHI_INSTANCE_T = mem::uninitialized();
        sys::vchi_initialise(&mut vchi_instance);
        sys::vchi_connect(ptr::null_mut(), 0, vchi_instance);
        let mut vchi_connection: *mut sys::VCHI_CONNECTION_T = ptr::null_mut();
        sys::vc_vchi_gencmd_init(vchi_instance, &mut vchi_connection, 1);
        debug!("sys::vc_vchi_gencmd_init(vchi_instance, &mut vchi_connection, 1);");

        let mut buffer: [ctype::c_char; 1024] = [0; 1024];
        let buffer_ptr = buffer.as_mut_ptr();
        let mut offset = 0;
        offset = sys::vcos_safe_strcpy(buffer_ptr, b"get_mem\0".as_ptr(), buffer.len(), offset);
        debug!(
            "sys::vcos_safe_strcpy(buffer_ptr, b\"get_mem\\0\".as_ptr(), {}, {});",
            buffer.len(),
            offset
        );
        offset = sys::vcos_safe_strcpy(buffer_ptr, b" \0".as_ptr(), buffer.len(), offset);
        debug!(
            "sys::vcos_safe_strcpy(buffer_ptr, b\" \\0\".as_ptr()      , buffer.len(), {});",
            offset
        );
        offset = sys::vcos_safe_strcpy(buffer_ptr, b"gpu\0".as_ptr(), buffer.len(), offset);
        debug!(
            "sys::vcos_safe_strcpy(buffer_ptr, b\"gpu\\0\".as_ptr()    , buffer.len(), {});",
            offset
        );

        sys::vc_gencmd_send("%s".as_ptr(), buffer.as_ptr());
        debug!("sys::vc_gencmd_send(\"%s\".as_ptr(), buffer);");
        sys::vc_gencmd_read_response(buffer.as_mut_ptr(), buffer.len() as i32);
        debug!("sys::vc_gencmd_read_response(buffer.as_mut_ptr(), buffer.len() as i32);");

        let cstr = CStr::from_ptr(buffer.as_ptr());
        debug!("let cstr = CStr::from_ptr(buffer.as_ptr());");
        let str = cstr.to_str().unwrap();
        debug!("{}\n", str);
        assert!(str.ends_with("M"));

        sys::vc_gencmd_stop();
        debug!("sys::vc_gencmd_stop();");
        sys::vchi_disconnect(vchi_instance);
        debug!("sys::vchi_disconnect(vchi_instance);");
    }
}
