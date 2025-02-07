package main

// NOTE: There should be NO space between the comments and the `import "C"` line.
// The -ldl is sometimes necessary to fix linker errors about `dlsym`.

/*
#cgo LDFLAGS: -L./go-sdk -lgo_sdk
#include "./go-sdk/go_sdk.h"
#include <stdlib.h>
*/
import "C"

type Ip struct {
	Origin string `json:"origin"`
}

func main() {
	resultChan := make(chan Ip)
	go func() {
		rawStr := C.hello()
		defer C.free_struct(rawStr)

		result := Ip{
			Origin: C.GoString(rawStr.origin),
		}
		resultChan <- result
	}()

	result := <-resultChan

	println(result.Origin)
}
