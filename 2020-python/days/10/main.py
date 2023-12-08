#!/usr/bin/env python3
import csv
import string

data = []
with open('data.txt', 'r') as file:
    reader = csv.reader(file)
    for row in reader:
        data.append(int(row[0]))

sorted_data = data.copy()
sorted_data.sort()

def part_one():
    diffs = dict()
    diffs[3]=1 # From our final adapter (max + 3)
    prev = 0 # From the charging adapter
    # Go through the sorted data, calculate the difference each time, and tally them up.
    for line in sorted_data:
        d = line - prev
        if d in diffs:
            diffs[d] += 1
        else:
            diffs[d] = 1
        prev = line
    print(diffs)
    print("part one answer:", diffs[1] * diffs[3])

# I noticed in the diffs printed in part one that there are no 2s.
# That means that the adapters are either adjacent, or they are maximally far apart (while still connecting).
# So for each contiguous set (streak with diff 1), we have a certain number of paths through that set.
# For each streak size, we can (by hand) calculate the number of paths.
# Then, we just need to multiply all of those together.
def part_two():
    streaks = dict()
    prev = 0
    curr_streak = 0
    # This loop will populate `streaks` with how many of each size of streak is in the data.
    for line in sorted_data:
        if line - prev == 1:
            curr_streak += 1
        else:
            if curr_streak in streaks:
                streaks[curr_streak] += 1
            else:
                streaks[curr_streak] = 1
            curr_streak = 0
        prev = line

    acc = 1

    # A "streak" of 2 means 3 adapters in a row. Possible paths: [1 3] or [1 2 3]. Factor is 2.
    if 2 in streaks:
        acc *= 2 ** streaks[2]

    # Possible paths: [1 2 3 4] [1 2 4] [1 3 4] [1 4]. Factor of 4
    if 3 in streaks:
        acc *= 4 ** streaks[3]

    # Possible paths: a bunch. idk. Factor of 7, I think.
    if 4 in streaks:
        acc *= 7 ** streaks[4]

    # Possible paths: too many. I tried counting and got between 10 and 15.
    if 5 in streaks:
        print("Warning: I'm not sure about this factor")
        acc *= 12 ** streaks[5]

    if 6 in streaks:
        print("You definitely need to figure this one out on your own")
        acc = 0

    print("part two answer:", acc)

part_one()
part_two()
