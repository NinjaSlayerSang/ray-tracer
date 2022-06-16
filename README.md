# Ray Tracing by Rust

## Learning Project

This is a project for studying **Ray Tracing** algorithm & **Rust** language, refering to [*Ray Tracing in One Weekend*](https://raytracing.github.io/books/RayTracingInOneWeekend.html) but implemented by Rust.

## Ray Tracing Guidebook

An excellent series of primer tutorial books for this fascinating Computer Graphics technology: [*Ray Tracing in One Weekend — The Book Series*](https://raytracing.github.io/)

## Multiple Threads

Render rays' color in multiple threads, only invloving Rust standard library.
Learn how to use ***std::sync::Arc*** sharing pointers among threads, ***std::sync::mpsc::channel*** sending messages between threads, and ***std::sync::Mutex*** & ***std::sync::Condvar*** implementing a ***Semaphore*** to control the amount of active threads.

## Run

Install ***Rust toolchain*** before running.

```shell
# pinballs
cargo run --example pinballs --release -- pinballs.ppm
# marble texture
cargo run --example marble --release -- marble.ppm
# add "--release" unless you have enough patience to wait for much longer time...

# test
cargo test --test ${test_crate_name} -- --color always --show-output
```

![image](https://user-images.githubusercontent.com/31197208/170239680-b4f41d38-4b29-43fc-b182-64ff1cf54782.png)
