use libd::{
    println,
    syscall::{utils::OpenFlags, *},
};

use crate::{ltp_script::LTP_RV, run_sh};

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
    write(fd as usize, LTP_RV.as_bytes());

    run_sh("./rv-musl-custom_ltptest.sh\0");
}
