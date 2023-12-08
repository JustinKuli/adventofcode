package day22

import (
	"bufio"
	"fmt"

	"github.com/JustinKuli/adventofcode/2022-go/aoc"
)

type tile uint8

const (
	off tile = iota
	open
	wall
)

func Run1(title, file string) {
	f := aoc.MustOpen(file)
	defer f.Close()

	board := make([][]tile, 0)

	fs := bufio.NewScanner(f)
	for fs.Scan() {
		line := fs.Text()
		bline := make([]tile, len(line))
		for i, c := range line {
			switch c {
			case '.':
				bline[i] = open
			case '#':
				bline[i] = wall
			}
		}

		board = append(board, bline)

		if len(line) == 0 {
			break // board ended, move on to the path
		}
	}

	fs.Scan()
	instructions := fs.Text()

	posX, posY := 0, 0
	for i, t := range board[0] {
		if t == open {
			posX = i
			break
		}
	}

	move := ""
	incX, incY := 1, 0
	for _, c := range instructions {
		if c != 'L' && c != 'R' {
			move += string(c)
			continue
		}

		toMove := aoc.MustInt(move)
		move = ""

		// do move
		for i := 0; i < toMove; i++ {
			tryX := (posX + incX + len(board[posY])) % len(board[posY]) // always non-negative
			t := board[posY][tryX]
			for t == off {
				tryX = (tryX + incX + len(board[posY])) % len(board[posY])
				t = board[posY][tryX]
			}
			if t == wall {
				break // must be done moving
			}
			posX = tryX

			tryY := (posY + incY + len(board)) % len(board)
			t = safeLookup1(board, tryY, posX)
			for t == off {
				tryY = (tryY + incY + len(board)) % len(board)
				t = safeLookup1(board, tryY, posX)
			}
			if t == wall {
				break
			}
			posY = tryY
		}

		// do turn
		if c == 'L' {
			if incX == 1 {
				incX, incY = 0, -1
			} else if incX == -1 {
				incX, incY = 0, 1
			} else if incY == 1 {
				incX, incY = 1, 0
			} else if incY == -1 {
				incX, incY = -1, 0
			}
		} else {
			if incX == 1 {
				incX, incY = 0, 1
			} else if incX == -1 {
				incX, incY = 0, -1
			} else if incY == 1 {
				incX, incY = -1, 0
			} else if incY == -1 {
				incX, incY = 1, 0
			}
		}
	}

	// undo last turn ( which I added )
	if incX == 1 {
		incX, incY = 0, -1
	} else if incX == -1 {
		incX, incY = 0, 1
	} else if incY == 1 {
		incX, incY = 1, 0
	} else if incY == -1 {
		incX, incY = -1, 0
	}

	// toMove := aoc.MustInt(move)
	// move = ""

	// // do last move
	// for i := 0; i < toMove; i++ {
	// 	tryX := (posX + incX + len(board[posY])) % len(board[posY]) // always non-negative
	// 	t := board[posY][tryX]
	// 	for t == off {
	// 		tryX = (tryX + incX + len(board[posY])) % len(board[posY])
	// 		t = board[posY][tryX]
	// 	}
	// 	if t == wall {
	// 		break // must be done moving
	// 	}
	// 	posX = tryX

	// 	tryY := (posY + incY + len(board)) % len(board)
	// 	t = safeLookup1(board, tryY, posX)
	// 	for t == off {
	// 		tryY = (tryY + incY + len(board)) % len(board)
	// 		t = safeLookup1(board, tryY, posX)
	// 	}
	// 	if t == wall {
	// 		break
	// 	}
	// 	posY = tryY
	// }

	facing := 0
	if incY == 1 {
		facing = 1
	} else if incX == -1 {
		facing = 2
	} else if incY == -1 {
		facing = 3
	}

	fmt.Println("row", posY+1)
	fmt.Println("col", posX+1)
	fmt.Println("facing", facing)

	password := 1000*(posY+1) + 4*(posX+1) + facing

	fmt.Printf("%v - part one: %v\n", title, password)
	fmt.Printf("%v - part two: %v\n", title, 22)
}

func safeLookup1(board [][]tile, y, x int) tile {
	line := board[y]
	if x >= len(line) {
		return off
	}
	return board[y][x]
}

type direction int

const (
	right direction = iota
	down
	left
	up
)

