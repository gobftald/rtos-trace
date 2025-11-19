# `rtos-trace`

<!-- cargo-rdme start -->

Set of traits used to trace RTOS internals.

## Features

- `trace_impl`: Activates tracing function (on by default). Can be used by
  the RTOS to deactivate tracing.

## Implementation

The backend is required implement [`RtosTrace`](https://docs.rs/rtos-trace/latest/rtos_trace/trait.RtosTrace.html).

Existing implementation:

- [SEGGER SystemView](https://docs.rs/systemview-target/)

## Usage

### RTOS

The RTOS can implement [`RtosTraceOSCallbacks`](https://docs.rs/rtos-trace/latest/rtos_trace/trait.RtosTraceOSCallbacks.html)
to provide additional information upon request from the tracing software. For
example:

```rust
rtos_trace::global_os_callbacks!{Scheduler}

impl rtos_trace::RtosTraceOSCallbacks for Scheduler {
    fn task_list() {
        /*..*/
        for task in tasks.iter() {
            trace::task_send_info(task.id(), task.info());
        }
    }
    /*..*/
}
```

Usage for the RTOS maintainer is simple:

```rust
use rtos_trace::{RtosTrace, trace}

pub fn spawn_task(/*..*/) {
    /*..*/
    trace::task_new(task_id);
}
```

### Application

Similar to a global logger the user must provide a tracing backend, i.e.:

```ignore
use systemview_target::SystemView;
rtos_trace::global_trace!{SystemView}
```

The user can implement [`RtosTraceApplicationCallbacks`] to provide additional
information upon request from the tracing software. For example:

```rust
struct Application;
rtos_trace::global_application_callbacks!{Application}

impl rtos_trace::RtosTraceApplicationCallbacks for Application {
    fn system_description() {
        systemview_target::send_system_desc_app_name!("Espresso Machine");
        systemview_target::send_system_desc_device!("STM32F769NI");
        systemview_target::send_system_desc_core!("Cortex-M7");
        systemview_target::send_system_desc_os!("Bern RTOS");
        systemview_target::send_system_desc_interrupt!(15, "SysTick");
    }
    /*..*/
}

```

<!-- cargo-rdme end -->
