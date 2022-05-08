```shell
# main
# Windows PowerShell
cargo run | Out-File -encoding ASCII asset/image.ppm
# macOS Zsh
cargo run > asset/image.ppm

# test
# vec3
cargo test --test vec3 -- --color always --show-output
```