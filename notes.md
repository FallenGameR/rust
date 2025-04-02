# Notes

## Commands

```ps1
cargo new <crate name> - creates git as well
cargo test
cargo run <arguments>
cargo add <crate name> - adds crate to the current project
cargo doc --open # generate docs for the current rust project
rustup doc --std
rustup docs --book
rustup docs --cargo # Build Scripts
rustup update
rustup override set stable|nightly
cargo +nightly build -p rust-analyzer --bin rust-analyzer -Z timings --release
```

## Patterns

- Command - <https://smallcultfollowing.com/babysteps/blog/2015/12/18/rayon-data-parallelism-in-rust/#appendix-implementing-sequential-fallback-without-code-duplication>
- Others - <https://rust-unofficial.github.io/patterns/>

## Crates

```ps1
- Errors
  - anyhow
  - error_chain
  - thiserror - short and useful custom errors via macros
  - failure - adds callstack to errors, useful in app code, but not expected in lib code
  - better-panic - colorful panics, but will conflict with failure
  - color-backtrace - color stack traces of failure manually
- https://fasterthanli.me/series/making-our-own-ping/part-10 # error handling use cases from Amos
- https://www.shuttle.rs/blog/2022/06/30/error-handling # makes error handling easier
- Colorful errors - https://lib.rs/crates/color-eyre # panics are using nice colored output in the console
- dashmap # high performance multithreaded hashmap
- bindgen # automates bindings to C libs
- encoding # work with different encodings
- messagepack # like protobuf but without schema
- websocket protocol # like http2 but faster and binary
- hdrhistogram # for ping times study or for temperatures study on Arduino
- maplit # more readable syntax for hashmap initialization
```

## Error notes

- Make errors that are function specific - they will have a mathcing name and be focused at only a single domain area.

## URLs

- [Crates available](https://crates.io/)
- [Performance improvements](https://endler.dev/2020/rust-compile-times/)
- [Web API testing sample](https://blog.logrocket.com/end-to-end-testing-for-rust-web-services/)
- [x64dbg](https://x64dbg.com/) open source windows debugger

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

## Performance tips

Enable link time optimization so calls across different crates will be optimized away.

```text
[profile.release]
lto = true
```

## rust-analyzer

- Ctrl+Alt: preview the types deduced by the compiler
- rust-analyzer: Expand macro recursively - expand macro
- Alt+Shift+(Left|Right) - semantic block selection
- Ctrl+Shift+O - symbol or section search
- "Result {result} is {2 + 2}".(format|panic|println|log...) - expands string to formatted output statement
- highlight related - cursor on something highlights all related control statements
- rust-analyzer: Join lines - collapse array definitions in a smart way
- postfix completions: .if .match .while .let .call .pd tfn tmod
- rust-analyzer: Find matching brace - goes to the matching brace
- rust-analyzer: Move Item Up|Down - moves block of code up or down semantically
- `// foo($a, $b) ==>> ($a).foo($b)` - semantic replace, use code assist to apply, must be a valid replacement
- Expland impl Trait to generic in code like `fn foo(bar: impl Bar) {}`
