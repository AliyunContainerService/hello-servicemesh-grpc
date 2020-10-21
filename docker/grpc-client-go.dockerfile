FROM golang:1.15-alpine
COPY proto_client grpc-client
# ENTRYPOINT ["./grpc-client"]