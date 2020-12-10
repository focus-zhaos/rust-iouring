extern crate libc;


mod uring;

use uring::*;
use uring::iouring::*;
use std::mem::size_of;
use libc::{mmap, PROT_READ, PROT_WRITE, MAP_SHARED};

fn main() {
    //default可以默认构造有#[derive(Default)]标示的结构体
    let mut p: io_uring_params = Default::default(); 
    let fd: i32 = unsafe{io_uring_setup(128, &mut p)};
    
    let sq_off: io_sqring_offsets = p.sq_off;
    let cq_off: io_cqring_offsets = p.cq_off;

    let sring_size = sq_off.array as usize + p.sq_entries as usize * size_of::<u32>();
    let cring_size = cq_off.cqes as usize + p.cq_entries as usize * size_of::<io_uring_cqe>();
    
    let null_ptr: *const libc::c_void = std::ptr::null();
    let sq_ptr = unsafe { mmap(null_ptr as *mut libc::c_void, sring_size, PROT_READ | PROT_WRITE, MAP_SHARED | MAP_POPULATE, fd, IORING_OFF_SQ_RING as i64) };


    println!("Hello, world!");
}
