#!/usr/bin/env node

const fs = require('node:fs');

// fs.readFile('./sample1.txt', (err, data) => {
fs.readFile('./input.txt', (err, data) => {
    if (err) {
        console.error(err);
    }
    
    const lines = data.toString().split('\n')

    let puzzmap = []
    let start = {}

    lines.forEach((line, lineidx) => {
        puzzmap.push(line.split(""))

        if (line.includes("S")) {
            start = {row: lineidx, col: line.indexOf("S")}
        }
    })

    let paths = []

    // below
    let bsp = puzzmap[start.row+1]?.[start.col]
    if (bsp === "|" || bsp === "J" || bsp === "L") {
        paths.push([start, {row: start.row+1, col: start.col}])
    }

    // right
    let rsp = puzzmap[start.row]?.[start.col+1]
    if (rsp === "-" || rsp === "J" || rsp === "7") {
        paths.push([start, {row: start.row, col: start.col+1}])
    }

    // above
    let asp = puzzmap[start.row-1]?.[start.col]
    if (asp === "|" || asp === "F" || asp === "7") {
        paths.push([start, {row: start.row-1, col: start.col}])
    }

    // left
    let lsp = puzzmap[start.row]?.[start.col-1]
    if (lsp === "-" || lsp === "L" || lsp === "F") {
        paths.push([start, {row: start.row, col: start.col-1}])
    }

    while (true) {
        let p1 = nextconnect(puzzmap, paths[0])
        let p2 = nextconnect(puzzmap, paths[1])

        paths[0].push(p1)
        paths[1].push(p2)

        if (p1.col === p2.col && p1.row === p2.row) {
            break
        }
    }

    // console.log("Part One:", path1.length - 1)
    
    let fullpath = [...paths[0], ...paths[1]]

    let fastpath = {} // allowing for fast lookups
    fullpath.forEach((p) => {
        fastpath[p.row+","+p.col] = true
    })

    const len = puzzmap.length // it's a square, btw

    let outside = {}
    let outstartlen = -1

    // start by finding outside points, starting at the border of the map and working inwards
    while (Object.keys(outside).length !== outstartlen) { // do multiple times to handle some odd shapes
        let outstartlen = Object.keys(outside).length

        for (let j = 0; j <= len/2; j++) {
            for (let i = j; i < len-j; i++) {
                let pt = {row: i, col: j}
                if (!fastpath[pt.row+","+pt.col] && (neighborin(pt, outside) || j === 0)) {
                    outside[pt.row+","+pt.col] = true
                }
        
                pt = {row: i, col: len-1-j}
                if (!fastpath[pt.row+","+pt.col] && (neighborin(pt, outside) || j === 0)) {
                    outside[pt.row+","+pt.col] = true
                }
        
                pt = {row: j, col: i}
                if (!fastpath[pt.row+","+pt.col] && (neighborin(pt, outside) || j === 0)) {
                    outside[pt.row+","+pt.col] = true
                }
        
                pt = {row: len-1-j, col: i}
                if (!fastpath[pt.row+","+pt.col] && (neighborin(pt, outside) || j === 0)) {
                    outside[pt.row+","+pt.col] = true
                }
            }
        }

        if (Object.keys(outside).length === outstartlen) {
            break
        }
    }

    let inside = {}

    // Go along the path and identify some inside and outside points
    // Do it twice because we might not start at a point we can label
    let twice = [0,1]
    twice.forEach(() => {
        fullpath.forEach((curr) => {
            let currpipe = puzzmap[curr.row][curr.col]

            let left = {row: curr.row, col: curr.col-1}
            let halfleft = {row: curr.row, col: curr.col-0.5}

            let right = {row: curr.row, col: curr.col+1}
            let halfright = {row: curr.row, col: curr.col+0.5}

            let above = {row: curr.row-1, col: curr.col}
            let halfabove = {row: curr.row-0.5, col: curr.col}

            let below = {row: curr.row+1, col: curr.col}
            let halfbelow = {row: curr.row+0.5, col: curr.col}

            let aboveright = {row: curr.row-1, col: curr.col+1}
            let halfaboveright = {row: curr.row-0.5, col: curr.col+0.5}

            let belowright = {row: curr.row+1, col: curr.col+1}
            let halfbelowright = {row: curr.row+0.5, col: curr.col+0.5}

            let aboveleft = {row: curr.row-1, col: curr.col-1}
            let halfaboveleft = {row: curr.row-0.5, col: curr.col-0.5}

            let belowleft = {row: curr.row+1, col: curr.col-1}
            let halfbelowleft = {row: curr.row+0.5, col: curr.col-0.5}

            if (currpipe === "|") {
                [inside, outside] = dopathwork(
                    [halfleft, halfbelowleft, halfaboveleft],
                    [halfright, halfbelowright, halfaboveright],
                    left,
                    right,
                    inside, outside, len
                )
            } else if (currpipe === "-") {
                [inside, outside] = dopathwork(
                    [halfabove, halfaboveleft, halfaboveright],
                    [halfbelow, halfbelowleft, halfbelowright],
                    above,
                    below,
                    inside, outside, len
                )
            } else if (currpipe === "L") {
                [inside, outside] = dopathwork(
                    [halfaboveright],
                    [halfaboveleft, halfleft, halfbelowleft, halfbelow, halfbelowright],
                    aboveright,
                    belowleft,
                    inside, outside, len
                )
            } else if (currpipe === "J") {
                [inside, outside] = dopathwork(
                    [halfaboveleft],
                    [halfaboveright, halfright, halfbelowright, halfbelow, halfbelowleft],
                    aboveleft,
                    belowright,
                    inside, outside, len
                )
            } else if (currpipe === "7") {
                [inside, outside] = dopathwork(
                    [halfbelowleft],
                    [halfaboveleft, halfabove, halfaboveright, halfright, halfbelowright],
                    belowleft,
                    aboveright,
                    inside, outside, len
                )
            } else if (currpipe === "F") {
                [inside, outside] = dopathwork(
                    [halfbelowright],
                    [halfaboveright, halfabove, halfaboveleft, halfleft, halfbelowleft],
                    belowright,
                    aboveleft,
                    inside, outside, len
                )
            }
        })
    })

    let totallen = -1

    while (Object.keys(inside).length + Object.keys(outside).length !== totallen) {
        totallen = Object.keys(inside).length + Object.keys(outside).length

        // label points based on their neighbors
        for (let i = 0; i < len; i++) {
            for (let j = 0; j < len; j++) {
                let curr = {row: i, col: j}

                // if it's already labelled, skip it
                if (fastpath[key(curr)] || outside[key(curr)] || inside[key(curr)]) {
                    continue
                }

                let left = {row: curr.row, col: curr.col-1}
                let halfleft = {row: curr.row, col: curr.col-0.5}
    
                let right = {row: curr.row, col: curr.col+1}
                let halfright = {row: curr.row, col: curr.col+0.5}
    
                let above = {row: curr.row-1, col: curr.col}
                let halfabove = {row: curr.row-0.5, col: curr.col}
    
                let below = {row: curr.row+1, col: curr.col}
                let halfbelow = {row: curr.row+0.5, col: curr.col}
    
                let aboveright = {row: curr.row-1, col: curr.col+1}
                let halfaboveright = {row: curr.row-0.5, col: curr.col+0.5}
    
                let belowright = {row: curr.row+1, col: curr.col+1}
                let halfbelowright = {row: curr.row+0.5, col: curr.col+0.5}
    
                let aboveleft = {row: curr.row-1, col: curr.col-1}
                let halfaboveleft = {row: curr.row-0.5, col: curr.col-0.5}
    
                let belowleft = {row: curr.row+1, col: curr.col-1}
                let halfbelowleft = {row: curr.row+0.5, col: curr.col-0.5}

                let pts = [left, halfleft, right, halfright, above, halfabove, below, halfbelow,
                    aboveright, halfaboveright, belowright, halfbelowright, aboveleft, halfaboveleft, belowleft, halfbelowleft]
    
                for (let pi = 0; pi < pts.length; pi++) {
                    let pt = pts[pi]

                    if (isout(pt, outside, len)) {
                        outside[key(curr)] = true
                        break
                    }

                    if (inside[key(pt)]) {
                        inside[key(curr)] = true
                        break
                    }
                }
            }
        }

        if (totallen === Object.keys(inside).length + Object.keys(outside).length) {
            break
        }
    }

    // printpoints(puzzmap, inside, outside, fastpath)

    // console.log(outside)
    // console.log(inside)

    let count = 0
    Object.keys(inside).forEach((inpt) => {
        if (!inpt.includes(".")) {
            count++
        }
    })

    console.log("Part Two:", count)
});

