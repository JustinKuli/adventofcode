#!/usr/bin/env python3

import sys

data = [9,6,0,10,18,2,1]

# Old version (saved for reference):
# def find_back(find):
#     global data
#     for i, d in enumerate(reversed(data[:-1])):
#         if d == find:
#             return len(data) - 1 - i

# def part_one():
#     global data
#     turn = 0
#     for d in data:
#         turn += 1
#         # print(turn, d)

#     last_num = data[-1]
#     while len(data) < 2020:
#         if data.count(last_num) == 1:
#             last_num = 0
#         else:
#             last_num = turn - find_back(last_num)
        
#         turn += 1
#         # print(turn, last_num)
#         data.append(last_num)

#     print("part one answer:", data[-1])

index = dict() # { 0: [10, 20], 3: [1,25], } 
def index_it(num, ind):
    global index
    if num in index:
        index[num][0] = index[num][1]
        index[num][1] = ind
    else:
        index[num] = [ind, ind]

def get_number(final_turn):
    global index
    turn = 1
    last_num = 0
    for d in data:
        last_num = d
        index_it(last_num, turn)
        turn += 1

    while turn < final_turn:
        last_num = index[last_num][1] - index[last_num][0]
        index_it(last_num, turn)

        # it will be nice to some some of the progress if it's going for a long time
        if turn % 1000000 == 0:
            print(int(turn/1000000), end ="..") # Don't send a newline.
            sys.stdout.flush() # Otherwise python might wait for a newline to send it to the terminal

        turn += 1
    
    return index[last_num][1] - index[last_num][0]

print("part one answer:", get_number(2020))

index = dict() # reset it before beginning
ans = get_number(30000000)
print() # add the newline after the "progress bar"
print("part two answer:", ans)
