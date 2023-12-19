#!/usr/bin/env node

const fs = require('node:fs');

// fs.readFile('./sample.txt', (err, data) => {
fs.readFile('./input.txt', (err, data) => {
    if (err) {
        console.error(err);
    }

    let map = []
    const lines = data.toString().split('\n')
    lines.forEach((line) => {
        map.push(line.split("").map((n) => Number(n)))
    })

    // pre-populating with the possible future ways into 0,0 so they're skipped
    let visited = {
        // Part One
        // "0,0,<": {path: "", loss: 0},
        // "0,0,<<": {path: "", loss: 0},
        // "0,0,<<<": {path: "", loss: 0},
        // "0,0,^": {path: "", loss: 0},
        // "0,0,^^": {path: "", loss: 0},
        // "0,0,^^^": {path: "", loss: 0},

        // Part Two
        "0,0,<<<<": {path: "", loss: 0},
        "0,0,<<<<<": {path: "", loss: 0},
        "0,0,<<<<<<": {path: "", loss: 0},
        "0,0,<<<<<<<": {path: "", loss: 0},
        "0,0,<<<<<<<<": {path: "", loss: 0},
        "0,0,<<<<<<<<<": {path: "", loss: 0},
        "0,0,<<<<<<<<<<": {path: "", loss: 0},
        "0,0,^^^^": {path: "", loss: 0},
        "0,0,^^^^^": {path: "", loss: 0},
        "0,0,^^^^^^": {path: "", loss: 0},
        "0,0,^^^^^^^": {path: "", loss: 0},
        "0,0,^^^^^^^^": {path: "", loss: 0},
        "0,0,^^^^^^^^^": {path: "", loss: 0},
        "0,0,^^^^^^^^^^": {path: "", loss: 0},
    }

    let tentByLoss = {} // like "4": ["0,1,>"]
    let tentByKey = {} // like "0,1,>" : {row: 0, col: 1, loss: 4, path: ">"}

    let curr = {row: 0, col: 0, loss: 0, path: ""}

    while (curr.row !== map.length-1 || curr.col !== map[0].length-1) {
        // look at neighbors, compare to tentatives and update as necessary
        neighbors2(curr, map).forEach((n) => {
            let end = lastSegment(n.path)
            let key = n.row + "," + n.col + "," + end
            if (key in visited) {
                return
            }

            if (key in tentByKey) {
                let existingLoss = tentByKey[key].loss
                if (n.loss < existingLoss) {
                    tentByKey[key] = n

                    let idxToRemove = tentByLoss[existingLoss].indexOf(key)
                    tentByLoss[existingLoss].splice(idxToRemove, 1)
                    if (tentByLoss[existingLoss].length === 0) {
                        delete tentByLoss[existingLoss]
                    }

                    if (n.loss in tentByLoss) {
                        tentByLoss[n.loss].push(key)
                    } else {
                        tentByLoss[n.loss] = [key]
                    }
                }
            } else {
                tentByKey[key] = n

                if (n.loss in tentByLoss) {
                    tentByLoss[n.loss].push(key)
                } else {
                    tentByLoss[n.loss] = [key]
                }
            }
        })

        // mark curr as visited; remove it from tentatives
        let currKey = curr.row + "," + curr.col + "," + lastSegment(curr.path)
        visited[currKey] = {path: curr.path, loss: curr.loss}
        if (currKey in tentByKey) {
            delete tentByKey[currKey]

            let idxToRemove = tentByLoss[curr.loss].indexOf(currKey)
            tentByLoss[curr.loss].splice(idxToRemove, 1)
            if (tentByLoss[curr.loss].length === 0) {
                delete tentByLoss[curr.loss]
            }
        }

        // Get next smallest tentative to check next
        let tentLosses = Object.keys(tentByLoss).map((l) => Number(l))
        let smallestLoss = tentLosses[0]
        tentLosses.forEach((l) => {
            if (l < smallestLoss) {
                smallestLoss = l
            }
        })

        let nextKey = tentByLoss[smallestLoss][0]
        curr = tentByKey[nextKey]
    }

    console.log(curr)
});