// for part 1
function nextconnect(puzzmap, path) {
    let curr = path[path.length-1]
    let currpipe = puzzmap[curr.row][curr.col]

    let prev = path[path.length-2]

    switch (currpipe) {
        case "|":
            if (prev.row === curr.row - 1) {
                return {col: curr.col, row: curr.row + 1}
            } else {
                return {col: curr.col, row: curr.row - 1}
            }
        case "-":
            if (prev.col === curr.col - 1) {
                return {col: curr.col + 1, row: curr.row}
            } else {
                return {col: curr.col -1, row: curr.row}
            }
        case "L":
            if (prev.col === curr.col + 1) {
                return {col: curr.col, row: curr.row - 1}
            } else {
                return {col: curr.col + 1, row: curr.row}
            }
        case "J":
            if (prev.col === curr.col) {
                return {col: curr.col - 1, row: curr.row}
            } else {
                return {col: curr.col, row: curr.row - 1}
            }
        case "7":
            if (prev.col === curr.col) {
                return {col: curr.col - 1, row: curr.row}
            } else {
                return {col: curr.col, row: curr.row + 1}
            }
        case "F":
            if (prev.col === curr.col) {
                return {col: curr.col + 1, row: curr.row}
            } else {
                return {col: curr.col, row: curr.row+1}
            }
    }

    console.log("Unexpected", currpipe, "at", curr, "path", path)
}

