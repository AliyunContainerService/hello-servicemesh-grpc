
### build all images
```sh
sh build.sh
```

### push all images
```sh
sh push.sh
```

### verify

#### clean all containers
```sh
sh clean_world.sh
```

#### java
```sh
docker run --rm --name grpc_server_java -p 9996:9996 \
registry.cn-beijing.aliyuncs.com/asm_repo/grpc_server_java:1.0.0
```

```sh
docker run --rm --name grpc_client_java -e GRPC_SERVER=$(ipconfig getifaddr en0) \
registry.cn-beijing.aliyuncs.com/asm_repo/grpc_client_java:1.0.0 java -jar /grpc-client.jar
```

#### go
```sh
docker run --rm --name grpc_server_go -p 9996:9996 \
registry.cn-beijing.aliyuncs.com/asm_repo/grpc_server_go:1.0.0
```

```sh
docker run --rm --name grpc_client_go -e GRPC_SERVER=$(ipconfig getifaddr en0) \
registry.cn-beijing.aliyuncs.com/asm_repo/grpc_client_go:1.0.0 ./grpc-client
```

#### node
```sh
docker run --rm --name grpc_server_node -p 9996:9996 \
registry.cn-beijing.aliyuncs.com/asm_repo/grpc_server_node:1.0.0
```

```sh
docker run --rm --name grpc_client_node -e GRPC_SERVER=$(ipconfig getifaddr en0) \
registry.cn-beijing.aliyuncs.com/asm_repo/grpc_client_node:1.0.0 node proto_client.js
```

#### python
```sh
docker run --rm --name grpc_server_python -p 9996:9996 \
registry.cn-beijing.aliyuncs.com/asm_repo/grpc_server_python:1.0.0
```

```sh
docker run --rm --name grpc_client_python -e GRPC_SERVER=$(ipconfig getifaddr en0) \
registry.cn-beijing.aliyuncs.com/asm_repo/grpc_client_python:1.0.0 sh /grpc-client/start_client.sh
```