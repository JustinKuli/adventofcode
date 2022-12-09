package day09

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func Run() {
	f, _ := os.Open("./day09/input.txt")
	defer f.Close()

	fs := bufio.NewScanner(f)

	i := make([]int, 10)
	j := make([]int, 10)
	pos := make([]map[string]bool, 10)

	for c := range pos {
		pos[c] = map[string]bool{"0,0": true}
	}

	for fs.Scan() {
		line := fs.Text()

		direction, countstr, _ := strings.Cut(line, " ")
		var di, dj int
		switch direction {
		case "U":
			di, dj = 0, 1
		case "D":
			di, dj = 0, -1
		case "L":
			di, dj = -1, 0
		case "R":
			di, dj = 1, 0
		}

		for count, _ := strconv.Atoi(countstr); count > 0; count-- {
			// update head
			i[0] += di
			j[0] += dj
			pos[0][fmt.Sprint(i[0], ",", j[0])] = true

			// update every other knot, in order, based on the previous one.
			for c := 1; c < 10; c++ {
				i[c], j[c] = update(i[c-1], j[c-1], i[c], j[c])
				pos[c][fmt.Sprint(i[c], ",", j[c])] = true
			}
		}

	}

	fmt.Printf("part one: %v\n", len(pos[1]))
	fmt.Printf("part two: %v\n", len(pos[9]))
}

func update(hi, hj, ti, tj int) (int, int) {
	if adjacent(hi, hj, ti, tj) {
		return ti, tj
	}

	idiff := hi - ti
	jdiff := hj - tj

	// cases where they are stil in a single row or column
	if jdiff == 0 {
		if idiff > 1 {
			return ti + 1, tj
		} else if idiff < -1 {
			return ti - 1, tj
		}
	}

	if idiff == 0 {
		if jdiff > 1 {
			return ti, tj + 1
		} else if jdiff < -1 {
			return ti, tj - 1
		}
	}

	// From the prompt:
	// > Otherwise, if the head and tail aren't touching and aren't in the same
	// > row or column, the tail always moves one step diagonally to keep up
	// So, just "shorten" the differences to units.
	if idiff > 1 {
		idiff = 1
	} else if idiff < -1 {
		idiff = -1
	}

	if jdiff > 1 {
		jdiff = 1
	} else if jdiff < -1 {
		jdiff = -1
	}

	return ti + idiff, tj + jdiff
}

// This was used more directly for part one, where the tail always just
// moved to the recently-vacated spot where the head was.
func adjacent(hi, hj, ti, tj int) bool {
	idiff := hi - ti
	if idiff > 1 || idiff < -1 {
		return false
	}

	jdiff := hj - tj
	return !(jdiff > 1 || jdiff < -1)
}
