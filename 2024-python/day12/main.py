#!/usr/bin/env python3

data = []
with open('data.txt', 'r') as file:
    for line in file:
        data.append(list(line[:-1]))

def get(row, col):
    if row < 0 or row >= len(data):
        return "."
    if col < 0 or col >= len(data[row]):
        return "."
    return data[row][col]

def toStr(row, col):
    return str(row)+","+str(col)

def fromStr(x):
    s = x.split(",")
    return {'row': int(s[0]), 'col': int(s[1])}

def neighbors(row, col):
    return {
        toStr(row-1, col): get(row-1, col),
        toStr(row+1, col): get(row+1, col),
        toStr(row, col-1): get(row, col-1),
        toStr(row, col+1): get(row, col+1),
    }

counted = set()
def region_cost(row, col):
    plant = get(row,col)
    area = 0
    perimeter = 0

    to_visit = {toStr(row, col)}
    while len(to_visit) > 0:
        next = set()
        for plot in to_visit:
            counted.add(plot)
            area += 1

            p = fromStr(plot)
            neighs = neighbors(p['row'], p['col'])
            for n in neighs:
                if neighs[n] == plant:
                    if n not in counted:
                        next.add(n)
                else:
                    perimeter += 1
        to_visit = next
    
    return area * perimeter

def part_one():
    acc = 0
    for row in range(len(data)):
        for col in range(len(data[row])):
            if toStr(row, col) not in counted:
                acc += region_cost(row, col)

    print("part one answer:", acc) 

def part_two():
    acc = 0
    print("part two answer:", acc)

part_one()
part_two()
