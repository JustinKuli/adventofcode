#!/usr/bin/env python3
# NOTE: Must use format.sh to get the data in a better format before starting.
import csv

data = []
with open('formatted.txt', 'r') as file:
    reader = csv.reader(file)
    for row in reader:
        data.append({
            "min": int(row[0]),
            "max": int(row[1]),
            "letter": row[2],
            "password": row[3]
        })

def part_one():
    valid = 0
    for line in data:
        num = line['password'].count(line['letter'])
        if num >= line['min'] and num <= line['max'] :
            valid += 1
    print('Part one answer:', valid) 

def part_two():
    valid = 0
    for line in data:
        char_one = line['password'][line['min'] - 1]
        char_two = line['password'][line['max'] - 1]
        if (char_one == line['letter']) ^ (char_two == line['letter']) :
            valid += 1
    print('Part two answer:', valid)

part_one()
part_two()
