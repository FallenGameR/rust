# Commands
cargo new <crate name> - creates git as well
cargo test
cargo run <arguments>
rustup doc --std
rustup update

# URLs
crates.io


# Hyperfine


 ## Not cached


 ## Cached

   - 236.8 ms
       - 209.7 ms




--prepare





```powershell

hyperfine `
    --warmup 1 `
    --shell "pwsh.exe -noprofile" `
    --prepare "Add-Content -Path .\src\bin\client.rs -Value ' '" `
    --export-markdown "test_performance.md" `
    -n "Build-in" "cargo test -q" `
    -n "NextTest" "cargo nextest run"

```