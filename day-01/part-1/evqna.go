package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"strconv"
	"strings"
	"time"
)

func calories(items []string) int {
	sum := 0
	for _, x := range items {
		n, _ := strconv.Atoi(x)
		sum += n
	}
	return sum
}

func run(s string) interface{} {
	max := 0
	for _, elf := range strings.Split(s, "\n\n") {
		c := calories(strings.Split(elf, "\n"))
		if c > max {
			max = c
		}
	}
	return max
}

func main() {
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
