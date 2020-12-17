#!/usr/bin/env python3

actives = set()
actives_4d = set()
with open('data.txt', 'r') as file:
    j = 0
    for line in file:
        line = line[:-1] # Strip the newline char!
        for i, char in enumerate(line):
            if char == "#":
                actives.add("{0},{1},0".format(i,j))
                actives_4d.add("{0},{1},0,0".format(i,j))
        j += 1

def count_neighbors(x,y,z):
    count = 0
    for i in range(x-1, x+2):
        for j in range(y-1, y+2):
            for k in range(z-1, z+2):
                if i == x and j == y and k == z:
                    continue
                if "{0},{1},{2}".format(i,j,k) in actives:
                    count += 1
                if count == 4: # slightly early return
                    return count
    return count

def advance():
    global actives
    new_actives = set()
    xs = [int(v.split(sep=",")[0]) for v in actives] 
    ys = [int(v.split(sep=",")[1]) for v in actives]
    zs = [int(v.split(sep=",")[2]) for v in actives]

    for i in range(min(xs)-1, max(xs)+2):
        for j in range(min(ys)-1, max(ys)+2):
            for k in range(min(zs)-1, max(zs)+2):
                n_count = count_neighbors(i,j,k)
                v = "{0},{1},{2}".format(i,j,k)
                if v in actives:
                    if  n_count == 2 or n_count == 3:
                        new_actives.add(v)
                elif n_count == 3:
                    new_actives.add(v)
    actives = new_actives

def print_actives():
    xs = [int(v.split(sep=",")[0]) for v in actives] 
    ys = [int(v.split(sep=",")[1]) for v in actives]
    zs = [int(v.split(sep=",")[2]) for v in actives]
    for k in range(min(zs)-1, max(zs)+2):
        print("z =", k)
        for j in range(min(ys)-1, max(ys)+2):
            for i in range(min(xs)-1, max(xs)+2):
                v = "{0},{1},{2}".format(i,j,k)
                if v in actives:
                    print("#", end="")
                else:
                    print(".", end="")
            print()
        print()
        print()

def part_one():
    for i in range(0,6):
        advance()
    print("part one answer:", len(actives))

def count_neighbors_4d(x,y,z,w):
    count = 0
    for i in range(x-1, x+2):
        for j in range(y-1, y+2):
            for k in range(z-1, z+2):
                for l in range(w-1, w+2):
                    if i == x and j == y and k == z and l == w:
                        continue
                    if "{0},{1},{2},{3}".format(i,j,k,l) in actives_4d:
                        count += 1
                    if count == 4: # slightly early return
                        return count
    return count

def advance_4d():
    global actives_4d
    new_actives = set()
    xs = [int(v.split(sep=",")[0]) for v in actives_4d] 
    ys = [int(v.split(sep=",")[1]) for v in actives_4d]
    zs = [int(v.split(sep=",")[2]) for v in actives_4d]
    ws = [int(v.split(sep=",")[3]) for v in actives_4d]

    for i in range(min(xs)-1, max(xs)+2):
        for j in range(min(ys)-1, max(ys)+2):
            for k in range(min(zs)-1, max(zs)+2):
                for l in range(min(ws)-1, max(ws)+2):
                    n_count = count_neighbors_4d(i,j,k,l)
                    v = "{0},{1},{2},{3}".format(i,j,k,l)
                    if v in actives_4d:
                        if  n_count == 2 or n_count == 3:
                            new_actives.add(v)
                    elif n_count == 3:
                        new_actives.add(v)
    actives_4d = new_actives

def part_two():
    for i in range(0,6):
        advance_4d()
    print("part two answer:", len(actives_4d))

part_one()
part_two()
