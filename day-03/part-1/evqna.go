package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strings"
	"time"
)

func intersect(a, b string) int {
	for i := 0; i < len(a); i++ {
		for j := 0; j < len(b); j++ {
			if a[i] == b[j] {
				return int(a[i])
			}
		}
	}
	return 0
}

func priority(sack string) int {
	n := len(sack) / 2
	item := intersect(sack[:n], sack[n:])
	if item >= 'a' && item <= 'z' {
		return item - 'a' + 1
	}
	return item - 'A' + 27
}

func run(s string) int {
	sum := 0
	for _, sack := range strings.Fields(s) {
		sum += priority(sack)
	}
	return sum
}

func main() {
	// Uncomment this line to disable garbage collection
	// debug.SetGCPercent(-1)

	var input []byte
	var err error
	if len(os.Args) > 1 {
		// Read input from file for local debugging
		input, err = ioutil.ReadFile(os.Args[1])
		if err != nil {
			panic(err)
		}
		// Remove extra newline
		input = input[:len(input)-1]
	} else {
		// Read input from stdin
		input, err = ioutil.ReadAll(os.Stdin)
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
