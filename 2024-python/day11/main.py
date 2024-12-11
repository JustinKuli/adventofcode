#!/usr/bin/env python3

data = []
with open('data.txt', 'r') as file:
    # there's only one line, but still...
    for line in file:
        data = line[:-1].split(" ")

memos1 = {}
def iterate(rock):
    if rock == "0":
        return ["1"]
    
    if rock in memos1:
        return memos1[rock]

    l = len(rock)
    if l % 2 == 0:
        half = l // 2
        ans = [rock[0:half], str(int(rock[half:]))]
        if l < 5:
            memos1[rock] = ans
        return ans
    
    # I had a tricky coder error here for a while!
    # I forgot to put the brackets around this 😱
    ans = [str(int(rock)*2024)]
    if l < 5:
        memos1[rock] = ans
    return ans

def parts():
    rocks = {}
    for d in data:
        if d in rocks:
            rocks[d] += 1
        else:
            rocks[d] = 1

    for _ in range(25):
        next = {}
        for r in rocks:
            for n in iterate(r):
                if n in next:
                    next[n] += rocks[r]
                else:
                    next[n] = rocks[r]
        rocks = next
    
    acc = 0
    for r in rocks:
        acc += rocks[r]
    
    print("part one answer:", acc)

    for _ in range(50):
        next = {}
        for r in rocks:
            for n in iterate(r):
                if n in next:
                    next[n] += rocks[r]
                else:
                    next[n] = rocks[r]
        rocks = next
    
    acc = 0
    for r in rocks:
        acc += rocks[r]
    
    print("part two answer:", acc)

parts()

