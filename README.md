```shell
# main
cargo run | Out-File -encoding ASCII asset/image.ppm

# test
# vec3
cargo test --test vec3 -- --color always --show-output
```