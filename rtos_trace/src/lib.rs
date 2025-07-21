//! Set of traits used to trace RTOS internals.
//!
//! # Features
//!
//! - `trace_impl`: Activates tracing function (on by default). Can be used by
//!    the RTOS to deactivate tracing.
//!
//! # Implementation
//!
//! The backend is required implement [`RtosTrace`](crate::RtosTrace).
//!
//! Existing implementation:
//! - [SEGGER SystemView](https://docs.rs/systemview-target/)
//!
//! # Usage
//!
//! ## RTOS
//!
//! The RTOS can implement [`RtosTraceOSCallbacks`](crate::RtosTraceOSCallbacks) to provide additional
//! information upon request from the tracing software. For example:
//! ```ignore
//! rtos_trace::global_os_callbacks!{Scheduler}
//!
//! impl rtos_trace::RtosTraceOSCallbacks for Scheduler {
//!     fn task_list() {
//!         /*..*/
//!         for task in tasks.iter() {
//!             trace::task_send_info(task.id(), task.info());
//!         }
//!     }
//!     /*..*/
//! }
//! ```
//!
//! Usage for the RTOS maintainer is simple:
//! ```ignore
//! use rtos_trace::{RtosTrace, trace}
//!
//! pub fn spawn_task(/*..*/) {
//!     /*..*/
//!     trace::task_new(task_id);
//! }
//! ```
//!
//! ## Application
//!
//! Similar to a global logger the user must provide a tracing backend, i.e.:
//! ```ignore
//! use systemview_target::SystemView;
//! rtos_trace::global_trace!{SystemView}
//! ```
//!
//! The user can implement [`RtosTraceApplicationCallbacks`] to provide
//! additional information upon request from the tracing software. For example:
//! ```ignore
//! struct Application;
//! rtos_trace::global_application_callbacks!{Application}
//!
//! impl rtos_trace::RtosTraceApplicationCallbacks for Application {
//!     fn system_description() {
//!         systemview_target::send_system_desc_app_name!("Espresso Machine");
//!         systemview_target::send_system_desc_device!("STM32F769NI");
//!         systemview_target::send_system_desc_core!("Cortex-M7");
//!         systemview_target::send_system_desc_os!("Bern RTOS");
//!         systemview_target::send_system_desc_interrupt!(15, "SysTick");
//!     }
//!     /*..*/
//! }
//!
//! ```

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
    /// A new task with `id` was created.
    fn task_new(id: u32);
    /// The task with `id` has `info` attributes.
    fn task_send_info(id: u32, info: TaskInfo);
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
