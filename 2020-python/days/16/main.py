#!/usr/bin/env python3

data = []
with open('data.txt', 'r') as file:
    for line in file:
        data.append(line[:-1]) # Strip the newline char!

your_ticket = [79,193,53,97,137,179,131,73,191,139,197,181,67,71,211,199,167,61,59,127]

def get_ranges():
    ranges = []
    for line in data:
        for word in line.split(sep=" "):
            if "-" in word:
                nums = word.split(sep="-")
                ranges.append([int(nums[0]), int(nums[1])])
    return ranges

def could_be_valid(num, ranges):
    for r in ranges:
        if num >= r[0] and num <= r[1]:
            return True        
    return False

def part_one():
    invalid_sum = 0
    ranges = get_ranges()
    with open('nearby-tickets.txt', 'r') as file:
        for line in file:
            line = line[:-1] # strip the newline
            for num in line.split(sep=","):
                if not could_be_valid(int(num), ranges):
                    invalid_sum += int(num)

    print("part one answer:", invalid_sum) 

def get_valid_tickets():
    tickets = []
    ranges = get_ranges()
    with open('nearby-tickets.txt', 'r') as file:
        for line in file:
            line = line[:-1] # strip the newline
            valid = True
            for num in line.split(sep=","):
                if not could_be_valid(int(num), ranges):
                    valid = False
                    break
            if valid:
                tickets.append(line.split(sep=","))
    return tickets

def get_labelled_ranges():
    ranges = dict()
    for line in data:
        label = line.split(sep=":")[0]
        r = []
        for word in line.split(sep=" "):
            if "-" in word:
                nums = word.split(sep="-")
                r.append([int(nums[0]), int(nums[1])])
        ranges[label] = r.copy()
    return ranges

def get_potential_labels(num, l_ranges):
    labels = set()
    for label, ranges in l_ranges.items():
        for r in ranges:
            if num >= r[0] and num <= r[1]:
                labels.add(label)
                break
    return labels

def part_two():
    tickets = get_valid_tickets()
    l_ranges = get_labelled_ranges()

    curr_labels = []
    for i, t0 in enumerate(tickets[0]):
        pot_labels = get_potential_labels(int(t0), l_ranges)
        for t in tickets[1:]:
            if len(pot_labels) == 1:
                break
            pot_labels = pot_labels.intersection(get_potential_labels(int(t[i]), l_ranges))
        curr_labels.append(pot_labels)

    while [len(x) for x in curr_labels] != [1 for _ in curr_labels]:
        for i, item in enumerate(curr_labels):
            if len(item) == 1:
                for jtem in curr_labels[:i] + curr_labels[i+1:]:
                    jtem.discard(next(iter(item)))
    
    departure_indexes = [i for i, x in enumerate(curr_labels) if "departure" in next(iter(x))]
    your_departures = [x for i, x in enumerate(your_ticket) if i in departure_indexes]
    
    prod = 1
    for d in your_departures:
        prod *= d

    print("part two answer:", prod)

part_one()
part_two()
