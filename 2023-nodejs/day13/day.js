#!/usr/bin/env node

const fs = require('node:fs');

// fs.readFile('./sample.txt', (err, data) => {
fs.readFile('./input.txt', (err, data) => {
    if (err) {
        console.error(err);
    }
    
    const lines = data.toString().split('\n')

    let pattern = []
    let sum1 = 0
    let sum2 = 0

    lines.forEach((line, i) => {
        if (line.length === 0) {
            let found = false

            for (let i=0; i+1 < pattern.length; i++) {
                if (reflectsAcrossHorizontal(pattern, i, 0)) {
                    // console.log("rows above:", i+1)
                    sum1 += (i+1)*100
                }

                if (reflectsAcrossHorizontal(pattern, i, 1)) {
                    sum2 += (i+1)*100
                    found = true
                }
            }

            let transposed = transpose(pattern)

            for (let i=0; i+1 < transposed.length; i++) {
                if (reflectsAcrossHorizontal(transposed, i, 0)) {
                    sum1 += (i+1)
                }

                if (reflectsAcrossHorizontal(transposed, i, 1)) {
                    sum2 += (i+1)
                    found = true
                }
            }

            if (!found) {
                console.log("No reflection found", i)
            }

            pattern = []
        } else {
            pattern.push(line)
        }
    })

    console.log("Part One", sum1)
    console.log("Part Two", sum2)
});

function reflectsAcrossHorizontal(pattern, above, smudge) {
    let smudgesFound = 0
    for (let i = 0; above-i>=0 && above+1+i<pattern.length; i++) {
        for (let j = 0; j < pattern[0].length; j++) {
            if (pattern[above-i][j] !== pattern[above+1+i][j]) {
                smudgesFound++
                if (smudgesFound > smudge) {
                    return false
                }    
            }
        }
    }
    return (smudgesFound === smudge)
}

function transpose(pattern) {
    let ans = []
    for (let i=0; i<pattern[0].length; i++) {
        let tline = ""
        pattern.forEach((line) => {
            tline += line[i]
        })
        ans.push(tline)
    }
    return ans
}