// Part One
function neighbors1(curr, map) {
    let end = lastSegment1(curr.path)
    let ans = []

    // Up and Down
    if (end.length === 0 || end[0] === "<" || end[0] === ">") {
        let upLoss = curr.loss
        let downLoss = curr.loss
        for (let i=1; i<=3; i++) {
            upLoss += map[curr.row - i]?.[curr.col]
            if (upLoss) {
                ans.push({
                    row: curr.row - i,
                    col: curr.col,
                    path: curr.path + "^".repeat(i),
                    loss: upLoss    
                })
            }

            downLoss += map[curr.row + i]?.[curr.col]
            if (downLoss) {
                ans.push({
                    row: curr.row + i,
                    col: curr.col,
                    path: curr.path + "v".repeat(i),
                    loss: downLoss    
                })
            }
        }
    }

    // Left and Right
    if (end.length === 0 || end[0] === "v" || end[0] === "^") {
        let leftLoss = curr.loss
        let rightLoss = curr.loss
        for (let i=1; i<=3; i++) {
            leftLoss += map[curr.row][curr.col - i]
            if (leftLoss) {
                ans.push({
                    row: curr.row,
                    col: curr.col - i,
                    path: curr.path + "<".repeat(i),
                    loss: leftLoss    
                })
            }

            rightLoss += map[curr.row][curr.col + i]
            if (rightLoss) {
                ans.push({
                    row: curr.row,
                    col: curr.col + i,
                    path: curr.path + ">".repeat(i),
                    loss: rightLoss
                })
            }
        }
    }

    return ans
}

function lastSegment1(path) {
    const l = path.length
    if (l === 0) {
        return ""
    }

    let end = path[l-1]

    if (path[l-2] === end) {
        end += path[l-2]
    } else {
        return end
    }

    if (path[l-3] === path[l-2]) {
        end += path[l-3]
    }

    return end
}

// Part Two
function neighbors2(curr, map) {
    let end = lastSegment(curr.path)
    let ans = []

    // Up and Down
    if (end.length === 0 || end[0] === "<" || end[0] === ">") {
        let upLoss = curr.loss
        let downLoss = curr.loss
        for (let i=1; i<=3; i++) {
            upLoss += map[curr.row - i]?.[curr.col]
            downLoss += map[curr.row + i]?.[curr.col]
        }
        for (let i=4; i<=10; i++) {
            upLoss += map[curr.row - i]?.[curr.col]
            if (upLoss) {
                ans.push({
                    row: curr.row - i,
                    col: curr.col,
                    path: curr.path + "^".repeat(i),
                    loss: upLoss    
                })
            }

            downLoss += map[curr.row + i]?.[curr.col]
            if (downLoss) {
                ans.push({
                    row: curr.row + i,
                    col: curr.col,
                    path: curr.path + "v".repeat(i),
                    loss: downLoss    
                })
            }
        }
    }

    // Left and Right
    if (end.length === 0 || end[0] === "v" || end[0] === "^") {
        let leftLoss = curr.loss
        let rightLoss = curr.loss
        for (let i=1; i<=3; i++) {
            leftLoss += map[curr.row][curr.col - i]
            rightLoss += map[curr.row][curr.col + i]
        }
        for (let i=4; i<=10; i++) {
            leftLoss += map[curr.row][curr.col - i]
            if (leftLoss) {
                ans.push({
                    row: curr.row,
                    col: curr.col - i,
                    path: curr.path + "<".repeat(i),
                    loss: leftLoss    
                })
            }

            rightLoss += map[curr.row][curr.col + i]
            if (rightLoss) {
                ans.push({
                    row: curr.row,
                    col: curr.col + i,
                    path: curr.path + ">".repeat(i),
                    loss: rightLoss
                })
            }
        }
    }

    return ans
}

// Just better
function lastSegment(path) {
    const l = path.length
    if (l === 0) {
        return ""
    }

    let end = path[l-1]
    let prev = end

    let i = 2
    while(path[l-i] === prev) {
        end += prev
        i++
    }

    return end
}