package main

import (
	"fmt"
	"io"
	"os"
	"strings"
	"time"
)

func overlap(a, b, c, d int) bool {
	return (a <= c && d <= b) || (c <= a && b <= d)
}

func run(s string) int {
	count := 0
	for _, line := range strings.Split(s, "\n") {
		var a, b, c, d int
		fmt.Sscanf(line, "%d-%d,%d-%d", &a, &b, &c, &d)
		if overlap(a, b, c, d) {
			count += 1
		}
	}
	return count
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
	result := run(string(input))

	// Print result
	fmt.Printf("_duration:%f\n", time.Since(start).Seconds()*1000)
	fmt.Println(result)
}
