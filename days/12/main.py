#!/usr/bin/env python3
import csv
import string

data = []
with open('data.txt', 'r') as file:
    reader = csv.reader(file)
    for row in reader:
        data.append(row[0])
# Note: no L or R instructions use anything other than 90, 180, or 270.

def part_one():
    pos = [0,0]
    heading = 0
    for line in data:
        if line[0] == 'F':
            if heading == 0:
                line = 'E' + line[1:]
            elif heading == 90:
                line = 'S' + line[1:]
            elif heading == 180:
                line = 'W' + line[1:]
            elif heading == 270:
                line = 'N' + line[1:]
        if line[0] == 'N':
            pos[1] += int(line[1:])
        elif line[0] == 'S':
            pos[1] -= int(line[1:])
        elif line[0] == 'E':
            pos[0] += int(line[1:])
        elif line[0] == 'W':
            pos[0] -= int(line[1:])
        elif line[0] == 'L': # convert the L instruction to an equivalent R instruction
            newline = "R"
            newline += str(-int(line[1:]) % 360)
            line = newline
        if line[0] == 'R':
            heading = (heading + int(line[1:])) % 360
    print("part one answer:", abs(pos[0]) + abs(pos[1])) 

def part_two():
    pos = [0,0]
    waypoint = [10, 1]
    for line in data:
        if line[0] == 'F':
            pos[0] += waypoint[0]*int(line[1:])
            pos[1] += waypoint[1]*int(line[1:])
        elif line[0] == 'N':
            waypoint[1] += int(line[1:])
        elif line[0] == 'S':
            waypoint[1] -= int(line[1:])
        elif line[0] == 'E':
            waypoint[0] += int(line[1:])
        elif line[0] == 'W':
            waypoint[0] -= int(line[1:])
        elif line[0] == 'L':
            newline = "R"
            newline += str(-int(line[1:]) % 360)
            line = newline
        if line[0] == 'R':
            heading = int(line[1:])
            tempx = waypoint[0]
            tempy = waypoint[1]
            if heading == 0:
                continue
            elif heading == 90:
                waypoint[0] = tempy
                waypoint[1] = -tempx
            elif heading == 180:
                waypoint[0] = -tempx
                waypoint[1] = -tempy
            elif heading == 270:
                waypoint[0] = -tempy
                waypoint[1] = tempx
    print("part two answer:", abs(pos[0]) + abs(pos[1]))

part_one()
part_two()
