#![no_std]
#![no_main]

mod ltp;
#[cfg(target_arch = "loongarch64")]
mod ltp_script_la;
#[cfg(target_arch = "riscv64")]
mod ltp_script_rv;
extern crate alloc;

use libd::{
    ioctl::{
        ioctl_log::{switch_log_off, switch_log_on},
        ioctl_ltp::{switch_into_ltp, switch_outof_ltp},
    },
    lib_basepath::BUSYBOX,
    println,
    syscall::{utils::OpenFlags, *},
};

use crate::ltp::run_ltp;

/// testpoints for all arch and lib
/// rv.musl / rv.glibc / la.musl / la.glibc
const TEST_POINTS: &[(&str, bool, bool, bool, bool)] = &[
    //                arch: riscv64      | loongarch64
    //                lib:  musl | glibc | musl | glibc
    #[cfg(feature = "basic")]
    ("./basic_testcode.sh\0", true, true, true, true),
    #[cfg(feature = "busybox")]
    ("./busybox_testcode.sh\0", true, true, true, true),
    #[cfg(feature = "lua")]
    ("./lua_testcode.sh\0", true, true, true, true),
    #[cfg(feature = "iozone")]
    ("./iozone_testcode.sh\0", true, true, true, true),
    #[cfg(feature = "libcbench")]
    ("./libcbench_testcode.sh\0", true, true, true, true),
    #[cfg(feature = "libctest")]
    ("./libctest_testcode.sh\0", true, true, true, true),
    #[cfg(feature = "iperf")]
    ("./iperf_testcode.sh\0", true, true, true, true),
    #[cfg(feature = "netperf")]
    ("./netperf_testcode.sh\0", true, true, true, true),
    #[cfg(feature = "lmbench")]
    ("./lmbench_testcode.sh\0", true, true, true, true),
    // ("./cyclictest_testcode.sh\0", false, false, false, false),
    // ("./ltp_testcode.sh\0", true, true, true, true),

    // ---------final test points-----------
    #[cfg(feature = "final")]
    ("./interrupts_testcode.sh\0", true, true, true, true),
    #[cfg(feature = "final")]
    ("./copy-file-range_testcode.sh\0", true, true, true, true),
    #[cfg(feature = "final")]
    ("./splice_testcode.sh\0", true, true, true, true),
];

const TEST_LAST: &[(&str, bool, bool, bool, bool)] =
    &[("./cyclictest_testcode.sh\0", false, false, false, false)];

fn run_with_args(app: &str, args: &[*const u8]) {
    let pid = fork();
    if pid == 0 {
        execve(
            app,
            args,
            &[
                "PATH=/bin\0".as_ptr(),
                "TERM=screen\0".as_ptr(),
                core::ptr::null::<u8>(),
            ],
        );
        exit(0);
    } else if pid > 0 {
        let mut exit_code: usize = 0;
        wait(pid, &mut exit_code);
    } else {
        println!("fork failed, ret: {}", pid);
    }
}

fn run(app: &str) {
    run_with_args(app, &[core::ptr::null::<u8>()]);
}

fn run_test_splice() {
    for i in 1..6 {
        run_with_args(
            "./test_splice\0",
            &[
                "test_splice\0".as_ptr(),
                alloc::format!("{}\0", i).as_ptr(),
                core::ptr::null::<u8>(),
            ],
        );
    }
}

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

// todo: use linkat
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
        run_sh("/musl/busybox --install /bin\0");
        run_sh("mkdir -p /lib\0");
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
        copy_file("/musl/lib/libc.so\0", "/lib/ld-musl-riscv64.so.1\0");

        println!("[riscv64] init glibc and musl libraries");
    }
    #[cfg(target_arch = "loongarch64")]
    {
        run_sh("/musl/busybox --install /bin\0");
        run_sh("mkdir -p /lib\0");
        run_sh("mkdir -p /lib64\0");
        run_sh("mkdir -p /usr/lib64\0");
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

    run_sh("mkdir -p /tmp\0");
    run_sh("mkdir -p /etc\0");

    run_sh("echo 'ip      0       IP      # Internet protocol' > /etc/protocols\0");
    run_sh(
        "echo 'icmp    1       ICMP    # Internet Control Message Protocol' >> /etc/protocols\0",
    );
    run_sh("echo 'tcp     6       TCP     # Transmission Control Protocol' >> /etc/protocols\0");
    run_sh("echo 'udp     17      UDP     # User Datagram Protocol' >> /etc/protocols\0");

    run_sh("echo 'hosts: files dns' > /etc/nsswitch.conf\0");
    run_sh("echo 'networks: files' >> /etc/nsswitch.conf\0");
    run_sh("echo 'protocols: files' >> /etc/nsswitch.conf\0");
    run_sh("echo 'services: files' >> /etc/nsswitch.conf\0");

    run_sh("echo 'root:x:0:0:root:/root:/bin/bash' > /etc/passwd\0");
    run_sh("echo 'nobody:x:65534:65534:nobody:/nonexistent:/usr/sbin/nologin' >> /etc/passwd\0");

    // Create /etc/group file with essential groups for chmod07 test
    run_sh("printf 'root:x:0:\\ndaemon:x:1:\\nbin:x:2:\\nsys:x:3:\\nusers:x:100:\\nnogroup:x:65534:\\nltp_test_700:x:700:\\nltp_test_701:x:701:\\nltp_test_702:x:702:\\nltp_test_703:x:703:\\nltp_test_704:x:704:\\nltp_test_705:x:705:\\n' > /etc/group\0");
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

        #[cfg(feature = "ltp")]
        {
            switch_into_ltp();
            run_ltp();
            switch_outof_ltp();
        }

        for &(test, rvm, rvg, _lam, _lag) in TEST_LAST {
            if rvm {
                chdir("/musl\0");
                run_sh(test);
            }
            if rvg {
                chdir("/glibc\0");
                run_sh(test);
            }
        }
        exit(0);
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

        #[cfg(feature = "ltp")]
        {
            switch_into_ltp();
            run_ltp();
            switch_outof_ltp();
        }

        for &(test, _rvm, _rvg, lam, lag) in TEST_LAST {
            if lam {
                chdir("/musl\0");
                run_sh(test);
            }
            if lag {
                chdir("/glibc\0");
                run_sh(test);
            }
        }
        exit(0);
    }
}

#[no_mangle]
fn main() -> i32 {
    println!("[init_proc] Hello, NoAxiom!");

    // initialize test environment
    println!("[init_proc] initializing test environment...");
    switch_log_off();
    init();
    switch_log_on();
    println!("[init_proc] Test environment initialized!");

    // run testsuits
    println!("[init_proc] Test begin!");
    println!("\n=======================\n");
    run_tests();
    println!("\n=======================\n");
    println!("[init_proc] Test finished!");

    0
}
