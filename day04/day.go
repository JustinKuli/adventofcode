package day04

import (
	"bufio"
	"fmt"

	"github.com/JustinKuli/aoc2022/aoc"
)

func Run() {
	f := aoc.MustOpen("./day04/input.txt")
	defer f.Close()

	fs := bufio.NewScanner(f)
	sum := 0
	sum2 := 0
	for fs.Scan() {
		// each line has a pair of section assignments
		line := fs.Text()
		left, right := aoc.MustCut(line, ",")

		lMinStr, lMaxStr := aoc.MustCut(left, "-")
		lMin := aoc.MustInt(lMinStr)
		lMax := aoc.MustInt(lMaxStr)

		rMinStr, rMaxStr := aoc.MustCut(right, "-")
		rMin := aoc.MustInt(rMinStr)
		rMax := aoc.MustInt(rMaxStr)

		if rMin < lMin {
			// swap so the left range actually starts to the left
			lMin, lMax, rMin, rMax = rMin, rMax, lMin, lMax
		}

		//  (  right entirely in left  )  or  ( left entirely in right )
		if (rMin <= lMax && rMax <= lMax) || (rMin == lMin) {
			sum += 1
		}

		if rMin <= lMax { // any overlap
			sum2 += 1
		}

	}

	fmt.Printf("part one: %v\n", sum)  // 483
	fmt.Printf("part two: %v\n", sum2) // 874
}
