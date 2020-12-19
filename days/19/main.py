#!/usr/bin/env python3
import sys

# NOTE! Part 2 uses a different data input.

data = []
with open('data.txt', 'r') as file:
    for line in file:
        data.append(line[:-1]) # Strip the newline char!

data_rules = []
with open('rules.txt', 'r') as file:
    for line in file:
        data_rules.append(line[:-1])

class Rule:
    def __init__(self, kind, data):
        self.kind = kind
        self.data = data
    def __repr__(self):
        return "<kind: '{0}', data: '{1}'>".format(self.kind, self.data)

custom_rule_ctr = -1
def build_rule(r):
    global custom_rule_ctr
    global rules
    r = r.strip()
    if '"' in r:
        # Simple rule of just a letter
        return Rule('simple', r.split(sep='"')[1])
    elif '|' in r:
        # Must satisfy at least one of two other rules
        rules[custom_rule_ctr] = build_rule(r.split('|')[0])
        custom_rule_ctr -= 1
        rules[custom_rule_ctr] = build_rule(r.split('|')[1])
        custom_rule_ctr -= 1
        return Rule('or', (custom_rule_ctr+2, custom_rule_ctr+1))
    elif len(r.split()) > 1:
        return Rule('and', [int(x) for x in r.split()])
    elif len(r.split()) == 1:
        return Rule('ref', int(r))
    else:
        print("Reached unreachable case in build_rule", r)
        sys.exit(1)

rules = dict()
def get_rules():
    global rules
    for r in data_rules:
        rule_n = int(r.split(sep=":")[0])
        rule = build_rule(r.split(sep=":")[1])
        rules[rule_n] = rule        

# returns the portion of the string after the part matching the rule is removed.
# if the string does not match the rule, 'c' is returned, as a sentinel value.
def matches_rule(inp, rules, i):
    if isinstance(inp, list):
        # Now we're really cooking with fire.
        matches = []
        for input in inp:
            m = matches_rule(input, rules, i)
            if m == "c":
                continue
            if isinstance(m, list):
                matches.extend(m)
            else:
                matches.append(m)
        if len(matches) == 0:
            return "c"
        return matches

    if len(inp) == 0:
        return "c"
    if inp == "c":
        return "c"

    rule = rules[i]
    if rule.kind == "simple":
        if inp[0] == rule.data:
            return inp[1:]
        return "c"
    if rule.kind == "and":
        for r in rule.data:
            if inp == "c":
                return "c"
            if len(inp) == 0:
                return "c"
            inp = matches_rule(inp, rules, r)
        return inp
    if rule.kind == "or":
        sub1 = matches_rule(inp, rules, rule.data[0])
        if sub1 == "c":
            return matches_rule(inp, rules, rule.data[1])
        
        sub2 = matches_rule(inp, rules, rule.data[1])
        if sub2 == "c":
            return sub1

        return [sub1, sub2]
    if rule.kind == "ref":
        return matches_rule(inp, rules, rule.data)
    print("Reached unreachable case in matches_rule", inp, i)
    sys.exit(1)

def part_one():
    get_rules()

    count = 0
    for d in data:
        m = matches_rule(d, rules, 0)
        if isinstance(m, list) and "" in m:
            count += 1
        if isinstance(m, str) and len(m) == 0:
            count += 1
    print("answer:", count)

part_one()
