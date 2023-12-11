#!/usr/bin/env node

const fs = require('node:fs');

// fs.readFile('./sample.txt', (err, data) => {
fs.readFile('./input.txt', (err, data) => {
    if (err) {
        console.error(err);
    }
    
    const lines = data.toString().split('\n')

    let starmapcols = []
    let p2map = []
    let xlines = []

    let first = true
    lines.forEach((line, linenum) => {
        // for part 2, this implementation assumes the top row always has a galaxy
        line.split("").forEach((c, i) => {
            if (first) {
                starmapcols.push([c])
                p2map.push([c])
            } else {
                starmapcols[i].push(c)
                p2map[i].push(c)
            }
        })

        // deal with "expanding" rows
        if (!line.includes("#")) {
            line.split("").forEach((c,i) => {
                starmapcols[i].push(c)
            })
            xlines.push(linenum)
        }

        first = false
    })

    let newstarmapcols = []
    let xcols = []

    // deal with "expanding" columns
    starmapcols.forEach((col, colnum) => {
        newstarmapcols.push(col)

        if (!col.includes("#")) {
            newstarmapcols.push(col)
            xcols.push(colnum)
        }
    })

    let galaxies = []
    newstarmapcols.forEach((col, i) => {
        col.forEach((pt, j) => {
            if (pt === "#") {
                galaxies.push({col: i, row: j})
            }
        })
    })

    let lengthsum = 0

    galaxies.forEach((g1, x) => {
        for (let y = x+1; y < galaxies.length; y++) {
            let g2 = galaxies[y]

            let col_diff = g1.col - g2.col
            if (col_diff < 0) {
                col_diff *= -1
            }

            let row_diff = g1.row - g2.row
            if (row_diff < 0) {
                row_diff *= -1
            }

            lengthsum += col_diff + row_diff // taxicab distance is easy!
        }
    })

    console.log("Part One", lengthsum)

    let p2galaxies = []
    p2map.forEach((col, i) => {
        col.forEach((pt, j) => {
            if (pt === "#") {
                p2galaxies.push({col: i, row: j})
            }
        })
    })

    let p2smallsum = 0
    let p2bigsum = 0

    p2galaxies.forEach((g1, x) => {
        for (let y = x+1; y < p2galaxies.length; y++) {
            let g2 = p2galaxies[y]

            let col1 = g1.col
            let col2 = g2.col
            if (col1 > col2) {
                col1 = g2.col
                col2 = g1.col
            }
            for (let d = col1; d <= col2; d++) {
                if (xcols.includes(d)) {
                    p2bigsum++
                } else {
                    p2smallsum++
                }
            }

            let row1 = g1.row
            let row2 = g2.row
            if (row1 > row2) {
                row1 = g2.row
                row2 = g1.row
            }
            for (let d = row1; d <= row2; d++) {
                if (xlines.includes(d)) {
                    p2bigsum++
                } else {
                    p2smallsum++
                }
            }
        }
    })

    let initialp2 = p2bigsum+p2smallsum
    let correction = lengthsum - initialp2 // I don't know why this is needed

    const xfactor = 1000000 - 1

    let finalp2 = p2bigsum*xfactor+p2smallsum + correction

    console.log("Part Two", finalp2)
});

function printem(mapcols) {
    for (let i = 0; i < mapcols[0].length; i++) {
        let str = ""
        for (let j = 0; j < mapcols.length; j++) {
            str += mapcols[j][i]
        }
        console.log(str)
    }
}
