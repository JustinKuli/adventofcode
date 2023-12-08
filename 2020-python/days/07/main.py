#!/usr/bin/env python3
import csv
import string

data = []
with open('formatted.txt', 'r') as file:
    reader = csv.reader(file)
    for row in reader:
        data.append(row[0])

# The "trick" here is that we include a space at the beggining of each "bag" in our set.
# That way, we don't catch the line which defines what goes in that bag;
#  we only catch the lines that definte bags which *include* that bag.
def add_all_to_bags(bags):
    oldbags = bags.copy()
    for line in data:
        for bag in oldbags:
            if bag in line:
                words = line.split()
                bags.add(" " + words[0] + " " + words[1])

    # If we found new bags, we must continue, otherwise we're done.
    if len(bags) != len(oldbags):
        return add_all_to_bags(bags)
    return bags

def part_one():
    bags ={" shiny gold"}
    add_all_to_bags(bags)
    # take away one because we don't want to count the original " shiny gold"
    print("part one answer:", len(bags) - 1) 

def contents_of(bag):
    multi = int(bag[0])
    for line in data:
        words = line.split()
        # Unlike part one, this time we only want the line defining what goes in the bag
        if (words[0] == bag[1]) and (words[1] == bag[2]):
            if "contain no other bags." in line:
                return multi
            sum = 0

            # Loop while there are still more bags in our bag
            while (len(words) > 4):
                # The last 4 words define a bag, recurse on it:
                sum += contents_of(words[-4:-1])
                words = words[:-4]

            # Add multi, because we need to count our bags, not just its contents
            return sum * multi + multi

def part_two():
    # Again, take away one because we count *inside* our (1) bag.
    print("part two answer:", contents_of(["1", "shiny", "gold"]) - 1)

part_one()
part_two()
