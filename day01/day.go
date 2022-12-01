package day01

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
)

func Run() {
	f, _ := os.Open("./day01/input.txt")
	defer f.Close()

	s := bufio.NewScanner(f)
	calSum := 0
	elfCals := make([]int, 0)
	for s.Scan() {
		line := s.Text()

		if len(line) == 0 {
			elfCals = append(elfCals, calSum)
			calSum = 0
		} else {
			lineCals, _ := strconv.Atoi(line)
			calSum += lineCals
		}
	}

	sort.Sort(sort.Reverse(sort.IntSlice(elfCals)))

	fmt.Printf("Part 1: %v\n", elfCals[0])
	fmt.Printf("Part 2: %v\n", elfCals[0]+elfCals[1]+elfCals[2])
}
