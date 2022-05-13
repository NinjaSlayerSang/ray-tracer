```shell
# main
# Windows PowerShell
cargo run | Out-File -encoding ASCII asset/image.ppm
# macOS Zsh
cargo run > asset/image.ppm

# test
cargo test --test test-crate-name -- --color always --show-output
```