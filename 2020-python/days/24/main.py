#!/usr/bin/env python3

import sys

data = []
with open('formatted.txt', 'r') as file:
    for line in file:
        data.append(line[:-1]) # Strip the newline char!

def calc_pos(inp):
    moves = inp.split(sep=",")
    moves = moves[:-1] # remove blank last entry
    x = 0
    y = 0
    for m in moves:
        if m == "e":
            x += 1
        elif m == "se":
            x += 1
            y -= 1
        elif m == "sw":
            y -= 1
        elif m == "w":
            x -= 1
        elif m == "nw":
            x -= 1
            y += 1
        elif m == "ne":
            y += 1
        else:
            print("Unexpected situation in calc_pos")
            sys.exit(1)
    return (x,y)

def part_one():
    flipped_tiles = set()
    for move in data:
        m = calc_pos(move)
        if m in flipped_tiles:
            flipped_tiles.remove(m)
        else:
            flipped_tiles.add(m)
    print("part one answer:", len(flipped_tiles))

def count_adj(pos, tiles):
    x = pos[0]
    y = pos[1]
    adj = 0

    if (x+1, y) in tiles:
        adj += 1
    if (x+1, y-1) in tiles:
        adj += 1
    if (x, y-1) in tiles:
        adj += 1
    if (x-1, y) in tiles:
        adj += 1
    if (x-1, y+1) in tiles:
        adj += 1
    if (x, y+1) in tiles:
        adj += 1

    return adj

def tomorrow(tiles):
    new_tiles = set()
    w_tiles = set() # not necessarily white

    for b_tile in tiles:
        x = b_tile[0]
        y = b_tile[1]

        w_tiles.add((x+1,y))
        w_tiles.add((x+1,y-1))
        w_tiles.add((x,y-1))
        w_tiles.add((x-1,y))
        w_tiles.add((x-1,y+1))
        w_tiles.add((x,y+1))

        adj = count_adj(b_tile, tiles)
        if adj == 1 or adj == 2:
            new_tiles.add(b_tile)
    
    for w_tile in w_tiles:
        if w_tile in tiles:
            continue # skip tiles we've already calculated
        adj = count_adj(w_tile, tiles)
        if adj == 2:
            new_tiles.add(w_tile)

    return new_tiles

def part_two():    
    flipped_tiles = set()
    for move in data:
        m = calc_pos(move)
        if m in flipped_tiles:
            flipped_tiles.remove(m)
        else:
            flipped_tiles.add(m)

    for _ in range(100):
        flipped_tiles = tomorrow(flipped_tiles)

    print("part two answer:", len(flipped_tiles))

part_one()
part_two()
