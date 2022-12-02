#!/usr/bin/env bash
# Use: `./init-day.sh 07`
set -euxo pipefail

saniday=$(echo ${1} | sed -z 's/^0//g') # remove leading 0 if present

mkdir "day${1}"

curl "https://adventofcode.com/2021/day/${saniday}/input" \
  -H 'pragma: no-cache' \
  -H 'cache-control: no-cache' \
  -H 'user-agent: idk' \
  -H "$(cat cookie-header.txt)" \
  -H 'accept: text/html' > "./day${1}/input.txt"

cat > "./day${1}/day.go" << EOF
package day${1}

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func Run(){
	f, _ := os.Open("./day${1}/input.txt")
	defer f.Close()

	fs := bufio.NewScanner(f)
	sum := 0
	for fs.Scan() {
		line := fs.Text()

		lineVal, err := strconv.Atoi(line)
		if err != nil {
			sum += lineVal
		}
	}

    fmt.Println("initialized day ${1}")
}
EOF

cat > "./main.go" << EOF
package main

import (
	"github.com/JustinKuli/aoc2022/day${1}"
)

func main() {
	day${1}.Run()
}
EOF

go run main.go
