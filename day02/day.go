package day02

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func Run() {
	f, _ := os.Open("./day02/input.txt")
	defer f.Close()

	fs := bufio.NewScanner(f)
	sum1 := 0
	sum2 := 0
	for fs.Scan() {
		line := fs.Text()
		col := strings.Split(line, " ")

		// PART 1
		switch col[0] {
		case "A":
			col[0] = "X" // Rock
		case "B":
			col[0] = "Y" // Paper
		case "C":
			col[0] = "Z" // Scissors
		}

		switch col[1] {
		case "X":
			sum1 += 1
		case "Y":
			sum1 += 2
		case "Z":
			sum1 += 3
		}

		// golfier, but less clear
		// sum1 += int(col[1][0] - 'X' + 1)

		if col[0] == col[1] {
			sum1 += 3
		} else if (col[0] == "X" && col[1] == "Y") ||
			(col[0] == "Y" && col[1] == "Z") ||
			(col[0] == "Z" && col[1] == "X") {
			sum1 += 6
		} // else: lose and gain 0 points

		// PART 2 (remember, part 1 mutated col[0])
		choose := ""
		switch col[1] {
		case "X": // lose
			switch col[0] {
			case "X": // Rock
				choose = "Z"
			case "Y":
				choose = "X"
			case "Z":
				choose = "Y"
			}
		case "Y": // tie
			choose = col[0]

			sum2 += 3
		case "Z": // win
			switch col[0] {
			case "X": // Rock
				choose = "Y"
			case "Y":
				choose = "Z"
			case "Z":
				choose = "X"
			}

			sum2 += 6
		}

		switch choose {
		case "X": // Rock
			sum2 += 1
		case "Y":
			sum2 += 2
		case "Z":
			sum2 += 3
		}

		// golfier, but less clear
		// sum2 += int(choose[0] - 'X' + 1)
	}

	fmt.Printf("Part 1: %v\n", sum1) // 9759
	fmt.Printf("Part 2: %v\n", sum2) // 12429
}
