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
		line := fs.Text()

		elfRange := strings.Split(line, ",")

		e0limits := strings.Split(elfRange[0], "-")
		e0min, _ := strconv.Atoi(e0limits[0])
		e0max, _ := strconv.Atoi(e0limits[1])

		e1limits := strings.Split(elfRange[1], "-")
		e1min, _ := strconv.Atoi(e1limits[0])
		e1max, _ := strconv.Atoi(e1limits[1])

		if e1min < e0min {
			// swap so e0min is always at or less than e1min
			e0min, e0max, e1min, e1max = e1min, e1max, e0min, e0max
		}

		if (e1min <= e0max && e1max <= e0max) || (e1min == e0min && e1max <= e1max) {
			sum += 1
			sum2 += 1
		} else if e1min <= e0max {
			sum2 += 1
		}

	}

	fmt.Printf("part one: %v\n", sum)  // 483
	fmt.Printf("part two: %v\n", sum2) // 874
}
