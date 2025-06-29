use libd::{
    println,
    syscall::{utils::OpenFlags, *},
};

#[cfg(target_arch = "loongarch64")]
use crate::{ltp_script_la::LTP_SH, run_sh};
#[cfg(target_arch = "riscv64")]
use crate::{ltp_script_rv::LTP_SH, run_sh};

pub fn run_ltp() {
    chdir("/\0");
    let fd = open(
        "/rv-musl-custom_ltptest.sh\0",
        OpenFlags::O_CREATE | OpenFlags::O_WRONLY,
    );
    if fd < 0 {
        println!("Failed to open custom_ltptest.sh, ret: {}", fd);
        return;
    }
    write(fd as usize, LTP_SH.as_bytes());

    run_sh("./rv-musl-custom_ltptest.sh\0");
}
