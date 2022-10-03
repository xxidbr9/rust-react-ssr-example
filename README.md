<img src="./.github/images/banner.png">

# Rust + React = SSR
Simple example running React in SSR using Rust as a backend

Learn V8 from [here](https://github.com/denoland/rusty_v8/blob/main/examples/hello_world.rs)

## Preparation
- First make sure you install [Rust](https://www.rust-lang.org/)
- And install LLVM & Clang

## How To Run
```bash
# For build react serve wit actix web server
cargo build --example=actix-v8
```

```bash
# Serve the example
cargo run --example=actix-v8
```

See the serve in : [http://localhost:8080](http://localhost:8080)

## TODO
- [ ] Babel transformer => using same api in Next js
- [ ] Example as Next js code base
- [ ] Benchmark to Next js
- [ ] Using JSC