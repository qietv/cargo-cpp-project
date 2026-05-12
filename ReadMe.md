# cargo-cpp-project

Template for building C/C++ projects with Rust Cargo infrastructure.

## About

This project is a starter template for building C/C++ applications while
using Cargo as the main build interface.

This repository shows how to keep Cargo as the build entry point while the
actual program entry point lives in C++. The Rust side is intentionally kept
to the smallest possible surface so Cargo can drive compilation and linking.

## Overview

The project demonstrates a simple hybrid layout:

- `build.rs` compiles `main.cpp` with the `cc` crate
- `dummy.rs` provides the Rust binary target Cargo expects
- `main.cpp` contains the real application entry point

This makes the repository a practical base for new C/C++ projects that want
to reuse Cargo's build workflow, dependency handling, and command interface.

## Project Layout

- `main.cpp`
	Implements the real `main` function and program logic.
- `build.rs`
	Compiles the C++ source file and forwards the generated object files to
	Cargo for linking.
- `dummy.rs`
	Acts as a placeholder Rust target so the package is accepted as a valid
	Cargo binary crate.

## Requirements

- Rust toolchain with Cargo installed
- A working C++ compiler supported by the `cc` crate

## Build

```bash
cargo build
```

## Run

```bash
cargo run
```

Expected output:

```text
Hello from C++ project built with Cargo!
```

## Why `dummy.rs` is required

Cargo expects a package to expose a Rust target such as `src/main.rs` or an
explicitly declared binary source. In this project, the real executable entry
point is implemented in C++, so Cargo still needs a minimal Rust file to treat
the package as a valid binary crate.

`dummy.rs` exists only to satisfy that requirement:

- it gives Cargo a binary target to build
- it does not contain the real application logic
- it uses `#![no_main]` because the actual `main` function comes from C++
- it uses `#![no_std]` and a minimal panic handler to satisfy Rust's build
	requirements

In practice, `dummy.rs` is structural glue for Cargo, while `main.cpp`
contains the executable's real entry point.

## How It Works

When you run `cargo build` or `cargo run`, Cargo executes `build.rs` first.
The build script compiles `main.cpp` with the `cc` crate, then passes the
resulting object files back to Cargo through linker arguments.

The Rust target defined by `dummy.rs` allows Cargo to complete the package
build, but the final executable behavior comes from the C++ program.

## Use Case

Use this repository as a minimal reference if you want to:

- start a new C or C++ project with Cargo as the build entry point
- prototype mixed Rust and C++ build flows
- keep native compilation under Cargo's command interface
