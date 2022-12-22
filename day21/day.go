package day21

import (
	"bufio"
	"fmt"
	"strconv"
	"strings"

	"github.com/JustinKuli/aoc2022/aoc"
)

var monkeys map[string]func() string
var memos map[string]string

func monkeyfunc(s string) func() string {
	return func() string {
		isRoot := false
		if strings.HasPrefix(s, "!!") {
			return "x"
		} else if strings.HasPrefix(s, "!?") {
			isRoot = true
			s = strings.TrimPrefix(s, "!?")
		}

		if val, found := memos[s]; found {
			return val
		}

		var m1, m2, op string

		if strings.Contains(s, "+") {
			m1, m2 = aoc.MustCut(s, " + ")
			op = " + "
		} else if strings.Contains(s, "-") {
			m1, m2 = aoc.MustCut(s, " - ")
			op = " - "
		} else if strings.Contains(s, "*") {
			m1, m2 = aoc.MustCut(s, " * ")
			op = " * "
		} else if strings.Contains(s, "/") {
			m1, m2 = aoc.MustCut(s, " / ")
			op = " / "
		} else {
			memos[s] = s
			return s
		}

		m1a := monkeys[m1]()
		m2a := monkeys[m2]()

		m1num, err1 := strconv.Atoi(m1a)
		m2num, err2 := strconv.Atoi(m2a)

		if isRoot {
			return fmt.Sprintf("%v = %v", m1a, m2a)
		}

		ans := ""

		if err1 != nil || err2 != nil {
			ans = fmt.Sprintf("(%v%v%v)", m1a, op, m2a)
		} else {
			switch op {
			case " + ":
				ans = strconv.Itoa(m1num + m2num)
			case " - ":
				ans = strconv.Itoa(m1num - m2num)
			case " * ":
				ans = strconv.Itoa(m1num * m2num)
			case " / ":
				ans = strconv.Itoa(m1num / m2num)
			}
		}

		memos[s] = ans
		return ans
	}
}

func Run(title, file string) {
	f := aoc.MustOpen(file)
	defer f.Close()

	monkeys = make(map[string]func() string)
	memos = make(map[string]string)

	fs := bufio.NewScanner(f)
	for fs.Scan() {
		line := fs.Text()
		name, job := aoc.MustCut(line, ": ")
		if name == "root" {
			job = "!?" + job
		} else if name == "humn" {
			job = "!!"
		}
		monkeys[name] = monkeyfunc(job)
	}

	fmt.Printf("%v - solve for x (sorry): %v\n", title, monkeys["root"]())
}
