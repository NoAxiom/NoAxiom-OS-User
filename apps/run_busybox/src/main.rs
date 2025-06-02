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
    let pid = fork();
    if pid == 0 {
        let res = execve(
            "/glibc/busybox\0",
            &[
                "busybox\0".as_ptr(),
                "sh\0".as_ptr(),
                core::ptr::null::<u8>(),
            ],
            &[
                "PATH=/glibc\0".as_ptr(),
                "LD_LIBRARY_PATH=/glibc\0".as_ptr(),
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
                    "PATH=/\0".as_ptr(),
                    "LD_LIBRARY_PATH=/\0".as_ptr(),
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
            } else if tid == pid {
                println!("[init_proc] busybox exited, exit_code: {}", exit_code);
                break;
            } else {
                // println!("[init_proc] wait tid: {}, exit_code: {}", tid,
                // exit_code);
            }
        }
    }
    0
}
