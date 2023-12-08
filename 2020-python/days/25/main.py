#!/usr/bin/env python3

import base64

pub_key1 = 11404017
pub_key2 = 13768789

def transform(val, subj):
    val *= subj
    val %= 20201227
    return val

def find_loop_size(result):
    i = 0
    val = 1
    while val != result:           
        i += 1
        val = transform(val, 7)
    return i

def part_one():
    ls = find_loop_size(pub_key1)
    val = 1
    for i in range(ls):
        val = transform(val, pub_key2)
    print("part one answer:", val) 

def part_two():
    enc = b'SGFwcHkgQ2hyaXN0bWFzIHRvIGFsbCwgYW5kIHRvIGFsbCBhIGdvb2QgbmlnaHQh'
    print("part two answer:", base64.b64decode(enc).decode('ascii'))

part_one()
part_two()
