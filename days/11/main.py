#!/usr/bin/env python3
import csv
import string

data = []
with open('data.txt', 'r') as file:
    reader = csv.reader(file)
    for row in reader:
        data.append(row[0])

# Reads `data` and counts how many adjacent seats are occupied.
def count_adj(i, j):
    count = 0
    for i_delta in [-1, 0, 1]:
        # If we're on the top or bottom edge of the puzzle, don't go out of bounds
        if i + i_delta < 0 or i + i_delta >= len(data[0]):
            continue
        for j_delta in [-1, 0, 1]:
            # If we're on the left or right edge of the puzzle, don't go out of bounds
            if j + j_delta < 0 or j + j_delta >= len(data[0]):
                continue
            if i_delta == 0 and j_delta == 0:
                continue
            check = data[i+i_delta][j+j_delta]
            if check == "#":
                count += 1
    return count

# This was refactored after the original submission, 
#  to combine what was 2 very similar functions: one for each part
def iterate(count_fn, adj_tol):
    global data
    # To avoid copying issues, we'll create a brand new data set each time.
    new_data = []
    changed = False
    for i, row in enumerate(data):
        new_row = ""
        for j, item in enumerate(row):
            adj = count_fn(i, j)
            if data[i][j] == "L" and adj == 0:
                new_row += "#"
                changed = True
            elif data[i][j] == "#" and adj >= adj_tol:
                new_row += "L"
                changed = True
            else:
                new_row += data[i][j]
        new_data.append(new_row)
    data = new_data
    return changed

def print_rows(debug):
    if debug:
        print("====================")
        for line in data:
            print(line)

def total_occupied():
    count = 0
    for line in data:
        for item in line:
            if item == "#":
                count += 1
    return count

def part_one():
    print_rows(False)
    while iterate(count_adj, 4):
        print_rows(False)
    print("part one answer:", total_occupied())

def count_sight(i, j):
    # an array of each direction "unit vector"
    directions = [[-1,-1],[-1,0],[-1,1],
                  [0,-1],        [0,1],
                  [1,-1], [1,0], [1,1]]

    count = 0
    max_dist = len(data[0])
    for d in directions:
        for l in range(max_dist):
            if l == 0:
                continue
            if i+l*d[0] < 0 or i+l*d[0] >= max_dist:
                break
            if j+l*d[1] < 0 or j+l*d[1] >= max_dist:
                break
            check = data[i+l*d[0]][j+l*d[1]]
            if check == "#":
                count += 1
                break
            elif check == "L":
                break
    return count

def part_two():
    print_rows(False)
    while iterate(count_sight, 5):
        print_rows(False)
    print("part two answer:", total_occupied())

part_one()
part_two()
