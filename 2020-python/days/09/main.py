#!/usr/bin/env python3
import csv
import string

data = []
with open('data.txt', 'r') as file:
    reader = csv.reader(file)
    for row in reader:
        data.append(int(row[0]))

def pair_sum_in(desired_sum, nums):
    for i, n in enumerate(nums):
        for m in nums[i+1:]:
            if n + m == desired_sum:
                return [n,m]
    return []

def part_one():
    i = 0
    for line in data[25:]:
        if not pair_sum_in(line, data[i:25+i]):
            print("part one answer:", line) 
            return line
        i += 1

def find_contig_sum(desired_sum, nums):
    for i, _ in enumerate(nums):
        sum = 0
        for j, line in enumerate(nums[i:]):
            sum += line
            if sum == desired_sum:
                return nums[i:i+j+1]
            elif sum > desired_sum:
                break
    return []
    
def part_two():
    contigs = find_contig_sum(part_one(), data)
    print("part two answer:", min(contigs) + max(contigs))

# part_one() # Commented because part_one is called in part_two
part_two()
