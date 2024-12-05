#!/usr/bin/env python3

import csv
import functools

rules = []
with open('data_rules.txt', 'r') as file:
    reader = csv.reader(file, delimiter='|')
    for row in reader:
        rules.append(row)

updates = []
with open('data_updates.txt', 'r') as file:
    reader = csv.reader(file)
    for row in reader:
        updates.append(row)

def in_order(pages):
    for rule in rules:
        if (rule[0] in pages) and (rule[1] in pages):
            idx0 = pages.index(rule[0])
            idx1 = pages.index(rule[1])
            if idx0 > idx1:
                return False
    
    return True

def middle(pages):
    return pages[int(len(pages)/2)]

def part_one():
    acc = 0
    for pages in updates:
        if in_order(pages):
            acc += int(middle(pages))
    print("part one answer:", acc) 

def rulecmp(x,y):
    for rule in rules:
        if (x in rule) and (y in rule):
            if x == rule[0]:
                return -1
            else:
                return 1
    
    return 0

def part_two():
    acc = 0
    for pages in updates:
        if in_order(pages):
            continue
        
        pages.sort(key=functools.cmp_to_key(rulecmp))
        acc += int(middle(pages))

    print("part two answer:", acc)

part_one()
part_two()
