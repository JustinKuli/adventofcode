#!/usr/bin/env python3
import csv
import string

data = []
with open('formatted.txt', 'r') as file:
    reader = csv.reader(file)
    for row in reader:
        data.append(row)

def part_one():
    count = 0
    for line in data:
        combined = ''
        for word in line:
            combined += word
        # string.ascii_lowercase = 'abcdefg...'
        for char in string.ascii_lowercase:
            if char in combined:
                count += 1
                
    print("part one answer:", count)

def part_two():
    count = 0
    for line in data:
        all_yes = line[0]
        for word in line[1:]:
            if len(word) > 0:
                all_yes = set(all_yes).intersection(word)
        count += len(all_yes)

    print("part two answer:", count)

part_one()
part_two()
