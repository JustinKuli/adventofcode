#!/usr/bin/env node

const fs = require('node:fs');

fs.readFile('./sample.txt', (err, data) => {
// fs.readFile('./input.txt', (err, data) => {
    if (err) {
        console.error(err);
    }

    const lines = data.toString().split('\n')

    let graph = []
    let unvisited = {}
    let tentative_bydist = {} // key: distance, val: ["row,col,direction_in"]
    let tentative_bykey = {} // key: "row,col,direction_in", val: distance

    lines.forEach((line, row) => {
        let graphline = []
       
        line.split("").forEach((n, col) => {
            graphline.push(Number(n))
            let key = row + "," + col
            // visiting horizontally and vertically are different
            unvisited[key+",H"] = true // can use `"x,y" in unvisited` OR `unvisited["x,y"]`
            unvisited[key+",V"] = true // can use `"x,y" in unvisited` OR `unvisited["x,y"]`
        })
        graph.push(graphline)
    })

    delete unvisited["0,0,H"]
    delete unvisited["0,0,V"]

    let curr = {row: 0, col: 0, dist: 0, direction: ""}

    getNeighbors(curr, 3, graph).forEach((node) => {
        let nodekey = node.row + "," + node.col + "," + node.direction
        if (unvisited[nodekey]) {
            if (nodekey in tentative_bykey) {
                let prevdist = tentative_bykey[nodekey]
                if (prevdist < node.dist) {
                    tentative_bykey[nodekey] = node.dist

                    let idx = tentative_bydist[prevdist].indexOf(nodekey)
                    tentative_bydist[prevdist].splice(idx, 1)

                    if (node.dist in tentative_bydist) {
                        tentative_bydist[node.dist].push(nodekey)
                    } else {
                        tentative_bydist[node.dist] = [nodekey]
                    }   
                }
            } else {
                tentative_bykey[nodekey] = node.dist

                if (node.dist in tentative_bydist) {
                    tentative_bydist[node.dist].push(nodekey)
                } else {
                    tentative_bydist[node.dist] = [nodekey]
                }   
            }
        }
    })

    let currkey = curr.row + "," + curr.col + "," + curr.direction
    delete unvisited[currkey]

    // TODO: go through tentatives, find smallest distance one, set curr to it.
    // TODO: figure out the loop break-out conditions

    console.log("Part One")
});

function getNeighbors(curr, maxStraight, graph) {
    let neighbors = []
    if (curr.direction === "V" || curr.direction === "") {
        // add left neighbors
        let leftdist = curr.dist
        for (let col = curr.col - 1; col >= curr.col - maxStraight; col--) {
            if (col >= 0 && col < graph.length) {
                leftdist += graph[curr.row][col]
                neighbors.push({row: curr.row, col: col, direction: "H", dist: leftdist})
            }
        }
        // add right neighbors
        let rightdist = curr.dist
        for (let col = curr.col + 1; col <= curr.col + maxStraight; col++) {
            if (col >= 0 && col < graph.length) {
                rightdist += graph[curr.row][col]
                neighbors.push({row: curr.row, col: col, direction: "H", dist: rightdist})
            }
        }
    }

    if (curr.direction === "H" || curr.direction === "") {
        // add up neighbors
        let updist = curr.dist
        for (let row = curr.row - 1; row >= curr.row - maxStraight; row--) {
            if (row >=0 && row < graph.length) {
                updist += graph[row][curr.col]
                neighbors.push({row: row, col: curr.col, direction: "V", dist: updist})
            }
        }
        // add down neighbors
        let downdist = curr.dist
        for (let row = curr.row + 1; row <= curr.row + maxStraight; row++) {
            if (row >=0 && row < graph.length) {
                downdist += graph[row][curr.col]
                neighbors.push({row: row, col: curr.col, direction: "V", dist: downdist})
            }
        }
    }

    return neighbors
}
