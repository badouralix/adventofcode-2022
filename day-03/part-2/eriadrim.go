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

func scanLine(s []byte, i int, buffer *[255]byte, token byte) (byte, int) {
	// first line
	for {
		r := s[i]
		i++
		if r == '\n' {
			break
		}
		buffer[r] = token
	}
	// second line
	for {
		r := s[i]
		i++
		if r == '\n' {
			break
		}
		buffer[r] = buffer[r] | 1
	}
	both := token | 1
	var res byte
	for {
		r := s[i]
		if buffer[r] == both {
			res = r
			break
		}
		i++
	}

	for i < len(s) {
		if s[i] == '\n' {
			break
		}
		i++
	}
	return res, i
}

func priority(x byte) byte {
	if x <= 'Z' {
		return x - upperShift
	} else {
		return x - lowerShift
	}
}

func run(s []byte) int {
	var score int
	var buffer [255]byte
	var token byte
	start := 0
	for start < len(s) {
		token += 2
		subScore, end := scanLine(s, start, &buffer, token)
		score += int(priority(subScore))
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
