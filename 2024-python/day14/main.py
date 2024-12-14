#!/usr/bin/env python3

robots = []
with open('data.txt', 'r') as file:
    for line in file:
        stuff = line[2:-1]
        parts = stuff.split(" v=")
        posStrs = parts[0].split(",")
        velStrs = parts[1].split(",")
        robots.append({
            'px': int(posStrs[0]),
            'py': int(posStrs[1]),
            'vx': int(velStrs[0]),
            'vy': int(velStrs[1])
        })
    
x_size = 101
y_size = 103
    
def move(robot, seconds):
    x = (robot['px'] + (robot['vx'] * seconds)) % x_size
    y = (robot['py'] + (robot['vy'] * seconds)) % y_size
    return (x, y)

def part_one():
    q1 = 0
    q2 = 0
    q3 = 0
    q4 = 0

    for r in robots:
        res = move(r, 100)
        if res[0] < x_size//2:
            if res[1] < y_size//2:
                q1 += 1
            elif res[1] > y_size//2:
                q2 += 1
        elif res[0] > x_size//2:
            if res[1] < y_size//2:
                q3 += 1
            elif res[1] > y_size//2:
                q4 += 1

    print("part one answer:", q1*q2*q3*q4) 

def part_two():
    best = 200*len(robots)
    for seconds in range(10000): #        
        lines = []
        emptyline = [0] * x_size
        for _ in range(y_size):
            lines.append(emptyline.copy())
        
        x_sum = 0
        y_sum = 0
        for r in robots:
            res = move(r, seconds)
            lines[res[1]][res[0]] += 1
            x_sum += res[1]
            y_sum += res[0]
        
        x_avg = x_sum // len(robots)
        y_avg = y_sum // len(robots)

        x_dist = 0
        y_dist = 0

        for r in robots:
            res = move(r, seconds)
            x_dist += abs(res[1] - x_avg)
            y_dist += abs(res[0] - y_avg)
        
        if x_dist + y_dist > best:
            continue

        best = x_dist + y_dist
        print()
        print(seconds, x_dist, y_dist)

        for line in lines:
            toPrint = ""
            for count in line:
                if count == 0:
                    toPrint += " "
                elif count > 9:
                    toPrint += "X"
                else:
                    toPrint += str(count)
            print(toPrint)

part_one()
part_two()
