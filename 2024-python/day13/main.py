#!/usr/bin/env python3

machines = []
with open('data.txt', 'r') as file:
    machine = {}
    for line in file:
        if "Button A:" in line:
            machine = {}
            part = line[12:-1]
            nums = part.split(", Y+")
            machine['a_x'] = int(nums[0])
            machine['a_y'] = int(nums[1])
        elif "Button B:" in line:
            part = line[12:-1]
            nums = part.split(", Y+")
            machine['b_x'] = int(nums[0])
            machine['b_y'] = int(nums[1])
        elif "Prize:" in line:
            part = line[9:-1]
            nums = part.split(", Y=")
            machine['p_x'] = int(nums[0])
            machine['p_y'] = int(nums[1])
            machines.append(machine.copy())

# By the power of math
def solver(prize_x, prize_y, a_x, a_y, b_x, b_y):
    b_top = a_y*prize_x - a_x*prize_y
    b_bot = a_y*b_x - a_x*b_y
    
    if b_bot == 0:
        return False
    
    if b_top % b_bot != 0:
        return False
    
    b = b_top // b_bot

    if a_x == 0:
        print('this is bad')
        return False

    a_top = prize_x - b_x*b

    if a_top % a_x != 0:
        return False
    
    a = a_top // a_x

    return (a,b)

def part_one():
    acc = 0
    for m in machines:
        sol = solver(m['p_x'], m['p_y'], m['a_x'], m['a_y'], m['b_x'], m['b_y'])
        if sol:
            if sol[0] <= 100 and sol[1] <= 100:
                acc += sol[0]*3 + sol[1]

    print("part one answer:", acc) 

def part_two():
    acc = 0
    for m in machines:
        big = 10000000000000 # glad python handles big numbers!
        sol = solver(big+m['p_x'], big+m['p_y'], m['a_x'], m['a_y'], m['b_x'], m['b_y'])
        if sol:
            acc += sol[0]*3 + sol[1]
    print("part two answer:", acc)

part_one()
part_two()
