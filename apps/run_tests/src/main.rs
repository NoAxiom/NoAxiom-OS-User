#![no_std]
#![no_main]

extern crate alloc;

use alloc::format;

use libd::{
    println,
    syscall::{utils::OpenFlags, *},
};

#[no_mangle]
fn main() -> i32 {
    let dir_fd = open("/\0", OpenFlags::O_DIRECTORY);
    if dir_fd < 0 {
        println!("Failed to open root directory");
        return 1;
    }
    let files = getdents(dir_fd as usize, &mut [0u8; 1024]).unwrap_or_default();
    close(dir_fd as usize);
    for file in files {
        if file.ends_with(".sh") {
            let path = format!("/{}", file);
            println!("Executing script: {}", path);
            let pid = fork();
            if pid == 0 {
                let ret = execve(
                    &path,
                    &[path.as_str().as_ptr(), core::ptr::null::<u8>()],
                    &[
                        "PATH=/\0".as_ptr(),
                        "LD_LIBRARY_PATH=/\0".as_ptr(),
                        core::ptr::null(),
                    ],
                );
                println!("Failed to exec {}: return code {}", path, ret);
                return 1;
            } else if pid > 0 {
                let mut exit_code = 0;
                wait(pid, &mut exit_code);
                println!("Script {} exited with code {}", path, exit_code);
            } else {
                println!("Failed to fork");
            }
        }
    }
    0
}
