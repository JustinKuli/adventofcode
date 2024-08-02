package day16a

import (
	"bufio"
	"fmt"
	"strings"

	"github.com/JustinKuli/adventofcode/2022-go/aoc"
)

type rawNode struct {
	rate        int
	connections []string
}

func rawNodeFrom(inp string) (string, rawNode) {
	// Example: "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB"
	//  OR    : "Valve JJ has flow rate=21; tunnel leads to valve II"
	// The differences are annoying...
	inp = strings.TrimPrefix(inp, "Valve ")
	name, inp := aoc.MustCut(inp, " has flow rate=")
	flowStr, inp := aoc.MustCut(inp, "; tunnel")
	inp = strings.TrimPrefix(inp, "s lead to valves ")
	inp = strings.TrimPrefix(inp, " leads to valve ")

	flow := aoc.MustInt(flowStr)
	cxns := strings.Split(inp, ", ")

	result := rawNode{rate: flow, connections: cxns}
	// fmt.Println("made", name, result)
	return name, result
}

func Run(title, file string) {
	f := aoc.MustOpen(file)
	defer f.Close()

	fs := bufio.NewScanner(f)
	nodes := make(map[string]rawNode)
	for fs.Scan() {
		name, node := rawNodeFrom(fs.Text())
		nodes[name] = node
	}

	// fmt.Println("raw nodes:")
	// for name, node := range nodes {
	// 	fmt.Printf("  %v -> %v\n", name, strings.Join(node.connections, ", "))
	// }

	cNodes := cleanerNodes(nodes)
	// fmt.Println("\ncleaned nodes:")
	// for name, node := range cNodes {
	// 	conns := make([]string, 0)
	// 	for n, c := range node.connections {
	// 		conns = append(conns, fmt.Sprintf("%v(%v)", n, c))
	// 	}

	// 	fmt.Printf("  %v -> %v\n", name, strings.Join(conns, ", "))
	// }

	pt1 := dfs1(cNodes, "AA", 30, map[string]bool{}, 0)

	fmt.Printf("%v - part one: %v\n", title, pt1)

	pt2 := dfs2(cNodes, gameState{
		pos1:     "AA",
		pos2:     "AA",
		wait2:    0,
		time:     26,
		opened:   map[string]bool{},
		released: 0,
	})

	fmt.Printf("%v - part two: %v\n", title, pt2)
}

type cleanNode struct {
	rate        int
	connections map[string]int
}

// "complete" the graph so every non-zero rate node (plus AA) knows how far it is from every other
// non-zero rate node.
func cleanerNodes(nodes map[string]rawNode) map[string]cleanNode {
	ans := make(map[string]cleanNode)

	nonzeroNodeCount := 0
	for _, node := range nodes {
		if node.rate != 0 {
			nonzeroNodeCount++
		}
	}

	for name, node := range nodes {
		// fmt.Println(name)
		if name != "AA" && node.rate == 0 {
			continue
		}

		connections := make(map[string]int)
		neededConnections := nonzeroNodeCount
		if name != "AA" {
			neededConnections-- // don't need to connect to self
		}

		distance := 1
		visited := map[string]bool{name: true}
		currs := map[string]bool{name: true}

		for len(connections) != neededConnections {
			newCurrs := make(map[string]bool)
			for curr := range currs {
				for _, next := range nodes[curr].connections {
					if visited[next] {
						continue
					}

					visited[next] = true
					newCurrs[next] = true

					if nodes[next].rate != 0 {
						connections[next] = distance
					}
				}
			}

			distance++
			currs = newCurrs
			// fmt.Println(connections)
		}

		ans[name] = cleanNode{
			rate:        node.rate,
			connections: connections,
		}
	}

	return ans
}

// depth-first-search recursively. The "completed" graph helps simplify finding the options a bit here.
func dfs1(nodes map[string]cleanNode, pos string, time int, opened map[string]bool, released int) int {
	options := []string{}
	for name, distance := range nodes[pos].connections {
		if !opened[name] && (distance+1) < time {
			options = append(options, name)
		}
	}

	releasing := active(nodes, opened)

	// No unopened valves reachable within time limit
	if len(options) == 0 {
		return released + releasing*time
	}

	best := 0
	for _, next := range options {
		newOpened := map[string]bool{next: true}
		for k := range opened {
			newOpened[k] = true
		}
		timeSpend := nodes[pos].connections[next] + 1
		newReleased := released + releasing*timeSpend

		ans := dfs1(nodes, next, time-timeSpend, newOpened, newReleased)
		if ans > best {
			best = ans
		}
	}

	return best
}

func active(nodes map[string]cleanNode, opened map[string]bool) int {
	sum := 0

	for name, node := range nodes {
		if opened[name] {
			sum += node.rate
		}
	}

	return sum
}

// It's slow, but... whatever.
func dfs2(nodes map[string]cleanNode, state gameState) int {
	options := []string{}
	for name, distance := range nodes[state.pos1].connections {
		if !state.opened[name] && name != state.pos2 && (distance+1) < state.time {
			options = append(options, name)
		}
	}

	if state.wait2 == 0 {
		state.opened[state.pos2] = true
	}

	releasing := active(nodes, state.opened)

	if len(options) == 0 {
		newOpened := map[string]bool{state.pos2: true}
		for k := range state.opened {
			newOpened[k] = true
		}
		newReleased := state.released + releasing*state.wait2

		// simplify to just player 2, since we've determined that player 1 can not make more moves
		return dfs1(nodes, state.pos2, state.time-state.wait2, newOpened, newReleased)
	}

	best := 0
	for _, next := range options {
		wait1 := nodes[state.pos1].connections[next] + 1

		if wait1 < state.wait2 {
			newOpened := map[string]bool{next: true}
			for k := range state.opened {
				newOpened[k] = true
			}

			ans := dfs2(nodes, gameState{
				pos1:     next,
				pos2:     state.pos2,
				wait2:    state.wait2 - wait1,
				time:     state.time - wait1,
				opened:   newOpened,
				released: state.released + releasing*wait1,
			})
			if ans > best {
				best = ans
			}
		} else {
			newOpened := map[string]bool{state.pos2: true}
			for k := range state.opened {
				newOpened[k] = true
			}

			ans := dfs2(nodes, gameState{
				pos1:     state.pos2,
				pos2:     next,
				wait2:    wait1 - state.wait2,
				time:     state.time - state.wait2,
				opened:   newOpened,
				released: state.released + releasing*state.wait2,
			})
			if ans > best {
				best = ans
			}
		}
	}

	return best
}

type gameState struct {
	pos1     string
	pos2     string // location player 2 *will* be at, after wait2 additional minutes
	wait2    int
	time     int
	opened   map[string]bool
	released int
}
