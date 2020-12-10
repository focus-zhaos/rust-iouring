//extern crate libc;

pub mod iouring;

use libc::*;
use iouring::*;

pub const __NR_io_uring_setup: c_int = 425;
pub const __NR_io_uring_enter: c_int = 426;
pub const __NR_io_uring_register: c_int =427;

//rust libc 0.2.81 docs中有这个宏，但我导入的时候却告诉我没有，我猜这跟我本地libc和kernel版本环境有关系
pub const MAP_POPULATE: c_int = 0x08000;

pub const IORING_OFF_SQ_RING: c_ulonglong = 0;
pub const IORING_OFF_CQ_RING: c_ulonglong = 0x8000000;
pub const IORING_OFF_SQES: c_ulonglong = 0x10000000;

pub unsafe fn io_uring_setup(entries: c_uint, p: *mut io_uring_params) -> c_int {
    syscall(__NR_io_uring_setup, entries, p)
}

pub unsafe fn io_uring_enter(ring_fd: c_int, to_submit: c_uint, min_complete: c_uint, flags: c_uint) -> c_int {
    let null_ptr: *const u16 = std::ptr::null();
    syscall(__NR_io_uring_enter, ring_fd, to_submit, min_complete, flags, null_ptr, 0)
}

pub unsafe fn io_uring_register(ring_fd: c_int, opcode: c_uint, arg: *const c_void, nr_args: c_uint) -> c_int {
    syscall(__NR_io_uring_register, ring_fd, opcode, arg, nr_args)
}