#!/usr/bin/env python3
import csv
import string

timestamp = 0
buses = []
full_line = [] # for part 2
with open('data.txt', 'r') as file:
    reader = csv.reader(file)
    for i, row in enumerate(reader):
        if i == 0:
            timestamp = int(row[0])
            continue
        for item in row:
            full_line.append(item) # for part 2
            if item != "x":
                buses.append(int(item))

def part_one():
    best_wait_time = 999999
    best_bus_id = 0
    for bus_id in buses:
        # Find the first departure of the bus after the timestamp
        departs_at = bus_id
        while departs_at < timestamp:
            departs_at += bus_id

        # Calc. wait time, and (possibly) update current best 
        wait_time = departs_at - timestamp
        if wait_time < best_wait_time:
            best_wait_time = wait_time
            best_bus_id = bus_id

    print("part one answer:", best_bus_id * best_wait_time)

# Unused; kept for explanation.
# This would take a timestamp and verify whether it passes the condition.
def check(i):
    for bus in full_line:
        if bus == "x":
            continue
        if i % int(bus) != 0:
            return False
        # print(bus, "passes internal check")
    return True

def get_index_of(bus_id):
    for i, item in enumerate(full_line):
        if item == str(bus_id):
            return i
    return -1

def part_two():
    # Earliest time that satisfies the first bus_id:
    time = buses[0] + get_index_of(buses[0])

    # increment by the bus_id, so that the first bus will always pass.
    interval = buses[0]

    for i, bus_id in enumerate(buses):
        if i == 0:
            continue

        # Find the timestamp that will satisfy the ith bus
        ind = get_index_of(bus_id)
        while ((time + ind) % bus_id) != 0:
            time += interval

        # Mutliplying the ids ensures that we don't try times where we know
        #  the previous buses won't run at the right time.
        interval *= bus_id

    print("part two answer:", time)

part_one()
part_two()
