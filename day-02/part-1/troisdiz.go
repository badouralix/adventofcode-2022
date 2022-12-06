package main

import (
	"fmt"
	"io"
	"os"
	"strings"
	"time"
)

const (
	OTHER_A = 1 // Rock
	OTHER_B = 2 // Paper
	OTHER_C = 3 // Scissors
	YOU_X   = 1 // Rock
	YOU_Y   = 2 // Paper
	YOU_Z   = 3 // Scissors
)

type Round struct {
	other int
	you   int
}

type Puzzle struct {
	rounds []Round
}

const (
	LOST_BONUS int = 0
	DRAW_BONUS int = 3
	WIN_BONUS  int = 6
)

func score(other int, you int) int {

	if you == other {
		return DRAW_BONUS + you
	}
	if other == OTHER_A {
		if you == YOU_Z {
			return LOST_BONUS + you
		} else {
			return WIN_BONUS + you
		}
	}
	if other == OTHER_B {
		if you == YOU_X {
			return LOST_BONUS + you
		} else {
			return WIN_BONUS + you
		}
	}
	if other == OTHER_C {
		if you == YOU_Y {
			return LOST_BONUS + you
		} else {
			return WIN_BONUS + you
		}
	}
	panic("Cannot happen")
}

func parseData(input string) Puzzle {

	lines := strings.Split(input, "\n")
	var result Puzzle
	result.rounds = make([]Round, len(lines))

	for idx, line := range lines {
		if len(line) > 0 {
			var other, you int
			parts := strings.Split(line, " ")
			switch parts[0] {
			case "A":
				other = OTHER_A
			case "B":
				other = OTHER_B
			case "C":
				other = OTHER_C
			}
			switch parts[1] {
			case "X":
				you = YOU_X
			case "Y":
				you = YOU_Y
			case "Z":
				you = YOU_Z
			}
			result.rounds[idx].other = other
			result.rounds[idx].you = you
		}
	}
	return result
}

func run(s string) interface{} {
	// Your code goes here
	puzzle := parseData(s)
	// fmt.Printf("%v\n", puzzle)
	result := 0
	for _, round := range puzzle.rounds {
		result += score(round.other, round.you)
	}
	return result
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
