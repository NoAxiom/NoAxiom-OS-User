pub const IOCTL_TESTCASE: usize = 0x114514;
pub const IOCTL_LOG: usize = 0x1919810;

pub mod ioctl_ltp {
    use super::*;
    pub const IOCTL_SWITCH_INTO_LTP: usize = 0;
    pub const IOCTL_SWITCH_OUTOF_LTP: usize = 1;
    pub fn switch_into_ltp() {
        crate::syscall::ioctl(1, IOCTL_TESTCASE, IOCTL_SWITCH_INTO_LTP)
    }
    pub fn switch_outof_ltp() {
        crate::syscall::ioctl(1, IOCTL_TESTCASE, IOCTL_SWITCH_OUTOF_LTP)
    }
}

pub mod ioctl_log {
    use super::*;
    pub const IOCTL_LOG_OFF: usize = 0;
    pub const IOCTL_LOG_ON: usize = 1;
    pub fn switch_log_off() {
        crate::syscall::ioctl(1, IOCTL_LOG, IOCTL_LOG_OFF);
    }
    pub fn switch_log_on() {
        crate::syscall::ioctl(1, IOCTL_LOG, IOCTL_LOG_ON);
    }
}
