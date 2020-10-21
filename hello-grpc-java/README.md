## grpc java demo
### Build
```sh
sh build.sh
```
### Run
```sh
mvn exec:java -Dexec.mainClass="org.feuyeux.grpc.server.ProtoServer"
```

```sh
mvn exec:java -Dexec.mainClass="org.feuyeux.grpc.client.ProtoClient"
```

### Reference
- [Language Guide (proto3)](https://developers.google.com/protocol-buffers/docs/proto3)
- [gRPC Java Tutorials](https://grpc.io/docs/tutorials/basic/java.html)
- [gRPC-Java - An RPC library and framework](https://github.com/grpc/grpc-java)