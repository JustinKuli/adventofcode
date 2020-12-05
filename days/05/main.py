#!/usr/bin/env python3
import csv

data = []
with open('data.txt', 'r') as file:
    reader = csv.reader(file)
    for row in reader:
        data.append(row[0])

def part_one():
    max_id = 0
    for ticket in data:
        row = 0
        col = 0

        row_bin = 64
        for c in ticket[0:7]:
            if c == 'B':
                row += row_bin
            row_bin = row_bin/2

        col_bin = 4
        for c in ticket[7:]:
            if c == 'R':
                col += col_bin
            col_bin = col_bin/2
        
        id = row * 8 + col
        if id > max_id:
            max_id = id

    print("part one answer:", int(max_id))

def part_two():
    seat_list = list(range(884)) # list of possible tickets from part one
    for ticket in data:
        row = 0
        col = 0

        row_bin = 64
        for c in ticket[0:7]:
            if c == 'B':
                row += row_bin
            row_bin = row_bin/2

        col_bin = 4
        for c in ticket[7:]:
            if c == 'R':
                col += col_bin
            col_bin = col_bin/2
        
        id = row * 8 + col
        seat_list.remove(int(id))
    
    for pos in seat_list:
        if ((pos + 1) in seat_list) and ((pos -1 ) in seat_list):
            continue
        print("part two answer possibility:", pos)

part_one()
part_two()
