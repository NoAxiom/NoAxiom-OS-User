#![no_std]
#![no_main]

extern crate alloc;

use alloc::{format, vec};

use libd::{
    println,
    syscall::{fork, pipe2, read, wait, write},
};

#[no_mangle]
fn main() -> i32 {
    let mut pipe_fd = [0u32; 2];
    pipe2(&mut pipe_fd, 0);
    println!("[pipe_test] pipe fd: {:?}", pipe_fd);

    const TEST_NUMS: usize = 1000;
    const TEST_EACH: usize = 4;
    let mut test_buf = [114; TEST_NUMS * TEST_EACH];
    let mut test_buf = &mut test_buf[..];

    for i in 0..TEST_NUMS {
        let sub_process_id = fork();
        if sub_process_id == 0 {
            let mut count = 0;
            loop {
                let mut buf = vec![0u8; TEST_EACH - count];
                let ret = read(pipe_fd[0] as usize, &mut buf);
                if ret > 0 {
                    for num in buf {
                        if num == 114 {
                            count += 1;
                        }
                    }
                    if count == TEST_EACH {
                        println!("child {}: read from pipe success, count: {}", i, count);
                        break;
                    }
                } else {
                    println!("child {}: read from pipe failed: {}", i, ret);
                }
            }
            return 0;
        } else if sub_process_id < 0 {
            println!("fork failed, ret: {}", sub_process_id);
            return -1;
        }
    }

    let mut rand: i32 = 0x12345678;
    while !test_buf.is_empty() {
        rand = rand.wrapping_mul(0x5DEECEDi32).wrapping_add(0xB);
        rand = (rand >> 16) & 0x7FFF;
        rand = (rand % test_buf.len() as i32);
        let write_len = (11).max(1).max(test_buf.len());
        // println!("[pipe_test] write to pipe: {:?}", &test_buf[..write_len]);
        write(pipe_fd[1] as usize, &test_buf[..write_len]);
        test_buf = &mut test_buf[write_len..];
    }

    loop {
        let mut exit_code: usize = 0;
        let tid = wait(-1, &mut exit_code);
        if tid < 0 {
            if tid == -10 {
                println!("[pipe_test] Initproc exited successfully");
            } else {
                println!("[pipe_test] wait failed, ret: {}", tid);
            }
            break;
        }
        println!("[pipe_test] wait tid: {}, exit_code: {}", tid, exit_code);
    }
    0
}
