# Changelog

## Unreleased

### crate systemview_target [0.3.0]

Fixes:

- Breaking Change: The log dependency no longer fixes the max logging level
  which conflicted with features set by library users. To upgrade please set the
  corresponding features in your application's Cargo.toml.
- Changes introduced in v0.2.1 were only documented in the README, not in
  lib.rs. Crate documentation is now included from the README to prevent drift.

### crate rtos-trace [0.2.1]

Fixes:

- Crate documentation is now included from the README to prevent drift.

## [0.2.1] - 2025-10-16

### crate systemview_target

Features:

- New `ext-rtt-channels-*` feature that allows systemview_target to share the
  RTT control block with other crates (e.g. defmt).

## [0.2.0] - 2025-07-23

### crate rtos_trace

Features:

- support for oneshot and post-mortem tracing \*)
- support for named markers \*)
- convenience function for stackless async tasks

\*) breaking change - requires simultaneous upgrade of the
systemview_target to v0.2.0

### crate systemview_target

Features:

- cortex-m as default feature (non-breaking change as this is the only
  available target anyway)
- implement support for oneshot and post-mortem tracing
- implement support for named markers

Fixes:

- rebuild after change to sysview config

Chore:

- upgraded dependencies
- upgraded systemview source code to 3.60a

Performance:

- removed OS callback stub

### build and development

Chore:

- VSCode default settings
