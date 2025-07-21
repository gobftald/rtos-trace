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

    #[cfg(not(feature = "callbacks-os-time"))]
    let os_time = "_DUMMY";
    #[cfg(feature = "callbacks-os-time")]
    let os_time = "CALLBACKS_OS_TIME";

    #[cfg(not(feature = "callbacks-os"))]
    let os_callbacks = "_DUMMY";
    #[cfg(feature = "callbacks-os")]
    let os_callbacks = "CALLBACKS_OS";

    // Compile SystemView
    cc::Build::new()
        .file("lib/SEGGER/SEGGER_SYSVIEW.c")
        .file("lib/SEGGER/SEGGER_RTT.c")
        .file("lib/SEGGER/SEGGER_RTT_ASM_ARMv7M.S")
        .file("lib/impl/systemview_conf_cortex_m.c")
        .include("lib/SEGGER")
        .include("lib/Config")
        .define(os_time, "")
        .define(os_callbacks, "")
        .compile("systemview");
}
