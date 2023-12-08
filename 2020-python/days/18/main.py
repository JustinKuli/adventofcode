#!/usr/bin/env python3

import sys

# Reverses the string given to it, 
# and takes care of the parentheses so that it is still valid.
def reverse_expression(line):
    l = line.replace("(", "#") # use '#' as a placeholder.
    l = l.replace(")", "(")
    l = l.replace("#", ")")
    return l[::-1]

data = []
with open('data.txt', 'r') as file:
    for line in file:
        # My first attempt ended up working from right to left.
        # ... that's not a problem, we'll just reverse the expression!
        data.append(reverse_expression(line[:-1]))

# Returns the given string up to the first unmatched right parenthesis.
# Example: given '1 + (2 * 3)) + 4'
#        returns '1 + (2 * 3)'
def to_match_paren(line):
    depth = 1
    for i, char in enumerate(line):
        if depth == 0:
            return line[:i]
        elif char == "(":
            depth += 1
        elif char == ")":
            depth -= 1
    if depth != 0:
        print("Exiting to_match_paren, but we didn't match the paren ...", line)
        sys.exit(1)
    return line[:-1]

# Evaluates the expression right-to-left (~bug?~ *feature*),
# honoring parentheses and disregarding other order of operations conventions.
def calc(line):
    ans = 0
    if line[0] == " ":
        line = line[1:]
    if line[0] == "(":
        substr = to_match_paren(line[1:])
        ans = calc(substr)
        line = line[len(substr)+2:]
    elif line[0].isdigit():
        ans = int(line[0])
        line = line[2:]
    else:
        print("Unexpected character (operator?) found in calc", line)
        sys.exit(1)
    
    if len(line) == 0:
        return ans
    elif line[0] == "*":
        return ans * calc(line[1:])
    elif line[0] == "+":
        return ans + calc(line[1:])
    else:
        print("Unexpected character (not an operator) found in calc", line)
        sys.exit(1)

def part_one():
    acc = 0
    for item in data:
        acc += calc(item)
    print("part one answer:", acc) 

# Modifies the input expression to add parentheses around all additions.
# Honors existing parentheses.
# Example input: '1 * 2 + (3 * 4) * 6'
#      returns: '1 * (2 + (3 * 4)) * 6'
def adv(line):
    # Do each '+' individually, we'll mark it done with a placeholder '$'
    while "+" in line:
        pos = line.find("+")

        # look left
        if line[pos-2].isdigit():
            put_at = pos-2
        else:
            # () group: we need to find how big it is to correctly place the new one.
            substr = to_match_paren(reverse_expression(line[:pos-2])) # reverse because we're looking left
            put_at = pos-2-len(substr)
        l = line[:put_at] + "(" + line[put_at:pos] + "$"

        # look right
        if line[pos+2].isdigit():
            put_at = pos+3
        else:
            substr = to_match_paren(line[pos+3:])
            put_at = pos+3+len(substr)
        l += line[pos+1:put_at] + ")" + line[put_at:]
        line = l

    return line.replace("$","+") # undo the placeholders.

def part_two():
    acc = 0
    for item in data:
        acc += calc(adv(item))
    print("part two answer:", acc)

part_one()
part_two()
