// io_uring相关的数据结构基于kernel 5.1
// author : zhaos@nbjl.nankai.edu.cn

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct io_sqring_offsets {
    pub head: u32,
    pub tail: u32,
    pub ring_mask: u32,
    pub ring_entries: u32,
    pub flags: u32,
    pub dropped: u32,
    pub array: u32,
    pub resv1: u32,
    pub resv2: u64,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct io_cqring_offsets {
    pub head: u32,
    pub tail: u32,
    pub ring_mask: u32,
    pub ring_entries: u32,
    pub overflow: u32,
    pub cqes: u32,
    pub resv: [u64; 2],
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct io_uring_params {
    pub sq_entries: u32,
    pub cq_entries: u32,
    pub flags: u32,
    pub sq_thread_cpu: u32,
    pub sq_thread_idle: u32,
    pub resv: [u32; 5],
    pub sq_off: io_sqring_offsets,
    pub cq_off: io_cqring_offsets,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union io_uring_sqe_union1 {
    pub rw_flags: i32,
    pub fsync_flags: u32,
    pub poll_events: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union io_uring_sqe_union2 {
    pub buf_index: u16,
    pub __pad2: [u64; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_uring_sqe {
    pub opcode: u8,
    pub flags: u8,
    pub ioprio: u16,
    pub fd: i32,
    pub off: u64,
    pub addr: u64,
    pub len: u32,
    pub sqe_union1: io_uring_sqe_union1,
    pub user_data: u32,
    pub sqe_union2: io_uring_sqe_union2,
}

#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct io_uring_cqe {
    pub user_data: u64,
    pub res: i32,
    pub flags: u32,
}

