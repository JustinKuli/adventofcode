package day16

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

type node struct {
	rate        int
	connections []string
}

func nodeFrom(inp string) (string, node) {
	// Example: "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB"
	//  OR    : "Valve JJ has flow rate=21; tunnel leads to valve II"
	// The differences are annoying...
	inp = strings.TrimPrefix(inp, "Valve ")
	name, inp, _ := strings.Cut(inp, " has flow rate=")
	flowStr, inp, _ := strings.Cut(inp, "; tunnel")
	inp = strings.TrimPrefix(inp, "s lead to valves ")
	inp = strings.TrimPrefix(inp, " leads to valve ")

	flow, _ := strconv.Atoi(flowStr)
	cxns := strings.Split(inp, ", ")

	result := node{rate: flow, connections: cxns}
	// fmt.Println("made", name, result)
	return name, result
}

var nodes map[string]node
var memos map[string]int

func docopy(open []string) []string {
	newcopy := make([]string, len(open), len(open)+1) // might add up to 1 item
	copy(newcopy, open)
	return newcopy
}

func Run(title, file string) {
	// if title == "real" {
	// 	return
	// }

	f, _ := os.Open(file)
	defer f.Close()

	fs := bufio.NewScanner(f)
	nodes = make(map[string]node)
	memos = make(map[string]int)
	for fs.Scan() {
		name, node := nodeFrom(fs.Text())
		nodes[name] = node
	}

	pt1, path := brute("AA", "AA", "\n", []string{}, 0, 30)

	fmt.Printf("%v - part one: %v, path: %v\n", title, pt1, path)
	fmt.Printf("%v - part two: %v\n", title, "")
}

func sumVenting(open []string) int {
	sum := 0
	for _, name := range open {
		sum += nodes[name].rate
	}
	return sum
}

func contains(open []string, name string) bool {
	for _, n := range open {
		if n == name {
			return true
		}
	}
	return false
}

const pathfmt string = "%v:%v-%v\n"
const debugpath = false

func brute(pos, prev, path string, open []string, released, timeLeft int) (int, string) {
	memopos := fmt.Sprintf("%v:%v:%v:%v", timeLeft, pos, strings.Join(open, ","), released)
	if memos[memopos] != 0 {
		return memos[memopos], ""
	}
	currentlyVenting := sumVenting(open)
	if debugpath {
		path += fmt.Sprintf(pathfmt, 31-timeLeft, strings.Join(open, ","), currentlyVenting)
	}
	if timeLeft < 1 { // never used? but good to have
		// fmt.Println("base 1", released)
		return released, path
	} else if timeLeft == 1 {
		released += sumVenting(open)
		// fmt.Println("base 2", released)
		return released, path
	} else if timeLeft == 2 {
		released += sumVenting(open) * 2
		if contains(open, pos) {
			// our position is already open
			// we can't move anywhere *and* open one *and* have it do anything
			if debugpath {
				path += fmt.Sprintf(pathfmt, 30-timeLeft, strings.Join(open, ","), currentlyVenting)
			}

			return released, path
		} else {
			// best move is to open this position and wait
			if debugpath {
				path += "!" + pos + "\n"
			}
			ourOpen := docopy(open)
			ourOpen = append(ourOpen, pos)
			sort.Strings(ourOpen)
			currentlyVenting = sumVenting(ourOpen)
			if debugpath {
				path += fmt.Sprintf(pathfmt, 30-timeLeft, strings.Join(ourOpen, ","), currentlyVenting)
			}

			released += nodes[pos].rate
			// fmt.Println("base 4", released)
			return released, path
		}
	}

	// fmt.Println("Bruting", pos, prev, open, released, timeLeft)

	released += sumVenting(open)

	// try moving immediately (in some cases, we'll know this is the best option)
	best := 0
	bestpath := ""
	for _, next := range nodes[pos].connections {
		if next == prev { // Note: this is only right when we're only moving immediately
			continue
		}

		thepath := ""
		if debugpath {
			thepath = path + ">" + next + "\n"
		}
		try, trypath := brute(next, pos, thepath, open, released, timeLeft-1)
		if try > best {
			best = try
			bestpath = trypath
		}
	}

	if contains(open, pos) || nodes[pos].rate == 0 {
		// fmt.Println("return 1")
		memos[memopos] = best
		return best, bestpath
	}

	// try opening this valve, then moving
	thepath := ""
	if debugpath {
		thepath = path + "!" + pos + "\n"
	}
	ourOpen := docopy(open)
	ourOpen = append(ourOpen, pos)
	sort.Strings(ourOpen)
	currentlyVenting = sumVenting(ourOpen)
	if debugpath {
		thepath += fmt.Sprintf(pathfmt, 32-timeLeft, strings.Join(ourOpen, ","), currentlyVenting)
	}
	released += currentlyVenting

	for _, next := range nodes[pos].connections {
		anotherpath := ""
		if debugpath {
			anotherpath = thepath + ">" + next + "\n"
		}
		try, trypath := brute(next, "", anotherpath, ourOpen, released, timeLeft-2)
		if try > best {
			best = try
			bestpath = trypath
		}
	}

	// fmt.Println("return 2")
	memos[memopos] = best
	return best, bestpath
}
