FROM golang:1.15-alpine
COPY proto_server grpc-server
ENTRYPOINT ["./grpc-server"]