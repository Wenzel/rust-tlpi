# rust-tlpi

> Exercises from The Linux Programing Interface course, reimplemented in Rust

## Requirements

- `cargo`

## Build

    cargo build

## Run

Exercises are located in `./target/debug/*`

## Notes

We are not using the Rust standard library because we absolutely want to call
the original functions of the `glibc` with the same arguments.

Therefore we use the crate `nix`, which is a Rust wrapper on top of `glibc`,
defining `safe` and Rust compatible functions, which will call the `glibc`
unsafe functions.
