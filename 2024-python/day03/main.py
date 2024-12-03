#!/usr/bin/env python3

import re

# this was originally a list of the lines, but part two needed
# all of the lines combined for proper handling of the commands.
data = [""]

with open('data.txt', 'r') as file:
    for line in file:
        data[0] += line[:-1] # Strip the newline char!

def part_one():
    acc = 0
    for line in data:
        matches = re.findall(r"mul\(\d+,\d+\)", line)
        for m in matches:
            # m looks like 'mul(123,45)'
            numpart = m[4:len(m)-1] # remove the starting 'mul(' and ending ')'
            nums = numpart.split(",")
            acc += (int(nums[0]) * int(nums[1]))

    print("part one answer:", acc)

def part_two():
    acc = 0
    for line in data:
        do_segments = line.split("do()")
        for seg in do_segments:
            if "don't()" in seg:
                parts = seg.split("don't()")
                # remove everything after the command.
                # The next do() starts the next segment.
                seg = parts[0]

            matches = re.findall(r"mul\(\d+,\d+\)", seg)
            for m in matches:
                numpart = m[4:len(m)-1]
                nums = numpart.split(",")
                acc += (int(nums[0]) * int(nums[1]))

    print("part two answer:", acc)

part_one()
part_two()
