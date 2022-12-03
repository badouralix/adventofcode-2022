package main

import (
	"fmt"
	"io"
	"os"
	"time"
)

func checkCurrent(currentSum, res3, res2, res1 int32) (int32, int32, int32) {
	if res3 > currentSum {
		return res3, res2, res1
	}
	if res2 > currentSum {
		return currentSum, res2, res1
	}
	if res1 > currentSum {
		return res2, currentSum, res1
	}
	return res2, res1, currentSum
}

func run(s []byte) int32 {
	var current, currentSum, res1, res2, res3 int32
	for _, r := range s {
		switch r {
		case '\n':
			if current == 0 {
				res3, res2, res1 = checkCurrent(currentSum, res3, res2, res1)
				currentSum = 0
				continue
			}
			currentSum += current
			current = 0
		default:
			current = current*10 + int32(r-'0')
		}
	}
	currentSum += current
	res3, res2, res1 = checkCurrent(currentSum, res3, res2, res1)

	return res1 + res2 + res3
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
