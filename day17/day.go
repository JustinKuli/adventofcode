package day17

import (
	"bufio"
	"fmt"

	"github.com/JustinKuli/aoc2022/aoc"
)

type coord struct {
	x int8
	y int
}

func getRock(i, height int) (r []coord) {
	switch i % 5 {
	case 0:
		r = []coord{
			{x: 3, y: height},
			{x: 4, y: height},
			{x: 5, y: height}, // 0123
			{x: 6, y: height},
		}
	case 1:
		r = []coord{
			{x: 3, y: height + 1},
			{x: 4, y: height},
			{x: 4, y: height + 1}, //  4
			{x: 5, y: height + 1}, // 023
			{x: 4, y: height + 2}, //  1
		}
	case 2:
		r = []coord{
			{x: 3, y: height}, //   4
			{x: 4, y: height}, //   3
			{x: 5, y: height}, // 012
			{x: 5, y: height + 1},
			{x: 5, y: height + 2},
		}
	case 3:
		r = []coord{
			{x: 3, y: height},     // 3
			{x: 3, y: height + 1}, // 2
			{x: 3, y: height + 2}, // 1
			{x: 3, y: height + 3}, // 0
		}
	case 4:
		r = []coord{
			{x: 3, y: height},
			{x: 4, y: height},
			{x: 3, y: height + 1}, // 23
			{x: 4, y: height + 1}, // 01
		}
	}
	return r
}

func doMove(rock []coord, occupied map[coord]bool, direction byte) ([]coord, bool) {
	newrock := make([]coord, len(rock))

	switch direction {
	case '<':
		for i := range newrock {
			newrock[i] = coord{x: rock[i].x - 1, y: rock[i].y}
		}
	case '>':
		for i := range newrock {
			newrock[i] = coord{x: rock[i].x + 1, y: rock[i].y}
		}
	case 'v':
		for i := range newrock {
			newrock[i] = coord{x: rock[i].x, y: rock[i].y - 1}
		}
	}

	for _, c := range newrock {
		if c.x == 0 || c.x == 8 || c.y == 0 {
			// fmt.Println("Stopped: wall/floor", c)
			return rock, false
		}

		if occupied[c] {
			// fmt.Println("Stopped: occupied", c)
			return rock, false
		}
	}

	// fmt.Println("moved to", newrock)
	return newrock, true
}

func Run(title, file string) {
	f := aoc.MustOpen(file)
	defer f.Close()
	fs := bufio.NewScanner(f)
	fs.Scan()
	input := fs.Text()

	rockCount := 0
	occupied := make(map[coord]bool)
	top := 0
	minPrint := 0
	canMove := false

	rock := getRock(rockCount, 4)
	rockCount++

	// start := time.Now()

outer:
	for bigloop := 1; true; bigloop++ {
		for i := 0; i < len(input); i++ {
			rock, _ = doMove(rock, occupied, input[i])
			rock, canMove = doMove(rock, occupied, 'v')

			if !canMove {
				for _, c := range rock {
					occupied[c] = true
					if c.y > top {
						top = c.y
					}
				}

				rock = getRock(rockCount, top+4)
				rockCount++

				if rockCount == 2022+1 {
					// if time.Since(start) > 5*time.Second {
					break outer
				}
			}
		}

		thousands := rockCount / 1000
		maxPrint := (thousands - 2) * 1000
		printRange := maxPrint - minPrint
		if printRange > 0 {
			toPrint := make([]byte, printRange)
			for c := range occupied {
				if c.y < maxPrint { // or strictly less than?
					bval := byte(0b10000000 >> c.x)
					toPrint[c.y-minPrint] += bval
					delete(occupied, c)
				}
			}
			// fmt.Printf("%x", toPrint)
			minPrint = maxPrint
		}

	}

	fmt.Printf("%v - part one: %v\n", title, top)
	// fmt.Printf("%v - part two: %v\n", title, 17)
}
