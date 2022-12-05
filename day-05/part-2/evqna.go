package main

import (
	"fmt"
	"io"
	"os"
	"strings"
	"time"
)

type Stack []byte

func (s *Stack) Push(v byte) {
	*s = append(*s, v)
}

func (s *Stack) Grab(n int) []byte {
	i := len(*s) - n
	v := (*s)[i:]
	*s = (*s)[:i]
	return v
}

func (s *Stack) Drop(v []byte) {
	*s = append(*s, v...)
}

func (s *Stack) Peek() (byte, bool) {
	n := len(*s) - 1
	if n >= 0 {
		return (*s)[n], true
	}
	return 0, false
}

func parse(layout string) []Stack {
	lines := strings.Split(layout, "\n")
	h := len(lines) - 1
	stacks := make([]Stack, len(strings.Fields(lines[h])))
	// Read rows bottom to top
	for i := 0; i < h; i++ {
		row := lines[h - i - 1]
		k := 1
		for k < len(row) {
			if row[k] != ' ' {
				stacks[k / 4].Push(row[k])
			}
			k += 4	// skip '] ['
		}
	}
	return stacks
}

func move(stacks []Stack, n, from, to int) {
	stacks[to].Drop(stacks[from].Grab(n))
}

func run(s string) string {
	groups := strings.Split(s, "\n\n")
	stacks := parse(groups[0])
	for _, operation := range strings.Split(groups[1], "\n") {
		var n, src, dest int
		fmt.Sscanf(operation, "move %d from %d to %d", &n, &src, &dest)
		move(stacks, n, src-1, dest-1)
	}
	top := ""
	for _, s := range stacks {
		c, ok := s.Peek()
		if ok {
			top += string(c)
		}
	}
	return top
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
