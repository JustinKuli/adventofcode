#!/usr/bin/env node

const fs = require('node:fs');

// fs.readFile('./sample.txt', (err, data) => {
fs.readFile('./input.txt', (err, data) => {
    if (err) {
        console.error(err);
    }
    
    const lines = data.toString().split('\n')

    let sum = 0
    let duplicity = []
    lines.forEach((line) => {
        if (!line) {
            return
        }
        duplicity.push(1)
    })

    lines.forEach((line, i) => {
        if (!line) {
            return
        }
        const linedata = line.split(": ")[1]
        const winnums = linedata.split(" | ")[0].split(" ")
        const gotnums = linedata.split(" | ")[1].split(" ")

        let wins = []
        winnums.forEach((n) => {
            wins.push(Number(n))
        })

        let matches = 0
        gotnums.forEach((n) => {
            let got = Number(n)
            if (!got) {
                return
            }
            if (wins.includes(got)) {
                // console.log("found", n)
                matches++ 
            }
        })

        if (matches) {
            // part 1
            sum += 2**(matches - 1)

            // part 2
            for (let j = 1; j <= matches; j++) {
                duplicity[i+j] += duplicity[i]
            }
        }

        // console.log("matches", matches)
    })

    console.log("Part One", sum)

    let p2sum = 0
    duplicity.forEach((d) => {
        p2sum += d
    })
    
    console.log("Part Two", p2sum)
});
