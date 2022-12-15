package day15

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

const (
	nobeacon int8 = iota + 1 // TODO: is +1 needed?
	beacon
	sensor
)

type coordmap struct {
	m map[int]map[int]int8
}

func (m *coordmap) put(y, x int, val int8) {
	if y != 10 {
		return
	}

	// don't overwrite those
	if m.m[y][x] == beacon || m.m[y][x] == sensor {
		return
	}

	if m.m[y] == nil {
		m.m[y] = make(map[int]int8)
	}

	m.m[y][x] = val
}

func (m *coordmap) fillmd(center_y, center_x, dist int, val int8) {
	for start := 1; start <= dist; start++ {
		py, px := center_y+start, center_x

		incy, incx := -1, -1
		for py > center_y {
			// fmt.Println(incy, incx)
			m.put(py, px, val)
			py += incy
			px += incx
		}

		incy, incx = -1, 1
		for px < center_x {
			// fmt.Println(incy, incx)
			m.put(py, px, val)
			py += incy
			px += incx
		}

		incy, incx = 1, 1
		for py < center_y {
			// fmt.Println(incy, incx)
			m.put(py, px, val)
			py += incy
			px += incx
		}

		incy, incx = 1, -1
		for px > center_x {
			// fmt.Println(incy, incx)
			m.put(py, px, val)
			py += incy
			px += incx
		}
	}
}

func RunOld(title, file string) {
	f, _ := os.Open(file)
	defer f.Close()

	coordyx := coordmap{m: make(map[int]map[int]int8)} // y, then x.

	fs := bufio.NewScanner(f)
	for fs.Scan() {
		// Ex: "Sensor at x=2, y=18: closest beacon is at x=-2, y=15"
		line := fs.Text()

		line = strings.TrimPrefix(line, "Sensor at x=")
		sxStr, line, _ := strings.Cut(line, ", y=")
		syStr, line, _ := strings.Cut(line, ": closest beacon is at x=")
		bxStr, byStr, _ := strings.Cut(line, ", y=")

		sx, _ := strconv.Atoi(sxStr)
		sy, _ := strconv.Atoi(syStr)
		bx, _ := strconv.Atoi(bxStr)
		by, _ := strconv.Atoi(byStr)

		coordyx.put(sy, sx, sensor)
		coordyx.put(by, bx, beacon)

		coordyx.fillmd(sy, sx, mandistOld(sx-bx, sy-by), nobeacon)

		fmt.Println(sx, sy, coordyx.m[10])
	}

	line_to_check := 10
	if title == "real" {
		line_to_check = 2000000
	}

	count := 0
	for _, val := range coordyx.m[line_to_check] {
		if val == nobeacon {
			count++
		}
	}

	fmt.Printf("%v - part one: %v\n", title, count)
	fmt.Printf("%v - part two: %v\n", title, 15)
}

func mandistOld(xdiff, ydiff int) int {
	if xdiff < 0 {
		xdiff = -1 * xdiff
	}
	if ydiff < 0 {
		ydiff = -1 * ydiff
	}
	fmt.Println("mandist: ", xdiff+ydiff)
	return xdiff + ydiff
}
