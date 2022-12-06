package main

import (
	"fmt"
	"io"
	"os"
	"strings"
	"time"
)

const WINDOW = 4

func distinct(buf []byte) bool {
	for i := 0; i < len(buf); i++ {
		for j := i + 1; j < len(buf); j++ {
			if buf[i] == buf[j] {
				return false
			}
		}
	}
	return true
}

func findPacket(stream string) int {
	buf := []byte(stream[:WINDOW])
	if distinct(buf) {
		return WINDOW
	}
	for i := WINDOW; i < len(stream); i++ {
		c := stream[i]
		buf = append(buf, c)
		buf = buf[1:]
		if distinct(buf) {
			return i + 1
		}
	}
	return -1
}

func run(s string) int {
	stream := strings.TrimSpace(s)
	return findPacket(stream)
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
