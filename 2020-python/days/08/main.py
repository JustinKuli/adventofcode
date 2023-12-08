#!/usr/bin/env python3
import csv

data = []
with open('data.txt', 'r') as file:
    reader = csv.reader(file)
    for row in reader:
        data.append(row[0])

actual_exit = False
# Runs through the program in `data` until it hits a loop or completes.
def go_until_loop():
    global actual_exit
    acc = 0
    visited_lines=set()
    i = 0
    while i not in visited_lines:
        # This condition means it's able to move beyond the data given.
        if i >= len(data):
            actual_exit = True
            return acc

        visited_lines.add(i)

        line = data[i].split()
        if line[0] == "nop":
            i += 1
        elif line[0] == "acc":
            acc += int(line[1])
            i += 1
        elif line[0] == "jmp":
            i += int(line[1])
    return acc

def part_one():
    print("part one answer:", go_until_loop())

# Mutates `data` to change the first nop to jmp (or vice versa)
def initial_increment():
    found_swappables = 0
    for j, line in enumerate(data):
        words = line.split()
        if words[0] == "nop":
            data[j] = line.replace("nop", "jmp")
            return
        if words[0] == "jmp":
            data[j] = line.replace("jmp","nop")
            return

inc = 0
# Mutates `data` to change the (inc)th and (inc+1)th nop to jmp (or vice versa)
# This has the effect of undoing the last increment, and doing a new one.
# If run repeatedly, eventually every possible jmp/nop will be flipped once.
def increment_puzzle():
    found_swappables = 0
    for j, line in enumerate(data):
        words = line.split()
        if words[0] == "nop" or words[0] == "jmp":
            found_swappables += 1
            if found_swappables < inc:
                continue
            if words[0] == "nop":
                data[j] = line.replace("nop", "jmp")
            if words[0] == "jmp":
                data[j] = line.replace("jmp","nop")
            if found_swappables > inc:
                return

def part_two():
    global inc
    initial_increment()
    acc = go_until_loop()
    while not actual_exit:
        inc += 1
        increment_puzzle()
        acc = go_until_loop()

    print("part two answer:", acc)

part_one()
part_two()
