# VoidCraft Cross-Platform Support

> Similar to [bevy_platform](https://github.com/bevyengine/bevy/blob/main/crates/bevy_platform/README.md).

Rust’s standard library is organized into three layers:

- core: the language core functionality.
- alloc: adds allocation-related APIs and common containers such as `String`.
- std: adds OS APIs (files, threads, etc.).

Ideally, engine code should target platforms that support `core` and be compatible with `alloc` (which may require providing an allocator).
`std` exposes operating-system interfaces, so each platform must provide its own implementation.

Rust provides extensive cross-platform work, but it cannot cover every host and embedded target.

> See: [Rust Platform Support](https://doc.rust-lang.org/nightly/rustc/platform-support.html)

A common solution is to define a thin abstraction layer for the OS functionality you need and supply platform-specific implementations.
This is a large engineering effort. This crate defines the basic abstraction layer but currently only provides an implementation based on `std`.

Good news: the `std`-based implementation covers major platforms (Windows, Linux, Android) and is sufficient for the demo projects in this repository.

