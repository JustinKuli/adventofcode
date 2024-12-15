#!/usr/bin/env python3

def point(row, col):
    return {'row': row, 'col': col}

def toStr(p):
    return str(p['row']) + "," + str(p['col'])

def fromStr(s):
    x = s.split(",")
    return {'row': int(x[0]), 'col': int(x[1])}

def gps(p):
    return 100*p['row'] + p['col']

walls = set()
boxes = set()
robot = {}
instructions = []
with open('data.txt', 'r') as file:
    mapcomplete = False
    row = -1
    for line in file:
        row += 1
        
        if not mapcomplete:
            if len(line) == 1: # only the newline
                mapcomplete = True
                continue

            col = -1
            for ch in line[:-1]:
                col += 1
                if ch == "#":
                    walls.add(toStr(point(row,col)))
                elif ch == ".":
                    continue
                elif ch == "@":
                    robot = point(row,col)
                elif ch == "O":
                    boxes.add(toStr(point(row,col)))
                else:
                    print("unknown char in map", ch)
                    exit()
        else:
            for ch in line[:-1]:
                if ch == "<":
                    instructions.append(point(0, -1))
                elif ch == "^":
                    instructions.append(point(-1, 0))
                elif ch == ">":
                    instructions.append(point(0, 1))
                elif ch == "v":
                    instructions.append(point(1, 0))
                else:
                    print("unknown char in instructions", ch)

def domove(inst):
    global robot
    move_to = point(robot['row'] + inst['row'], robot['col'] + inst['col'])
    if toStr(move_to) in walls:
        return
    if toStr(move_to) not in boxes:
        robot = move_to
        return
    
    # considering whether to move the boxes and the robot
    next = point(move_to['row'] + inst['row'], move_to['col'] + inst['col'])
    while toStr(next) in boxes:
        next = point(next['row'] + inst['row'], next['col'] + inst['col'])
    
    if toStr(next) in walls:
        # can not move anything
        return
    
    robot = move_to
    boxes.remove(toStr(move_to))
    boxes.add(toStr(next))

def part_one():
    for inst in instructions:
        domove(inst)

    acc = 0
    for b in boxes:
        acc += gps(fromStr(b))

    print("part one answer:", acc) 

part_one()
