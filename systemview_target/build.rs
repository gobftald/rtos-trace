use std::env;
use std::path::PathBuf;

fn main() {
    // Create SystemView bindings
    println!("cargo:rerun-if-changed=src/wrapper.h");
    println!("cargo:rerun-if-changed=lib/impl/systemview_conf_cortex_m.c");
    let bindings = bindgen::Builder::default()
        // prefix `cty` instead of `std` for `no_std`
        .ctypes_prefix("cty")
        .use_core()
        .header("src/wrapper.h")
        .clang_arg("-Ilib/Config")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Compile SystemView
    let mut build = cc::Build::new();
    build
        .file("lib/SEGGER/SEGGER_SYSVIEW.c")
        .file("lib/impl/systemview_conf_cortex_m.c")
        .include("lib/SEGGER")
        .include("lib/Config")
        .file("lib/SEGGER/SEGGER_RTT.c")
        .file("lib/SEGGER/SEGGER_RTT_ASM_ARMv7M.S");

    #[cfg(feature = "callbacks-os-time")]
    build.define("SYSTEMVIEW_TARGET_CALLBACKS_OS_TIME", "");

    #[cfg(feature = "callbacks-os")]
    build.define("SYSTEMVIEW_TARGET_CALLBACKS_OS", "");

    #[cfg(feature = "_ext-rtt-channels")]
    build.define("SYSTEMVIEW_TARGET_EXT_RTT_CHANNELS", "");

    #[cfg(any(feature = "ext-rtt-channels-2", not(feature = "_ext-rtt-channels")))]
    let num_rtt_buffers = "2";

    #[cfg(feature = "ext-rtt-channels-3")]
    let num_rtt_buffers = "3";

    #[cfg(feature = "ext-rtt-channels-4")]
    let num_rtt_buffers = "4";

    #[cfg(feature = "ext-rtt-channels-5")]
    let num_rtt_buffers = "5";

    build.define("SEGGER_RTT_MAX_NUM_UP_BUFFERS", num_rtt_buffers);
    build.define("SEGGER_RTT_MAX_NUM_DOWN_BUFFERS", num_rtt_buffers);

    build.compile("systemview");
}

#[cfg(any(
    all(
        feature = "ext-rtt-channels-2",
        any(
            feature = "ext-rtt-channels-3",
            feature = "ext-rtt-channels-4",
            feature = "ext-rtt-channels-5"
        )
    ),
    all(
        feature = "ext-rtt-channels-3",
        any(
            feature = "ext-rtt-channels-2",
            feature = "ext-rtt-channels-4",
            feature = "ext-rtt-channels-5"
        )
    ),
    all(
        feature = "ext-rtt-channels-4",
        any(
            feature = "ext-rtt-channels-2",
            feature = "ext-rtt-channels-3",
            feature = "ext-rtt-channels-5"
        )
    ),
    all(
        feature = "ext-rtt-channels-5",
        any(
            feature = "ext-rtt-channels-2",
            feature = "ext-rtt-channels-3",
            feature = "ext-rtt-channels-4"
        )
    ),
))]
compile_error!("Please select only one of the 'ext-rtt-channels-*' features.");
