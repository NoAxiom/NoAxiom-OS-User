#![no_std]
#![no_main]

extern crate alloc;

use libd::{
    lib_basepath::BUSYBOX,
    println,
    syscall::{utils::OpenFlags, *},
};

/// testpoints for all arch and lib
/// rv.musl / rv.glibc / la.musl / la.glibc
const TEST_POINTS: &[(&str, bool, bool, bool, bool)] = &[
    //                arch: riscv64      | loongarch64
    //                lib:  musl | glibc | musl | glibc
    ("./basic_testcode.sh\0", true, true, true, true),
    ("./busybox_testcode.sh\0", true, true, true, true),
    ("./lua_testcode.sh\0", true, true, true, true),
    ("./iozone_testcode.sh\0", true, true, true, true),
    ("./libcbench_testcode.sh\0", true, true, true, true),
    ("./libctest_testcode.sh\0", true, true, true, true),
    ("./lmbench_testcode.sh\0", true, true, true, true),
    ("./iperf_testcode.sh\0", true, true, true, true),
    ("./netperf_testcode.sh\0", true, true, true, true),
    ("./cyclictest_testcode.sh\0", true, true, false, true),
    // "./ltp_testcode.sh\0", // not supported
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
                "PATH=/:/bin\0".as_ptr(),
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
        run_sh("/musl/busybox --install /bin\0");
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
        run_sh("/musl/busybox --install /bin\0");
        println!("[loongarch64] init glibc and musl libraries");
    }
}

fn run_tests() {
    #[cfg(target_arch = "riscv64")]
    {
        for &(test, rvm, rvg, _lam, _lag) in TEST_POINTS {
            if rvm {
                chdir("/musl\0");
                run_sh(test);
            }
            if rvg {
                chdir("/glibc\0");
                run_sh(test);
            }
        }
    }
    #[cfg(target_arch = "loongarch64")]
    {
        for &(test, _rvm, _rvg, lam, lag) in TEST_POINTS {
            if lam {
                chdir("/musl\0");
                run_sh(test);
            }
            if lag {
                chdir("/glibc\0");
                run_sh(test);
            }
        }
    }
}

#[no_mangle]
fn main() -> i32 {
    println!("[init_proc] Hello, NoAxiom!");
    init();
    run_tests();
    println!("[init_proc] Test finished!");
    0
}
