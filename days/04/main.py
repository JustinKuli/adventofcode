#!/usr/bin/env python3
import csv

data = []
with open('formatted.txt', 'r') as file:
    reader = csv.reader(file)
    for row in reader:
        data_obj = {}
        for item in row:
            data_obj[item[:3]] = item[4:]
        data.append(data_obj)


def part_one():
    print("part one answer:", "232 (gotten from the grep)")

def valid_byr(input):
    try:
        byr = int(input)
        return (byr >= 1920) and (byr <= 2002)  
    except ValueError:
        return False

def valid_iyr(input):
    try:
        iyr = int(input)
        return (iyr >= 2010) and (iyr <= 2020)
    except ValueError:
        return False

def valid_eyr(input):
    try:
        eyr = int(input)
        return (eyr >= 2020) and (eyr <= 2030)
    except ValueError:
        return False

def valid_hgt(input):
    if (input[-2:] == "cm"):
        try:
            hgt = int(input[:-2])
            return (hgt >= 150) and (hgt <= 193)
        except ValueError:
            return False
    elif (input[-2:] == "in"):
        try:
            hgt = int(input[:-2])
            return (hgt >= 59) and (hgt <= 76)
        except ValueError:
            return False
    else:
        return False

def valid_hex(i):
    try:
        hex = int(i)
        return True
    except ValueError:
        return (i == "a") or (i == "b") or (i == "c") or (i == "d") or (i == "d") or (i == "e") or (i == "f")

def valid_hcl(input):
    if (input[0] != "#") or (len(input) != 7):
        return False
    if (not valid_hex(input[1])) or (not valid_hex(input[2])):
        return False
    if (not valid_hex(input[3])) or (not valid_hex(input[4])):
        return False
    if (not valid_hex(input[5])) or (not valid_hex(input[6])):
        return False
    return True

def valid_ecl(input):
    if input == "amb":
        return True
    if input == "blu":
        return True
    if input == "brn":
        return True
    if input == "gry":
        return True
    if input == "grn":
        return True
    if input == "hzl":
        return True
    if input == "oth":
        return True
    return False

def valid_pid(input):
    if len(input) != 9:
        return False
    try:
        num = int(input)
        return True
    except ValueError:
        return False

def part_two():
    count = 0
    for item in data:
        if (not 'byr' in item) or (not valid_byr(item['byr'])):
            continue
        if (not 'iyr' in item) or (not valid_iyr(item['iyr'])):
            continue
        if (not 'eyr' in item) or (not valid_eyr(item['eyr'])):
            continue
        if (not 'hgt' in item) or (not valid_hgt(item['hgt'])):
            continue
        if (not 'hcl' in item) or (not valid_hcl(item['hcl'])):
            continue
        if (not 'ecl' in item) or (not valid_ecl(item['ecl'])):
            continue
        if (not 'pid' in item) or (not valid_pid(item['pid'])):
            continue
        count += 1
    print("part two answer:", count)

part_one()
part_two()
