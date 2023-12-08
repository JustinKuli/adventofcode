package day15

import (
	"bufio"
	"fmt"
	"strconv"
	"strings"

	"github.com/JustinKuli/adventofcode/2022-go/aoc"
)

type interval struct {
	start int
	end   int
}

func makeInterval(s, e, max int) interval {
	if e < s {
		s, e = e, s
	}

	if s < 0 {
		s = 0
	}

	if e > max {
		e = max
	}

	return interval{start: s, end: e}
}

func overlap(i1, i2 interval, maxcoord int) (interval, bool) {
	if i1.start > i2.start {
		i1, i2 = i2, i1
	}

	if i1.end < i2.start {
		// no overlap!
		return interval{}, false
	}

	max := i1.end
	if i2.end > max {
		max = i2.end
	}

	return makeInterval(i1.start, max, maxcoord), true
}

func deduplicate(intervals map[string]interval, maxcoord int) bool {
	// fmt.Println("deduplicating ", intervals)
	for k1, i1 := range intervals {
		for k2, i2 := range intervals {
			if k1 == k2 {
				continue
			}

			newint, doesOverlap := overlap(i1, i2, maxcoord)
			if doesOverlap {
				delete(intervals, k1)
				intervals[k2] = newint
				return true
			}
		}
	}
	return false
}

func Run(title, file string) {
	f := aoc.MustOpen(file)
	defer f.Close()

	fs := bufio.NewScanner(f)
	lines := make([]string, 0)
	for fs.Scan() {
		lines = append(lines, fs.Text())
	}

	max_coord := 20
	if title == "real" {
		max_coord = 4000000
	}

	for yToCheck := 0; yToCheck <= max_coord; yToCheck++ {
		// fmt.Println("Checking y = ", yToCheck)
		intervals := make(map[string]interval)
		sensors := make(map[int]bool)
		beacons := make(map[int]bool)

		for inputnumber, text := range lines {
			// Ex: "Sensor at x=2, y=18: closest beacon is at x=-2, y=15"
			line := text

			line = strings.TrimPrefix(line, "Sensor at x=")
			sxStr, line := aoc.MustCut(line, ", y=")
			syStr, line := aoc.MustCut(line, ": closest beacon is at x=")
			bxStr, byStr := aoc.MustCut(line, ", y=")

			sx := aoc.MustInt(sxStr)
			sy := aoc.MustInt(syStr)
			bx := aoc.MustInt(bxStr)
			by := aoc.MustInt(byStr)

			md := mandist(sx-bx, sy-by)

			start := sx - md + intabs(yToCheck-sy)
			end := sx + md - intabs(yToCheck-sy)
			if start < end {
				intervals[strconv.Itoa(inputnumber)] = makeInterval(start, end, max_coord)
			}

			for deduplicate(intervals, max_coord) {
			}

			if sy == yToCheck {
				sensors[sx] = true
			}

			if by == yToCheck {
				beacons[bx] = true
			}
		}

		count := 0
		for _, myInterval := range intervals {
			size := myInterval.end - myInterval.start + 1
			for b := range beacons {
				if b >= myInterval.start && b <= myInterval.end {
					size--
				}
			}
			count += size
		}

		if len(intervals) == 2 {
			keys := make([]string, 0, 2)
			for k := range intervals {
				keys = append(keys, k)
			}

			i1 := intervals[keys[0]]
			i2 := intervals[keys[1]]

			if i1.start != 0 {
				i1, i2 = i2, i1
			}

			if i1.end+1 != i2.start {
				x := i1.end + 1
				y := yToCheck
				fmt.Println("part two", x*4000000+y)
			}
		}
	}
}

func mandist(xdiff, ydiff int) int {
	return intabs(xdiff) + intabs(ydiff)
}

func intabs(x int) int {
	if x < 0 {
		return -1 * x
	}
	return x
}
