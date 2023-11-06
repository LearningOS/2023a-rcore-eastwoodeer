//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{exit_current_and_run_next, suspend_current_and_run_next, TaskStatus},
    timer::get_time_us,
};

#[repr(C)]
#[derive(Debug)]
/// TimeVal
pub struct TimeVal {
    /// sec
    pub sec: usize,
    /// usec
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
}

impl TaskInfo {
    /// init new TaskInfo
    pub fn zero_init() -> Self {
        Self {
            status: TaskStatus::Ready,
            syscall_times: [0; MAX_SYSCALL_NUM],
            time: 0,
        }
    }
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
    let now = get_time_us();
    unsafe {
        *ts = TimeVal {
            sec: now / 1_000_000,
            usec: now % 1_000_000,
        };
    }
    
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    let info = crate::task::get_task_info();
    unsafe {
        (*ti).status = info.0;
        (*ti).time = info.1;
        (*ti).syscall_times = info.2;
    }
    0
}
