#![no_std]
#![no_main]

extern crate alloc;

use alloc::format;

use libd::{
    lib_basepath::ROOT_NAME,
    println,
    syscall::{execve, fork, wait},
};

#[no_mangle]
fn main() -> i32 {
    if fork() == 0 {
        let path = format!("PATH=/\0");
        let ld_lib_path = format!("LD_LIBRARY_PATH=/\0");
        let res = execve(
            "/glibc/busybox\0",
            &[
                "busybox\0".as_ptr(),
                "sh\0".as_ptr(),
                core::ptr::null::<u8>(),
            ],
            &[
                path.as_str().as_ptr(),
                ld_lib_path.as_str().as_ptr(),
                "TERM=screen\0".as_ptr(),
                core::ptr::null::<u8>(),
            ],
        );
        // exec failed, try execute under root
        if res < 0 {
            execve(
                "/busybox\0",
                &[
                    "busybox\0".as_ptr(),
                    "sh\0".as_ptr(),
                    core::ptr::null::<u8>(),
                ],
                &[
                    path.as_str().as_ptr(),
                    ld_lib_path.as_str().as_ptr(),
                    "TERM=screen\0".as_ptr(),
                    core::ptr::null::<u8>(),
                ],
            );
            unreachable!();
        }
    } else {
        loop {
            let mut exit_code: usize = 0;
            let tid = wait(-1, &mut exit_code);
            if tid < 0 {
                break;
            }
            println!("wait tid: {}, exit_code: {}", tid, exit_code);
        }
    }
    0
}
