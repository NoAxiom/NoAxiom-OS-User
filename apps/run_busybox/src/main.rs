#![no_std]
#![no_main]

extern crate alloc;

use libd::{
    lib_basepath::BUSYBOX,
    println,
    syscall::{execve, fork, hard_link, symlinkat, wait},
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

    hard_link("/bin/busybox\0", "/bin/arch\0");
    hard_link("/bin/busybox\0", "/bin/ash\0");
    hard_link("/bin/busybox\0", "/bin/base64\0");
    hard_link("/bin/busybox\0", "/bin/cat\0");
    hard_link("/bin/busybox\0", "/bin/chattr\0");
    hard_link("/bin/busybox\0", "/bin/chgrp\0");
    hard_link("/bin/busybox\0", "/bin/chmod\0");
    hard_link("/bin/busybox\0", "/bin/chown\0");
    hard_link("/bin/busybox\0", "/bin/cp\0");
    hard_link("/bin/busybox\0", "/bin/date\0");
    hard_link("/bin/busybox\0", "/bin/dd\0");
    hard_link("/bin/busybox\0", "/bin/df\0");
    hard_link("/bin/busybox\0", "/bin/dmesg\0");
    hard_link("/bin/busybox\0", "/bin/dnsdomainname\0");
    hard_link("/bin/busybox\0", "/bin/dumpkmap\0");
    hard_link("/bin/busybox\0", "/bin/echo\0");
    hard_link("/bin/busybox\0", "/bin/egrep\0");
    hard_link("/bin/busybox\0", "/bin/false\0");
    hard_link("/bin/busybox\0", "/bin/fatattr\0");
    hard_link("/bin/busybox\0", "/bin/fdflush\0");
    hard_link("/bin/busybox\0", "/bin/fgrep\0");
    hard_link("/bin/busybox\0", "/bin/fsync\0");
    hard_link("/bin/busybox\0", "/bin/getopt\0");
    hard_link("/bin/busybox\0", "/bin/grep\0");
    hard_link("/bin/busybox\0", "/bin/gunzip\0");
    hard_link("/bin/busybox\0", "/bin/gzip\0");
    hard_link("/bin/busybox\0", "/bin/hostname\0");
    hard_link("/bin/busybox\0", "/bin/ionice\0");
    hard_link("/bin/busybox\0", "/bin/iostat\0");
    hard_link("/bin/busybox\0", "/bin/ipcalc\0");
    hard_link("/bin/busybox\0", "/bin/kbd_mode\0");
    hard_link("/bin/busybox\0", "/bin/kill\0");
    hard_link("/bin/busybox\0", "/bin/link\0");
    hard_link("/bin/busybox\0", "/bin/linux32\0");
    hard_link("/bin/busybox\0", "/bin/linux64\0");
    hard_link("/bin/busybox\0", "/bin/ln\0");
    hard_link("/bin/busybox\0", "/bin/login\0");
    hard_link("/bin/busybox\0", "/bin/ls\0");
    hard_link("/bin/busybox\0", "/bin/lsattr\0");
    hard_link("/bin/busybox\0", "/bin/lzop\0");
    hard_link("/bin/busybox\0", "/bin/makemime\0");
    hard_link("/bin/busybox\0", "/bin/mkdir\0");
    hard_link("/bin/busybox\0", "/bin/mknod\0");
    hard_link("/bin/busybox\0", "/bin/mktemp\0");
    hard_link("/bin/busybox\0", "/bin/more\0");
    hard_link("/bin/busybox\0", "/bin/mount\0");
    hard_link("/bin/busybox\0", "/bin/mountpoint\0");
    hard_link("/bin/busybox\0", "/bin/mpstat\0");
    hard_link("/bin/busybox\0", "/bin/mv\0");
    hard_link("/bin/busybox\0", "/bin/netstat\0");
    hard_link("/bin/busybox\0", "/bin/nice\0");
    hard_link("/bin/busybox\0", "/bin/pidof\0");
    hard_link("/bin/busybox\0", "/bin/ping\0");
    hard_link("/bin/busybox\0", "/bin/ping6\0");
    hard_link("/bin/busybox\0", "/bin/pipe_progress\0");
    hard_link("/bin/busybox\0", "/bin/printenv\0");
    hard_link("/bin/busybox\0", "/bin/ps\0");
    hard_link("/bin/busybox\0", "/bin/pwd\0");
    hard_link("/bin/busybox\0", "/bin/reformime\0");
    hard_link("/bin/busybox\0", "/bin/rev\0");
    hard_link("/bin/busybox\0", "/bin/rm\0");
    hard_link("/bin/busybox\0", "/bin/rmdir\0");
    hard_link("/bin/busybox\0", "/bin/run-parts\0");
    hard_link("/bin/busybox\0", "/bin/sed\0");
    hard_link("/bin/busybox\0", "/bin/setpriv\0");
    hard_link("/bin/busybox\0", "/bin/setserial\0");
    hard_link("/bin/busybox\0", "/bin/sh\0");
    hard_link("/bin/busybox\0", "/bin/sleep\0");
    hard_link("/bin/busybox\0", "/bin/stat\0");
    hard_link("/bin/busybox\0", "/bin/stty\0");
    hard_link("/bin/busybox\0", "/bin/su\0");
    hard_link("/bin/busybox\0", "/bin/sync\0");
    hard_link("/bin/busybox\0", "/bin/tar\0");
    hard_link("/bin/busybox\0", "/bin/touch\0");
    hard_link("/bin/busybox\0", "/bin/true\0");
    hard_link("/bin/busybox\0", "/bin/umount\0");
    hard_link("/bin/busybox\0", "/bin/uname\0");
    hard_link("/bin/busybox\0", "/bin/usleep\0");
    hard_link("/bin/busybox\0", "/bin/watch\0");
    hard_link("/bin/busybox\0", "/bin/zcat\0");

    #[cfg(target_arch = "riscv64")]
    {
        symlinkat("/usr/lib/libatomic.so\0", "/usr/lib/libatomic.so.1.2.0\0");
        symlinkat("/usr/lib/libatomic.so.1\0", "/usr/lib/libatomic.so.1.2.0\0");
        symlinkat(
            "/usr/lib/libbrotlicommon.so.1\0",
            "libbrotlicommon.so.1.1.0\0",
        );
        symlinkat(
            "/usr/lib/libbrotlidec.so.1\0",
            "/usr/lib/libbrotlidec.so.1.1.0\0",
        );
        symlinkat(
            "/usr/lib/libbrotlienc.so.1\0",
            "/usr/lib/libbrotlienc.so.1.1.0\0",
        );
        symlinkat("/usr/lib/libcares.so.2\0", "/usr/lib/libcares.so.2.19.4\0");
        symlinkat("/usr/lib/libcc1.so\0", "/usr/lib/libcc1.so.0.0.0\0");
        symlinkat("/usr/lib/libcc1.so.0\0", "/usr/lib/libcc1.so.0.0.0\0");
        symlinkat("/usr/lib/libc.so\0", "/lib/ld-musl-riscv64.so.1\0");
        symlinkat(
            "/usr/lib/libctf-nobfd.so.0\0",
            "/usr/lib/libctf-nobfd.so.0.0.0\0",
        );
        symlinkat("/usr/lib/libctf.so.0\0", "/usr/lib/libctf.so.0.0.0\0");
        symlinkat("/usr/lib/libcurl.so.4\0", "/usr/lib/libcurl.so.4.8.0\0");
        symlinkat("/usr/lib/libexpat.so.1\0", "/usr/lib/libexpat.so.1.10.2\0");
        symlinkat("/usr/lib/libffi.so.8\0", "/usr/lib/libffi.so.8.1.4\0");
        symlinkat("/usr/lib/libgmp.so.10\0", "/usr/lib/libgmp.so.10.5.0\0");
        symlinkat("/usr/lib/libgomp.so\0", "/usr/lib/libgomp.so.1.0.0\0");
        symlinkat("/usr/lib/libgomp.so.1\0", "/usr/lib/libgomp.so.1.0.0\0");
        symlinkat("/usr/lib/libidn2.so.0\0", "/usr/lib/libidn2.so.0.4.0\0");
        symlinkat("/usr/lib/libisl.so.23\0", "/usr/lib/libisl.so.23.3.0\0");
        symlinkat(
            "/usr/lib/libjansson.so.4\0",
            "/usr/lib/libjansson.so.4.14.1\0",
        );
        symlinkat("/usr/lib/libLLVM-20.so\0", "/usr/lib/libLLVM.so.20.1\0");
        symlinkat("/usr/lib/liblzma.so.5\0", "/usr/lib/liblzma.so.5.8.1\0");
        symlinkat("/usr/lib/libmpc.so.3\0", "/usr/lib/libmpc.so.3.3.1\0");
        symlinkat("/usr/lib/libmpfr.so.6\0", "/usr/lib/libmpfr.so.6.2.1\0");
        symlinkat(
            "/usr/lib/libncursesw.so.6\0",
            "/usr/lib/libncursesw.so.6.5\0",
        );
        symlinkat(
            "/usr/lib/libnghttp2.so.14\0",
            "/usr/lib/libnghttp2.so.14.28.4\0",
        );
        symlinkat(
            "/usr/lib/libpcre2-8.so.0\0",
            "/usr/lib/libpcre2-8.so.0.12.0\0",
        );
        symlinkat(
            "/usr/lib/libpcre2-posix.so.3\0",
            "libpcre2-posix.so.3.0.5\0",
        );
        symlinkat("/usr/lib/libpsl.so.5\0", "/usr/lib/libpsl.so.5.3.5\0");
        symlinkat(
            "/usr/lib/libreadline.so.8\0",
            "/usr/lib/libreadline.so.8.2\0",
        );
        symlinkat("/usr/lib/libsframe.so.1\0", "/usr/lib/libsframe.so.1.0.0\0");
        symlinkat(
            "/usr/lib/libstdc++.so.6\0",
            "/usr/lib/libstdc++.so.6.0.33\0",
        );
        symlinkat(
            "/usr/lib/libunistring.so.5\0",
            "/usr/lib/libunistring.so.5.2.0\0",
        );
        symlinkat("/usr/lib/libxml2.so.2\0", "/usr/lib/libxml2.so.2.13.8\0");
        symlinkat("/usr/lib/libz.so.1\0", "/usr/lib/libz.so.1.3.1\0");
        symlinkat("/usr/lib/libzstd.so.1\0", "/usr/lib/libzstd.so.1.5.7\0");
    }

    #[cfg(target_arch = "loongarch64")]
    {
        symlinkat("/usr/lib/libatomic.so\0", "/usr/lib/libatomic.so.1.2.0\0");
        symlinkat("/usr/lib/libatomic.so.1\0", "/usr/lib/libatomic.so.1.2.0\0");
        symlinkat(
            "/usr/lib/libbrotlicommon.so.1\0",
            "/usr/lib/libbrotlicommon.so.1.1.0\0",
        );
        symlinkat(
            "/usr/lib/libbrotlidec.so.1\0",
            "/usr/lib/libbrotlidec.so.1.1.0\0",
        );
        symlinkat(
            "/usr/lib/libbrotlienc.so.1\0",
            "/usr/lib/libbrotlienc.so.1.1.0\0",
        );
        symlinkat("/usr/lib/libcares.so.2\0", "/usr/lib/libcares.so.2.19.4\0");
        symlinkat("/usr/lib/libcc1.so\0", "/usr/lib/libcc1.so.0.0.0\0");
        symlinkat("/usr/lib/libcc1.so.0\0", "/usr/lib/libcc1.so.0.0.0\0");
        symlinkat(
            "/usr/lib/libc.so\0",
            "/usr/lib/../../lib/ld-musl-loongarch64.so.1\0",
        );
        symlinkat(
            "/usr/lib/libctf-nobfd.so.0\0",
            "/usr/lib/libctf-nobfd.so.0.0.0\0",
        );
        symlinkat("/usr/lib/libctf.so.0\0", "/usr/lib/libctf.so.0.0.0\0");
        symlinkat("/usr/lib/libcurl.so.4\0", "/usr/lib/libcurl.so.4.8.0\0");
        symlinkat("/usr/lib/libexpat.so.1\0", "/usr/lib/libexpat.so.1.10.1\0");
        symlinkat("/usr/lib/libffi.so.8\0", "/usr/lib/libffi.so.8.1.4\0");
        symlinkat("/usr/lib/libgomp.so\0", "/usr/lib/libgomp.so.1.0.0\0");
        symlinkat("/usr/lib/libgomp.so.1\0", "/usr/lib/libgomp.so.1.0.0\0");
        symlinkat("/usr/lib/libidn2.so.0\0", "/usr/lib/libidn2.so.0.4.0\0");
        symlinkat("/usr/lib/libisl.so.23\0", "/usr/lib/libisl.so.23.3.0\0");
        symlinkat(
            "/usr/lib/libjansson.so.4\0",
            "/usr/lib/libjansson.so.4.14.0\0",
        );
        symlinkat("/usr/lib/libLLVM-19.so\0", "/usr/lib/libLLVM.so.19.1\0");
        symlinkat("/usr/lib/liblzma.so.5\0", "/usr/lib/liblzma.so.5.6.3\0");
        symlinkat("/usr/lib/libmagic.so.1\0", "/usr/lib/libmagic.so.1.0.0\0");
        symlinkat("/usr/lib/libmpc.so.3\0", "/usr/lib/libmpc.so.3.3.1\0");
        symlinkat("/usr/lib/libgmp.so.10\0", "/usr/lib/libgmp.so.10.5.0\0");
        symlinkat("/usr/lib/libmpfr.so.6\0", "/usr/lib/libmpfr.so.6.2.1\0");
        symlinkat(
            "/usr/lib/libncursesw.so.6\0",
            "/usr/lib/libncursesw.so.6.5\0",
        );
        symlinkat(
            "/usr/lib/libnghttp2.so.14\0",
            "/usr/lib/libnghttp2.so.14.28.3\0",
        );
        symlinkat(
            "/usr/lib/libpcre2-8.so.0\0",
            "/usr/lib/libpcre2-8.so.0.12.0\0",
        );
        symlinkat(
            "/usr/lib/libpcre2-posix.so.3\0",
            "/usr/lib/libpcre2-posix.so.3.0.5\0",
        );
        symlinkat("/usr/lib/libpsl.so.5\0", "/usr/lib/libpsl.so.5.3.5\0");
        symlinkat(
            "/usr/lib/libreadline.so.8\0",
            "/usr/lib/libreadline.so.8.2\0",
        );
        symlinkat("/usr/lib/libsframe.so.1\0", "/usr/lib/libsframe.so.1.0.0\0");
        symlinkat(
            "/usr/lib/libstdc++.so.6\0",
            "/usr/lib/libstdc++.so.6.0.33\0",
        );
        symlinkat(
            "/usr/lib/libunistring.so.5\0",
            "/usr/lib/libunistring.so.5.1.0\0",
        );
        symlinkat("/usr/lib/libxml2.so.2\0", "/usr/lib/libxml2.so.2.13.4\0");
        symlinkat("/usr/lib/libz.so.1\0", "/usr/lib/libz.so.1.3.1\0");
        symlinkat("/usr/lib/libzstd.so.1\0", "/usr/lib/libzstd.so.1.5.6\0");
    }

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
