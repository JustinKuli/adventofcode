#!/usr/bin/env python3

data = []
guard_pos = [-1, -1]
guard_delta = [-1, 0]
visited = {}
starting = []

def pos_str(pos):
    return str(pos[0]) + "," + str(pos[1])

with open('data.txt', 'r') as file:
    row = 0
    for line in file:
        data.append(list(line[:-1])) # Strip the newline char!
        if "^" in line:
            guard_pos = [row, line.index("^")]
            starting = guard_pos
            visited = {pos_str(guard_pos)}
        row += 1

def turn_right(delta):
    if delta == [-1,0]:
        return [0,1]
    if delta == [0,1]:
        return [1,0]
    if delta == [1,0]:
        return [0,-1]
    if delta == [0,-1]:
        return [-1,0]

def move_forward():
    # I didn't know python hoisted local variables! (if that's the right term)
    global guard_pos
    global guard_delta

    new_loc = [guard_pos[0] + guard_delta[0], guard_pos[1] + guard_delta[1]]

    if new_loc[0] < 0 or new_loc[0] >= len(data):
        return False
    if new_loc[1] < 0 or new_loc[1] >= len(data[new_loc[0]]):
        return False

    if data[new_loc[0]][new_loc[1]] == "#":
        guard_delta = turn_right(guard_delta)
        return True
    
    guard_pos = new_loc
    visited.add(pos_str(guard_pos))

    return True

while move_forward():
    pass

print("part one answer:", len(visited))

possibilities = visited.copy()
possibilities.remove(pos_str(starting))

def rec_str(a,b):
    return str(a[0]) + "," + str(a[1]) + "," + str(b[0]) + "," + str(b[1])

loops = 0
i = 0

for poss in possibilities:
    pos = poss.split(",")

    # place an obstacle and reset the guard
    data[int(pos[0])][int(pos[1])] = "#"
    guard_delta = [-1, 0]
    guard_pos = starting
    
    # simulate, checking for loops.
    seen = {rec_str(guard_pos, guard_delta)}
    while move_forward():
        curr = rec_str(guard_pos, guard_delta)
        if curr in seen:
            loops += 1
            break
        seen.add(curr)

    # reset the map
    data[int(pos[0])][int(pos[1])] = "."

    i += 1
    if i % 500 == 0:
        print(i, loops) # 'progress bar'

print("part two answer:", loops)
