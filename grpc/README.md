# Hello gRPC

## Protocol buffer version 3 Compiler(protoc)

- sourcecode: https://github.com/protocolbuffers/protobuf
- installation: https://grpc.io/docs/protoc-installation/

```bash
$ brew install protobuf
$ protoc --version  # Ensure compiler version is 3+
```

```bash
$ brew info protobuf
protobuf: stable 3.17.3 (bottled), HEAD
Protocol buffers (Google's data interchange format)
https://github.com/protocolbuffers/protobuf/
/usr/local/Cellar/protobuf/3.17.3 (210 files, 18.0MB) *
  Poured from bottle on 2021-06-09 at 20:51:53
From: https://github.com/Homebrew/homebrew-core/blob/HEAD/Formula/protobuf.rb
License: BSD-3-Clause
==> Dependencies
Build: python@3.9 ✔
Required: six ✔
==> Options
--HEAD
	Install HEAD version
==> Caveats
Emacs Lisp files have been installed to:
  /usr/local/share/emacs/site-lisp/protobuf
==> Analytics
install: 109,150 (30 days), 428,962 (90 days), 1,644,093 (365 days)
install-on-request: 21,285 (30 days), 85,527 (90 days), 310,042 (365 days)
build-error: 0 (30 days)

$ protoc --version
libprotoc 3.17.3
```

## Generated-code

### Java

- Generate automatically during building

  - [protobuf-maven-plugin](https://github.com/xolstice/protobuf-maven-plugin)
  - [protobuf-gradle-plugin](https://github.com/google/protobuf-gradle-plugin)

- CLI

  ```bash
  rm -rf target/generated-sources
  mkdir -p target/generated-sources/protobuf/java
  mkdir -p target/generated-sources/protobuf/grpc-java
  export GEN_PATH=$(pwd)/target/protoc-plugins/protoc-gen-grpc-java-1.39.0-osx-x86_64.exe
  export OUTPUT_FILE=$(pwd)/target/generated-sources/protobuf/java
  export GOUTPUT_FILE=$(pwd)/target/generated-sources/protobuf/grpc-java
  export DIR_OF_PROTO_FILE=$(pwd)/proto
  export PROTO_FILE=landing.proto
  protoc --plugin=protoc-gen-grpc-java=$GEN_PATH \
  --proto_path="$DIR_OF_PROTO_FILE" --grpc-java_out="$GOUTPUT_FILE" --java_out="$OUTPUT_FILE" "$PROTO_FILE"
  ```

### Go

```bash
# Setup
go get github.com/golang/protobuf/protoc-gen-go

# Generate
export OUTPUT_FILE=$(pwd)
export DIR_OF_PROTO_FILE=$(pwd)/proto
export PROTO_FILE=landing.proto
protoc --proto_path="$DIR_OF_PROTO_FILE" --go_out=plugins=grpc:"$OUTPUT_FILE" "$PROTO_FILE"
```

### NodeJs

```bash
# Setup
npm install -g grpc-tools

# Generate
export OUTPUT_FILE=$(pwd)
export DIR_OF_PROTO_FILE=$(pwd)/proto
export PROTO_FILE=landing.proto
grpc_tools_node_protoc \
  --js_out=import_style=commonjs,binary:"${js_proto_path}" \
  --grpc_out="${js_proto_path}" \
  --plugin=protoc-gen-grpc=$(which grpc_tools_node_protoc_plugin) landing.proto
```

### Python

```bash
# Setup
pip install grpcio-tools

# Generate
python -m grpc.tools.protoc \
  -I $(pwd)/proto \
  --python_out=${py_proto_path} \
  --grpc_python_out=${py_proto_path} \
  $(pwd)/proto/landing.proto
```

### Rust

Generate automatically during building
- [tonic](https://github.com/hyperium/tonic)

### Cpp
(TODO)

### Kotlin
(TODO)
