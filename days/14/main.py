#!/usr/bin/env python3
import csv
import string
import sys

instructions = []
with open('data.txt', 'r') as file:
    reader = csv.reader(file)
    for row in reader:
        by_space = row[0].split()
        instructions.append([by_space[0], by_space[-1]])

memory = dict()
mask0 = 0
mask1 = 0
mask = "" # for part 2 only

def sum_memory():
    acc = 0
    for val in memory:
        acc += int(memory[val])
    return acc

# mask0 will have 1s where there is a 0 in the given mask (and 0 everywhere else)
# mask1 will have 1s where there is a 1 in the given mask (and 0 everywhere else)
def update_mask(inp):
    global mask0
    global mask1
    new_mask0 = ""
    new_mask1 = ""
    for c in inp:
        if c == "X":
            new_mask0 += "0"
            new_mask1 += "0"
        elif c == "1":
            new_mask0 += "0"
            new_mask1 += "1"
        elif c == "0":
            new_mask0 += "1"
            new_mask1 += "0"
        else:
            sys.exit("update_mask received an invalid character")
    mask0 = int(new_mask0, 2)
    mask1 = int(new_mask1, 2)

def update_memory(addr, val):
    global memory
    # This is tricky ... there's probably an equivalent form which is more clear.
    # I just figured this out by manually getting mask1 and mask0 for the example,
    #  and working out what steps could work to get me to the answer.
    memory[addr] = ~(~(val | mask1) | (mask0))

def part_one():
    for inst in instructions:
        if inst[0] == "mask":
            update_mask(inst[1])
        else:
            update_memory(inst[0][4:-1], int(inst[1]))
    print("part one answer:", sum_memory())

# Returns a set of addresses that use the current mask on the given address. 
def get_masked_addrs(addr):
    addrs = set()
    addrs.add(addr | mask1)
    for i, c in enumerate(mask):
        if c == "X":
            # The binary number with a 1 at the same spot as the current X:
            x_val = 1 << (36 - i - 1)
            # equivalent: x_val = 2 ** (36 - i - 1)
            # This was tricky - my shift was originally off by one.

            # We will double the size of the set (for X = 0 and X = 1)
            new_addrs = set()
            for a in addrs:
                new_addrs.add(a)
                new_addrs.add(a ^ x_val) # Flip the X bit
            addrs = new_addrs
    return addrs

def update_floaters(addrs, val):
    for a in addrs:
        memory[a] = val

def part_two():
    global mask
    global memory
    memory = dict() # reset the memory after part 1

    for inst in instructions:
        if inst[0] == "mask":
            mask = inst[1] # save the actual mask (with X's)
            update_mask(mask)
        else:
            addrs = get_masked_addrs(int(inst[0][4:-1]))
            update_floaters(addrs, int(inst[1]))
    print("part two answer:", sum_memory())

part_one()
part_two()
