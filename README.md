# Rust-Python

**WORK IN PROGRESS - DO NOT USE THIS CODE!!!!**

A python interpreter written in Rust.

## Outline

CPython has the global interpreter lock, which turns possibly concurrent
programs into serial programs. My understanding is that this is used to
ensure reference counts are accurate.

Rust has concurrent primitives such as
[`Arc`](https://doc.rust-lang.org/std/sync/struct.Arc.html) which handle
atomic reference counting.

Can we write a bytecode-interpreter that is concurrent?
