extern crate rlibc;
use rlibc::memcmp;
#[no_mangle]
pub unsafe extern "C" fn bcmp(s1: *const u8, s2: *const u8, n: usize) -> i32 {
    memcmp(s1, s2, n)
}
#[no_mangle]
pub extern "C" fn _Unwind_Resume() {}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

#[inline]
unsafe fn my_strlen(p: *const u8) -> usize {
    let mut n = 0;
    while *p.offset(n as isize) != 0 {
        n += 1;
    }
    n
}

#[no_mangle]
pub extern "C" fn strlen(p: *const u8) -> usize {
    unsafe { my_strlen(p) }
}
