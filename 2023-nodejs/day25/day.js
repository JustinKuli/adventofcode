#!/usr/bin/env node

const fs = require('node:fs');
const { connect } = require('node:http2');

// fs.readFile('./sample.txt', (err, data) => {
fs.readFile('./input.txt', (err, data) => {
    if (err) {
        console.error(err);
    }
    
    const lines = data.toString().split('\n')

    let nodes = {}

    lines.forEach((line) => {
        let [start, destStr] = line.split(": ")
        let dests = destStr.split(" ")

        if (start in nodes) {
            dests.forEach((d) => {
                nodes[start].push(d)
            })
        } else {
            nodes[start] = dests
        }

        dests.forEach((d) => {
            if (d in nodes) {
                nodes[d].push(start)
            } else {
                nodes[d] = [start]
            }
        })
    })

    let weightedConnections = {}
    let len = Object.keys(nodes).length
    let acceptedPaths = 0
    let rejectedPaths = 0
    let loopCount = 0
    let attemptedCuts = {}

    while (true) {
        for (let i = 0; i < 25; i++){
            Object.keys(nodes).forEach((start) => {
                let res = randomWalk(start, nodes)
                
                let weight = res.total / len
                if (weight < 0.5) {
                    rejectedPaths++
                    return
                }
    
                acceptedPaths++
    
                weight *= weight
    
                res.connections.forEach((cxn) => {
                    if (cxn in weightedConnections) {
                        weightedConnections[cxn] += weight
                    } else {
                        weightedConnections[cxn] = weight
                    }
                })
            })
        }
    
        console.log("accepted paths", acceptedPaths)
        console.log("rejected paths", rejectedPaths)
    
        let cxns = Object.keys(weightedConnections)
        let sorted = cxns.toSorted((a, b) => {
            return weightedConnections[b] - weightedConnections[a]
        })
    
        let maxDepth = 20 + 5*loopCount // go a little deeper each loop
        const start = Object.keys(nodes)[0]
    
        for (let i = 0; i <= maxDepth; i++) {
            for (let j = i+1; j <= maxDepth; j++) {
                for (let k = j+1; k <= maxDepth; k++) {
                    let removals = [sorted[i], sorted[j], sorted[k]]
                    let attempt = removals.sort().join(",")
                    if (attempt in attemptedCuts) {
                        continue
                    }

                    attemptedCuts[attempt] = true

                    let size = connectedSize(start, removeConnections(nodes, removals))
                    if (size !== len) {
                        console.log("Found one side!", size)
                        console.log("Part One", size * (len-size))
                        return
                    }
                }
            }
        }
    
        console.log("Unsuccessful. Trying some more random walks.")
        loopCount++
    }
});

function randomWalk(start, nodes) {
    let visited = {}
    visited[start] = true
    
    let connections = []

    let curr = start

    while(true) {
        let allNeighbors = nodes[curr]

        let possible = []
        allNeighbors.forEach((n) => {
            if (!(n in visited)) {
                possible.push(n)
            }
        })

        if (possible.length === 0) {
            return {
                connections: connections,
                total: Object.keys(visited).length
            }
        }

        let next = possible[0]
        if (possible.length > 1) {
            next = possible[getRandomInt(possible.length)]
        }

        visited[next] = true
        if (curr < next) {
            connections.push(curr+"-"+next)
        } else {
            connections.push(next+"-"+curr)
        }
        curr = next
    }
}

function removeConnections(nodes, connections) {
    let ans = {...nodes}

    connections.forEach((cxn) => {
        let [a, b] = cxn.split("-")

        ans[a] = ans[a].toSpliced(ans[a].indexOf(b), 1)
        ans[b] = ans[b].toSpliced(ans[b].indexOf(a), 1)
    })

    return ans
}

function connectedSize(start, nodes) {
    let unchecked = [start]
    let visited = {}

    while (unchecked.length !== 0) {
        let check = unchecked.pop()

        nodes[check].forEach((next) => {
            if (!(next in visited)) {
                visited[next] = true
                unchecked.push(next)
            }
        })
    }

    return Object.keys(visited).length
}

// 0 up to, but not including max
function getRandomInt(max) {
    return Math.floor(Math.random() * max);
}
