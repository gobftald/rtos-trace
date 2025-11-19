#![doc = include_str!("../README.md")]
#![no_std]

mod macros;
pub mod trace;

/// Task info block.
pub struct TaskInfo {
    /// Names as static string.
    pub name: &'static str,
    /// Task priority number.
    pub priority: u32,
    /// Start address of the stack.
    pub stack_base: usize,
    /// Size of the stack in bytes.
    pub stack_size: usize,
}

/// Collection of tracing functions which are called by the RTOS.
pub trait RtosTrace {
    /// Start tracing.
    fn start();
    /// Stop tracing.
    fn stop();

    /// A new task with `id` was created.
    fn task_new(id: u32);
    /// The task with `id` has `info` attributes.
    fn task_send_info(id: u32, info: TaskInfo);
    /// Convenience function to create a new task with a name only.
    fn task_new_stackless(id: u32, name: &'static str, priority: u32);
    /// The task with `id` has been terminated.
    fn task_terminate(id: u32);
    /// The task with `id` will start to run on the CPU now.
    fn task_exec_begin(id: u32);
    /// Execution of the current task has ended.
    fn task_exec_end();
    /// The task with `id` is ready to run.
    fn task_ready_begin(id: u32);
    /// The task with `id` is being blocked/suspended.
    fn task_ready_end(id: u32);

    /// The RTOS enters idle mode.
    fn system_idle();

    /// Enter an ISR.
    fn isr_enter();
    /// Exit an ISR.
    fn isr_exit();
    /// Exit an ISR to the scheduler.
    fn isr_exit_to_scheduler();

    /// Create a new marker with `id`.
    fn name_marker(id: u32, name: &'static str);
    /// Create a new marker with `id`.
    fn marker(id: u32);
    /// Begin event of marker with `id`.
    fn marker_begin(id: u32);
    /// End event of marker with `id`.
    fn marker_end(id: u32);
}

/// Callbacks to the OS invoked by the tracing system.
/// This trait can be implemented in the RTOS.
pub trait RtosTraceOSCallbacks {
    /// Send a list of all tasks to the tracing system.
    fn task_list();
    /// Get system time in microseconds.
    fn time() -> u64;
}

/// Callbacks to the application invoked by the tracing system.
/// This trait can be implemented by user.
pub trait RtosTraceApplicationCallbacks {
    /// Send a system and application description to the tracing system.
    fn system_description();
    /// Get system clock in Hertz.
    fn sysclock() -> u32;
}
