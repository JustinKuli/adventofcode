#!/usr/bin/env python3

puzzle_input = "739862541"
cups = []
for c in puzzle_input:
    cups.append(int(c))

def do_move(curr):
    global cups
    curr_index = cups.index(curr)
    # print("curr", curr, "index", curr_index)

    # remove 3 cups
    removed = []
    for i in [1,2,3]:
        r = cups[(curr_index + i) % len(cups)] 
        removed.append(r)
    for r in removed:
        cups.remove(r)
    # print("removed", removed)

    # select destination cup
    dest = curr - 1
    while dest not in cups:
        dest -= 1
        if dest < 1:
            dest = max(cups)
    dest_index = cups.index(dest)
    # print("dest", dest, "index", dest_index)
    
    # place removed cups
    new_cups = cups[:dest_index+1]
    new_cups.extend(removed)
    new_cups.extend(cups[dest_index+1:])
    cups = new_cups
    # print("new cups", cups)

    # return next cup
    curr_index = cups.index(curr)
    return cups[(curr_index+1) % len(cups)]

def part_one():
    curr = int(puzzle_input[0])
    for _ in range(100):
        curr = do_move(curr)
    print(cups)
part_one()

class ll_node:
    def __init__(self, data):
        self.data = data
        self.next = None

puzzle_input = "739862541"
ll_cups = ll_node(int(puzzle_input[0]))
ll_dict = dict() # Thanks @WinstonPHz
ll_curr = ll_cups
ll_dict[ll_curr.data] = ll_curr
for c in puzzle_input[1:]:
    ll_curr.next = ll_node(int(c))
    ll_curr = ll_curr.next
    ll_dict[ll_curr.data] = ll_curr

# MAX_VAL = 9

c = 10
MAX_VAL = 1000000
while c <= MAX_VAL:
    ll_curr.next = ll_node(c)
    ll_curr = ll_curr.next
    ll_dict[ll_curr.data] = ll_curr
    c += 1

ll_curr.next = ll_cups # Make it circular
ll_curr = ll_curr.next

def do_move_ll():
    global ll_cups
    global ll_curr

    # remove 3 cups
    removed_arr = []
    removed = ll_curr.next
    temp = removed
    for _ in [1,2,3]:
       removed_arr.append(temp.data)
       temp = temp.next
    ll_curr.next = temp

    # select destination cup
    dest_val = ll_curr.data - 1
    if dest_val < 1:
        dest_val = MAX_VAL
    while dest_val in removed_arr:
        dest_val -= 1
        if dest_val < 1:
            dest_val = MAX_VAL
    dest = ll_dict[dest_val]
    
    # place removed cups
    temp = dest.next
    dest.next = removed
    removed.next.next.next = temp

    # choose next cup
    ll_curr = ll_curr.next

def part_two():
    for i in range(10000000):
        if i % 1000000 == 0:
            print("========== Making progress ==========")
        do_move_ll()
    one_cup = ll_dict[1]
    p1 = one_cup.next.data
    p2 = one_cup.next.next.data
    print("Part two answer:", p1*p2)

part_two()
