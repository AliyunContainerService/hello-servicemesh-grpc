#!/bin/bash
cd "$(
  cd "$(dirname "$0")" >/dev/null 2>&1
  pwd -P
)/" || exit
# brew install grpc protobuf
export PATH="$PATH:$(go env GOPATH)/bin"
proto_path="$(pwd)/proto"
go_proto_path="$(pwd)"
echo "proto_path=$proto_path,go_proto_path=$go_proto_path"
rm -rf "${go_proto_path}/common"
echo "protoc --proto_path=${proto_path} --go_out=plugins=grpc:${go_proto_path} ${proto_path}/landing.proto"
protoc --proto_path="${proto_path}" --go_out=plugins=grpc:"${go_proto_path}" "${proto_path}"/landing.proto