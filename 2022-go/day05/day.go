package day05

import (
	"bufio"
	"fmt"
	"strings"

	"github.com/JustinKuli/adventofcode/2022-go/aoc"
)

/* Initial state
            [J]             [B] [W]
            [T]     [W] [F] [R] [Z]
        [Q] [M]     [J] [R] [W] [H]
    [F] [L] [P]     [R] [N] [Z] [G]
[F] [M] [S] [Q]     [M] [P] [S] [C]
[L] [V] [R] [V] [W] [P] [C] [P] [J]
[M] [Z] [V] [S] [S] [V] [Q] [H] [M]
[W] [B] [H] [F] [L] [F] [J] [V] [B]
 1   2   3   4   5   6   7   8   9
*/

func Run() {
	f := aoc.MustOpen("./day05/input.txt")
	defer f.Close()

	stack := make([][]rune, 10)
	stack[0] = []rune{' '}
	stack[1] = []rune{'W', 'M', 'L', 'F'}
	stack[2] = []rune{'B', 'Z', 'V', 'M', 'F'}
	stack[3] = []rune{'H', 'V', 'R', 'S', 'L', 'Q'}
	stack[4] = []rune{'F', 'S', 'V', 'Q', 'P', 'M', 'T', 'J'}
	stack[5] = []rune{'L', 'S', 'W'}
	stack[6] = []rune{'F', 'V', 'P', 'M', 'R', 'J', 'W'}
	stack[7] = []rune{'J', 'Q', 'C', 'P', 'N', 'R', 'F'}
	stack[8] = []rune{'V', 'H', 'P', 'S', 'Z', 'W', 'R', 'B'}
	stack[9] = []rune{'B', 'M', 'J', 'C', 'G', 'H', 'Z', 'W'}

	fs := bufio.NewScanner(f)
	for fs.Scan() {
		line := fs.Text()

		p1, p2 := aoc.MustCut(line, " from ")

		numstr := strings.TrimPrefix(p1, "move ")
		num := aoc.MustInt(numstr)

		startstr, endstr := aoc.MustCut(p2, " to ")
		start := aoc.MustInt(startstr)
		end := aoc.MustInt(endstr)

		// Part one
		// for num > 0 {
		// 	var crate rune
		// 	stack[start], crate = pop(stack[start])
		// 	stack[end] = append(stack[end], crate)
		// 	num--
		// }

		// Part two
		num2 := num
		for num > 0 {
			var crate rune
			stack[start], crate = pop(stack[start])
			stack[0] = append(stack[0], crate)
			num--
		}
		for num2 > 0 {
			var crate rune
			stack[0], crate = pop(stack[0])
			stack[end] = append(stack[end], crate)
			num2--
		}

	}

	ans := ""
	for _, s := range stack {
		ans += string(s[len(s)-1])
	}

	fmt.Printf("answer: %v\n", ans) // part one: VRWBSFZWM // part two: RBTWJWMCF
}

func pop(list []rune) ([]rune, rune) {
	ans := list[len(list)-1]
	list = list[:len(list)-1]
	return list, ans
}
