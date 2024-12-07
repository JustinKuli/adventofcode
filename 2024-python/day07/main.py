#!/usr/bin/env python3

data = []
with open('data.txt', 'r') as file:
    for line in file:
        parts = line[:-1].split(": ")
        numbers = parts[1].split(" ")
        nums = []
        for n in numbers:
            nums.append(int(n))

        data.append({
            'val': int(parts[0]),
            'nums': nums, 
        })

def possible(val, nums):
    if len(nums) == 1:
        return val == nums[0]

    if nums[0] > val:
        return False

    # Try multiplication
    if possible(val, [nums[0]*nums[1]] + nums[2:]):
        return True
    
    return possible(val, [nums[0]+nums[1]] + nums[2:])

def part_one():
    acc = 0
    for d in data:
        if possible(d['val'], d['nums']):
            acc += d['val']
    print("part one answer:", acc)

def possible2(val, nums):
    if len(nums) == 1:
        return val == nums[0]

    if nums[0] > val:
        return False

    # Try concatenation
    if possible2(val, [int(str(nums[0]) + str(nums[1]))] + nums[2:]):
        return True

    # Try multiplication
    if possible2(val, [nums[0]*nums[1]] + nums[2:]):
        return True

    return possible2(val, [nums[0]+nums[1]] + nums[2:])

def part_two():
    acc = 0
    for d in data:
        if possible2(d['val'], d['nums']):
            acc += d['val']
    print("part two answer:", acc)

part_one()
part_two()
