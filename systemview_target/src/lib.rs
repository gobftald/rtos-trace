#![doc = include_str!("../README.md")]
#![no_std]

//#[cfg(not(any(feature = "cortex-m")))]
//compile_error!("You must select a target architecture. Supported are: cortex-m");

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
pub use wrapper::*;

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
    fn start() {
        unsafe {
            SEGGER_SYSVIEW_Start();
        }
    }

    fn stop() {
        unsafe {
            SEGGER_SYSVIEW_Stop();
        }
    }

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
            StackUsage: 0,
        };
        unsafe {
            SEGGER_SYSVIEW_SendTaskInfo(&info);
        }
    }

    fn task_new_stackless(id: u32, name: &'static str, priority: u32) {
        Self::task_new(id);
        Self::task_send_info(
            id,
            TaskInfo {
                name,
                priority,
                stack_base: 0,
                stack_size: 0,
            },
        );
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

    fn name_marker(id: u32, name: &'static str) {
        unsafe {
            SEGGER_SYSVIEW_NameMarker(id, name.as_ptr());
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

impl Default for SystemView {
    fn default() -> Self {
        Self::new()
    }
}
