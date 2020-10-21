# https://cr.console.aliyun.com/cn-beijing/instances/credentials
# ACR_USER=$(head $HOME/shop_config/cr)
# docker login --username=$ACR_USER registry.cn-beijing.aliyuncs.com

# java
docker push registry.cn-beijing.aliyuncs.com/asm_repo/grpc_server_java:1.0.0
docker push registry.cn-beijing.aliyuncs.com/asm_repo/grpc_client_java:1.0.0
# go
docker push registry.cn-beijing.aliyuncs.com/asm_repo/grpc_server_go:1.0.0
docker push registry.cn-beijing.aliyuncs.com/asm_repo/grpc_client_go:1.0.0
# nodejs
docker push registry.cn-beijing.aliyuncs.com/asm_repo/grpc_server_node:1.0.0
docker push registry.cn-beijing.aliyuncs.com/asm_repo/grpc_client_node:1.0.0
# python
docker push registry.cn-beijing.aliyuncs.com/asm_repo/grpc_server_python:1.0.0
docker push registry.cn-beijing.aliyuncs.com/asm_repo/grpc_client_python:1.0.0