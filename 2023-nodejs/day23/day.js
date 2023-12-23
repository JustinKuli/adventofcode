#!/usr/bin/env node

const fs = require('node:fs');

// fs.readFile('./sample.txt', (err, data) => {
fs.readFile('./input.txt', (err, data) => {
    if (err) {
        console.error(err);
    }
    
    const lines = data.toString().split('\n')

    let map = []
    let start = {}

    lines.forEach((line, i) => {
        map.push(line.split(""))

        if (i === 0) {
            start = {row: 0, col: line.indexOf(".")}
        }
    })

    let nodes = {}
    nodes[str(start)] = {neighbors: {}}

    let end = {}

    let uncalculated = [{
        node: start, 
        next: {row: 1, col: start.col}
    }]

    while (uncalculated.length !== 0) {
        let start = uncalculated.pop()
        let fun = followUntilNode(start.next, start.node, map)

        if (fun.nexts.length === 0) {
            if (fun.nodePos.row === map.length-1) {
                // you have reached your destination
                end = fun.nodePos
            } else {
                // leads to a dead end, don't record anything
                continue
            }
        }


        if (!(str(fun.nodePos) in nodes[str(start.node)].neighbors)) {
            nodes[str(start.node)].neighbors[str(fun.nodePos)] = fun.distance
        } else {
            let existing = nodes[str(start.node)].neighbors[str(fun.nodePos)]
            if (fun.distance > existing) {
                nodes[str(start.node)].neighbors[str(fun.nodePos)] = fun.distance
            }
        }

        if (!(str(fun.nodePos) in nodes)) {
            // first time at this node, initialize it and add its outs to be calculated
            nodes[str(fun.nodePos)] = {neighbors: {}}

            fun.nexts.forEach((next) => {
                uncalculated.push({
                    node: fun.nodePos,
                    next: next,
                })
            })    
        }
    }

    // ok! now we have the thing in a nice graph.

    let initial = {
        path: [str(start)],
        distance: 0
    }

    let incompletes = [initial]
    let longest = 0
    let longestPath = []
    let endStr = str(end)

    while (incompletes.length !== 0) {
        let curr = incompletes.pop()
        let currNode = nodes[curr.path[curr.path.length-1]]

        Object.keys(currNode.neighbors).forEach((next) => {
            if (next === endStr) {
                let completedDist = curr.distance + currNode.neighbors[next]
                if (completedDist > longest) {
                    longest = completedDist
                    longestPath = curr.path
                }
            } else if (!curr.path.includes(next)) {
                incompletes.push({
                    path: [...curr.path, next],
                    distance: curr.distance + currNode.neighbors[next]
                })
            }
        })
    }

    console.log(longestPath)

    // let debug = {...nodes}
    // longestPath.forEach((p) => {
    //     delete debug[p]
    // })

    // console.log(debug)

    console.log("Part One", longest) // 5418 is too low
});

function followUntilNode(loc_in, prev_in, map) {
    let loc = {...loc_in}
    let prev = {...prev_in}

    let distance = 1

    let next = nextAdj(loc, prev, map)
    while (next.length === 1) {
        prev = loc
        loc = next[0]
        distance++

        next = nextAdj(loc, prev, map)
    }

    return {
        nodePos: loc,
        distance: distance,
        nexts: next
    }
}

function nextAdj(loc, prev, map) {
    let ans = []

    let up = map[loc.row-1]?.[loc.col] || "#"
    // if (up === "." || up === "^") { // part 1 condition
    if (up !== "#") {
        if (prev.row !== loc.row-1) {
            ans.push({row: loc.row-1, col: loc.col})
        }
    }

    let down = map[loc.row+1]?.[loc.col] || "#"
    // if (down === "." || down === "v") { // part 1 condition
    if (down !== "#") {
        if (prev.row !== loc.row+1) {
            ans.push({row: loc.row+1, col: loc.col})
        }
    }

    let left = map[loc.row]?.[loc.col-1] || "#"
    // if (left === "." || left === "<") { // part 1 condition
    if (left !== "#") {
        if (prev.col !== loc.col-1) {
            ans.push({row: loc.row, col: loc.col-1})
        }
    }

    let right = map[loc.row]?.[loc.col+1] || "#"
    // if (right === "." || right === ">") { // part 1 condition
    if (right !== "#") {
        if (prev.col !== loc.col+1) {
            ans.push({row: loc.row, col: loc.col+1})
        }
    }

    return ans
}

function str(pt) {
    return pt.row + "," + pt.col
}

function nonWorkingDjkistras() {
    // Let's try djikstra's

    // 1
    let unvisited = {}
    Object.keys(nodes).forEach((n) => {
        unvisited[n] = true

        // 2
        nodes[n].tentative = 999 // anything positive might work, this just helps debug
    })
    nodes[str(start)].tentative = 0

    let curr = {...start}
    console.log("curr", curr)

    while (Object.keys(unvisited).length !== 0) {
        console.log(nodes)
        console.log(unvisited)

        // 3
        Object.keys(nodes[str(curr)].neighbors).forEach((neighbor) => {
            if (neighbor in unvisited) {
                let tent = nodes[str(curr)].tentative - nodes[str(curr)].neighbors[neighbor]
                if (tent < nodes[neighbor].tentative) {
                    nodes[neighbor].tentative = tent
                }
            }
        })

        // 4
        delete unvisited[str(curr)]

        // 5 might not apply - so just end when there are no more unvisiteds

        // 6 - brute looop to find the biggest unvisited
        let smallest = 9999
        let next = ""
        Object.keys(unvisited).forEach((n) => {
            if (nodes[n].tentative < smallest) {
                smallest = nodes[n].tentative
                next = n
            }
        })

        curr.row = next.split(",")[0]
        curr.col = next.split(",")[1]

        console.log("curr", curr)
    }
}
