#!/usr/bin/env bash

set -e

REPO_ROOT=$(git rev-parse --show-toplevel)
GO_OUT=$REPO_ROOT/steward_proto_go
PROTO_PATH=$REPO_ROOT/proto/steward/v4

echo
echo Building Go bindings. Output will be in ./steward_proto_go/steward_proto
mkdir -p $GO_OUT
protoc --proto_path=$PROTO_PATH \
	--proto_path=$PROTO_PATH/boring_vault/v1 \
	--go_out=$GO_OUT/ \
	--go-grpc_out=$GO_OUT/ \
    $PROTO_PATH/*.proto \
	$PROTO_PATH/boring_vault/v1/*.proto

echo Done!
