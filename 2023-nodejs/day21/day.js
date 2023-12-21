#!/usr/bin/env node

const fs = require('node:fs');

let memos = {}

fs.readFile('./sample.txt', (err, data) => {
// fs.readFile('./input.txt', (err, data) => {
    if (err) {
        console.error(err);
    }
    
    const lines = data.toString().split('\n')
    let map = []
    let ptSet = {}
    lines.forEach((line, i) => {
        map.push(line.split(""))
        let sIdx = line.indexOf("S")
        if (sIdx != -1) {
            ptSet[i + "," + sIdx] = true
            map[i][sIdx] = "."
        }
    })

    let l = map.length

    for (let i=0; i<500; i++) {
        let nextPts = {}

        // Maybe I can use division to assign a "segment" to each point...
        // and if I can identify when a segment is full,
        // then I can skip points in that segment.
        // That would kind of only track the perimeter,
        // which I guess would grow linearly as opposed to quadratically? 

        Object.keys(ptSet).forEach((ptStr) => {
            let pt = toPt(ptStr)
            neighbors(ptStr, map, l).forEach((n) => {
                let newPt = {...pt}
                if (n === "u") {
                    newPt.row--
                } else if (n === "d") {
                    newPt.row++
                } else if (n === "l") {
                    newPt.col--
                } else {
                    newPt.col++
                }
                nextPts[str(newPt)] = true
            })
        })
        ptSet = nextPts
    }

    console.log("Part One", Object.keys(ptSet).length)
});

// the memoization and remainder stuff is probably helpful,
// but definitely not enough on its own.
function neighbors(ptStr, map, l) {
    if (ptStr in memos) {
        return memos[ptStr]
    }

    let pt = toPt(ptStr)

    let remPt = {...pt}
    pt.row = posRem(pt.row, l)
    pt.col = posRem(pt.col, l)

    if (remPt in memos) {
        return memos[remPt]
    }

    let posUp = {...pt}
    let posDown = {...pt}
    let posLeft = {...pt}
    let posRight = {...pt}

    posUp.row--
    posDown.row++
    posLeft.col--
    posRight.col++

    let ans = []
    if (map[posRem(posUp.row, l)]?.[posRem(posUp.col, l)] === ".") {
        ans.push("u")
    }
    if (map[posRem(posDown.row, l)]?.[posRem(posDown.col, l)] === ".") {
        ans.push("d")
    }
    if (map[posRem(posLeft.row, l)]?.[posRem(posLeft.col, l)] === ".") {
        ans.push("l")
    }
    if (map[posRem(posRight.row, l)]?.[posRem(posRight.col, l)] === ".") {
        ans.push("r")
    }

    memos[ptStr] = ans
    return ans
}

function str(pt) {
    return pt.row + "," + pt.col
}

function toPt(str) {
    let parts = str.split(",")
    return {row: Number(parts[0]), col: Number(parts[1])}
}

function posRem(x, d) {
    let ans = x % d
    if (ans < 0) {
        ans += d
    }
    return ans
}