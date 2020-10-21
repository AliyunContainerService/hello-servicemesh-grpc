export GO111MODULE="on"
export GOPROXY=https://mirrors.aliyun.com/goproxy/
go get github.com/golang/protobuf/protoc-gen-go
sh proto2go.sh
go list -mod=mod -json all
go build