package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strings"
	"time"
)

func badge(a, b, c string) int {
	// Go 🤮
	for i := 0; i < len(a); i++ {
		for j := 0; j < len(b); j++ {
			if a[i] == b[j] {
				for k := 0; k < len(c); k++ {
					if c[k] == a[i] {
						return int(a[i])
					}
				}
			}
		}
	}
	return 0
}

func priority(item int) int {
	if item >= 'a' && item <= 'z' {
		return item - 'a' + 1
	}
	return item - 'A' + 27
}

func run(s string) int {
	sacks := strings.Fields(s)
	i := 0
	sum := 0
	for i < len(sacks) {
		sum += priority(badge(sacks[i], sacks[i+1], sacks[i+2]))
		i += 3
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
