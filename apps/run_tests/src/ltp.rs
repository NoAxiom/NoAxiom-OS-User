use libd::{
    println,
    syscall::{utils::OpenFlags, *},
};

use crate::run_sh;

pub fn run_ltp() {
    #[cfg(feature = "ltp_full")]
    use crate::ltp_full::LTP_SH;
    #[cfg(not(feature = "ltp_full"))]
    #[cfg(target_arch = "loongarch64")]
    use crate::ltp_script_la::LTP_SH;
    #[cfg(not(feature = "ltp_full"))]
    #[cfg(target_arch = "riscv64")]
    use crate::ltp_script_rv::LTP_SH;
    chdir("/\0");
    let fd = open(
        "/custom_ltptest.sh\0",
        OpenFlags::O_CREATE | OpenFlags::O_WRONLY,
    );
    if fd < 0 {
        println!("Failed to open custom_ltptest.sh, ret: {}", fd);
        return;
    }
    write(fd as usize, LTP_SH.as_bytes());

    run_sh("./custom_ltptest.sh\0");
}
