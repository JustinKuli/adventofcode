#!/usr/bin/env python3

data = []
zeroes = []
with open('data.txt', 'r') as file:
    row = -1
    for line in file:
        row += 1
        data.append(list(line[:-1]))
        col = -1
        for ch in line:
            col += 1
            if ch == "0":
                zeroes.append({'row': row, 'col': col})

def get(data, row, col):
    if row < 0 or row >= len(data):
        return "."
    if col < 0 or col >= len(data[row]):
        return "."
    return data[row][col]

def adjacentAt(data, row, col, level):
    ans = []

    up = get(data, row-1, col)
    if up == level:
        ans.append({'row': row-1, 'col': col})
    
    down = get(data, row+1, col)
    if down == level:
        ans.append({'row': row+1, 'col': col})

    left = get(data, row, col-1)
    if left == level:
        ans.append({'row': row, 'col': col-1})

    right = get(data, row, col+1)
    if right == level:
        ans.append({'row': row, 'col': col+1})

    return ans

def toStr(x):
    return str(x['row']) + "," + str(x['col'])

def fromStr(x):
    c = x.split(',')
    return {'row': int(c[0]), 'col': int(c[1])}

def part_one():
    acc = 0

    for head in zeroes:
        active = set()
        active.add(toStr(head))
        level = 0

        while level < 9:
            # print('level', level)
            # print('active', active)
            next = set()
            level += 1
            for a in active:
                at = fromStr(a)
                for n in adjacentAt(data, at['row'], at['col'], str(level)):
                    next.add(toStr(n))

            active = next

        acc += len(active)

    print("part one answer:", acc) 

def part_two():
    acc = 0

    for head in zeroes:
        active = set()
        active.add(toStr(head))
        level = 0

        while level < 9:
            # print('level', level)
            # print('active', active)
            next = set()
            level += 1
            for a in active:
                at = fromStr(a)
                for n in adjacentAt(data, at['row'], at['col'], str(level)):
                    next.add(toStr(n) + "," + a)

            active = next

        acc += len(active)
    print("part two answer:", acc)

part_one()
part_two()
