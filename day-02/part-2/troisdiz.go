package main

import (
	"fmt"
	"io"
	"os"
	"strings"
	"time"
)

const (
	ROCK     = 1 // Rock
	PAPER    = 2 // Paper
	SCISSORS = 3 // Scissors
	YOU_LOSE = 1 // Loose
	YOU_DRAW = 2 // Draw
	YOU_WIN  = 3 // Win
)

type Round struct {
	other       int
	matchResult int
}

type Puzzle struct {
	rounds []Round
}

const (
	LOST_BONUS int = 0
	DRAW_BONUS int = 3
	WIN_BONUS  int = 6
)

func yourPlayAndScore(other int, matchResult int) (int, int) {
	if matchResult == YOU_DRAW {
		return other, DRAW_BONUS + other
	}
	if other == ROCK {
		if matchResult == YOU_WIN {
			return PAPER, WIN_BONUS + PAPER
		} else {
			return SCISSORS, LOST_BONUS + SCISSORS
		}
	}
	if other == PAPER {
		if matchResult == YOU_WIN {
			return SCISSORS, WIN_BONUS + SCISSORS
		} else {
			return ROCK, LOST_BONUS + ROCK
		}
	}
	if other == SCISSORS {
		if matchResult == YOU_WIN {
			return ROCK, WIN_BONUS + ROCK
		} else {
			return PAPER, LOST_BONUS + PAPER
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
			var other, matchResult int
			parts := strings.Split(line, " ")
			switch parts[0] {
			case "A":
				other = ROCK
			case "B":
				other = PAPER
			case "C":
				other = SCISSORS
			}
			switch parts[1] {
			case "X":
				matchResult = YOU_LOSE
			case "Y":
				matchResult = YOU_DRAW
			case "Z":
				matchResult = YOU_WIN
			}
			result.rounds[idx].other = other
			result.rounds[idx].matchResult = matchResult
		}
	}
	return result
}

func run(s string) interface{} {
	// Your code goes here
	puzzle := parseData(s)
	result := 0
	for _, round := range puzzle.rounds {
		_, score := yourPlayAndScore(round.other, round.matchResult)
		result += score
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
