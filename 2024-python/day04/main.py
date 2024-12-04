#!/usr/bin/env python3

import csv

data = []
with open('data.txt', 'r') as file:
    for line in file:
        data.append(line[:-1]) # Strip the newline char!

def get(row, col):
    if (row < 0) or (row > len(data) - 1):
        return "."
    
    if (col < 0) or (col > len(data[row]) -1):
        return "."
    
    return data[row][col]

def xmas_count(row, col):
    center = get(row, col)
    if center != "X":
        return 0

    right = get(row, col+1) + get(row, col+2) + get(row, col+3)
    left = get(row, col-1) + get(row, col-2) + get(row, col-3)

    up = get(row-1, col) + get(row-2, col) + get(row-3, col) 
    down = get(row+1, col) + get(row+2, col) + get(row+3, col)

    upleft = get(row-1, col-1) + get(row-2, col-2) + get(row-3, col-3)
    upright = get(row-1, col+1) + get(row-2, col+2) + get(row-3, col+3)
    downleft = get(row+1, col-1) + get(row+2, col-2) + get(row+3, col-3)
    downright = get(row+1, col+1) + get(row+2, col+2) + get(row+3, col+3)

    count = 0

    if right == "MAS":
        count += 1
    if left == "MAS":
        count += 1
    if up == "MAS":
        count += 1
    if down == "MAS":
        count += 1
    if upleft == "MAS":
        count += 1
    if upright == "MAS":
        count += 1
    if downleft == "MAS":
        count += 1
    if downright == "MAS":
        count += 1
        
    return count

def part_one():
    acc = 0
    for row in range(len(data)):
        for col in range(len(data[row])):
            acc += xmas_count(row, col)

    print("part one answer:", acc) 

def crossmas_count(row, col):
    center = get(row, col)
    if center != "A":
        return 0

    m_count = 0
    s_count = 0

    upleft = get(row-1, col-1)
    upright = get(row-1, col+1)
    downleft = get(row+1, col-1)
    downright = get(row+1, col+1)

    if upleft == "M":
        m_count += 1
    if upleft == "S":
        s_count += 1
    
    if upright == "M":
        m_count += 1
    if upright == "S":
        s_count += 1

    if downleft == "M":
        m_count += 1
    if downleft == "S":
        s_count += 1

    if downright == "M":
        m_count += 1
    if downright == "S":
        s_count += 1
    
    # correction for
    # M.S    S.M
    # .A. or .A.
    # S.M    M.S
    if upleft == downright:
        return 0

    if m_count == 2 and s_count == 2:
        return 1
    
    return 0

def part_two():
    acc = 0
    for row in range(len(data)):
        for col in range(len(data[row])):
            acc += crossmas_count(row, col)

    print("part two answer:", acc)

part_one()
part_two()
