package main

// NOTE: There should be NO space between the comments and the `import "C"` line.
// The -ldl is sometimes necessary to fix linker errors about `dlsym`.

/*
#cgo LDFLAGS: -L./go-sdk -lgo_sdk
#include "./go-sdk/go_sdk.h"
#include <stdlib.h>
*/
import "C"
import "unsafe"

type Ip struct {
	Origin string `json:"origin"`
}

func main() {
	rawStr := C.hello()
	defer func() {
		C.free(unsafe.Pointer(rawStr.origin))
		C.free(unsafe.Pointer(rawStr))
	}()

	result := Ip{
		Origin: C.GoString(rawStr.origin),
	}

	println(result.Origin)
}
