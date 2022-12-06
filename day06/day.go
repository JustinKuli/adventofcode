package day06

import (
	"bufio"
	"fmt"
	"os"
)

func Run() {
	f, _ := os.Open("./day06/input.txt")
	defer f.Close()

	fs := bufio.NewScanner(f)
	for fs.Scan() {
		line := fs.Text()

		fmt.Printf("part one: %v\n", firstIdxOfNDistinct(line, 4))
		fmt.Printf("part two: %v\n", firstIdxOfNDistinct(line, 14))
	}
}

func firstIdxOfNDistinct(line string, n int) int {
	last := ""
	for idx, r := range line {
		if idx < n {
			last += string(r)
		} else {
			last = last[1:] + string(r)

			dupe := false
			for i, c1 := range last {
				for _, c2 := range last[i+1:] {
					if c1 == c2 {
						dupe = true
					}
				}
			}

			if !dupe {
				return idx
			}
		}
	}

	return -1
}
