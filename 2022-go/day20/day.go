package day20

import (
	"bufio"
	"fmt"

	"github.com/JustinKuli/adventofcode/2022-go/aoc"
)

type dllNode struct {
	prev *dllNode
	next *dllNode
	val  int
}

func Run(title, file string) {
	f := aoc.MustOpen(file)
	defer f.Close()

	data := make([]*dllNode, 0, 5000)
	var zero, prevLine *dllNode

	fs := bufio.NewScanner(f)
	for fs.Scan() {
		line := fs.Text()
		n := aoc.MustInt(line)
		curr := &dllNode{
			prev: prevLine,
			val:  n,
		}

		if prevLine != nil {
			prevLine.next = curr
		}
		prevLine = curr

		if n == 0 {
			zero = curr
		}

		data = append(data, curr)
	}

	// now to close the loop
	prevLine.next = data[0]
	data[0].prev = prevLine

	dkey := 1
	if doPart2 {
		dkey = 811589153
	}

	m := len(data) - 1 // modulo base
	for mix := 1; mix <= 10; mix++ {
		for _, d := range data {
			dv := d.val * dkey

			if dv%m != 0 {
				moveFW := dv % m
				if dv < 0 {
					moveFW += m
				}
				moveBW := -(moveFW - m) + 1 // one more so that both ways can insert "after" curr

				d.prev.next = d.next
				d.next.prev = d.prev
				curr := d

				// move the shorter distance
				if moveFW < moveBW {
					for i := 0; i < moveFW; i++ {
						curr = curr.next
					}
				} else {
					for i := 0; i < moveBW; i++ {
						curr = curr.prev
					}
				}

				curr.next.prev = d
				d.next = curr.next
				d.prev = curr
				curr.next = d
			}
		}
		if !doPart2 {
			break
		}
	}

	ans := 0
	anscurr := zero.next
	for i := 1; i <= 3000; i++ {
		if i%1000 == 0 {
			ans += anscurr.val * dkey
		}
		anscurr = anscurr.next
	}

	part := "one"
	if doPart2 {
		part = "two"
	}
	fmt.Printf("%v - part %v: %v\n", title, part, ans)
}

const doPart2 = false
