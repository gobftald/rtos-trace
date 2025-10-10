# `systemview-target`

<!-- cargo-rdme start -->

RTOS tracing trait implementation for SEGGER SystemView.

SEGGER
[SystemView](https://www.segger.com/products/development-tools/systemview/) can
be used for free in non-commercial projects.

## Features

- `callbacks-os`: Check if RTOS supports tracing callbacks from SystemView.
- `callbacks-os-time`: Check if RTOS supports timestamp callback from SystemView.
- `callbacks-app`: Check if your application supports callback from SystemView.
- `log`: Activates global `log` over RTT.
- `cortex-m`: Enables Arm Cortex-M support.
- `ext-rtt-channels-*`: Co-existence with other RTT clients (e.g. defmt).

## Usage

If you are using an RTOS which supports [`rtos-trace`](https://docs.rs/rtos-trace/)
add the following dependencies:

```toml
# Cargo.toml
[dependencies]
rtos-trace = "0.2"
systemview-target = { version = "0.2", features = ["log", "callbacks-app", "callbacks-os", "callbacks-os-time", "cortex-m"] }
log = "0.4"
```

and add to your code

```ignore
// for tracing
use systemview_target::SystemView;
rtos_trace::global_trace!{SystemView}

static SYSTEMVIEW: systemview_target::SystemView = systemview_target::SystemView::new();

fn main() -> ! {
    SYSTEMVIEW.init();
    // for logs
    log::set_logger(&SYSTEMVIEW).ok();
    log::set_max_level(log::LevelFilter::Trace);
    /*..*/
}
```

## Co-existence with other RTT clients (e.g. defmt)

By default this crate defines the `_SEGGER_RTT` symbol. This conflicts with
other crates like [`defmt_rtt`](https://docs.rs/defmt_rtt/) which declare the
same global symbol.

To use this crate with `defmt` or other RTT clients, let your application depend
on [`rtt-target`](https://docs.rs/rtt-target/) rather than the `defmt_rtt` crate
and enable one of the `ext-rtt-channels-[N]` features of this crate.

The parameter `N` is an integer and defines the overall number of externally
allocated RTT channels.

Note: This feature requires at least version 0.2.1 of the `systemview-target`
crate and 0.6.2 of the `rtt-target` crate.

### Safety

The following conditions must be met by your application, otherwise it will
expose undefined behavior:

- The integer `N` of the enabled `ext-rtt-channels-[N]` feature must correspond
  exactly to the number of externally allocated channels.

- At least one up and down channel must be allocated without a buffer (see
  example below).

- Both pre-allocated channels must have the same channel id.

Note: Currently we only support symmetric configurations, i.e. the number of
available up and down channels must be equal.

### Example

To use `systemview-target` with `defmt` let your crate depend on `rtt-target`
and enable the appropriate feature corresponding to the number of channels
defined in your application. In this example we'll define two RTT channels:

```toml
# Cargo.toml
[dependencies]
defmt = "1.0"
rtos-trace = "0.2"
rtt-target = "0.6.2"
systemview-target = { version = "0.2.1", features = ["ext-rtt-channels-2", ...] }
```

Then we declare the RTT control block and `defmt` channel explicitly:

```ignore
use systemview_target::SystemView;
rtos_trace::global_trace!{SystemView}

static SYSTEMVIEW: systemview_target::SystemView = systemview_target::SystemView::new();

fn main() -> ! {
    let channels = rtt_init! {
        up: {
            0: { size: 1024, name: "Terminal" }
            1: { } // Pre-allocated RTT channel for SystemView.
        }
        down: {
            0: { size: 16, name: "Terminal" }
            1: { } // Pre-allocated RTT channel for SystemView.
        }
    };

    set_defmt_channel(channels.up.0);

    // Initialize SystemView _after_ allocating its RTT channel.
    SYSTEMVIEW.init();
}
```

Segger's SystemView code will now identify, allocate and use RTT channel one
while `defmt` owns channel zero.

<!-- cargo-rdme end -->
