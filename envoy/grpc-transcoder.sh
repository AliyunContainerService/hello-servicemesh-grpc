#!/bin/bash
cd "$(
    cd "$(dirname "$0")" >/dev/null 2>&1
    pwd -P
)/" || exit

#!/usr/bin/env bash
script_path=$(PWD)

echo "1 Checking protoc installation"
if ! [ -x "$(command -v protoc)" ]; then
    echo "you do not seem to have the protoc executable on your path"
    echo "we need protoc to generate a service defintion (*.pb file) that envoy can understand"
    echo "download the precompiled protoc executable and place it in somewhere in your systems PATH!"
    echo "goto: https://github.com/protocolbuffers/protobuf/releases/latest"
    echo "choose:"
    echo "       for linux:   protoc-3.14.0-linux-x86_64.zip"
    echo "       for windows: protoc-3.14.0-win64.zip"
    echo "       for mac:     protoc-3.14.0-osx-x86_64.zip"
    exit 1
else
    protoc --version
fi

echo "2 Generate Proto Descriptors"
cd ..
proto_path="$(PWD)/proto"
proto_dep_path="$(PWD)/hello-grpc-java/target/protoc-dependencies/3495bb91958122dbbfb579bead6834ec"
cd ${script_path}

# -IPATH, --proto_path=PATH
# Specify the directory in which to search for imports.
# May be specified multiple times; directories will be searched in order.
# If not given, the current working directory is used.
# If not found in any of the these directories, the --descriptor_set_in descriptors will be checked for required proto file.

# -oFILE, --descriptor_set_out=FILE
# Writes a FileDescriptorSet (a protocol buffer, defined in descriptor.proto) containing all of the input files to FILE.

# --include_imports
# When using --descriptor_set_out, also include all dependencies of the input files in the set, so that the set is self-contained.

# --include_source_info
# When using --descriptor_set_out, do not strip SourceCodeInfo from the FileDescriptorProto.
# This results in vastly larger descriptors that include information about the original location of each decl in the source file as well as surrounding comments.

echo "proto_path=$proto_path"
echo "proto_dep_path=$proto_dep_path"

protoc \
    --proto_path=${proto_path} \
    --proto_path=${proto_dep_path} \
    --include_imports \
    --include_source_info \
    --descriptor_set_out=landing.pb \
    "${proto_path}"/landing.proto

if ! [ $? -eq 0 ]; then
    echo "protobuf compilation failed"
    exit 1
fi

echo "3 Start envoy container on $(PWD)"
getenvoy run standard:1.16.2 -- --config-path ./envoy-config.yaml
