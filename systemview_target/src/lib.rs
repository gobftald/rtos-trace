//! RTOS tracing trait implementation for SEGGER SystemView.
//!
//! SEGGER SystemView can be used for non-commercial project for free and is
//! available [`here`](https://www.segger.com/products/development-tools/systemview/).
//!
//! # Features
//!
//! - `callbacks-os`: Check if RTOS supports tracing callbacks from SystemView.
//! - `callbacks-os-time`: Check if RTOS supports timestamp callback from SystemView.
//! - `callbacks-app`: Check if your application supports callback from SystemView.
//! - `log`: Activates global `log` over RTT.
//! - `cortex-m`: Enables Arm Cortex-M support.
//!
//! # Usage
//!
//! If you are using an RTOS which supports [`rtos-trace`](https://docs.rs/rtos-trace/)
//! add the following dependencies:
//!
//! ```toml
//! # Cargo.toml
//! [dependencies]
//! rtos-trace = "0.1"
//! systemview-target = { version = "0.1", features = ["log", "callbacks-app", "callbacks-os", "callbacks-os-time", "cortex-m"] }
//! log = { version = "0.4", features = ["max_level_trace", "release_max_level_warn"] }
//! ```
//!
//! and add to your code
//! ```ignore
//! // for tracing
//! use systemview_target::SystemView;
//! rtos_trace::global_trace!{SystemView}
//!
//! static LOGGER: systemview_target::SystemView = systemview_target::SystemView::new();
//!
//! fn main() -> ! {
//!     LOGGER.init();
//!     // for logs
//!     log::set_logger(&LOGGER).ok();
//!     log::set_max_level(log::LevelFilter::Trace);
//!     /*..*/
//! }
//! ```

#![no_std]

#[cfg(not(any(feature = "cortex-m")))]
compile_error!("You must select a target architecture. Supported are: cortex-m");

#[cfg(feature = "log")]
pub mod log;
mod macros;
mod stub;
mod wrapper;

use core::ptr::null;
#[cfg(feature = "log")]
pub use heapless;
pub use rtos_trace::RtosTrace;
use rtos_trace::TaskInfo;
use wrapper::*;

pub struct SystemView;

impl SystemView {
    pub const fn new() -> SystemView {
        SystemView
    }

    pub fn init(&self) {
        unsafe {
            SEGGER_SYSVIEW_Conf();
        }
    }

    pub fn send_system_description(desc: &str) {
        unsafe {
            SEGGER_SYSVIEW_SendSysDesc(desc.as_ptr());
        }
    }
}

impl RtosTrace for SystemView {
    fn task_new(id: u32) {
        unsafe {
            SEGGER_SYSVIEW_OnTaskCreate(id);
        }
    }

    fn task_send_info(id: u32, info: TaskInfo) {
        let name = if info.name.is_empty() {
            null()
        } else {
            info.name.as_ptr()
        };
        let info = SEGGER_SYSVIEW_TASKINFO {
            TaskID: id,
            sName: name,
            Prio: info.priority,
            StackBase: info.stack_base as u32,
            StackSize: info.stack_size as u32,
        };
        unsafe {
            SEGGER_SYSVIEW_SendTaskInfo(&info);
        }
    }

    fn task_terminate(id: u32) {
        unsafe {
            SEGGER_SYSVIEW_OnTaskTerminate(id);
        }
    }

    fn task_exec_begin(id: u32) {
        unsafe {
            SEGGER_SYSVIEW_OnTaskStartExec(id);
        }
    }

    fn task_exec_end() {
        unsafe {
            SEGGER_SYSVIEW_OnTaskStopExec();
        }
    }

    fn task_ready_begin(id: u32) {
        unsafe {
            SEGGER_SYSVIEW_OnTaskStartReady(id);
        }
    }

    fn task_ready_end(id: u32) {
        unsafe {
            SEGGER_SYSVIEW_OnTaskStopReady(id, 0);
        }
    }

    fn system_idle() {
        unsafe {
            SEGGER_SYSVIEW_OnIdle();
        }
    }

    fn isr_enter() {
        unsafe {
            SEGGER_SYSVIEW_RecordEnterISR();
        }
    }

    fn isr_exit() {
        unsafe {
            SEGGER_SYSVIEW_RecordExitISR();
        }
    }

    fn isr_exit_to_scheduler() {
        unsafe {
            SEGGER_SYSVIEW_RecordExitISRToScheduler();
        }
    }

    fn marker(id: u32) {
        unsafe {
            SEGGER_SYSVIEW_Mark(id);
        }
    }

    fn marker_begin(id: u32) {
        unsafe {
            SEGGER_SYSVIEW_MarkStart(id);
        }
    }

    fn marker_end(id: u32) {
        unsafe {
            SEGGER_SYSVIEW_MarkStop(id);
        }
    }
}
