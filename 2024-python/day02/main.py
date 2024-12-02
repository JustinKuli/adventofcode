#!/usr/bin/env python3

import csv

data = []
# with open('data.txt', 'r') as file:
with open('formatted.txt', 'r') as file:
    reader = csv.reader(file)
    for row in reader:
        report = []
        for item in row:
            report.append(int(item))
        data.append(report)

def is_safe(report):
    curr = report[0]
    positive = report[1] > report[0]

    for level in report[1:]:
        diff = level - curr
        if (abs(diff) < 1) or (abs(diff) > 3):
            return False
        if positive and (diff < 0):
            return False
        if not positive and (diff > 0):
            return False
        curr = level

    return True

def part_one():
    acc = 0
    for report in data:
        if is_safe(report):
            acc += 1

    print("part one answer:", acc)

def part_two():
    acc = 0
    for report in data:
        if is_safe(report):
            acc += 1
            continue
        
        for i in range(len(report)):
            repcopy = report.copy()
            del repcopy[i]
            if is_safe(repcopy):
                acc += 1
                break

    print("part two answer:", acc)

part_one()
part_two()


