#!/usr/bin/env node

const fs = require('node:fs');

// fs.readFile('./sample.txt', (err, data) => {
fs.readFile('./input.txt', (err, data) => {
    if (err) {
        console.error(err);
    }
    
    const lines = data.toString().split('\n')

    let trenchPts = {"0,0": true}
    let pos = {x:0, y:0}
    let xMin = 0
    let xMax = 0
    let yMin = 0
    let yMax = 0

    lines.forEach((line) => {
        const split = line.split(" ")
        const direction = split[0]
        let steps = Number(split[1])

        let move = (p) => {
            if (direction === "U") {
                p.y += 1
            } else if (direction === "D") {
                p.y -= 1
            } else if (direction === "L") {
                p.x -= 1
            } else if (direction === "R") {
                p.x += 1
            }
            return p
        }

        while (steps > 0) {
            pos = move(pos)

            if (pos.x > xMax) {
                xMax = pos.x
            }
            if (pos.x < xMin) {
                xMin = pos.x
            }
            if (pos.y > yMax) {
                yMax = pos.y
            }
            if (pos.y < yMin) {
                yMin = pos.y
            }

            let posStr = pos.x + "," + pos.y
            trenchPts[posStr] = true

            steps--
        }
    })

    let outside = {}
    // populate outside with border points that are definitely outside of the lagoon
    for (let i = xMin-1; i <= xMax+1; i++) {
        outside[i+","+(yMin-1)] = true
        outside[i+","+(yMax+1)] = true
    }
    for (let j = yMin-1; j <= yMax+1; j++) {
        outside[(xMin-1)+","+j] = true
        outside[(xMax+1)+","+j] = true
    }

    let prevLength = 0
    while (Object.keys(outside).length !== prevLength) {
        prevLength = Object.keys(outside).length

        for (let i = xMin; i <= xMax; i++) {
            for (let j = yMin; j <= yMax; j++) {
                let key = i + "," + j
                if (!(key in outside) && !(key in trenchPts)) {
                    let anyOutside = false
                    neighbors(i, j).forEach((neighborKey) => {
                        if (neighborKey in outside) {
                            anyOutside = true
                        }
                    })

                    if (anyOutside) {
                        outside[key] = true
                    }
                }
            }
        }
    }

    let totalArea = (xMax - xMin + 1) * (yMax - yMin + 1)
    let p1WrongAns = Object.keys(trenchPts).length + (totalArea - prevLength)
    let printAns = printMap(trenchPts, outside, xMin, xMax, yMin, yMax)
    // not exactly sure why the faster count only works on the sample...
    // anyway, it's all obsoleted by the method for part 2.
    console.log("Part One", p1WrongAns, printAns)

    let p2Pts = []
    let p2pos = {x:0, y:0}
    p2Pts.push({...p2pos})
    let trenchShare = 1

    lines.forEach((line) => {
        const split = line.split(" ")
        const distance = parseInt(split[2].slice(2,7), 16)
        const dir = split[2][7]

        // The points we're recording can be considered the "middle" of each
        // square. Along the edges, half of each square is "outside" of the
        // polygon these points define. This will account for that.
        trenchShare += distance / 2

        if (dir === "0") { // R
            p2pos.x += distance
        } else if (dir === "1") { // D
            p2pos.y -= distance
        } else if (dir === "2") { // L
            p2pos.x -= distance
        } else if (dir === "3") { // U
            p2pos.y += distance
        }

        p2Pts.push({...p2pos})
    })

    // A vague memory of Green's theorem led me to the
    // "shoelace" formula for the area of polygons
    let doubleArea = 0
    for (let i=0; i < (p2Pts.length - 1); i++) {
        let x_1 = p2Pts[i].x
        let x_2 = p2Pts[i+1].x
        let y_1 = p2Pts[i].y
        let y_2 = p2Pts[i+1].y

        doubleArea += (x_1 * y_2) - (x_2 * y_1)
    }

    console.log(Math.abs(doubleArea / 2) + trenchShare)
});

function neighbors(x, y) {
    return [
        (x-1) + "," + (y),
        (x+1) + "," + (y),
        (x) + "," + (y-1),
        (x) + "," + (y+1),
        // diagonals:
        (x-1) + "," + (y-1),
        (x-1) + "," + (y+1),
        (x+1) + "," + (y-1),
        (x+1) + "," + (y+1),
    ]
}

function printMap(trenchPts, outside, xMin, xMax, yMin, yMax) {
    let count = 0
    for (let j = yMax+1; j >= yMin-1; j--) {
        let line = ""
        for (let i = xMin-1; i <= xMax+1; i++) {
            let key = i + "," + j
            let identified = false

            if (key in trenchPts) {
                line += "#"
                identified = true
                count++
            }

            if (key in outside) {
                line += "O"
                if (identified) {
                    console.log(i, j, "double identified?")
                }
                identified = true
            }

            if (!identified) {
                line += "."
                count++
            }
        }
        // console.log(line)
    }
    return count
}
