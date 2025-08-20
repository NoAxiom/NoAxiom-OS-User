#![no_std]
#![no_main]

extern crate alloc;

use libd::{
    lib_basepath::BUSYBOX,
    println,
    syscall::{execve, fork, hard_link, wait},
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
    run_sh("/bin/busybox --install /bin\0");
    run_sh("/bin/busybox --install /usr/bin\0");

    // hard_link("/usr/lib/libisl.so.23.3.0\0", "/lib/libisl.so.23\0");
    // hard_link("/usr/lib/libisl.so.23.3.0\0", "/usr/lib/libisl.so.23\0");
    // hard_link("/usr/lib/libmpc.so.3.3.1\0", "/usr/lib/libmpc.so.3\0");
    // hard_link("/usr/lib/libmpc.so.3.3.1\0", "/lib/libmpc.so.3\0");
    // hard_link("/usr/lib/libmpfr.so.6.2.1\0", "/lib/libmpfr.so.6\0");
    // hard_link("/usr/lib/libgmp.so.10.5.0\0", "/lib/libgmp.so.10\0");

    // hard_link("/usr/lib/libatomic.so.1.2.0\0", "/usr/lib/libatomic.so.1\0");
    // hard_link(
    //     "/usr/lib/libbrotlicommon.so.1.1.0\0",
    //     "/usr/lib/libbrotlicommon.so.1\0",
    // );
    // hard_link(
    //     "/usr/lib/libbrotlienc.so.1.1.0\0",
    //     "/usr/lib/libbrotlienc.so.1\0",
    // );

    // hard_link("/usr/lib/libcares.so.2.19.4\0", "/usr/lib/libcares.so.2\0");
    // hard_link("/usr/lib/libcc1.so.0.0.0\0", "/usr/lib/libcc1.so.0\0");

    // hard_link(
    //     "/usr/lib/libbrotlienc.so.1.1.0\0",
    //     "/usr/lib/libbrotlienc.so.1\0",
    // );
    // hard_link(
    //     "/usr/lib/libbrotlienc.so.1.1.0\0",
    //     "/usr/lib/libbrotlienc.so.1\0",
    // );

    hard_link("/usr/lib/libbrotlienc.so.1\0", "/lib/libbrotlienc.so\0");
    hard_link("/usr/lib/libjansson.so.4\0", "/lib/libjansson.so\0");
    hard_link("/usr/lib/libapk.so.2.14.9\0", "/lib/libapk.so.2\0");
    hard_link("/usr/lib/libreadline.so.8.2\0", "/lib/libreadline.so.8\0");
    hard_link("/usr/lib/libreadline.so.8\0", "/lib/libreadline.so\0");
    hard_link(
        "/usr/lib/libbrotlicommon.so.1.1.0\0",
        "/lib/libbrotlicommon.so.1\0",
    );
    hard_link("/usr/lib/libisl.so.23.3.0\0", "/lib/libisl.so.23\0");
    hard_link("/usr/lib/libgcc_s.so.1\0", "/lib/libgcc_s.so\0");
    hard_link("/usr/lib/libcares.so.2.19.4\0", "/lib/libcares.so.2\0");
    hard_link("/usr/lib/libgomp.so.1.0.0\0", "/lib/libgomp.so.1\0");
    hard_link("/usr/lib/libatomic.so.1\0", "/lib/libatomic.so\0");
    hard_link("/usr/lib/libzstd.so.1\0", "/lib/libzstd.so\0");
    hard_link("/usr/lib/libncursesw.so.6.5\0", "/lib/libncursesw.so.6\0");
    hard_link("/usr/lib/libmpfr.so.6\0", "/lib/libmpfr.so\0");
    hard_link("/usr/lib/libgmp.so.10\0", "/lib/libgmp.so\0");
    hard_link(
        "/usr/lib/libunistring.so.5.2.0\0",
        "/lib/libunistring.so.5\0",
    );
    hard_link("/usr/lib/libatomic.so.1.2.0\0", "/lib/libatomic.so.1\0");
    hard_link("/usr/lib/libctf.so.0\0", "/lib/libctf.so\0");
    hard_link("/usr/lib/libgomp.so.1\0", "/lib/libgomp.so\0");
    hard_link(
        "/usr/lib/libbrotlicommon.so.1\0",
        "/lib/libbrotlicommon.so\0",
    );
    hard_link("/usr/lib/libffi.so.8.1.4\0", "/lib/libffi.so.8\0");
    hard_link("/usr/lib/libexpat.so.1\0", "/lib/libexpat.so\0");
    hard_link("/usr/lib/libffi.so.8\0", "/lib/libffi.so\0");
    hard_link("/usr/lib/libLLVM.so.20.1\0", "/lib/libLLVM.so.20\0");
    hard_link("/usr/lib/libzstd.so.1.5.7\0", "/lib/libzstd.so.1\0");
    hard_link("/usr/lib/libgmp.so.10.5.0\0", "/lib/libgmp.so.10\0");
    hard_link("/usr/lib/libsframe.so.1\0", "/lib/libsframe.so\0");
    hard_link("/usr/lib/libcc1.so.0\0", "/lib/libcc1.so\0");
    hard_link("/usr/lib/libz.so.1\0", "/lib/libz.so\0");
    hard_link("/usr/lib/libmpc.so.3\0", "/lib/libmpc.so\0");
    hard_link("/usr/lib/libcrypto.so.3\0", "/lib/libcrypto.so\0");
    hard_link("/usr/lib/liblzma.so.5.8.1\0", "/lib/liblzma.so.5\0");
    hard_link("/usr/lib/libidn2.so.0\0", "/lib/libidn2.so\0");
    hard_link("/usr/lib/libcares.so.2\0", "/lib/libcares.so\0");
    hard_link("/usr/lib/libssl.so.3\0", "/lib/libssl.so\0");
    hard_link("/usr/lib/libcurl.so.4\0", "/lib/libcurl.so\0");
    hard_link("/usr/lib/liblzma.so.5\0", "/lib/liblzma.so\0");
    hard_link("/usr/lib/libnghttp2.so.14\0", "/lib/libnghttp2.so\0");
    hard_link("/usr/lib/libunistring.so.5\0", "/lib/libunistring.so\0");
    hard_link("/usr/lib/libbrotlidec.so.1\0", "/lib/libbrotlidec.so\0");
    hard_link(
        "/usr/lib/libnghttp2.so.14.28.4\0",
        "/lib/libnghttp2.so.14\0",
    );
    hard_link("/usr/lib/libxml2.so.2\0", "/lib/libxml2.so\0");
    hard_link("/usr/lib/libsframe.so.1.0.0\0", "/lib/libsframe.so.1\0");
    hard_link("/usr/lib/libmpc.so.3.3.1\0", "/lib/libmpc.so.3\0");
    hard_link("/usr/lib/libcc1.so.0.0.0\0", "/lib/libcc1.so.0\0");
    hard_link("/usr/lib/libctf.so.0.0.0\0", "/lib/libctf.so.0\0");
    hard_link(
        "/usr/lib/libbrotlidec.so.1.1.0\0",
        "/lib/libbrotlidec.so.1\0",
    );
    hard_link("/usr/lib/libmpfr.so.6.2.1\0", "/lib/libmpfr.so.6\0");
    hard_link("/usr/lib/libcurl.so.4.8.0\0", "/lib/libcurl.so.4\0");
    hard_link("/usr/lib/libpsl.so.5\0", "/lib/libpsl.so\0");
    hard_link("/usr/lib/libncursesw.so.6\0", "/lib/libncursesw.so\0");
    hard_link("/usr/lib/libisl.so.23\0", "/lib/libisl.so\0");
    hard_link("/usr/lib/libpsl.so.5.3.5\0", "/lib/libpsl.so.5\0");
    hard_link("/usr/lib/libxml2.so.2.13.8\0", "/lib/libxml2.so.2\0");
    hard_link("/usr/lib/libidn2.so.0.4.0\0", "/lib/libidn2.so.0\0");
    hard_link("/usr/lib/libz.so.1.3.1\0", "/lib/libz.so.1\0");
    hard_link("/usr/lib/libjansson.so.4.14.1\0", "/lib/libjansson.so.4\0");
    hard_link(
        "/usr/lib/libbrotlienc.so.1.1.0\0",
        "/lib/libbrotlienc.so.1\0",
    );
    hard_link("/usr/lib/libexpat.so.1.10.2\0", "/lib/libexpat.so.1\0");

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
