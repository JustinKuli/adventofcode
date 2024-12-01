#!/usr/bin/env python3

import csv

data = []
with open('data.txt', 'r') as file:
# with open('formatted.txt', 'r') as file:
    reader = csv.reader(file)
    for row in reader:
        data.append(row)

def part_one():
    acc = 0
    print("part one answer:", acc) 

def part_two():
    acc = 0
    print("part two answer:", acc)

part_one()
part_two()
