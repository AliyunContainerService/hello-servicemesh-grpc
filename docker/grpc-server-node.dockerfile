FROM node:14.11-alpine
RUN apk add --update \
      python \
      make \
      g++ \
  && rm -rf /var/cache/apk/*
RUN npm config set registry https://registry.npm.taobao.org && npm install -g node-pre-gyp grpc-tools --unsafe-perm
COPY node/package.json .
RUN npm install --unsafe-perm
COPY node .
ENTRYPOINT ["node","proto_server.js"]