# Notes

## Commands

```ps1
cargo new <crate name> - creates git as well
cargo test
cargo run <arguments>
rustup doc --std
rustup docs --book
rustup docs --cargo # Build Scripts
rustup update
rustup override set stable|nightly
```

## Crates

```ps1
- Errors: anyhow, error_chain, thiserror - https://www.shuttle.rs/blog/2022/06/30/error-handling # makes error handling easier
- dashmap # high performance multithreaded hashmap
- bindgen # automates bindings to C libs
- encoding # work with different encodings
- messagepack # like protobuf but without schema
- websocket protocol # like http2 but faster and binary
- hdrhistogram # for ping times study or for temperatures study on Arduino
```

## URLs

- [Crates available](https://crates.io/)
- [Performance improvements](https://endler.dev/2020/rust-compile-times/)
- [Web API testing sample](https://blog.logrocket.com/end-to-end-testing-for-rust-web-services/)

## Performance measurement

Turns out that warnings disabling `$env:RUSTFLAGS = "-Awarnings"` causes unnecessary rebuilds. Leaving the warnings be makes tests go fast.

In case performance measurements are needed here is how to compare options:

```powershell
    hyperfine `
        --warmup 1 `
        --shell "pwsh.exe -noprofile" `
        --prepare "Add-Content -Path .\src\bin\client.rs -Value ' '" `
        --export-markdown "test_performance.md" `
        -n "Build-in" "cargo test -q" `
        -n "NextTest" "cargo nextest run"
```
