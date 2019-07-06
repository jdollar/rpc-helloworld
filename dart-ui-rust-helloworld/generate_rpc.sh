#!/usr/bin/env sh

PROTO_PATH=$HOME/repos/rpc-helloworld/proto

protoc --dart_out=grpc:lib/src/generated --proto_path=$PROTO_PATH $PROTO_PATH/*.proto
