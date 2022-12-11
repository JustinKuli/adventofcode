package day11

import (
	"fmt"
	"sort"
)

type monkey struct {
	items       []int
	operation   func(int) int // includes the divide by 3
	throw       func(int) int // returns which monkey to throw to
	inspections int
}

func Run(title, file string) {
	monkeys := make([]monkey, 0)
	mod := 1
	switch title {
	case "sample":
		mod = 23 * 19 * 13 * 17
		monkeys = append(monkeys, monkey{
			items: []int{79, 98},
			operation: func(old int) int {
				return (old * 19) % mod
			},
			throw: func(val int) int {
				if val%23 == 0 {
					return 2
				}
				return 3
			},
		})

		monkeys = append(monkeys, monkey{
			items: []int{54, 65, 75, 74},
			operation: func(old int) int {
				return (old + 6) % mod
			},
			throw: func(val int) int {
				if val%19 == 0 {
					return 2
				}
				return 0
			},
		})

		monkeys = append(monkeys, monkey{
			items: []int{79, 60, 97},
			operation: func(old int) int {
				return (old * old) % mod
			},
			throw: func(val int) int {
				if val%13 == 0 {
					return 1
				}
				return 3
			},
		})

		monkeys = append(monkeys, monkey{
			items: []int{74},
			operation: func(old int) int {
				return (old + 3) % mod
			},
			throw: func(val int) int {
				if val%17 == 0 {
					return 0
				}
				return 1
			},
		})
	case "real":
		mod = 2 * 17 * 19 * 3 * 5 * 13 * 7 * 11
		// Monkey 0
		monkeys = append(monkeys, monkey{
			items: []int{83, 62, 93},
			operation: func(old int) int {
				return (old * 17) % mod
			},
			throw: func(val int) int {
				if val%2 == 0 {
					return 1
				}
				return 6
			},
		})
		// Monkey 1
		monkeys = append(monkeys, monkey{
			items: []int{90, 55},
			operation: func(old int) int {
				return (old + 1) % mod
			},
			throw: func(val int) int {
				if val%17 == 0 {
					return 6
				}
				return 3
			},
		})
		// Monkey 2
		monkeys = append(monkeys, monkey{
			items: []int{91, 78, 80, 97, 79, 88},
			operation: func(old int) int {
				return (old + 3) % mod
			},
			throw: func(val int) int {
				if val%19 == 0 {
					return 7
				}
				return 5
			},
		})
		// Monkey 3
		monkeys = append(monkeys, monkey{
			items: []int{64, 80, 83, 89, 59},
			operation: func(old int) int {
				return (old + 5) % mod
			},
			throw: func(val int) int {
				if val%3 == 0 {
					return 7
				}
				return 2
			},
		})
		// Monkey 4
		monkeys = append(monkeys, monkey{
			items: []int{98, 92, 99, 51},
			operation: func(old int) int {
				return (old * old) % mod
			},
			throw: func(val int) int {
				if val%5 == 0 {
					return 0
				}
				return 1
			},
		})
		// Monkey 5
		monkeys = append(monkeys, monkey{
			items: []int{68, 57, 95, 85, 98, 75, 98, 75},
			operation: func(old int) int {
				return (old + 2) % mod
			},
			throw: func(val int) int {
				if val%13 == 0 {
					return 4
				}
				return 0
			},
		})
		// Monkey 6
		monkeys = append(monkeys, monkey{
			items: []int{74},
			operation: func(old int) int {
				return (old + 4) % mod
			},
			throw: func(val int) int {
				if val%7 == 0 {
					return 3
				}
				return 2
			},
		})
		// Monkey 7
		monkeys = append(monkeys, monkey{
			items: []int{68, 64, 60, 68, 87, 80, 82},
			operation: func(old int) int {
				return (old * 19) % mod
			},
			throw: func(val int) int {
				if val%11 == 0 {
					return 4
				}
				return 5
			},
		})
	}

	for round := 1; round <= 10000; round++ {
		for i, m := range monkeys {
			for _, worry := range m.items {
				worry = m.operation(worry)
				dest := m.throw(worry)
				monkeys[dest].items = append(monkeys[dest].items, worry)
				monkeys[i].inspections++
			}
			monkeys[i].items = make([]int, 0)
		}
	}

	counts := make([]int, len(monkeys))
	for i, m := range monkeys {
		counts[i] = m.inspections
	}
	sort.Sort(sort.Reverse(sort.IntSlice(counts)))

	fmt.Printf("%v - part one: %v\n", title, counts[0]*counts[1])
}
