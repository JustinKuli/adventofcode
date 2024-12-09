#!/usr/bin/env python3

input = ""
with open('data.txt', 'r') as file:
    # there's only one line, but still...
    for line in file:
        input = line[:-1]

def part_one():
    acc = 0

    pos = 0
    left_id = 0
    right_id = (len(input)-1) // 2
    right_size = int(input[right_id*2])

    while left_id < right_id:
        left_size = int(input[left_id*2])
        while left_size > 0:
            acc += pos*left_id
            pos += 1
            left_size -= 1

        empty_size = int(input[left_id*2 + 1])
        while empty_size > 0:
            acc += pos*right_id
            pos += 1
            empty_size -= 1

            right_size -= 1
            while right_size == 0:
                right_id -= 1
                right_size = int(input[right_id*2])
            
            if right_id <= left_id:
                right_size = 0
                break
        
        left_id += 1
        
    while right_size > 0:
        acc += pos*right_id
        pos += 1
        right_size -= 1

    print("part one answer:", acc)

data = []
pos = 0
i = 0

for ch in list(input):
    id = -1 # "free space"
    if i % 2 == 0:
        id = i // 2

    val = int(ch)
    if val != 0: # don't include blocks of 0 free space
        data.append({'id': id, 'size': val, 'pos': pos})
        pos += val
    
    i += 1


def part_two():
    acc = 0
    handled_ids = set()

    # go through backwards, try to move, then accumulate the checksum
    # based on its final position (moved or not)
    while len(data) > 0:
        moving = data.pop()
        if moving['id'] == -1:
            continue
        if moving['id'] in handled_ids:
            continue

        i = -1
        for block in data:
            i += 1
            if block['id'] != -1:
                continue
            if block['size'] < moving['size']:
                continue
        
            # same size: just "convert" the empty block in-place
            if block['size'] == moving['size']:
                block['id'] = moving['id']
                moving = block
                break

            # empty block is bigger than necessary, the empty block
            # must be moved, and the moved block has to be inserted.
            # (looking at this later, maybe actually inserting it
            # wasn't necessary - but it makes things easier for debugging)
            moving['pos'] = block['pos']
            block['size'] -= moving['size']
            block['pos'] += moving['size']
            data.insert(i, moving)
            break
        
        pos = moving['pos']
        size = moving['size']
        while size > 0:
            acc += pos*int(moving['id'])
            size -= 1
            pos += 1
        
        handled_ids.add(moving['id'])

    print("part two answer:", acc)

part_one()
part_two()
