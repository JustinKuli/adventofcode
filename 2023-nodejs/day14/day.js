#!/usr/bin/env node

const fs = require('node:fs');
let cyclememo = []

// fs.readFile('./sample.txt', (err, data) => {
fs.readFile('./input.txt', (err, data) => {
    if (err) {
        console.error(err);
    }
    
    let rows1 = data.toString().split('\n').map((r) => r.split(""))

    rollAll(rows1, "N")

    let sum1 = 0
    rows1.forEach((r, i) => {
        r.forEach((c) => {
            if (c === "O") {
                sum1 += (rows1.length - i)
            }
        })
    })

    console.log("Part One", sum1)

    let rows2 = data.toString().split('\n').map((r) => r.split(""))
    rows2 = cycleN(rows2, 1000000000)

    let sum2 = 0
    rows2.forEach((r, i) => {
        r.forEach((c) => {
            if (c === "O") {
                sum2 += (rows2.length - i)
            }
        })
    })

    console.log("Part Two", sum2)
});

function cycleN(rows, count) {
    let prevLen = -1
    
    while (prevLen !== cyclememo.length) {
        prevLen = cyclememo.length

        let memokey = rows.map((r) => r.join("")).join("")
        let knownAns = cyclememo.filter((c) => c.key === memokey)
        if (knownAns.length === 0) {
            rows = cycle(rows)
            cyclememo.push({key: memokey, ans: [...rows.map((r) => [...r])]})
        } else {
            rows = knownAns[0].ans
        }
    }

    let lastpos = cyclememo[cyclememo.length - 1]
    let loopbackkey = lastpos.ans.map((r) => r.join("")).join("")
    let loopbackidx = 0
    for (let i=0; i < cyclememo.length; i++) {
        if (cyclememo[i].key === loopbackkey) {
            loopbackidx = i
        }
    }

    let repeatLength = cyclememo.length - loopbackidx
    let ansidxoffset = (count - loopbackidx) % repeatLength

    return cyclememo[ansidxoffset+loopbackidx-1].ans
}

function cycle(rows) {
    rows = rollAll(rows, "N")
    rows = rollAll(rows, "W")
    rows = rollAll(rows, "S")
    rows = rollAll(rows, "E")    

    return rows
}

function rollAll(rows, direction) {
    if (direction === "N" || direction === "W") {
        rows.forEach((row, i) => {
            for (let j = 0; j < row.length; j++) {
                if (row[j] === "O") {
                    rows = roll(rows, {row: i, col: j}, direction)
                }
            }
        })
    } else {
        for (let i = rows.length-1; i>=0; i--) {
            for (let j = rows[i].length-1; j>=0; j--) {
                if (rows[i][j] === "O") {
                    rows = roll(rows, {row: i, col: j}, direction)
                }
            }
        }
    }
    return rows
}

function roll(rows, coord, direction) {
    let dest_row = coord.row
    let dest_col = coord.col

    if (direction === "N") {
        while (rows[dest_row-1]?.[coord.col] === ".") {
            dest_row--
        }
    } else if (direction === "S") {
        while (rows[dest_row+1]?.[coord.col] === ".") {
            dest_row++
        }
    } else if (direction === "W") {
        while (rows[coord.row][dest_col-1] === ".") {
            dest_col--
        }
    } else {
        while (rows[coord.row][dest_col+1] === ".") {
            dest_col++
        }
    }

    rows[coord.row][coord.col] = "."
    rows[dest_row][dest_col] = "O"
    return rows
}
