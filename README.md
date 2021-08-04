## hello-servicemesh-grpc

### GRPC Diagram
![](img/grpc_diagram.png)

**client** [end of stream (EOS)]->[Length-Prefixed Message][]->[Headers] **server**

**client** [Headers]<-[Length-Prefixed Message][]<-[Trailers] **server**

### Proto3
- [proto](grpc/proto)

### Service
- [1 hello-grpc-java](grpc/hello-grpc-java)
- [2 hello-grpc-go](grpc/hello-grpc-go)
- [3 hello-grpc-nodejs](grpc/hello-grpc-nodejs)
- [4 hello-grpc-python](grpc/hello-grpc-python)
- [5 hello-grpc-rust](grpc/hello-grpc-rust)
- [6 hello-grpc-cpp](grpc/hello-grpc-cpp)
- [docker](grpc/docker)

### Cross Access Test
- [cross](cross)

### Propagate Test
- [propagate](propagate)

### ServiceMesh
- [kube](kube)
- [mesh](mesh)
- [tracing](tracing)

### Http2gRPC
- [transcoder](transcoder)