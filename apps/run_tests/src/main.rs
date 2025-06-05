#![no_std]
#![no_main]

extern crate alloc;

use libd::{lib_basepath::BUSYBOX, println, syscall::*};

const TEST_LIST: &[&str] = &[
    // "./basic_testcode.sh\0",
    // "./busybox_testcode.sh\0",
    // "./lua_testcode.sh\0",
    "./iozone_testcode.sh\0",
    // "./cyclictest_testcode.sh\0",
];

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
                "PATH=/\0".as_ptr(),
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
    // chdir("/musl\0");
    // for test in TEST_LIST {
    //     run_sh(test);
    // }

    chdir("/glibc\0");
    for test in TEST_LIST {
        run_sh(test);
    }
    0
}
