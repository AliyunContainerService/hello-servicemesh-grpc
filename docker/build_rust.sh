#!/bin/bash
set -e

cd "$(
  cd "$(dirname "$0")" >/dev/null 2>&1
  pwd -P
)/" || exit

# Croess Platform Support
# https://doc.rust-lang.org/nightly/rustc/platform-support.html
# https://doc.rust-lang.org/edition-guide/rust-2018/platform-and-target-support/musl-support-for-fully-static-binaries.html
echo "build grpc server rust"
cd ../hello-grpc-rust
# rustup target add x86_64-unknown-linux-musl
# rustup show

# Error: Your CLT does not support macOS 11.2.
# sudo rm -rf /Library/Developer/CommandLineTools
# sudo xcode-select --install
# https://download.developer.apple.com/Developer_Tools/Command_Line_Tools_for_Xcode_12.5_beta/Command_Line_Tools_for_Xcode_12.5_beta.dmg

# $ /usr/bin/xcodebuild -version ↵
# Xcode 12.4
# Build version 12D4e

# $ pkgutil --pkg-info=com.apple.pkg.CLTools_Executables
# package-id: com.apple.pkg.CLTools_Executables
# version: 12.5.0.0.1.1611946261
# volume: /
# location: /
# install-time: 1612700387
# groups: com.apple.FindSystemFiles.pkg-group

# code .cargo/config
# [target.x86_64-unknown-linux-musl]
# linker = "x86_64-linux-musl-gcc"

# brew install FiloSottile/musl-cross/musl-cross
# ln -s /usr/local/bin/x86_64-linux-musl-gcc /usr/local/bin/musl-gcc

CROSS_COMPILE=x86_64-linux-musl-gcc cargo build --release --bin proto-server --target=x86_64-unknown-linux-musl
mv target/x86_64-unknown-linux-musl/release/proto-server ../docker/
cd ../docker
echo "build server image"
docker build -f grpc-server-rust.dockerfile -t registry.cn-beijing.aliyuncs.com/asm_repo/grpc_server_rust:1.0.0 .
rm -rf proto-server
echo

echo "build grpc client rust"
cd ../hello-grpc-rust
CROSS_COMPILE=x86_64-linux-musl-gcc cargo build --release --bin proto-client --target=x86_64-unknown-linux-musl
mv target/x86_64-unknown-linux-musl/release/proto-client ../docker/
cd ../docker
echo "build client image"
docker build -f grpc-client-rust.dockerfile -t registry.cn-beijing.aliyuncs.com/asm_repo/grpc_client_rust:1.0.0 .
rm -rf proto-client
echo