#![no_std]
#![no_main]

extern crate alloc;

use libd::{
    lib_basepath::BUSYBOX,
    println,
    syscall::{execve, fork, wait},
};

fn run_sh(cmd: &str) {
    let pid = fork();
    if pid == 0 {
        // default use musl busybox
        execve(
            BUSYBOX,
            &[
                "busybox\0".as_ptr(),
                "sh\0".as_ptr(),
                "-c\0".as_ptr(),
                cmd.as_ptr(),
                core::ptr::null::<u8>(),
            ],
            &[
                "PATH=/bin\0".as_ptr(),
                "TERM=screen\0".as_ptr(),
                core::ptr::null::<u8>(),
            ],
        );
    } else if pid > 0 {
        let mut exit_code: usize = 0;
        wait(pid, &mut exit_code);
    } else {
        println!("fork failed, ret: {}", pid);
    }
}

#[no_mangle]
fn main() -> i32 {
    let pid = fork();
    if pid == 0 {
        execve(
            "/bin/busybox\0",
            &[
                "busybox\0".as_ptr(),
                "sh\0".as_ptr(),
                core::ptr::null::<u8>(),
            ],
            &[
                "PATH=/bin:/usr/bin\0".as_ptr(),
                "TERM=screen\0".as_ptr(),
                core::ptr::null::<u8>(),
            ],
        );
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
