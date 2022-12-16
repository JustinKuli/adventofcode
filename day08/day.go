package day08

import (
	"bufio"
	"fmt"

	"github.com/JustinKuli/aoc2022/aoc"
)

func Run() {
	f := aoc.MustOpen("./day08/input.txt")
	defer f.Close()

	fs := bufio.NewScanner(f)
	grid := make([][]int, 0)
	for fs.Scan() {
		line := fs.Text()
		lineHeights := make([]int, len(line))
		for i, r := range line {
			h := aoc.MustInt(string(r))
			lineHeights[i] = h
		}

		grid = append(grid, lineHeights)
	}

	count := 0
	best := -1
	for i := range grid {
		for j := range grid[i] {
			// p1
			if isVisible(grid, i, j) {
				count++
			}

			// p2
			score := scenicScore(grid, i, j)
			if score > best {
				best = score
			}
		}
	}

	fmt.Printf("part one: %v\n", count) // 1816
	fmt.Printf("part two: %v\n", best)  // 383520
}

func isVisible(grid [][]int, i, j int) bool {
	h := grid[i][j]

	visible := true
	for ii := i - 1; ii >= 0; ii-- {
		if h <= grid[ii][j] {
			visible = false
			break
		}
	}

	if visible {
		return true
	}

	visible = true
	for ii := i + 1; ii < len(grid); ii++ {
		if h <= grid[ii][j] {
			visible = false
			break
		}
	}

	if visible {
		return true
	}

	visible = true
	for jj := j + 1; jj < len(grid[i]); jj++ {
		if h <= grid[i][jj] {
			visible = false
			break
		}
	}

	if visible {
		return true
	}

	visible = true
	for jj := j - 1; jj >= 0; jj-- {
		if h <= grid[i][jj] {
			visible = false
			break
		}
	}

	return visible
}

func scenicScore(grid [][]int, i, j int) int {
	h := grid[i][j]
	score := 1

	count := 0
	for ii := i - 1; ii >= 0; ii-- {
		count++
		if h <= grid[ii][j] {
			break
		}
	}
	score *= count

	count = 0
	for ii := i + 1; ii < len(grid); ii++ {
		count++
		if h <= grid[ii][j] {
			break
		}
	}
	score *= count

	count = 0
	for jj := j + 1; jj < len(grid[i]); jj++ {
		count++
		if h <= grid[i][jj] {
			break
		}
	}
	score *= count

	count = 0
	for jj := j - 1; jj >= 0; jj-- {
		count++
		if h <= grid[i][jj] {
			break
		}
	}

	return score * count
}
