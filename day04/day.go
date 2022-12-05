package day04

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func Run() {
	f, _ := os.Open("./day04/input.txt")
	defer f.Close()

	fs := bufio.NewScanner(f)
	sum := 0
	sum2 := 0
	for fs.Scan() {
		// each line has a pair of section assignments
		line := fs.Text()
		sectionAssignment := strings.Split(line, ",")

		leftLimits := strings.Split(sectionAssignment[0], "-")
		lMin, _ := strconv.Atoi(leftLimits[0])
		lMax, _ := strconv.Atoi(leftLimits[1])

		rightLimits := strings.Split(sectionAssignment[1], "-")
		rMin, _ := strconv.Atoi(rightLimits[0])
		rMax, _ := strconv.Atoi(rightLimits[1])

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
