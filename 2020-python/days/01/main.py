#!/usr/bin/env python3
import csv

data = []
with open('data.txt', 'r') as file:
    reader = csv.reader(file)
    for row in reader:
        data.append(int(row[0]))

data.sort()

def part_one():
    for i, lo in enumerate(data):
        # Go backwards through the sorted list, until it gets to `i`:
        for hi in data[:i:-1]:
            if lo + hi < 2020: 
                break
            if lo + hi == 2020:
                 print("Part one answer:", lo * hi)
                 return

def part_two():
    for i, bot in enumerate(data):
        for j, mid in enumerate(data[i+1::]):
            psum = bot + mid
            for top in data[:j+i:-1]:
                sum = psum + top
                if sum < 2020:
                    break
                if sum == 2020:
                    print("Part two answer:", bot * mid * top)
                    return

part_one()
part_two()
