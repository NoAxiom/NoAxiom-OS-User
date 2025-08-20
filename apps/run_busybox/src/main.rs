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

    #[cfg(target_arch = "riscv64")]
    {
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
        hard_link("/bin/busybox\0", "/usr/bin/uuencode\0");
        hard_link("/bin/busybox\0", "/usr/bin/pkill\0");
        hard_link("/bin/busybox\0", "/usr/bin/unexpand\0");
        hard_link("/bin/busybox\0", "/usr/bin/pgrep\0");
        hard_link("/bin/busybox\0", "/usr/bin/unlzop\0");
        hard_link("/bin/busybox\0", "/usr/bin/groups\0");
        hard_link("/bin/busybox\0", "/usr/bin/udhcpc6\0");
        hard_link("/bin/busybox\0", "/usr/bin/unix2dos\0");
        hard_link("/bin/busybox\0", "/usr/bin/seq\0");
        hard_link("/bin/busybox\0", "/usr/bin/dc\0");
        hard_link("/bin/busybox\0", "/usr/bin/nsenter\0");
        hard_link("/bin/busybox\0", "/usr/bin/nslookup\0");
        hard_link("/bin/busybox\0", "/usr/bin/[[\0");
        hard_link("/bin/busybox\0", "/usr/bin/tr\0");
        hard_link("/bin/busybox\0", "/usr/bin/whois\0");
        hard_link("/bin/busybox\0", "/usr/bin/bc\0");
        hard_link("/bin/busybox\0", "/usr/bin/resize\0");
        hard_link("/bin/busybox\0", "/usr/bin/xargs\0");
        hard_link("/bin/busybox\0", "/usr/bin/nproc\0");
        hard_link("/bin/busybox\0", "/usr/bin/comm\0");
        hard_link("/bin/busybox\0", "/usr/bin/nc\0");
        hard_link("/bin/busybox\0", "/usr/bin/tail\0");
        hard_link("/bin/busybox\0", "/usr/bin/od\0");
        hard_link("/bin/busybox\0", "/usr/bin/install\0");
        hard_link("/bin/busybox\0", "/usr/bin/mesg\0");
        hard_link("/bin/busybox\0", "/usr/bin/unshare\0");
        hard_link("/bin/busybox\0", "/usr/bin/whoami\0");
        hard_link("/bin/busybox\0", "/usr/bin/vlock\0");
        hard_link("/bin/busybox\0", "/usr/bin/cryptpw\0");
        hard_link("/bin/busybox\0", "/usr/bin/cpio\0");
        hard_link("/bin/busybox\0", "/usr/bin/beep\0");
        hard_link("/bin/busybox\0", "/usr/bin/nmeter\0");
        hard_link("/bin/busybox\0", "/usr/bin/head\0");
        hard_link("/bin/busybox\0", "/usr/bin/blkdiscard\0");
        hard_link("/bin/busybox\0", "/usr/bin/bunzip2\0");
        hard_link("/bin/busybox\0", "/usr/bin/expr\0");
        hard_link("/bin/busybox\0", "/usr/bin/pstree\0");
        hard_link("/bin/busybox\0", "/usr/bin/traceroute6\0");
        hard_link("/bin/busybox\0", "/usr/bin/killall\0");
        hard_link("/bin/busybox\0", "/usr/bin/tree\0");
        hard_link("/bin/busybox\0", "/usr/bin/fallocate\0");
        hard_link("/bin/busybox\0", "/usr/bin/mkfifo\0");
        hard_link("/bin/busybox\0", "/usr/bin/who\0");
        hard_link("/bin/busybox\0", "/usr/bin/hd\0");
        hard_link("/bin/busybox\0", "/usr/bin/uniq\0");
        hard_link("/bin/busybox\0", "/usr/bin/lsusb\0");
        hard_link("/bin/busybox\0", "/usr/bin/ipcrm\0");
        hard_link("/bin/busybox\0", "/usr/bin/bzip2\0");
        hard_link("/bin/busybox\0", "/usr/bin/eject\0");
        hard_link("/bin/busybox\0", "/usr/bin/deallocvt\0");
        hard_link("/bin/busybox\0", "/usr/bin/wc\0");
        hard_link("/bin/busybox\0", "/usr/bin/fuser\0");
        hard_link("/bin/busybox\0", "/usr/bin/logger\0");
        hard_link("/bin/busybox\0", "/usr/bin/top\0");
        hard_link("/bin/busybox\0", "/usr/bin/timeout\0");
        hard_link("/bin/busybox\0", "/usr/bin/passwd\0");
        hard_link("/bin/busybox\0", "/usr/bin/sha3sum\0");
        hard_link("/bin/busybox\0", "/usr/bin/cmp\0");
        hard_link("/bin/busybox\0", "/usr/bin/pwdx\0");
        hard_link("/bin/busybox\0", "/usr/bin/shred\0");
        hard_link("/bin/busybox\0", "/usr/bin/pscan\0");
        hard_link("/bin/busybox\0", "/usr/bin/clear\0");
        hard_link("/bin/busybox\0", "/usr/bin/split\0");
        hard_link("/bin/busybox\0", "/usr/bin/vi\0");
        hard_link("/bin/busybox\0", "/usr/bin/dirname\0");
        hard_link("/bin/busybox\0", "/usr/bin/sha512sum\0");
        hard_link("/bin/busybox\0", "/usr/bin/test\0");
        hard_link("/bin/busybox\0", "/usr/bin/cksum\0");
        hard_link("/bin/busybox\0", "/usr/bin/id\0");
        hard_link("/bin/busybox\0", "/usr/bin/setkeycodes\0");
        hard_link("/bin/busybox\0", "/usr/bin/flock\0");
        hard_link("/bin/busybox\0", "/usr/bin/time\0");
        hard_link("/bin/busybox\0", "/usr/bin/env\0");
        hard_link("/bin/busybox\0", "/usr/bin/which\0");
        hard_link("/bin/busybox\0", "/usr/bin/less\0");
        hard_link("/bin/busybox\0", "/usr/bin/uptime\0");
        hard_link("/bin/busybox\0", "/usr/bin/traceroute\0");
        hard_link("/bin/busybox\0", "/usr/bin/tee\0");
        hard_link("/bin/busybox\0", "/usr/bin/yes\0");
        hard_link("/bin/busybox\0", "/usr/bin/realpath\0");
        hard_link("/bin/busybox\0", "/usr/bin/microcom\0");
        hard_link("/bin/busybox\0", "/usr/bin/unzip\0");
        hard_link("/bin/busybox\0", "/usr/bin/sum\0");
        hard_link("/bin/busybox\0", "/usr/bin/ttysize\0");
        hard_link("/bin/busybox\0", "/usr/bin/paste\0");
        hard_link("/bin/busybox\0", "/usr/bin/xzcat\0");
        hard_link("/bin/busybox\0", "/usr/bin/last\0");
        hard_link("/bin/busybox\0", "/usr/bin/bzcat\0");
        hard_link("/bin/busybox\0", "/usr/bin/[\0");
        hard_link("/bin/busybox\0", "/usr/bin/hostid\0");
        hard_link("/bin/busybox\0", "/usr/bin/renice\0");
        hard_link("/bin/busybox\0", "/usr/bin/nl\0");
        hard_link("/bin/busybox\0", "/usr/bin/shuf\0");
        hard_link("/bin/busybox\0", "/usr/bin/showkey\0");
        hard_link("/bin/busybox\0", "/usr/bin/tty\0");
        hard_link("/bin/busybox\0", "/usr/bin/chvt\0");
        hard_link("/bin/busybox\0", "/usr/bin/reset\0");
        hard_link("/bin/busybox\0", "/usr/bin/basename\0");
        hard_link("/bin/busybox\0", "/usr/bin/ipcs\0");
        hard_link("/bin/busybox\0", "/usr/bin/dos2unix\0");
        hard_link("/bin/busybox\0", "/usr/bin/openvt\0");
        hard_link("/bin/busybox\0", "/usr/bin/crontab\0");
        hard_link("/bin/busybox\0", "/usr/bin/lzcat\0");
        hard_link("/bin/busybox\0", "/usr/bin/unlink\0");
        hard_link("/bin/busybox\0", "/usr/bin/tac\0");
        hard_link("/bin/busybox\0", "/usr/bin/cut\0");
        hard_link("/bin/busybox\0", "/usr/bin/uudecode\0");
        hard_link("/bin/busybox\0", "/usr/bin/setsid\0");
        hard_link("/bin/busybox\0", "/usr/bin/awk\0");
        hard_link("/bin/busybox\0", "/usr/bin/sha256sum\0");
        hard_link("/bin/busybox\0", "/usr/bin/sha1sum\0");
        hard_link("/bin/busybox\0", "/usr/bin/nohup\0");
        hard_link("/bin/busybox\0", "/usr/bin/find\0");
        hard_link("/bin/busybox\0", "/usr/bin/cal\0");
        hard_link("/bin/busybox\0", "/usr/bin/free\0");
        hard_link("/bin/busybox\0", "/usr/bin/wget\0");
        hard_link("/bin/busybox\0", "/usr/bin/volname\0");
        hard_link("/bin/busybox\0", "/usr/bin/lzopcat\0");
        hard_link("/bin/busybox\0", "/usr/bin/expand\0");
        hard_link("/bin/busybox\0", "/usr/bin/pmap\0");
        hard_link("/bin/busybox\0", "/usr/bin/unlzma\0");
        hard_link("/bin/busybox\0", "/usr/bin/hexdump\0");
        hard_link("/bin/busybox\0", "/usr/bin/truncate\0");
        hard_link("/bin/busybox\0", "/usr/bin/sort\0");
        hard_link("/bin/busybox\0", "/usr/bin/printf\0");
        hard_link("/bin/busybox\0", "/usr/bin/fold\0");
        hard_link("/bin/busybox\0", "/usr/bin/du\0");
        hard_link("/bin/busybox\0", "/usr/bin/lzma\0");
        hard_link("/bin/busybox\0", "/usr/bin/readlink\0");
        hard_link("/bin/busybox\0", "/usr/bin/lsof\0");
        hard_link("/bin/busybox\0", "/usr/bin/md5sum\0");
        hard_link("/bin/busybox\0", "/usr/bin/factor\0");
        hard_link("/bin/busybox\0", "/usr/bin/unxz\0");
        hard_link("/bin/busybox\0", "/usr/bin/diff\0");
        hard_link("/bin/busybox\0", "/usr/bin/mkpasswd\0");

        symlinkat("/usr/bin/cc\0", "/usr/bin/gcc\0");

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
        hard_link("/bin/busybox\0", "/bin/arch\0");
        hard_link("/bin/busybox\0", "/bin/ash\0");
        hard_link("/bin/busybox\0", "/bin/base64\0");
        hard_link("/bin/busybox\0", "/bin/bbconfig\0");
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

        hard_link("/bin/busybox\0", "/usr/bin/uuencode\0");
        hard_link("/bin/busybox\0", "/usr/bin/pkill\0");
        hard_link("/bin/busybox\0", "/usr/bin/unexpand\0");
        hard_link("/bin/busybox\0", "/usr/bin/pgrep\0");
        hard_link("/bin/busybox\0", "/usr/bin/unlzop\0");
        hard_link("/bin/busybox\0", "/usr/bin/groups\0");
        hard_link("/bin/busybox\0", "/usr/bin/udhcpc6\0");
        hard_link("/bin/busybox\0", "/usr/bin/unix2dos\0");
        hard_link("/bin/busybox\0", "/usr/bin/seq\0");
        hard_link("/bin/busybox\0", "/usr/bin/dc\0");
        hard_link("/bin/busybox\0", "/usr/bin/nsenter\0");
        hard_link("/bin/busybox\0", "/usr/bin/nslookup\0");
        hard_link("/bin/busybox\0", "/usr/bin/[[\0");
        hard_link("/bin/busybox\0", "/usr/bin/tr\0");
        hard_link("/bin/busybox\0", "/usr/bin/whois\0");
        hard_link("/bin/busybox\0", "/usr/bin/bc\0");
        hard_link("/bin/busybox\0", "/usr/bin/resize\0");
        hard_link("/bin/busybox\0", "/usr/bin/xargs\0");
        hard_link("/bin/busybox\0", "/usr/bin/nproc\0");
        hard_link("/bin/busybox\0", "/usr/bin/comm\0");
        hard_link("/bin/busybox\0", "/usr/bin/nc\0");
        hard_link("/bin/busybox\0", "/usr/bin/tail\0");
        hard_link("/bin/busybox\0", "/usr/bin/od\0");
        hard_link("/bin/busybox\0", "/usr/bin/install\0");
        hard_link("/bin/busybox\0", "/usr/bin/mesg\0");
        hard_link("/bin/busybox\0", "/usr/bin/unshare\0");
        hard_link("/bin/busybox\0", "/usr/bin/whoami\0");
        hard_link("/bin/busybox\0", "/usr/bin/vlock\0");
        hard_link("/bin/busybox\0", "/usr/bin/cryptpw\0");
        hard_link("/bin/busybox\0", "/usr/bin/cpio\0");
        hard_link("/bin/busybox\0", "/usr/bin/beep\0");
        hard_link("/bin/busybox\0", "/usr/bin/nmeter\0");
        hard_link("/bin/busybox\0", "/usr/bin/head\0");
        hard_link("/bin/busybox\0", "/usr/bin/blkdiscard\0");
        hard_link("/bin/busybox\0", "/usr/bin/bunzip2\0");
        hard_link("/bin/busybox\0", "/usr/bin/expr\0");
        hard_link("/bin/busybox\0", "/usr/bin/pstree\0");
        hard_link("/bin/busybox\0", "/usr/bin/traceroute6\0");
        hard_link("/bin/busybox\0", "/usr/bin/killall\0");
        hard_link("/bin/busybox\0", "/usr/bin/tree\0");
        hard_link("/bin/busybox\0", "/usr/bin/fallocate\0");
        hard_link("/bin/busybox\0", "/usr/bin/mkfifo\0");
        hard_link("/bin/busybox\0", "/usr/bin/who\0");
        hard_link("/bin/busybox\0", "/usr/bin/hd\0");
        hard_link("/bin/busybox\0", "/usr/bin/uniq\0");
        hard_link("/bin/busybox\0", "/usr/bin/lsusb\0");
        hard_link("/bin/busybox\0", "/usr/bin/ipcrm\0");
        hard_link("/bin/busybox\0", "/usr/bin/bzip2\0");
        hard_link("/bin/busybox\0", "/usr/bin/eject\0");
        hard_link("/bin/busybox\0", "/usr/bin/deallocvt\0");
        hard_link("/bin/busybox\0", "/usr/bin/wc\0");
        hard_link("/bin/busybox\0", "/usr/bin/fuser\0");
        hard_link("/bin/busybox\0", "/usr/bin/logger\0");
        hard_link("/bin/busybox\0", "/usr/bin/top\0");
        hard_link("/bin/busybox\0", "/usr/bin/timeout\0");
        hard_link("/bin/busybox\0", "/usr/bin/passwd\0");
        hard_link("/bin/busybox\0", "/usr/bin/sha3sum\0");
        hard_link("/bin/busybox\0", "/usr/bin/cmp\0");
        hard_link("/bin/busybox\0", "/usr/bin/pwdx\0");
        hard_link("/bin/busybox\0", "/usr/bin/shred\0");
        hard_link("/bin/busybox\0", "/usr/bin/pscan\0");
        hard_link("/bin/busybox\0", "/usr/bin/clear\0");
        hard_link("/bin/busybox\0", "/usr/bin/split\0");
        hard_link("/bin/busybox\0", "/usr/bin/vi\0");
        hard_link("/bin/busybox\0", "/usr/bin/dirname\0");
        hard_link("/bin/busybox\0", "/usr/bin/sha512sum\0");
        hard_link("/bin/busybox\0", "/usr/bin/test\0");
        hard_link("/bin/busybox\0", "/usr/bin/cksum\0");
        hard_link("/bin/busybox\0", "/usr/bin/id\0");
        hard_link("/bin/busybox\0", "/usr/bin/setkeycodes\0");
        hard_link("/bin/busybox\0", "/usr/bin/flock\0");
        hard_link("/bin/busybox\0", "/usr/bin/time\0");
        hard_link("/bin/busybox\0", "/usr/bin/env\0");
        hard_link("/bin/busybox\0", "/usr/bin/which\0");
        hard_link("/bin/busybox\0", "/usr/bin/less\0");
        hard_link("/bin/busybox\0", "/usr/bin/uptime\0");
        hard_link("/bin/busybox\0", "/usr/bin/traceroute\0");
        hard_link("/bin/busybox\0", "/usr/bin/tee\0");
        hard_link("/bin/busybox\0", "/usr/bin/yes\0");
        hard_link("/bin/busybox\0", "/usr/bin/realpath\0");
        hard_link("/bin/busybox\0", "/usr/bin/microcom\0");
        hard_link("/bin/busybox\0", "/usr/bin/unzip\0");
        hard_link("/bin/busybox\0", "/usr/bin/sum\0");
        hard_link("/bin/busybox\0", "/usr/bin/ttysize\0");
        hard_link("/bin/busybox\0", "/usr/bin/paste\0");
        hard_link("/bin/busybox\0", "/usr/bin/xzcat\0");
        hard_link("/bin/busybox\0", "/usr/bin/last\0");
        hard_link("/bin/busybox\0", "/usr/bin/bzcat\0");
        hard_link("/bin/busybox\0", "/usr/bin/[\0");
        hard_link("/bin/busybox\0", "/usr/bin/hostid\0");
        hard_link("/bin/busybox\0", "/usr/bin/renice\0");
        hard_link("/bin/busybox\0", "/usr/bin/nl\0");
        hard_link("/bin/busybox\0", "/usr/bin/shuf\0");
        hard_link("/bin/busybox\0", "/usr/bin/showkey\0");
        hard_link("/bin/busybox\0", "/usr/bin/tty\0");
        hard_link("/bin/busybox\0", "/usr/bin/chvt\0");
        hard_link("/bin/busybox\0", "/usr/bin/reset\0");
        hard_link("/bin/busybox\0", "/usr/bin/basename\0");
        hard_link("/bin/busybox\0", "/usr/bin/ipcs\0");
        hard_link("/bin/busybox\0", "/usr/bin/dos2unix\0");
        hard_link("/bin/busybox\0", "/usr/bin/openvt\0");
        hard_link("/bin/busybox\0", "/usr/bin/crontab\0");
        hard_link("/bin/busybox\0", "/usr/bin/lzcat\0");
        hard_link("/bin/busybox\0", "/usr/bin/unlink\0");
        hard_link("/bin/busybox\0", "/usr/bin/tac\0");
        hard_link("/bin/busybox\0", "/usr/bin/cut\0");
        hard_link("/bin/busybox\0", "/usr/bin/uudecode\0");
        hard_link("/bin/busybox\0", "/usr/bin/setsid\0");
        hard_link("/bin/busybox\0", "/usr/bin/awk\0");
        hard_link("/bin/busybox\0", "/usr/bin/sha256sum\0");
        hard_link("/bin/busybox\0", "/usr/bin/sha1sum\0");
        hard_link("/bin/busybox\0", "/usr/bin/nohup\0");
        hard_link("/bin/busybox\0", "/usr/bin/find\0");
        hard_link("/bin/busybox\0", "/usr/bin/cal\0");
        hard_link("/bin/busybox\0", "/usr/bin/free\0");
        hard_link("/bin/busybox\0", "/usr/bin/wget\0");
        hard_link("/bin/busybox\0", "/usr/bin/volname\0");
        hard_link("/bin/busybox\0", "/usr/bin/lzopcat\0");
        hard_link("/bin/busybox\0", "/usr/bin/expand\0");
        hard_link("/bin/busybox\0", "/usr/bin/pmap\0");
        hard_link("/bin/busybox\0", "/usr/bin/unlzma\0");
        hard_link("/bin/busybox\0", "/usr/bin/hexdump\0");
        hard_link("/bin/busybox\0", "/usr/bin/truncate\0");
        hard_link("/bin/busybox\0", "/usr/bin/sort\0");
        hard_link("/bin/busybox\0", "/usr/bin/printf\0");
        hard_link("/bin/busybox\0", "/usr/bin/fold\0");
        hard_link("/bin/busybox\0", "/usr/bin/du\0");
        hard_link("/bin/busybox\0", "/usr/bin/lzma\0");
        hard_link("/bin/busybox\0", "/usr/bin/readlink\0");
        hard_link("/bin/busybox\0", "/usr/bin/lsof\0");
        hard_link("/bin/busybox\0", "/usr/bin/md5sum\0");
        hard_link("/bin/busybox\0", "/usr/bin/factor\0");
        hard_link("/bin/busybox\0", "/usr/bin/unxz\0");
        hard_link("/bin/busybox\0", "/usr/bin/diff\0");
        hard_link("/bin/busybox\0", "/usr/bin/mkpasswd\0");

        symlinkat("/usr/bin/cc\0", "/usr/bin/gcc\0");

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

    run_sh("mkdir -p /home/noaxiom\0");

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
                "HOME=/home/noaxiom\0".as_ptr(),
                "GIT_PAGER=\0".as_ptr(),
                "CARGO_BUILD_JOBS=1\0".as_ptr(),
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
