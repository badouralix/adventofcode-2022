package main

import (
	"fmt"
	"io"
	"os"
	"time"
)

func run(s []byte) int32 {
	var current, currentSum, res int32
	for _, r := range s {
		switch r {
		case '\n':
			if current == 0 {
				if res < currentSum {
					res = currentSum
				}
				currentSum = 0
				continue
			}
			currentSum += current
			current = 0
		default:
			current = current*10 + int32(r-'0')
		}
	}

	return res
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
