ROOT_DIR := $(dir $(realpath $(lastword $(MAKEFILE_LIST))))

# PHONY means that it doesn't correspond to a file; it always runs the build commands.

.PHONY: build
build:
	@cd go-sdk && cargo build --release
	@cp go-sdk/target/release/libgo_sdk.dylib ./go-sdk
	go run -ldflags="-r ./go-sdk" main.go

