#!/bin/bash
docker run --rm --name grpc_client_java -e GRPC_SERVER=$(ipconfig getifaddr en0) \
-e GRPC_HELLO_SECURE=Y \
registry.cn-beijing.aliyuncs.com/asm_repo/grpc_client_java:1.0.0 java -jar /grpc-client.jar