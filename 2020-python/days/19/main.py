#!/usr/bin/env python3
import sys

# NOTE! Part 1 and 2 use different data inputs!

data = []
with open('data.txt', 'r') as file:
    for line in file:
        data.append(line[:-1]) # Strip the newline char!

data_rules = []
with open('rules.txt', 'r') as file:
    for line in file:
        data_rules.append(line[:-1])

extra_index = -1
rules = dict()
class Rule:
    def __init__(self, rule):
        global rules
        global extra_index

        rule = rule.strip()
        if '"' in rule:
            # Just a simple character match
            self.kind = "char"
            self.char = rule.split(sep='"')[1]

        elif "|" in rule:
            # Needs to match one "or" the other
            self.kind = "or"
            rules[extra_index] = Rule(rule.split('|')[0])
            rules[extra_index-1] = Rule(rule.split('|')[1])
            self.subrules = (extra_index, extra_index-1)
            extra_index -= 2

        else:
            # Needs to match all of them in sequence.
            # Note that a single ref will also fall into this category.
            self.kind = "and"
            self.subrules = [int(r) for r in rule.split()]

    def __repr__(self):
        if self.kind == "char":
            return "Char_Rule({0})".format(self.char)
        if self.kind == "or":
            return "Or_Rule({0})".format(self.subrules)
        if self.kind == "and":
            return "And_Rule({0})".format(self.subrules)
        return "Invalid_Rule(no 'kind' set)"

    # Returns the input after the part matching the rule is removed.
    # If the input does not match the rule, 'c' is returned as a sentinel value.
    # If the rule (or a subrule) is of kind 'or', then this will return a list of
    #  possible strings that could be left after matching pieces are removed.
    # If the input is a list, it will handle each individually, and combine the lists.
    def remove_match(self, inp):
        if isinstance(inp, list):
            rms = []
            for inp_str in inp:
                rm = self.remove_match(inp_str)
                if rm == "c":
                    continue # if it doesn't match, don't even add it to the list.
                if isinstance(rm, list):
                    rms.extend(rm)
                rms.append(rm)

            if len(rms) == 0:
                return "c"
            return rms

        if len(inp) == 0 or inp == "c":
            return "c"
        
        if self.kind == "char":
            if inp[0] == self.char:
                return inp[1:]
            return "c"
        if self.kind == "and":
            for r in self.subrules:
                inp = rules[r].remove_match(inp)
            return inp
        if self.kind == "or":
            rm0 = rules[self.subrules[0]].remove_match(inp)
            rm1 = rules[self.subrules[1]].remove_match(inp)
            if rm0 == "c":
                return rm1
            if rm1 == "c":
                return rm0
            return [rm0, rm1]

def get_rules():
    global rules
    for r in data_rules:
        piece = r.split(sep=":")
        rules[int(piece[0])] = Rule(piece[1])

def part_one():
    get_rules()

    count = 0
    for d in data:
        rm = rules[0].remove_match(d)
        if isinstance(rm, list) and "" in rm:
            count += 1
        if isinstance(rm, str) and len(rm) == 0:
            count += 1
    print("answer:", count)

part_one()