// For part 2
function neighborin(pos, outside) {
    let up = pos.row+","+(pos.col-1)
    if (outside[up]) {
        return true
    }

    let down = pos.row+","+(pos.col+1)
    if (outside[down]) {
        return true
    }

    let left = (pos.row-1)+","+pos.col
    if (outside[left]) {
        return true
    }

    let right = (pos.row+1)+","+pos.col
    if (outside[right]) {
        return true
    }

    return false
}

function key(pos) {
    return pos.row+","+pos.col
}

function isout(pos, outside, len) {
    if (outside[key(pos)]) {
        return true
    }

    if (pos.col < 0 || pos.col >= len || pos.row < 0 || pos.row >= len) {
        return true
    }

    return false
}

function dopathwork(a, b, a_pt, b_pt, inside, outside, len) {
    let a_in = inside[key(a_pt)]
    let a_out = isout(a_pt, outside, len)

    let b_in = inside[key(b_pt)]
    let b_out = isout(b_pt, outside, len)

    if (!a_in && !a_out) {
        a.forEach((p) => {
            if (inside[key(p)]) {
                a_in = true
            } else if (isout(p, outside, len)) {
                a_out = true
            }
        })
    }

    if (a_in) {
        a.forEach((p) => {
            inside[key(p)] = true
        })
        b.forEach((p) => {
            outside[key(p)] = true
        })
    }
    if (a_out) {
        a.forEach((p) => {
            outside[key(p)] = true
        })
        b.forEach((p) => {
            inside[key(p)] = true
        })
    }

    if (!a_in && !a_out) {
        if (!b_in && !b_out) {
            b.forEach((p) => {
                if (inside[key(p)]) {
                    b_in = true
                } else if (isout(p, outside, len)) {
                    b_out = true
                }
            })
        }

        if (b_in) {
            b.forEach((p) => {
                inside[key(p)] = true
            })
            a.forEach((p) => {
                outside[key(p)] = true
            })
        }
        if (b_out) {
            b.forEach((p) => {
                outside[key(p)] = true
            })
            a.forEach((p) => {
                inside[key(p)] = true
            })
        }
    }

    return [inside, outside]
}

// for debug
function printpoints(puzz, inside, outside, path) {
    let output = []
    for (let i = 0; i < puzz.length; i++) {
        let row = []
        for (let j = 0; j < puzz.length; j++) {
            let pt = i+","+j
            if (path[pt]) {
                row.push(puzz[i][j])
            } else if (inside[pt]){
                row.push("i")
            } else if (outside[pt]) {
                row.push("o")
            } else {
                row.push(" ")
            }
        }
        output.push(row.join(""))
    }

    output.forEach((l) => {
        console.log(l)
    })
}

