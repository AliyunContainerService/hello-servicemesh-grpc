#!/usr/bin/env bash
SCRIPT_PATH="$(
  cd "$(dirname "$0")" >/dev/null 2>&1 || exit
  pwd -P
)"
cd "$SCRIPT_PATH" || exit

rm -f src/main/proto/landing.proto
cd ..
ln -s "$PWD"/proto/landing2.proto hello-grpc-java/src/main/proto/landing.proto
cd hello-grpc-java
ls -l src/main/proto
mvn clean install -DskipTests
