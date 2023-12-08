#!/usr/bin/env python3

class Item:
    def __init__(self, inp):
        inp = inp.split(sep=" (contains ")
        self.ingredients = inp[0].split(sep=" ")
        inp[1] = inp[1]
        self.allergens = inp[1].split(sep=", ")
    
    def __repr__(self):
        return "Item(ingredients: {0}, allergens: {1}".format(self.ingredients, self.allergens)

items = []
with open('data.txt', 'r') as file:
    for line in file:
        items.append(Item(line[:-2])) # Strip the ) and newline chars!

def list_intersect(one, two):
    return [i for i in one if i in two]

def part_one():
    single_allergen_items = [i for i in items if len(i.allergens) == 1]

    allergen_map = dict()
    for sai in single_allergen_items:
        allergen = sai.allergens[0]
        matching_allergen_items = [i for i in items if allergen in i.allergens]
        poss_ingredients = set()
        for mai in matching_allergen_items:
            if len(poss_ingredients) == 0:
                poss_ingredients = set(mai.ingredients)
            else:
                poss_ingredients = poss_ingredients.intersection(set(mai.ingredients))
        allergen_map[allergen] = poss_ingredients
    
    # print(allergen_map)

    possible_allergens = set()
    for item in allergen_map:
        for a in allergen_map[item]:
            possible_allergens.add(a)
    
    # print(possible_allergens)

    count = 0
    for ings in [i.ingredients for i in items]:
        for ing in ings:
            if ing not in possible_allergens:
                count += 1

    for _ in range(0,10):
        single_possibilities = [i for i in allergen_map if len(allergen_map[i]) == 1]
        multi_possibilities = [i for i in allergen_map if len(allergen_map[i]) != 1]
        for sp in single_possibilities:
            allergen = allergen_map[sp].pop()
            allergen_map[sp].add(allergen)

            for mp in multi_possibilities:
                allergen_map[mp].discard(allergen)
            
    l = [a for a in allergen_map]
    l.sort()
    for a in l:
        print(allergen_map[a])
    print("part one answer:", count)

def part_two():
    # Do it by hand based on the extra output in part one.
    pass

part_one()
part_two()
