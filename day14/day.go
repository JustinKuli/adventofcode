package day14

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type coord struct {
	x int
	y int
}

func getCoord(str string) coord {
	xstr, ystr, _ := strings.Cut(str, ",")
	x, _ := strconv.Atoi(xstr)
	y, _ := strconv.Atoi(ystr)
	return coord{
		x: x,
		y: y,
	}
}

func Run(title, file string) {
	f, _ := os.Open(file)
	defer f.Close()

	occupied := make(map[coord]bool)

	fs := bufio.NewScanner(f)
	for fs.Scan() {
		line := fs.Text()

		points := strings.Split(line, " -> ")
		for i := 0; i < len(points)-1; i++ {
			curr := getCoord(points[i])
			next := getCoord(points[i+1])

			if next.x == curr.x { // increment y
				inc := 1
				if curr.y > next.y {
					inc = -1
				}
				for curr.y != next.y {
					occupied[curr] = true
					curr.y += inc
				}
				occupied[curr] = true
			} else { // increment x
				inc := 1
				if curr.x > next.x {
					inc = -1
				}
				for curr.x != next.x {
					occupied[curr] = true
					curr.x += inc
				}
				occupied[curr] = true
			}
		}
	}

	maxY := -1
	for p := range occupied {
		if p.y > maxY {
			maxY = p.y
		}
	}

	count := 0
	partOneComplete := false
newsand:
	for {
		sand := coord{x: 500, y: 0}

		for {
			if sand.y > maxY && !partOneComplete {
				fmt.Printf("%v - part one: %v\n", title, count)
				partOneComplete = true
			}

			if sand.y == maxY+1 {
				count++
				occupied[sand] = true
				continue newsand
			}

			down := coord{x: sand.x, y: sand.y + 1}
			if !occupied[down] {
				sand = down
				continue
			}

			dl := coord{x: sand.x - 1, y: sand.y + 1}
			if !occupied[dl] {
				sand = dl
				continue
			}

			dr := coord{x: sand.x + 1, y: sand.y + 1}
			if !occupied[dr] {
				sand = dr
				continue
			}

			count++
			occupied[sand] = true
			if sand.x == 500 && sand.y == 0 {
				break newsand
			}
			continue newsand
		}
	}

	fmt.Printf("%v - part two: %v\n", title, count)
}
