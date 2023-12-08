#!/usr/bin/env python3

import sys

player1 = []
with open('player1.txt', 'r') as file:
    for line in file:
        player1.append(int(line[:-1])) # Strip the newline char!
player1.reverse()

player2 = []
with open('player2.txt', 'r') as file:
    for line in file:
        player2.append(int(line[:-1])) # Strip the newline char!
player2.reverse()

def play_hand():
    global player1
    global player2
    p1_card = player1.pop()
    p2_card = player2.pop()
    
    p1_save = player1
    p2_save = player2

    if p1_card > p2_card:
        player1 = [p2_card, p1_card]
        player1.extend(p1_save)
    else:
        player2 = [p1_card, p2_card]
        player2.extend(p2_save)

def score_hand(hand):
    score = 0
    for i, card in enumerate(hand):
        score += (i+1) * card
    return score

def part_one():
    global player1
    global player2
    while len(player1) != 0 and len(player2) != 0:
        play_hand()

    print("part one answer:", score_hand(player1), score_hand(player2))

def stringify_hands(hand1, hand2):
    ans = "("
    for c in hand1:
        ans += str(c) + ","
    ans += "|"
    for c in hand2:
        ans += str(c) + ","
    ans += ")"
    return ans

def copy_list(l, length=-1):
    ans = []
    for x in l[::-1]:
        if len(ans) == length:
            ans.reverse()
            return ans
        ans.append(x)
    ans.reverse()
    return ans 

def play_recursive_combat(hand1, hand2, history):
    string_hand = stringify_hands(hand1, hand2)
    # print(string_hand)
    if string_hand in history:
        # print("Hand was in history. Player 1 wins!")
        return hand1, [], history
    
    history.append(string_hand)

    p1_card = hand1.pop()
    p2_card = hand2.pop()

    p1_save = hand1
    p2_save = hand2

    # print(p1_card, hand1, p2_card, hand2)

    if p1_card > len(hand1) or p2_card > len(hand2):
        # print("Simple hand")
        if p1_card > p2_card:
            # print("Player 1 wins")
            hand1 = [p2_card, p1_card]
            hand1.extend(p1_save)
            return hand1, hand2, history
        else:
            # print("Player 2 wins")
            hand2 = [p1_card, p2_card]
            hand2.extend(p2_save)
            return hand1, hand2, history
    
    # print("Recursive hand")
    res1, res2 = recursive_game(copy_list(hand1, p1_card), copy_list(hand2, p2_card))
    if len(res1) > len(res2):
        # print("Player 1 wins")
        hand1 = [p2_card, p1_card]
        hand1.extend(p1_save)
        return hand1, hand2, history
    else:
        # print("Player 2 wins")
        hand2 = [p1_card, p2_card]
        hand2.extend(p2_save)
        return hand1, hand2, history

def recursive_game(hand1, hand2):
    hist = []
    while len(hand1) != 0 and len(hand2) != 0:
        hand1, hand2, hist = play_recursive_combat(hand1, hand2, hist)
    return hand1, hand2

def part_two():
    p1 = copy_list(player1)
    p2 = copy_list(player2)
    h1, h2 = recursive_game(p1, p2)
    print("part two answer:", score_hand(h1), score_hand(h2))

part_one()

player1 = []
with open('player1.txt', 'r') as file:
    for line in file:
        player1.append(int(line[:-1])) # Strip the newline char!
player1.reverse()

player2 = []
with open('player2.txt', 'r') as file:
    for line in file:
        player2.append(int(line[:-1])) # Strip the newline char!
player2.reverse()

part_two()
