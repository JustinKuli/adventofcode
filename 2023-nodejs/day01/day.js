#!/usr/bin/env node

const fs = require('node:fs');

// fs.readFile('./sample.txt', (err, data) => {
fs.readFile('./input.txt', (err, data) => {
    if (err) {
        console.error(err);
    }
    
    const lines = data.toString().split('\n')

    // There's probably a much more clever way to do this kind of thing,
    // but this actually works out pretty well.
    // This was added for Part 2.
    const digits = {
        "1": 1,
        "2": 2,
        "3": 3,
        "4": 4,
        "5": 5,
        "6": 6,
        "7": 7,
        "8": 8,
        "9": 9,
        "one": 1,
        "two": 2,
        "three": 3,
        "four": 4,
        "five": 5,
        "six": 6,
        "seven": 7,
        "eight": 8,
        "nine": 9
    }

    let sum = 0

    lines.forEach((line) => {
        let first = NaN
        let last = NaN
        for (let i = 0; i < line.length; i++) {
            // Part 1 just checked if each character was a number, something like
            // `if (Number(line[i]))` ...
            // but it had the same `if (!(first))` thing as below

            for (const [key, value] of Object.entries(digits)) {
                if (line.slice(i, i+key.length) === key) {
                    if (!(first)) {
                        first = value
                    }

                    last = value
                }
            }
        }

        if (first) {
            // console.log(Number(first + "" + last))
            sum += Number(first + "" + last)
        }
    })

    console.log(sum)
});
