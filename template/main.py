#!/usr/bin/env python3
import csv
import string

data = []
with open('data.txt', 'r') as file:
    reader = csv.reader(file)
    for row in reader:
        data.append(row[0])

def part_one():
    acc = 0
    print("part one answer:", acc) 

def part_two():
    acc = 0
    print("part two answer:", acc)

part_one()
part_two()
