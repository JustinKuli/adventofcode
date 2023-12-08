#!/usr/bin/env node

const fs = require('node:fs');

// part 1
function rankhand1(cards) {
    let counts = {}
    Array.from(cards).forEach((c) => {
        if (counts[c]) {
            counts[c]++
        } else {
            counts[c] = 1
        }
    })

    const countlen = Object.keys(counts).length
    if (countlen === 1) {
        return 7 // Five of a kind
    } else if (countlen === 2) {
        let bigcount = 0
        for (const [key, value] of Object.entries(counts)) {
            if (value > bigcount) {
                bigcount = value
            }
        }

        if (bigcount === 4) {
            return 6 // Four of a kind
        } else {
            return 5 // Full house
        }
    } else if (countlen === 3) {
        // three of a kind OR two pair
        let bigcount = 0
        for (const [key, value] of Object.entries(counts)) {
            if (value > bigcount) {
                bigcount = value
            }
        }

        if (bigcount === 3) {
            return 4 // Three of a kind
        } else {
            return 3 // Two pair
        }
    } else if (countlen === 4) {
        return 2 // One pair
    } else {
        return 1 // High card
    }
}

// part 2
function rankhand2(cards) {
    let counts = {}
    Array.from(cards).forEach((c) => {
        if (counts[c]) {
            counts[c]++
        } else {
            counts[c] = 1
        }
    })

    const jokers = counts["J"] || 0

    if (jokers === 0) {
        return rankhand1(cards)
    }

    // console.log("With jokers", counts, cards)

    const countlen = Object.keys(counts).length
    if (countlen === 1) {
        return 7 // Five of a kind
    } else if (countlen === 2) {
        return 7 // Kinda cool
    } else if (countlen === 3) {
        // (usually) three of a kind OR two pair
        if (jokers === 3) {
            return 6 // 3 jokers plus one other, four of a kind
        }

        if (jokers === 2) {
            return 6 // 2 jokers matching the other pair
        }

        // one joker + three of a kind OR two pair
        let bigcount = 0
        for (const [key, value] of Object.entries(counts)) {
            if (value > bigcount) {
                bigcount = value
            }
        }

        if (bigcount === 3) {
            return 6 // three of a kind, plus 1 joker makes a FOUR OF A KIND
        } else {
            return 5 // 2 pair, plus 1 joker matching one of them, makes a FULL HOUSE
        }
    } else if (countlen === 4) {
        return 4 // one pair, plus 1 joker matching it, makes a THREE OF A KIND
    } else {
        return 2 // High card, plus 1 joker matching it makes a PAIR
    }
}

// fs.readFile('./sample.txt', (err, data) => {
fs.readFile('./input.txt', (err, data) => {
    if (err) {
        console.error(err);
    }
    
    const lines = data.toString().split('\n')

    const cardpower = {
        "J": 1, // part 2
        "2": 2,
        "3": 3,
        "4": 4,
        "5": 5,
        "6": 6,
        "7": 7,
        "8": 8,
        "9": 9,
        "T": 10,
        // "J": 11, // part 1
        "Q": 12,
        "K": 13,
        "A": 14,
    }

    let hands = []

    lines.forEach((line) => {
        const parts = line.split(" ")

        hands.push({
            cards: parts[0],
            bid: Number(parts[1]),
            // rank: rankhand1(parts[0]) // part 1
            rank: rankhand2(parts[0]) // part 2
        })
    })

    hands.sort((h1, h2) => { // should return a negative if h1 is "less than" h2
        const rankdiff = h1.rank - h2.rank
        if (rankdiff !== 0) {
            return rankdiff
        }

        for (let i = 0; i < 5; i++) {
            const carddiff = cardpower[h1.cards.slice(i, i+1)] - cardpower[h2.cards.slice(i, i+1)]
            if (carddiff !== 0) {
                return carddiff
            }
        }

        console.log("a tie? that can't be good...", h1.cards, h2.cards)
    })

    console.log(hands)

    let winnings = 0
    for (let i = 0; i < hands.length; i++) {
        winnings += hands[i].bid * (i+1)
    }

    console.log(winnings)
});
