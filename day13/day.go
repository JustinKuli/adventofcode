package day13

import (
	"bufio"
	"encoding/json"
	"fmt"
	"math"
	"sort"

	"github.com/JustinKuli/aoc2022/aoc"
)

// This is my second attempt at the day. This time I won't parse everything myself.
func Run(title, file string) {
	f := aoc.MustOpen(file)
	defer f.Close()

	fs := bufio.NewScanner(f)
	idx, sum, lineNum := 1, 0, 0
	line1, line2 := "", ""
	packets := make([]string, 2)
	packets[0] = "[[2]]"
	packets[1] = "[[6]]"
	for fs.Scan() {
		switch lineNum % 3 {
		case 0:
			line1 = fs.Text()
			packets = append(packets, line1)
		case 1:
			line2 = fs.Text()
			packets = append(packets, line2)
		case 2:
			if compare(line1, line2) < 0 {
				sum += idx
			}
			idx++
		}

		lineNum++
	}

	sort.Slice(packets, func(i, j int) bool {
		return compare(packets[i], packets[j]) < 0
	})

	div1idx, div2idx := 0, 0
	for i, p := range packets {
		if title == "sample" {
			fmt.Println(i+1, p)
		}
		if p == "[[2]]" {
			div1idx = i + 1
		}
		if p == "[[6]]" {
			div2idx = i + 1
		}
	}

	fmt.Printf("%v - part one: %v\n", title, sum)
	fmt.Printf("%v - part two: %v\n", title, div1idx*div2idx)
}

func compare(left, right interface{}) int {
	lStr, lStrOk := left.(string)
	rStr, rStrOk := right.(string)
	if lStrOk || rStrOk {
		if !(lStrOk && rStrOk) {
			fmt.Println(left, right)
			fmt.Println(lStr, rStr)
			panic("only one was a string?")
		}

		lNew := make([]interface{}, 0)
		rNew := make([]interface{}, 0)
		if err := json.Unmarshal([]byte(lStr), &lNew); err != nil {
			fmt.Println(left, right)
			fmt.Println(lStr, rStr)
			fmt.Println(err)
			panic("unmarshal lStr error")
		}
		if err := json.Unmarshal([]byte(rStr), &rNew); err != nil {
			fmt.Println(left, right)
			fmt.Println(lStr, rStr)
			fmt.Println(err)
			panic("unmarshal rStr error")
		}

		return compare(lNew, rNew)
	}

	lNum := 0
	lFloat, lNumOk := left.(float64)
	if lNumOk {
		lNum = int(math.Round(lFloat))
	}

	rNum := 0
	rFloat, rNumOk := right.(float64)
	if rNumOk {
		rNum = int(math.Round(rFloat))
	}

	if lNumOk && rNumOk { // both values are numbers
		return lNum - rNum
	}

	lSlice, lSliceOk := left.([]interface{})
	rSlice, rSliceOk := right.([]interface{})

	if lSliceOk && rSliceOk { // both are lists
		for i := 0; ; i++ {
			if i >= len(lSlice) {
				if i < len(rSlice) { // left ran out first
					return -1
				} else { // ran out at same time
					return 0
				}
			} else if i >= len(rSlice) { // right ran out first
				return 1
			}

			if cmp := compare(lSlice[i], rSlice[i]); cmp != 0 {
				return cmp
			}
		}
	}

	// exactly one must be a number.
	if lNumOk {
		return compare([]interface{}{lFloat}, rSlice)
	} else {
		return compare(lSlice, []interface{}{rFloat})
	}
}
