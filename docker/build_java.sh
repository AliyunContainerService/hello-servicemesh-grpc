#!/bin/bash
cd "$(
  cd "$(dirname "$0")" >/dev/null 2>&1
  pwd -P
)/" || exit

echo "build grpc server java"
cd ../hello-grpc-java
mvn clean install -DskipTests -f server_pom
cp target/hello-grpc-java.jar ../docker/
cd ../docker
docker build -f grpc-server-java.dockerfile -t registry.cn-beijing.aliyuncs.com/asm_repo/grpc_server_java:1.0.0 .
rm -rf hello-grpc-java.jar
echo

echo "build grpc client java"
cd ../hello-grpc-java
mvn clean install -DskipTests -f client_pom
cp target/hello-grpc-java.jar ../docker/
cd ../docker
docker build -f grpc-client-java.dockerfile -t registry.cn-beijing.aliyuncs.com/asm_repo/grpc_client_java:1.0.0 .
rm -rf hello-grpc-java.jar
echo