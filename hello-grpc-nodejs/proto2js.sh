#!/bin/bash
cd "$(
  cd "$(dirname "$0")" >/dev/null 2>&1
  pwd -P
)/" || exit
rm -rf common && mkdir common
js_proto_path=$(pwd)/common
cd proto
grpc_tools_node_protoc \
  --js_out=import_style=commonjs,binary:"${js_proto_path}" \
  --grpc_out="${js_proto_path}" \
  --plugin=protoc-gen-grpc=$(which grpc_tools_node_protoc_plugin) landing.proto
