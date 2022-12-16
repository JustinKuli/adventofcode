#!/usr/bin/env bash
# Use: `./init-day.sh 07`
set -euxo pipefail

saniday=$(echo ${1} | sed -z 's/^0//g') # remove leading 0 if present

mkdir "day${1}"

touch "./day${1}/sample.txt"
touch "./day${1}/input.txt"

cat > "./day${1}/day.go" << EOF
package day${1}

import (
	"bufio"
	"fmt"
	"os"
	"strconv"

	"github.com/JustinKuli/aoc2022/aoc"
)

func Run(title, file string) {
	f := aoc.MustOpen(file)
	defer f.Close()

	fs := bufio.NewScanner(f)
	sum := 0
	for fs.Scan() {
		line := fs.Text()
		sum += aoc.MustInt(line)
	}

    fmt.Printf("%v - part one: %v\n", title, ${1})
    fmt.Printf("%v - part two: %v\n", title, ${1})
}
EOF

cat > "./main.go" << EOF
package main

import (
	"github.com/JustinKuli/aoc2022/day${1}"
)

func main() {
	day${1}.Run("sample", "./day${1}/sample.txt")
	day${1}.Run("real", "./day${1}/input.txt")
}
EOF

go run main.go
