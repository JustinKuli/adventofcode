package day03

import (
	"bufio"
	"fmt"
	"log"

	"github.com/JustinKuli/adventofcode/2022-go/aoc"
)

func Run() {
	f := aoc.MustOpen("./day03/input.txt")
	defer f.Close()

	fs := bufio.NewScanner(f)
	sum1 := 0
	sum2 := 0
	c := 0
	common := ""
	for fs.Scan() {
		line := fs.Text()

		sum1 += priority(getMispackedType(line))

		switch c {
		case 0:
			common = line
		case 1:
			common = getCommonTypes(common, line)
		case 2:
			common = getCommonTypes(common, line)
			if len(common) != 1 {
				log.Fatalf("no single common item, common=%v", common)
			}
			sum2 += priority([]rune(common)[0])
			c = -1
		}
		c++
	}

	fmt.Printf("Part 1: %v\n", sum1) // 7850
	fmt.Printf("Part 2: %v\n", sum2) // 2581
}

func getMispackedType(ruck string) rune {
	for _, t1 := range ruck[:len(ruck)/2] {
		for _, t2 := range ruck[len(ruck)/2:] {
			if t1 == t2 {
				return t1
			}
		}
	}
	log.Fatal("no mispacked items")
	return 0
}

func getCommonTypes(ruck1, ruck2 string) string {
	// use a map (as a set) to de-duplicate
	common := make(map[rune]bool)
	for _, r1 := range ruck1 {
		for _, r2 := range ruck2 {
			if r1 == r2 {
				common[r1] = true
			}
		}
	}

	ans := ""
	for r := range common {
		ans += string(r)
	}
	return ans
}

func priority(r rune) int {
	if 'a' <= r && r <= 'z' {
		return int(r - 'a' + 1)
	} else {
		return int(r - 'A' + 27)
	}
}
