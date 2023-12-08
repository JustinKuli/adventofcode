package day07

import (
	"bufio"
	"fmt"
	"path/filepath"
	"strings"

	"github.com/JustinKuli/adventofcode/2022-go/aoc"
)

func Run() {
	f := aoc.MustOpen("./day07/input.txt")
	defer f.Close()

	fs := bufio.NewScanner(f)
	sizes := make(map[string]int)
	dir := "/"
	for fs.Scan() {
		line := fs.Text()
		parts := strings.Split(line, " ")

		switch parts[0] {
		case "$":
			if parts[1] == "cd" {
				if parts[2] == "/" {
					dir = "/"
				} else {
					dir = filepath.Join(dir, parts[2])
				}
			} // else: ls, so we can skip it
		case "dir":
			// Ensure that every directory ever listed is in our sizes map
			sizes[filepath.Join(dir, parts[1])] += 0
		default:
			size := aoc.MustInt(parts[0])
			sizes[dir] += size
		}
	}

	finalsizes := make(map[string]int, len(sizes))
	for path := range sizes {
		for dir, size := range sizes {
			// Sum all directories that start with the path
			if strings.HasPrefix(dir, path) {
				finalsizes[path] += size
			}
		}
	}

	sum := 0
	for _, s := range finalsizes {
		if s <= 100000 {
			sum += s
		}
	}

	fmt.Printf("part one: %v\n", sum) // 1648397

	need := 30000000 - (70000000 - finalsizes["/"])

	best := finalsizes["/"]
	for _, s := range finalsizes {
		if s < best && s > need {
			best = s
		}
	}

	fmt.Printf("part two: %v\n", best) // 1815525
}
