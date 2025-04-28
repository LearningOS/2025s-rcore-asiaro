//! Process management syscalls
use crate::{
    task::{exit_current_and_run_next, suspend_current_and_run_next},
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}



#[repr(usize)]
#[derive(Debug, PartialEq)]
pub enum TraceRequest {
    /// 读取用户空间内存的一个字节
    /// id参数被视为 *const u8
    ReadMemory = 0,
    
    /// 写入用户空间内存的一个字节
    /// id参数被视为 *mut u8，data的最低字节被写入
    WriteMemory = 1,
    
    /// 查询系统调用统计
    /// id参数指定要查询的系统调用号
    QuerySyscall = 2,
    
    /// 无效请求
    Invalid,
}
/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    trace!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// get time with second and microsecond
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: us / 1_000_000,
            usec: us % 1_000_000,
        };
    }
    0
}

// TODO: implement the syscall
pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    // trace!("kernel: sys_trace");
    // -1
    match _trace_request{
        TraceRequest::ReadMemory=>{
            let addr =id as *const u8;
            let byte=unsafe{*addr}
            byte as isize
        }
        TraceRequest::WriteMemory>{
            let addr = id as *mut u8;
            unsafe { *addr = data as u8 };
                0
        }
        TraceRequest::QuerySyscall => {
            // 查询系统调用统计
            if id <= MAX_SYSCALL_NUM {
                task.syscall_count[id] as isize
            } else {
                -1
            }
        }
        TraceRequest::Invalid => -1,

    }



}
