package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strings"
	"time"
)

func roundScore(a, r int) int {
	b := (a + r + 2) % 3
	return (b + 1) + 3 * r
}

func run(s string) int {
	score := 0
	for _, round := range strings.Split(s, "\n") {
		a := int(round[0] - 'A')
		r := int(round[2] - 'X')
		score += roundScore(a, r)
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
