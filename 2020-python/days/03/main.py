#!/usr/bin/env python3
import csv

data = []
with open('data.txt', 'r') as file:
    reader = csv.reader(file)
    for row in reader:
        data.append(row[0])

def part_one():
    pos = 0
    count = 0
    for line in data:
        if line[pos] == '#':
            count += 1
        pos = (pos + 3) % len(line)
    print(count)

def count_trees(x_delta, y_delta):
    x = 0
    y = 0
    count = 0
    for line in data:
        if y == 0:
            if line[x] == '#':
                count += 1
            x = (x + x_delta) % len(line)
        y = (y + 1) % y_delta
    return count

def part_two():
    prod = 1
    prod *= count_trees(1,1)
    prod *= count_trees(3,1)
    prod *= count_trees(5,1)
    prod *= count_trees(7,1)
    prod *= count_trees(1,2)
    print(prod)

part_one()
part_two()
