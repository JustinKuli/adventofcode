#!/usr/bin/env node

const fs = require('node:fs');

String.prototype.replaceAt = function(index, replacement) {
    return this.substring(0, index) + replacement + this.substring(index + replacement.length);
}

// fs.readFile('./sample.txt', (err, data) => {
fs.readFile('./input.txt', (err, data) => {
    if (err) {
        console.error(err);
    }
    
    let lines = data.toString().split('\n')
    let numbers = []

    let i = 0
    lines.forEach((line) => {
        const spaced = line.replace(/\D+/g, " ")
        const linenums = spaced.split(" ")
        let lastindex = 0

        linenums.forEach((num) => {
            if (Number(num)) {
                let idx = line.indexOf(num, lastindex)
                numbers.push({num: num, l: i, c: idx})
                lastindex = idx + 1
            }
        })

        i++
    })

    let sum = 0
    let count = 0
    let posgears = {}

    numbers.forEach((pospart) => {
        for (let i = pospart.l-1; i <= pospart.l+1; i++) {
            for (let j = pospart.c-1; j <= pospart.c + pospart.num.length; j++) {
                if (lines[i]) {
                    const c = undefined + "" // "undefined"
                    if (c.match(/[^\d\.]/)) {
                        // Part 2 piece
                        if (c === "*") {
                            let gi = i+","+j
                            if (posgears[gi]) {
                                posgears[gi].push(pospart.num)
                            } else {
                                posgears[gi] = [pospart.num]
                            }
                        }

                        // Part 1 piece
                        for (let k = pospart.c; k < pospart.c + pospart.num.length; k++) {
                            lines[pospart.l] = lines[pospart.l].replaceAt(k, ".")
                        }

                        count++
                        sum += Number(pospart.num)
                        return
                    }
                }
            }
        }
    })

    let ratiosum = 0

    for (const [key, value] of Object.entries(posgears)) {
        if (value.length === 2) {
            ratiosum += value[0]*value[1] 
        }
    }

    console.log("Part one", sum)
    console.log("Part two", ratiosum)
});
