#!/usr/bin/env python3

antennae = {}
data = []

size = 50

with open('data.txt', 'r') as file:
    row = 0
    for line in file:
        data.append(list(line[:-1])) # Strip the newline char!
        col = 0
        for ch in list(line[:-1]):
            if ch == ".":
                col += 1
                continue
            if ch in antennae:
                antennae[ch].append({'row': row, 'col': col})
            else:
                antennae[ch] = [{'row': row, 'col': col}]
            col += 1
        row += 1

def part_one():
    antinodes = set()
    for freq in antennae:
        i = 1
        for ant1 in antennae[freq]:
            for ant2 in antennae[freq][i:]:
                row_diff = ant2['row'] - ant1['row']
                col_diff = ant2['col'] - ant1['col']

                loc1_row = ant1['row'] - row_diff
                loc1_col = ant1['col'] - col_diff
                if loc1_row >= 0 and loc1_row < size and loc1_col >= 0 and loc1_col < size:
                    antinodes.add(str(loc1_row)+","+str(loc1_col))
                
                loc2_row = ant2['row'] + row_diff
                loc2_col = ant2['col'] + col_diff
                if loc2_row >= 0 and loc2_row < size and loc2_col >= 0 and loc2_col < size:
                    antinodes.add(str(loc2_row)+","+str(loc2_col))

            i+=1

    print("part one answer:", len(antinodes))

def part_two():
    antinodes = set()
    for freq in antennae:
        i = 1
        for ant1 in antennae[freq]:
            for ant2 in antennae[freq][i:]:
                antinodes.add(str(ant1['row'])+","+str(ant1['col']))
                antinodes.add(str(ant2['row'])+","+str(ant2['col']))

                row_diff = ant2['row'] - ant1['row']
                col_diff = ant2['col'] - ant1['col']
                
                loc1_row = ant1['row'] - row_diff
                loc1_col = ant1['col'] - col_diff
                while loc1_row >= 0 and loc1_row < size and loc1_col >= 0 and loc1_col < size:
                    antinodes.add(str(loc1_row)+","+str(loc1_col))
                    loc1_row -= row_diff
                    loc1_col -= col_diff

                loc2_row = ant2['row'] + row_diff
                loc2_col = ant2['col'] + col_diff
                while loc2_row >= 0 and loc2_row < size and loc2_col >= 0 and loc2_col < size:
                    antinodes.add(str(loc2_row)+","+str(loc2_col))
                    loc2_row += row_diff
                    loc2_col += col_diff

            i+=1

    print("part one answer:", len(antinodes))

part_one()
part_two()

