#!/usr/bin/env python3

data = []
with open('data.txt', 'r') as file:
    for line in file:
        data.append(line[:-1]) # Strip the newline char!

def part_one():
    acc = 0
    print("part one answer:", acc) 

def part_two():
    acc = 0
    print("part two answer:", acc)

part_one()
part_two()
