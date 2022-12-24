package day13

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func origRun(title, file string) {
	f, _ := os.Open(file)
	defer f.Close()

	idx := 1
	sum := 0
	lineType := 0
	line1 := ""
	line2 := ""
	fs := bufio.NewScanner(f)
	for fs.Scan() {
		switch lineType {
		case 0:
			line1 = fs.Text()
		case 1:
			line2 = fs.Text()
		case 2:
			if origcompare(line1, line2, "") < 0 {
				fmt.Println(idx)
				sum += idx
			}
			idx++
		}

		lineType = (lineType + 1) % 3
	}

	fmt.Printf("%v - part one: %v\n", title, sum) // 5994 was too high, 5618 is too low, 5750 is too low, 5850 is ???
	fmt.Printf("%v - part two: %v\n", title, 13)
}

func origcompare(left, right, indent string) int {
	fmt.Printf("%vCompare: %v vs %v\n", indent, left, right)

	leftInt, leftIntErr := strconv.Atoi(left)
	rightInt, rightIntErr := strconv.Atoi(right)

	if leftIntErr == nil && rightIntErr == nil { // both integers
		fmt.Println(indent, "simple comparison")
		return leftInt - rightInt
	}

	// there was a section here for adding [] to lonely integers, but it didn't really seem necessary

	left = strings.TrimPrefix(left, "[")
	left = strings.TrimSuffix(left, "]")

	right = strings.TrimPrefix(right, "[")
	right = strings.TrimSuffix(right, "]")

	if left == "" && right == "" {
		fmt.Println(indent, "both are empty?")
		return 0
	}

	if left == "" {
		fmt.Println(indent, "left ended first")
		return -1
	} else if right == "" {
		fmt.Println(indent, "right ended first")
		return 1
	}

	// case like 1 vs 1 now
	leftInt, leftIntErr = strconv.Atoi(left)
	rightInt, rightIntErr = strconv.Atoi(right)

	if leftIntErr == nil && rightIntErr == nil { // both single item lists
		fmt.Println(indent, "single item lists")
		return leftInt - rightInt
	}

	if leftIntErr == nil { // left is a single integer, right is a list
		fmt.Println(indent, "weird case 1")
		right, _, _ = origgetItem(right)
		c := origcompare(left, right, indent+" ")
		if c != 0 {
			return c
		} else {
			return -1
		}
	}

	if leftIntErr == nil {
		fmt.Println(indent, "weird case 2")
		left, _, _ = origgetItem(left)
		c := origcompare(left, right, indent+" ")
		if c != 0 {
			return c
		} else {
			return 1
		}
	}

	leftItem, rightItem, found := "", "", false
	leftItem, left, found = origgetItem(left)
	for ; found; leftItem, left, found = origgetItem(left) {
		rightItem, right, found = origgetItem(right)
		if !found {
			fmt.Println(indent, "right ended first")
			return 1
		}

		c := origcompare(leftItem, rightItem, indent+" ")
		if c != 0 {
			fmt.Println(indent, "comparison inside a list")
			return c
		}
	}
	// left ran out
	_, _, found = origgetItem(right)
	if !found {
		fmt.Println(indent, "lists same length")
		return 0
	} else {
		fmt.Println(indent, "left ended first")
		return -1
	}
}

func origgetItem(in string) (string, string, bool) {
	if in == "" {
		return "", "", false
	}

	if !strings.HasPrefix(in, "[") {
		c1, c2, _ := strings.Cut(in, ",")
		return c1, c2, true
	}

	item := ""
	depth := 0
parse:
	for _, r := range in {
		switch r {
		case '[':
			depth++
			item += string(r)
		case ']':
			depth--
			item += string(r)
			if depth == 0 {
				break parse
			}
		default:
			item += string(r)
		}
	}

	in = strings.TrimPrefix(in, item)
	in = strings.TrimPrefix(in, ",")
	return item, in, true
}
