#![no_std]
#![no_main]

extern crate alloc;

use libd::{
    lib_basepath::BUSYBOX,
    println,
    syscall::{utils::OpenFlags, *},
};

// tested in one hart
// "./unixbench_testcode.sh\0" is not in contest
const TEST_LIST: &[&str] = &[
    // PASSED in both glibc and musl with both riscv64 and loongarch64
    // "./basic_testcode.sh\0",
    // "./busybox_testcode.sh\0",
    // "./lua_testcode.sh\0",
    // "./iperf_testcode.sh\0", // remain process UNEXITED
    // "./netperf_testcode.sh\0", // remain process UNEXITED

    // UNTESTED
    // "./lmbench_testcode.sh\0",
    // "./ltp_testcode.sh\0",

    // PASS: musl-rv musl-la
    // FAIL: glibc-rv glibc-la
    // "./iozone_testcode.sh\0", // page_table fault
    // "./libcbench_testcode.sh\0",
    /*
    [ERROR, HART0, TID6] invalid syscall id: 435
    [ERROR, HART0, TID6] [175981693] kernel/src/syscall/process.rs:301 Errno: [EPERM] Operation not permitted
    */

    // PASS:
    // FAIL: glibc-rv glibc-la musl-rv musl-la
    // "./cyclictest_testcode.sh\0", // miss some files, fs error
    // "./libctest_testcode.sh\0",
    /*
    (glibc errors more)
    socket error
    statx error

    [ERROR, HART0, TID6] invalid syscall id: 435
    [ERROR, HART0, TID6] [2874513002] kernel/src/syscall/process.rs:301 Errno: [EPERM] Operation not permitted
     */
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

fn copy_file(src: &str, dst: &str) {
    let src_fd = open(src, OpenFlags::O_RDONLY);
    if src_fd < 0 {
        unreachable!("Failed to open source file: {}", src);
    }

    let dst_fd = open(
        dst,
        OpenFlags::O_WRONLY | OpenFlags::O_CREATE | OpenFlags::O_TRUNC,
    );
    if dst_fd < 0 {
        unreachable!("Failed to open destination file: {}", dst);
    }

    let mut buf = [0u8; 4096];
    loop {
        let bytes_read = read(src_fd as usize, &mut buf);
        if bytes_read == 0 {
            break;
        } else if bytes_read < 0 {
            unreachable!("Failed to read from source file: {}", src);
        }
        write(dst_fd as usize, &buf[..bytes_read as usize]);
    }

    close(src_fd as usize);
    close(dst_fd as usize);
}

fn init() {
    #[cfg(target_arch = "riscv64")]
    {
        copy_file(
            "/glibc/lib/ld-linux-riscv64-lp64d.so.1\0",
            "/lib/ld-linux-riscv64-lp64d.so.1\0",
        );
        copy_file(
            "/glibc/lib/ld-linux-riscv64-lp64d.so.1\0",
            "/lib/ld-linux-riscv64-lp64.so.1\0",
        );
        copy_file("/glibc/lib/libc.so\0", "/lib/libc.so.6\0");
        copy_file("/glibc/lib/libm.so\0", "/lib/libm.so.6\0");
        copy_file("/musl/lib/libc.so\0", "/lib/ld-musl-riscv64-sf.so.1\0");
        println!("[riscv64] init glibc and musl libraries");
    }
    #[cfg(target_arch = "loongarch64")]
    {
        copy_file(
            "/glibc/lib/ld-linux-loongarch-lp64d.so.1\0",
            "/lib64/ld-linux-loongarch-lp64d.so.1\0",
        );
        copy_file("/glibc/lib/libc.so.6\0", "/lib64/libc.so.6\0");
        copy_file("/glibc/lib/libm.so.6\0", "/lib64/libm.so.6\0");
        copy_file("/glibc/lib/libc.so.6\0", "/usr/lib64/libc.so.6\0");
        copy_file("/glibc/lib/libm.so.6\0", "/usr/lib64/libm.so.6\0");
        copy_file(
            "/musl/lib/libc.so\0",
            "/lib/ld-musl-loongarch64-lp64d.so.1\0",
        );
        copy_file(
            "/musl/lib/libc.so\0",
            "/lib64/ld-musl-loongarch-lp64d.so.1\0",
        );
        println!("[loongarch64] init glibc and musl libraries");
    }
}

#[no_mangle]
fn main() -> i32 {
    init();
    chdir("/musl\0");
    for test in TEST_LIST {
        run_sh(test);
    }

    chdir("/glibc\0");
    for test in TEST_LIST {
        run_sh(test);
    }
    0
}
