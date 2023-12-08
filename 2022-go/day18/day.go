package day18

import (
	"bufio"
	"fmt"
	"strconv"
	"strings"

	"github.com/JustinKuli/adventofcode/2022-go/aoc"
)

type coord3d struct {
	x int
	y int
	z int
}

func coord3dX10From(s string) coord3d {
	strs := strings.Split(s, ",")
	x, _ := strconv.Atoi(strs[0])
	y, _ := strconv.Atoi(strs[1])
	z, _ := strconv.Atoi(strs[2])
	return coord3d{x: x * 10, y: y * 10, z: z * 10}
}

func Run(title, file string) {
	f := aoc.MustOpen(file)
	defer f.Close()

	fs := bufio.NewScanner(f)
	faceCenterX10Counts := make(map[coord3d]int)
	for fs.Scan() {
		c := coord3dX10From(fs.Text())
		c1 := coord3d{x: c.x + 5, y: c.y, z: c.z}
		c2 := coord3d{x: c.x - 5, y: c.y, z: c.z}
		c3 := coord3d{x: c.x, y: c.y + 5, z: c.z}
		c4 := coord3d{x: c.x, y: c.y - 5, z: c.z}
		c5 := coord3d{x: c.x, y: c.y, z: c.z + 5}
		c6 := coord3d{x: c.x, y: c.y, z: c.z - 5}
		faceCenterX10Counts[c1] += 1
		faceCenterX10Counts[c2] += 1
		faceCenterX10Counts[c3] += 1
		faceCenterX10Counts[c4] += 1
		faceCenterX10Counts[c5] += 1
		faceCenterX10Counts[c6] += 1
	}

	count := 0
	for _, instances := range faceCenterX10Counts {
		if instances == 1 {
			count++
		}
	}

	fmt.Printf("%v - part one: %v\n", title, count)
	fmt.Printf("%v - part two: %v\n", title, 18)
}
