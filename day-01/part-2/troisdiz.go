package main

import (
	"fmt"
	"io/ioutil"
	"os"
	"sort"
	"strconv"
	"strings"
	"time"
)

func parseData(input string) []int {
	var currentSum int = 0
	var result []int
	for _, line := range strings.Split(input, "\n") {
		if len(line) > 0 {
			startsCount, _ := strconv.Atoi(line)
			currentSum += startsCount
		} else {
			result = append(result, currentSum)
			currentSum = 0
		}
	}
	result = append(result, currentSum)
	return result
}

func run(s string) interface{} {
	// Your code goes here
	starsPerElve := parseData(s)
	sort.Ints(starsPerElve)
	elfCount := len(starsPerElve)
	return starsPerElve[elfCount-3] + starsPerElve[elfCount-2] + starsPerElve[elfCount-1]
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
