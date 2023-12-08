package day23

import (
	"bufio"
	"fmt"

	"github.com/JustinKuli/adventofcode/2022-go/aoc"
)

type coord struct {
	x int
	y int
}

func Run(title, file string) {
	f := aoc.MustOpen(file)
	defer f.Close()

	elves := make(map[coord]bool)

	fs := bufio.NewScanner(f)
	y := 0
	for fs.Scan() {
		line := fs.Text()
		for x, c := range line {
			if c == '#' {
				elves[coord{x: x, y: y}] = true
			}
		}
		y++
	}

	p1 := 0
	p2 := 0
	for i := 0; ; i++ {
		proposals := make(map[coord]coord) // key: destination, value: elf moving there
		for elf, found := range elves {
			if !found { // we might remove spots when elves move by setting them to false
				continue
			}

			// check all adjacent
			ne := elves[coord{x: elf.x + 1, y: elf.y - 1}]
			n := elves[coord{x: elf.x, y: elf.y - 1}]
			nw := elves[coord{x: elf.x - 1, y: elf.y - 1}]
			w := elves[coord{x: elf.x - 1, y: elf.y}]
			sw := elves[coord{x: elf.x - 1, y: elf.y + 1}]
			s := elves[coord{x: elf.x, y: elf.y + 1}]
			se := elves[coord{x: elf.x + 1, y: elf.y + 1}]
			e := elves[coord{x: elf.x + 1, y: elf.y}]

			if !(ne || n || nw || w || sw || s || se || e) {
				continue // no adjacent elves - no need to move
			}

			propX, propY := 0, 0
			switch i % 4 {
			case 0: // N, S, W, E
				if !(ne || n || nw) { // N
					propY = -1
				} else if !(se || s || sw) { // S
					propY = 1
				} else if !(nw || w || sw) { // W
					propX = -1
				} else if !(ne || e || se) { // E
					propX = 1
				}
			case 1: // S, W, E, N
				if !(se || s || sw) { // S
					propY = 1
				} else if !(nw || w || sw) { // W
					propX = -1
				} else if !(ne || e || se) { // E
					propX = 1
				} else if !(ne || n || nw) { // N
					propY = -1
				}
			case 2: // W, E, N, S
				if !(nw || w || sw) { // W
					propX = -1
				} else if !(ne || e || se) { // E
					propX = 1
				} else if !(ne || n || nw) { // N
					propY = -1
				} else if !(se || s || sw) { // S
					propY = 1
				}
			case 3: // E, N, S, W
				if !(ne || e || se) { // E
					propX = 1
				} else if !(ne || n || nw) { // N
					propY = -1
				} else if !(se || s || sw) { // S
					propY = 1
				} else if !(nw || w || sw) { // W
					propX = -1
				}
			}

			if propX == propY {
				// no move
				continue
			}

			prop := coord{x: elf.x + propX, y: elf.y + propY}
			if _, alreadyProposed := proposals[prop]; alreadyProposed {
				proposals[prop] = coord{x: 13371337, y: 13371337}
			} else {
				proposals[prop] = elf
			}
		}

		anyMoves := false
		for dest, elf := range proposals {
			if elf.x == 13371337 {
				continue
			}

			delete(elves, elf)
			elves[dest] = true
			anyMoves = true
		}

		if !anyMoves {
			p2 = i + 1
			break
		}

		if i == 9 {
			minX, minY, maxX, maxY := 13371337, 13371337, -13371337, -13371337
			count := 0
			for elf, isActive := range elves {
				if !isActive {
					continue
				}
				count++

				if elf.x < minX {
					minX = elf.x
				}
				if elf.y < minY {
					minY = elf.y
				}
				if elf.x > maxX {
					maxX = elf.x
				}
				if elf.y > maxY {
					maxY = elf.y
				}
			}

			area := (maxX - minX + 1) * (maxY - minY + 1)
			p1 = area - count
		}
	}

	fmt.Printf("%v - part one: %v\n", title, p1)
	fmt.Printf("%v - part two: %v\n", title, p2)
}
