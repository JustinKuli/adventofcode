#!/usr/bin/env node

const fs = require('node:fs');

// fs.readFile('./sample.txt', (err, data) => {
fs.readFile('./input.txt', (err, data) => {
    if (err) {
        console.error(err);
    }
    
    const layout = data.toString().split('\n')

    let beams = [{row: 0, col: -1, direction: "R"}]
    let p1result = energize(layout, beams)
    let p1ans = Object.keys(p1result).length

    console.log("Part One", p1ans)

    let p2best = 0
    for (let i=0; i<layout.length; i++) {
        let candidate = energize(layout, [{row: i, col: -1, direction: "R"}])
        let candLength = Object.keys(candidate).length
        if (candLength > p2best) {
            p2best = candLength
        }

        candidate = energize(layout, [{row: i, col: layout[0].length, direction: "L"}])
        candLength = Object.keys(candidate).length
        if (candLength > p2best) {
            p2best = candLength
        }
    }

    for (let j=0; j<layout[0].length; j++) {
        let candidate = energize(layout, [{row: -1, col: j, direction: "D"}])
        let candLength = Object.keys(candidate).length
        if (candLength > p2best) {
            p2best = candLength
        }

        candidate = energize(layout, [{row: layout.length, col: j, direction: "U"}])
        candLength = Object.keys(candidate).length
        if (candLength > p2best) {
            p2best = candLength
        }
    }

    console.log("Part Two", p2best)
});

function energize(layout, beams) {
    let visited = {}
    
    while (beams.length > 0) {
        let nextbeams = []
        beams.forEach((b) => {
            let next = {}
            if (b.direction === "R") {
                next = {row: b.row, col: b.col+1}
            } else if (b.direction === "L") {
                next = {row: b.row, col: b.col-1}
            } else if (b.direction === "D") {
                next = {row: b.row+1, col: b.col}
            } else if (b.direction === "U") {
                next = {row: b.row-1, col: b.col}
            }

            if (next.row < 0 || next.row >= layout.length ||
                next.col < 0 || next.col >= layout[0].length) {
                    return
            }

            let nextTile = layout[next.row][next.col]
            if (nextTile === ".") {
                next.direction = b.direction

                nextbeams.push(next)
            } else if (nextTile === "/") {
                if (b.direction === "R") {
                    next.direction = "U"
                } else if (b.direction === "L") {
                    next.direction = "D"
                } else if (b.direction === "D") {
                    next.direction = "L"
                } else if (b.direction === "U") {
                    next.direction = "R"
                }

                nextbeams.push(next)
            } else if (nextTile === "\\") {
                if (b.direction === "R") {
                    next.direction = "D"
                } else if (b.direction === "L") {
                    next.direction = "U"
                } else if (b.direction === "D") {
                    next.direction = "R"
                } else if (b.direction === "U") {
                    next.direction = "L"
                }

                nextbeams.push(next)
            } else if (nextTile === "|") {
                if (b.direction === "U" || b.direction === "D") {
                    next.direction = b.direction

                    nextbeams.push(next)
                } else {
                    let next2 = {...next}
                    next.direction = "U"
                    next2.direction = "D"

                    nextbeams.push(next, next2)
                }
            } else if (nextTile === "-") {
                if (b.direction === "L" || b.direction === "R") {
                    next.direction = b.direction

                    nextbeams.push(next)
                } else {
                    let next2 = {...next}
                    next.direction = "L"
                    next2.direction = "R"

                    nextbeams.push(next, next2)
                }
            }
        })

        beams = []

        nextbeams.forEach((nb) => {
            let key = nb.row + "," + nb.col
            if (key in visited) {
                if (!visited[key].includes(nb.direction)) {
                    visited[key].push(nb.direction)
                    beams.push(nb)
                }
            } else {
                visited[key] = [nb.direction]
                beams.push(nb)
            }
        })

        // console.log(beams)
    }

    return visited
}