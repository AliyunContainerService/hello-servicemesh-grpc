## grpc rust[tonic] demo

```sh
export RUSTUP_DIST_SERVER="https://mirrors.tuna.tsinghua.edu.cn/rustup"
```

```sh
cargo build
```

```sh
$ find . -name "*.rs"

./target/debug/build/hello-grpc-rust-b1aedb8eb000afe4/out/org.feuyeux.grpc.rs
./target/debug/build/anyhow-db0336bb83000618/out/probe.rs
./build.rs
./main.rs
./src/client.rs
./src/server.rs
```

```sh
cargo run --bin proto-server
```

```sh
cargo run --bin proto-client
```


- https://github.com/hyperium/tonic