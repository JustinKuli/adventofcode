package day10

import (
	"bufio"
	"fmt"

	"github.com/JustinKuli/aoc2022/aoc"
)

type communicator struct {
	cycle     int
	registerX int
}

func (c *communicator) Tick() int {
	pixel := c.cycle % 40
	switch pixel - c.registerX {
	case -1, 0, 1:
		fmt.Print(string('\u2588')) // █ 'FULL BLOCK', aka ASCII (extended) 219
	default:
		fmt.Print(" ")
	}

	c.cycle++

	if c.cycle%40 == 0 {
		fmt.Println()
	}

	// part one
	if (c.cycle-20)%40 == 0 {
		return c.registerX * c.cycle
	}
	return 0
}

func Run(title, file string) {
	f := aoc.MustOpen(file)
	defer f.Close()

	c := communicator{
		cycle:     0,
		registerX: 1,
	}

	strengthSum := 0

	fs := bufio.NewScanner(f)
	for fs.Scan() {
		line := fs.Text()

		instruction, valstr := aoc.MustCut(line, " ")

		switch instruction {
		case "addx":
			strengthSum += c.Tick()
			strengthSum += c.Tick()

			val := aoc.MustInt(valstr)
			c.registerX += val
		case "noop":
			strengthSum += c.Tick()
		}
	}

	fmt.Printf("%v - part one: %v\n", title, strengthSum)
	fmt.Printf("%v - part two: %v\n", title, "see above")
}
