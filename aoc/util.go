package aoc

import (
	"log"
	"os"
	"strconv"
	"strings"
)

func MustOpen(file string) *os.File {
	f, err := os.Open(file)
	if err != nil {
		log.Fatalf("Could not open file '%v': %v", file, err)
	}
	return f
}

func MustInt(s string) int {
	i, err := strconv.Atoi(s)
	if err != nil {
		log.Fatalf("Could not convert '%v' to int: %v", s, err)
	}
	return i
}

func MustCut(s, sep string) (string, string) {
	s1, s2, found := strings.Cut(s, sep)
	if !found {
		log.Fatalf("Could not cut '%v' at '%v': not found", s, sep)
	}
	return s1, s2
}
