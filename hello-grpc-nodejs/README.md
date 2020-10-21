## grpc nodejs demo
###
```sh
npm install -g grpc-tools
```

### Generate
```sh
proto2js.sh
```

### Build
```sh
npm install
```
### Run
```sh
node proto_server.js
```

```sh
node proto_client.js
```

### Diagnose
```sh
# find
lsof -i tcp:9996
# kill
kill $(lsof -ti:9996)
```
### Reference
- https://github.com/grpc/grpc-node https://www.npmjs.com/package/grpc
- https://github.com/grpc/grpc-node/tree/master/packages/proto-loader https://www.npmjs.com/package/@grpc/proto-loader
- https://github.com/caolan/async https://www.npmjs.com/package/async
- https://github.com/protocolbuffers/protobuf/tree/master/js https://www.npmjs.com/package/google-protobuf
- https://github.com/lodash/lodash https://www.npmjs.com/package/lodash
- https://github.com/substack/minimist https://www.npmjs.com/package/minimist
- https://github.com/winstonjs/winston https://www.npmjs.com/package/winston
- https://github.com/erikdubbelboer/node-sleep https://www.npmjs.com/package/sleep
- https://github.com/uuidjs/uuid https://www.npmjs.com/package/uuid