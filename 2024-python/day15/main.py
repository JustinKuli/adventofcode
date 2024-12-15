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
robot = {}
instructions = []

# These are named for their keys, not their values.
boxPoints = {}
boxIDs = {}
with open('data.txt', 'r') as file:
    boxID = 0
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
                    col += 1
                    walls.add(toStr(point(row,col)))
                elif ch == ".":
                    col += 1
                elif ch == "@":
                    robot = point(row,col)
                    col += 1
                elif ch == "O":
                    boxID += 1

                    p1 = toStr(point(row,col))
                    boxPoints[p1] = boxID

                    col += 1
                    p2 = toStr(point(row,col))
                    boxPoints[p2] = boxID

                    boxIDs[boxID] = [p1, p2]
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
    if toStr(move_to) not in boxPoints:
        robot = move_to
        return
    
    firstID = boxPoints[toStr(move_to)]
    pushingIDs = set()
    pushingIDs.add(firstID)
    nextIDs = set()
    nextIDs.add(firstID)

    while len(nextIDs) != 0:
        nextPoints = set()
        for id in nextIDs:
            points = boxIDs[id]
            # up and down is "easy"
            # but for left and right we need to prevent the box from
            # interacting with itself.
            if inst['col'] == 0: 
                nextPoints.add(points[0])
                nextPoints.add(points[1])
            elif inst['col'] == -1: 
                nextPoints.add(points[0])
            elif inst['col'] == 1:
                nextPoints.add(points[1])
        
        nextIDs = set()
        for pStr in nextPoints:
            p = fromStr(pStr)
            newp = toStr(point(p['row'] + inst['row'], p['col'] + inst['col']))
            if newp in walls:
                # can not move anything
                return
            if newp in boxPoints:
                pushingIDs.add(boxPoints[newp])
                nextIDs.add(boxPoints[newp])
    
    robot = move_to

    boxPointsToAdd = {}

    for id in pushingIDs:
        origStrs = boxIDs[id]
        orig1 = fromStr(origStrs[0])
        orig2 = fromStr(origStrs[1])
        newStr1 = toStr(point(orig1['row'] + inst['row'], orig1['col'] + inst['col']))
        newStr2 = toStr(point(orig2['row'] + inst['row'], orig2['col'] + inst['col']))

        boxIDs[id] = [newStr1, newStr2]
        del boxPoints[origStrs[0]]
        del boxPoints[origStrs[1]]
        boxPointsToAdd[newStr1] = id # can't just add these immediately because
        boxPointsToAdd[newStr2] = id # of other boxes that might be moving

    for b in boxPointsToAdd:
        boxPoints[b] = boxPointsToAdd[b]

def part_two():
    for inst in instructions:
        domove(inst)

    acc = 0
    for id in boxIDs:
        acc += gps(fromStr(boxIDs[id][0]))

    print("part one answer:", acc) 

part_two()

# Debug printing
# for row in range(30):
#     line = ""
#     for col in range(30):
#         spot = toStr(point(row, col))
#         if spot in walls:
#             line += "#"
#         elif spot in boxPoints:
#             line += "O"
#         elif spot == toStr(robot):
#             line += "@"
#         else:
#             line += "."
#     print(line)
