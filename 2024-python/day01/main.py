#!/usr/bin/env python3

import csv

left = []
right = []
with open('formatted.txt', 'r') as file:
    reader = csv.reader(file)
    for row in reader:
        left.append(int(row[0]))
        right.append(int(row[1]))

def part_one():
    # copy in case I need the originals in part 2 or something
    left1 = left.copy()
    right1 = right.copy()

    left1.sort()
    right1.sort()

    acc = 0
    for i in range(len(left1)):
        acc += abs(left1[i]-right1[i])

    print("part one answer:", acc)    

def part_two():
    rightcounts = {}

    for r in right:
        if r in rightcounts:
            rightcounts[r] += 1
        else:
            rightcounts[r] = 1
    
    acc = 0
    for l in left:
        if l in rightcounts:
            acc += l*rightcounts[l]

    print("part two answer:", acc)

part_one()
part_two()
