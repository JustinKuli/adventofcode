#!/usr/bin/env python3

def loc(row, col, dir):
    return {'row': row, 'col': col, 'dir': dir}

def toStr(p):
    return str(p['row']) + "," + str(p['col']) + "," + p['dir']

def fromStr(s):
    x = s.split(",")
    return {'row': int(x[0]), 'col': int(x[1]), 'dir': x[2]}

maze = []
known_costs = {}
known_locs = {}
with open('data.txt', 'r') as file:
    row = -1
    for line in file:
        row += 1
        maze.append(line[:-1])
        if "S" in line:
            startStr = toStr(loc(row, line.index("S"), ">"))
            known_costs[0] = set()
            known_costs[0].add(startStr)
            known_locs[startStr] = 0

def part_one():
    acc = -1
    while True:
        acc += 1
        if acc not in known_costs:
            continue

        for lStr in known_costs[acc]:
            l = fromStr(lStr)
            to_check = []
            if l['dir'] == '>':
                to_check.append([loc(l['row'], l['col'] + 1, '>'), acc + 1])
                to_check.append([loc(l['row'], l['col'], 'v'), acc + 1000])
                to_check.append([loc(l['row'], l['col'], '^'), acc + 1000])
            if l['dir'] == 'v':
                to_check.append([loc(l['row'], l['col'], '>'), acc + 1000])
                to_check.append([loc(l['row'] + 1, l['col'], 'v'), acc + 1])
                to_check.append([loc(l['row'], l['col'], '<'), acc + 1000])
            if l['dir'] == '<':
                to_check.append([loc(l['row'], l['col'], 'v'), acc + 1000])
                to_check.append([loc(l['row'], l['col'] - 1, '<'), acc + 1])
                to_check.append([loc(l['row'], l['col'], '^'), acc + 1000])
            if l['dir'] == '^':
                to_check.append([loc(l['row'], l['col'], '>'), acc + 1000])
                to_check.append([loc(l['row'], l['col'], '<'), acc + 1000])
                to_check.append([loc(l['row'] - 1, l['col'], '^'), acc + 1])

            for c in to_check:
                checkloc = c[0]
                cost = c[1]
                val = maze[checkloc['row']][checkloc['col']]
                if val == "E":
                    print("part one answer:", cost)
                    return
                if val == "#":
                    continue

                locStr = toStr(checkloc)
                if locStr in known_locs:
                    if cost < known_locs[locStr]:
                        known_costs[known_locs[locStr]].remove(locStr)
                    else:
                        continue

                known_locs[locStr] = cost
                if cost not in known_costs:
                    known_costs[cost] = set()
                known_costs[cost].add(locStr)

part_one()

def part_two():
    acc = 0
    print("part two answer:", acc)

part_two()
