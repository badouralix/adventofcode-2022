package main

import (
	"fmt"
	"io"
	"os"
	"time"
)

const (
	upperShift = 'A' - 27
	lowerShift = 'a' - 1
)

func scanLine(s []byte, i int) int {
	for i < len(s) {
		if s[i] == '\n' {
			return i
		}
		i++
	}
	return i
}

func priority(x byte) byte {
	if x <= 'Z' {
		return x - upperShift
	} else {
		return x - lowerShift
	}
}

func intersect(s []byte, buffer *[255]int, token int) byte {
	mid := len(s) >> 1
	for _, r1 := range s[:mid] {
		buffer[r1] = token
	}
	for _, r2 := range s[mid:] {
		if buffer[r2] == token {
			return r2
		}
	}
	return 0
}

func run(s []byte) int {
	var score int
	var buffer [255]int
	start := 0
	for start < len(s) {
		end := scanLine(s, start)
		score += int(priority(intersect(s[start:end], &buffer, end)))
		start = end + 1
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
