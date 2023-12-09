#!/usr/bin/env node

const fs = require('node:fs');

// fs.readFile('./sample.txt', (err, data) => {
fs.readFile('./input.txt', (err, data) => {
    if (err) {
        console.error(err);
    }
    
    const lines = data.toString().split('\n')

    let extras = []
    let earlies = []

    lines.forEach((line) => {
        let histories = [[]]
        line.split(" ").forEach((n) => {
            if (!Number(n) && Number(n) !== 0) {
                console.log("Unable to convert to number", n, "?")
                process.exit(1)
            }

            histories[0].push(Number(n))
        })

        let depth = 0
        while (true) {
            let nexthist = []
            for (let i = 1; i < histories[depth].length; i++) {
                nexthist.push(histories[depth][i] - histories[depth][i-1])
            }

            histories.push(nexthist)
            depth++

            let allsame = true
            nexthist.forEach((h) => {
                if (h !== nexthist[0]) {
                    allsame = false
                }
            })

            if (allsame) {
                break
            }
        }

        while (depth > 0) {
            // part 1
            let diff = histories[depth].pop()
            let last = histories[depth-1].pop()
            histories[depth-1].push(last+diff)

            // part 2
            let prevdiff = histories[depth].shift()
            let first = histories[depth-1].shift()
            histories[depth-1].unshift(first-prevdiff)

            depth--
        }

        extras.push(histories[0].pop())
        earlies.push(histories[0].shift())
    })

    let sum = 0
    extras.forEach((e) => {
        sum += e
    })

    console.log("Part One:", sum)

    let sum2 = 0
    earlies.forEach((e) => {
        sum2 += e
    })

    console.log("Part 2:", sum2)
});