func Run2(title, file string) {
	f := aoc.MustOpen(file)
	defer f.Close()

	board := make([][]tile, 0)

	fs := bufio.NewScanner(f)
	for fs.Scan() {
		line := fs.Text()
		bline := make([]tile, len(line))
		for i, c := range line {
			switch c {
			case '.':
				bline[i] = open
			case '#':
				bline[i] = wall
			}
		}

		if len(line) == 0 {
			break // board ended, move on to the path
		}

		board = append(board, bline)
	}

	fs.Scan()
	instructions := fs.Text()

	posX, posY := 0, 0
	for i, t := range board[0] {
		if t == open {
			posX = i
			break
		}
	}

	move := ""
	d := right
	for _, c := range instructions {
		if c != 'L' && c != 'R' {
			move += string(c)
			continue
		}

		toMove := aoc.MustInt(move)
		move = ""

		// do move
		for i := 0; i < toMove; i++ {
			startD := d
			incX, incY := incs(d)
			tryX, tryY := posX+incX, posY+incY
			if !isOnBoard(board, tryX, tryY) {
				d, tryX, tryY = doCubeMove(board, d, tryX, tryY)
				if !isOnBoard(board, tryX, tryY) {
					fmt.Println(d, tryX, tryY)
					panic("bad1")
				}

				d2 := turn(turn(d, 'L'), 'L')
				incX, incY := incs(d2)
				try2X, try2Y := tryX+incX, tryY+incY
				_, try3X, try3Y := doCubeMove(board, d2, try2X, try2Y)
				if !isOnBoard(board, try3X, try3Y) {
					fmt.Println(d, tryX, tryY)
					panic("bad2")
				}

				if try3X != posX || try3Y != posY {
					fmt.Println(d, posX, posY)
					fmt.Println(d, tryX, tryY)
					fmt.Println(d, try3X, try3Y)
					panic("did not return to beginning")
				}
			}

			// fmt.Println(tryX, tryY)
			if board[tryY][tryX] == wall {
				d = startD
				break // stop moving
			}

			posX, posY = tryX, tryY
		}

		// do turn
		d = turn(d, c)
	}

	// undo extra turn I added
	d = turn(d, 'L')

	password := 1000*(posY+1) + 4*(posX+1) + int(d)

	fmt.Printf("%v - part one: %v\n", title, password)
	fmt.Printf("%v - part two: %v\n", title, 22)
}

func incs(d direction) (x int, y int) {
	switch d {
	case right:
		return 1, 0
	case left:
		return -1, 0
	case down:
		return 0, 1
	case up:
		return 0, -1
	}
	panic("bad directions")
}

func turn(d direction, r rune) direction {
	if r == 'L' {
		switch d {
		case right:
			return up
		case left:
			return down
		case down:
			return right
		case up:
			return left
		}
	} else {
		switch d {
		case right:
			return down
		case left:
			return up
		case down:
			return left
		case up:
			return right
		}
	}
	panic("bad turn")
}

func isOnBoard(board [][]tile, x, y int) bool {
	if y < 0 || y >= len(board) {
		return false
	}

	line := board[y]
	if x < 0 || x >= len(line) {
		return false
	}

	return line[x] != off
}

func doCubeMove(board [][]tile, d direction, tryX, tryY int) (direction, int, int) {
	// input looks like:
	//    A B
	//    C
	//  D E
	//  F
	// and each square is 50x50

	if d == up && tryY < 0 && tryX >= 100 {
		// fmt.Println("B up and around to F")
		d = up
		tryY = len(board) - 1
		tryX -= 100
	} else if d == down && tryY >= len(board) {
		// fmt.Println("F down to B")
		d = down
		tryY = 0
		tryX += 100
	} else if d == up && tryY < 0 && tryX < 100 {
		// fmt.Println("A around to F")
		d = right
		tryY = tryX + 100
		tryX = 0
	} else if d == left && tryY >= 150 && tryX < 0 {
		// fmt.Println("F around to A")
		d = down
		tryX = tryY - 100
		tryY = 0
	} else if d == right && tryX >= 150 {
		// fmt.Println("B around to E")
		d = left
		tryX = 99
		tryY = 149 - tryY
	} else if d == right && tryX >= 100 && tryY >= 100 {
		// fmt.Println("E around to B")
		d = left
		tryX = 149
		tryY = 149 - tryY
	} else if d == right && tryX >= 100 && tryY >= 50 && tryY < 100 {
		// fmt.Println("C over to B")
		d = up
		tryX = tryY + 50
		tryY = 49
	} else if d == down && tryY >= 50 && tryX >= 100 {
		// fmt.Println("B down to C")
		d = left
		tryY = tryX - 50
		tryX = 99
	} else if d == right && tryY >= 150 {
		// fmt.Println("F up to E")
		d = up
		tryX = tryY - 100
		tryY = 149
	} else if d == down && tryY >= 150 && tryX >= 50 {
		// fmt.Println("E down to F")
		d = left
		tryY = tryX + 100
		tryX = 49
	} else if d == left && tryX < 0 && tryY < 150 {
		// fmt.Println("D around to A")
		d = right
		tryX = 50
		tryY = 149 - tryY
	} else if d == left && tryY < 50 {
		// fmt.Println("A to D")
		d = right
		tryX = 0
		tryY = 149 - tryY
	} else if d == left && tryY >= 50 && tryY < 100 {
		// fmt.Println("C to D")
		d = down
		tryX = tryY - 50
		tryY = 100
	} else if d == up && tryX < 50 {
		// fmt.Println("D to C")
		d = right
		tryY = tryX + 50
		tryX = 50
	} else {
		fmt.Println(d, tryX, tryY)
		panic("give up")
	}

	return d, tryX, tryY
}
