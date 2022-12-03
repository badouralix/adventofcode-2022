package main

import (
	"fmt"
	"io"
	"os"
	"time"
)

func run(s []byte) int32 {
	var i int
	var score int32
	for i < len(s) {
		r1 := int32(s[i] - 'A')
		r2 := int32(s[i+2] - 'W')

		score += r2 + ((r2+3-r1)%3)*3
		i += 4
	}

	return score
}

func main() {
	// Uncomment this line to disable garbage collection
	// debug.SetGCPercent(-1)

	var input []byte
	var err error
	if len(os.Args) > 1 {
		// Read input from file for local debugging
		input, err = os.ReadFile(os.Args[1])
		if err != nil {
			panic(err)
		}
		// Remove extra newline
		input = input[:len(input)-1]
	} else {
		// Read input from stdin
		input, err = io.ReadAll(os.Stdin)
		if err != nil {
			panic(err)
		}
	}

	// Start resolution
	start := time.Now()
	result := run(input)

	// Print result
	fmt.Printf("_duration:%f\n", time.Since(start).Seconds()*1000)
	fmt.Println(result)
}
